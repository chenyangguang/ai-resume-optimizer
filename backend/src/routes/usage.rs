use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::utils::auth::verify_jwt;

#[derive(serde::Serialize)]
pub struct UsageCheckResponse {
    pub can_use: bool,
    pub remaining: i32,
    pub usage_count: i32,
    pub usage_limit: i32,
}

pub async fn check_usage(
    State(pool): State<PgPool>,
    req: axum::extract::Request,
) -> Result<Json<UsageCheckResponse>, (StatusCode, String)> {
    // 从请求头获取 token
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing auth token".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid auth header".to_string()))?;

    // 验证 token
    let claims = verify_jwt(token)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    // 查询用户
    let user = sqlx::query!(
        "SELECT usage_count, usage_limit FROM users WHERE id = $1",
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let remaining = user.usage_limit - user.usage_count;
    let can_use = remaining > 0;

    Ok(Json(UsageCheckResponse {
        can_use,
        remaining,
        usage_count: user.usage_count,
        usage_limit: user.usage_limit,
    }))
}

pub async fn get_usage_stats(
    State(pool): State<PgPool>,
    req: axum::extract::Request,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // 从请求头获取 token
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing auth token".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid auth header".to_string()))?;

    // 验证 token
    let claims = verify_jwt(token)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    // 查询使用统计
    let stats = sqlx::query!(
        r#"
        SELECT 
            u.usage_count,
            u.usage_limit,
            u.subscription_tier,
            COUNT(ul.id) as total_actions
        FROM users u
        LEFT JOIN usage_logs ul ON u.id = ul.user_id
        WHERE u.id = $1
        GROUP BY u.id
        "#,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Stats not found".to_string()))?;

    Ok(Json(serde_json::json!({
        "usage_count": stats.usage_count,
        "usage_limit": stats.usage_limit,
        "remaining": stats.usage_limit - stats.usage_count,
        "tier": stats.subscription_tier,
        "total_actions": stats.total_actions.unwrap_or(0),
    })))
}
