use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use argon2::{self, Config, ThreadMode, Variant, Version};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::*;
use crate::utils::auth::create_jwt;

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // 验证邮箱格式
    if !payload.email.contains('@') {
        return Err((StatusCode::BAD_REQUEST, "Invalid email format".to_string()));
    }

    // 验证密码长度
    if payload.password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, "Password must be at least 8 characters".to_string()));
    }

    // 检查邮箱是否已存在
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if existing_user.is_some() {
        return Err((StatusCode::CONFLICT, "Email already registered".to_string()));
    }

    // 哈希密码
    let password_hash = hash_password(&payload.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 计算使用重置日期（下个月1号）
    let now = Utc::now();
    let usage_reset_date = now
        .with_day(1)
        .unwrap()
        .checked_add_months(chrono::Months::new(1))
        .unwrap()
        .naive_utc();

    // 创建用户
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password_hash, name, usage_reset_date)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        payload.email,
        password_hash,
        payload.name,
        usage_reset_date
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 生成 JWT
    let token = create_jwt(&user)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse { token, user }))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // 查找用户
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    // 验证密码
    if !verify_password(&payload.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // 检查用户是否激活
    if !user.is_active {
        return Err((StatusCode::FORBIDDEN, "Account is deactivated".to_string()));
    }

    // 更新最后登录时间
    sqlx::query!(
        "UPDATE users SET last_login_at = NOW() WHERE id = $1",
        user.id
    )
    .execute(&pool)
    .await
    .ok();

    // 生成 JWT
    let token = create_jwt(&user)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse { token, user }))
}

// 辅助函数
fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 3,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        secret: None,
        ad: None,
        hash_length: None,
    };

    let salt = Uuid::new_v4().to_string();
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password.as_bytes())
}
