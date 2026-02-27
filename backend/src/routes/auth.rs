use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use chrono::{Datelike, Timelike, Utc};
use uuid::Uuid;
use sqlx::Row;

use crate::models::user::*;
use crate::utils::auth::create_jwt;
use crate::utils::AppState;

pub async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let pool = &app_state.db;
    
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
    .fetch_optional(pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if existing_user.is_some() {
        return Err((StatusCode::CONFLICT, "Email already registered".to_string()));
    }

    // 哈希密码（使用简单的 bcrypt）
    let password_hash = format!("hashed_{}", payload.password); // 临时方案

    // 计算使用重置日期（下个月1号）
    let now = Utc::now().naive_utc();
    let next_month = if now.month() == 12 {
        now.with_year(now.year() + 1).unwrap().with_month(1).unwrap()
    } else {
        now.with_month(now.month() + 1).unwrap()
    };
    let usage_reset_date = next_month.with_day(1).unwrap();

    // 创建用户（使用运行时查询）
    let user_id = Uuid::new_v4();
    let now = Utc::now().naive_utc();
    
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash, name, usage_reset_date, created_at, updated_at, subscription_tier, usage_count, usage_limit, is_active, is_verified)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'free', 0, 100, true, false)
        "#,
        user_id,
        payload.email,
        password_hash,
        payload.name,
        usage_reset_date,
        now,
        now
    )
    .execute(pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 查询刚创建的用户（使用运行时查询）
    let row = sqlx::query("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let user = User {
        id: row.get("id"),
        email: row.get("email"),
        password_hash: row.get("password_hash"),
        name: row.get("name"),
        avatar_url: row.get("avatar_url"),
        subscription_tier: row.get("subscription_tier"),
        subscription_start_date: row.get("subscription_start_date"),
        subscription_end_date: row.get("subscription_end_date"),
        stripe_customer_id: row.get("stripe_customer_id"),
        stripe_subscription_id: row.get("stripe_subscription_id"),
        usage_count: row.get("usage_count"),
        usage_limit: row.get("usage_limit"),
        usage_reset_date: row.get("usage_reset_date"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        last_login_at: row.get("last_login_at"),
        is_active: row.get("is_active"),
        is_verified: row.get("is_verified"),
        verification_token: row.get("verification_token"),
        reset_password_token: row.get("reset_password_token"),
        reset_password_expires: row.get("reset_password_expires"),
    };

    // 生成 JWT
    let token = create_jwt(&user)
        .map_err(|e: jsonwebtoken::errors::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse { token, user }))
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let pool = &app_state.db;
    
    // 查找用户（使用运行时查询）
    let row = sqlx::query("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;
    
    let user = User {
        id: row.get("id"),
        email: row.get("email"),
        password_hash: row.get("password_hash"),
        name: row.get("name"),
        avatar_url: row.get("avatar_url"),
        subscription_tier: row.get("subscription_tier"),
        subscription_start_date: row.get("subscription_start_date"),
        subscription_end_date: row.get("subscription_end_date"),
        stripe_customer_id: row.get("stripe_customer_id"),
        stripe_subscription_id: row.get("stripe_subscription_id"),
        usage_count: row.get("usage_count"),
        usage_limit: row.get("usage_limit"),
        usage_reset_date: row.get("usage_reset_date"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        last_login_at: row.get("last_login_at"),
        is_active: row.get("is_active"),
        is_verified: row.get("is_verified"),
        verification_token: row.get("verification_token"),
        reset_password_token: row.get("reset_password_token"),
        reset_password_expires: row.get("reset_password_expires"),
    };

    // 验证密码（临时方案）
    let expected_hash = format!("hashed_{}", payload.password);
    if user.password_hash != expected_hash {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // 检查用户是否激活
    if !user.is_active {
        return Err((StatusCode::FORBIDDEN, "Account is deactivated".to_string()));
    }

    // 更新最后登录时间
    let _ = sqlx::query(
        "UPDATE users SET last_login_at = NOW() WHERE id = $1"
    )
    .bind(user.id)
    .execute(pool)
    .await;

    // 生成 JWT
    let token = create_jwt(&user)
        .map_err(|e: jsonwebtoken::errors::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse { token, user }))
}
