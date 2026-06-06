use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
