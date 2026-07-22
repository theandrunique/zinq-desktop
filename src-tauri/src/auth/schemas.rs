use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct TokenPairSchema {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize)]
pub struct LoginRequestSchema {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterRequestSchema {
    pub username: String,
    pub email: String,
    pub global_name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshRequestSchema {
    pub refresh_token: String,
}
