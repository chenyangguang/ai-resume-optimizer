use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use chrono::Datelike;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;
use crate::utils::auth::verify_jwt;

/// 检查用户使用次数的中间件
pub async fn check_usage_limit(
    State(pool): State<PgPool>,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
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
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::UNAUTHORIZED, "User not found".to_string()))?;

    // 检查是否需要重置使用次数（新的一月）
    let now = chrono::Utc::now().naive_utc();
    if user.usage_reset_date <= now {
        // 重置使用次数
        let next_reset = now
            .with_day(1)
            .unwrap()
            .checked_add_months(chrono::Months::new(1))
            .unwrap();

        sqlx::query!(
            "UPDATE users SET usage_count = 0, usage_reset_date = $1 WHERE id = $2",
            next_reset,
            user.id
        )
        .execute(&pool)
        .await
        .ok();
    } else {
        // 检查使用次数是否超限
        if user.usage_count >= user.usage_limit {
            return Err((
                StatusCode::FORBIDDEN,
                format!("Usage limit exceeded. Limit: {}, Used: {}", user.usage_limit, user.usage_count)
            ));
        }
    }

    // 继续处理请求
    Ok(next.run(req).await)
}

/// 记录使用次数
pub async fn record_usage(
    pool: &PgPool,
    user_id: Uuid,
    action: &str,
) -> Result<(), sqlx::Error> {
    // 增加使用次数
    sqlx::query!(
        "UPDATE users SET usage_count = usage_count + 1 WHERE id = $1",
        user_id
    )
    .execute(pool)
    .await?;

    // 记录使用日志
    sqlx::query!(
        "INSERT INTO usage_logs (user_id, action) VALUES ($1, $2)",
        user_id,
        action
    )
    .execute(pool)
    .await?;

    Ok(())
}
