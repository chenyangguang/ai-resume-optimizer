pub mod auth;

use sqlx::PgPool;
use redis::Client;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: Client,
}

impl AppState {
    pub fn new(db: PgPool, redis: Client) -> Self {
        Self { db, redis }
    }
}

pub fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "ai-resume-secret-key-2026".to_string())
}

pub use auth::*;
