use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub subscription_tier: String,
    pub subscription_start_date: Option<NaiveDateTime>,
    pub subscription_end_date: Option<NaiveDateTime>,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub usage_count: i32,
    pub usage_limit: i32,
    pub usage_reset_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_login_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub reset_password_token: Option<String>,
    pub reset_password_expires: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // User ID
    pub email: String,
    pub tier: String,
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}
