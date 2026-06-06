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
pub struct TauriAppError {
    pub kind: ErrorKind,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_error: Option<ApiError>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorKind {
    Network,
    Api,
    Serialization,
    Unexpected,
}

impl From<ClientError> for TauriAppError {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::Network(_) => TauriAppError {
                kind: ErrorKind::Network,
                message: "No internet connection or server unavailable".into(),
                api_error: None,
            },
            ClientError::Api(e) => TauriAppError {
                kind: ErrorKind::Api,
                message: e.message.clone(),
                api_error: Some(e),
            },
            ClientError::Serialization(e) => TauriAppError {
                kind: ErrorKind::Serialization,
                message: format!("Failed to process server response: {}", e),
                api_error: None,
            },
            ClientError::UnexpectedStatus(status, _) => TauriAppError {
                kind: ErrorKind::Unexpected,
                message: format!("Server returned unexpected response ({})", status),
                api_error: None,
            },
        }
    }
}
