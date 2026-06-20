use serde::{Deserialize, Serialize};

use crate::schemas::UserPrivate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
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
    pub status: AuthEventStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserPrivate>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthEventStatus {
    Initializing,
    Refreshing,
    LoadingUser,
    Authenticated,
    Unauthenticated,
}
