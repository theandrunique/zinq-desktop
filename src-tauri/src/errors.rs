use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api_client::ClientError;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    AuthInvalidCredentials,
    AuthInvalidToken,
    AuthTotpRequired,
    InvalidJson,
    InvalidRequestBody,
    UsernameAlreadyInUse,
    UserNotFound,
    UsersNotFound,
    UserNotMember,
    ChatNotFound,
    AttachmentInvalidUploadFilename,
    AttachmentObjectNotFound,
    AttachmentInUse,
    ChatTypeNotSupported,
    InsufficientPermissions,
    UserAlreadyMember,
    EmailAlreadyInUse,
    MessageNotFound,
    MessageWasSentByAnotherUser,
    InternalServerError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppError {
    Network { message: String },
    Api { #[serde(flatten)] error: ApiError },
    Internal { message: String },
}

impl From<ClientError> for AppError {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::Network(e) => {
                tracing::warn!(error = ?e, "Network error");
                AppError::Network {
                    message: "No internet connection or server unavailable".into(),
                }
            }
            ClientError::Api(e) => AppError::Api { error: e },
            ClientError::Serialization(e) => {
                tracing::warn!(error = ?e, "Serialization error");
                AppError::Internal {
                    message: format!("Failed to process server response: {}", e),
                }
            }
            ClientError::UnexpectedStatus(status, body) => {
                tracing::warn!(status = %status, body = %body, "Unexpected status");
                AppError::Internal {
                    message: format!("Server returned unexpected response: {}", status),
                }
            }
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        tracing::error!("Internal error: {value:#}");
        AppError::Internal {
            message: value.to_string(),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        tracing::error!(error = ?value, "Database error");
        AppError::Internal {
            message: "A database error occurred".into(),
        }
    }
}
