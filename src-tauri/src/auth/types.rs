use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionLifetime {
    Week,
    Month,
    Month3,
    Month6,
    Month12,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub global_name: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub sessions_lifetime: SessionLifetime,
    pub mfa: bool,
    pub email: String,
    pub is_email_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub global_name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthEventPayload {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
