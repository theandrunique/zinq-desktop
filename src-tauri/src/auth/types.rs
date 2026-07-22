use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{auth::schemas::TokenPairSchema, schemas::UserPrivate};

#[derive(Clone, PartialEq)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

impl TokenPair {
    pub fn from_response(response: TokenPairSchema) -> Self {
        let gap = Duration::seconds(response.expires_in / 10);
        let lifetime = Duration::seconds(response.expires_in) - gap;

        let expires_at = Utc::now() + lifetime;

        tracing::trace!(expires_in = %&response.expires_in, calculated_expires_at = %expires_at, "Token lifetime");

        Self {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_at,
        }
    }
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
