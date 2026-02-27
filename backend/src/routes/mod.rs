pub mod resume;
pub mod auth;
pub mod usage;

use axum::{Json, extract::State};
use serde_json::{json, Value};
use crate::utils::AppState;

pub async fn health(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({
        "service": "ai-resume-optimizer",
        "status": "ok",
        "version": "0.1.0"
    }))
}
