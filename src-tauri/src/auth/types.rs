use serde::{Deserialize, Serialize};

use crate::schemas::UserPrivate;

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
