use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use crate::utils::AppState;
use crate::utils::auth::verify_jwt;

#[derive(serde::Serialize)]
pub struct UsageCheckResponse {
    pub can_use: bool,
    pub remaining: i32,
    pub usage_count: i32,
    pub usage_limit: i32,
}

pub async fn check_usage(
    State(app_state): State<AppState>,
) -> Result<Json<UsageCheckResponse>, (StatusCode, String)> {
    // 临时返回，后续实现
    Ok(Json(UsageCheckResponse {
        can_use: true,
        remaining: 100,
        usage_count: 0,
        usage_limit: 100,
    }))
}

pub async fn get_usage_stats(
    State(app_state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // 临时返回，后续实现
    Ok(Json(serde_json::json!({
        "usage_count": 0,
        "usage_limit": 100,
        "remaining": 100,
        "tier": "free",
    })))
}
