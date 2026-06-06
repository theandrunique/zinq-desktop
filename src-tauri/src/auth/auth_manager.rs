use std::sync::Arc;

use tauri::{AppHandle, Emitter, async_runtime::RwLock};
use tauri_plugin_keyring::KeyringExt;

use crate::api_client::{ApiClient, ClientError};
use crate::auth::types::*;
use crate::errors::{ErrorKind, TauriAppError};

const KEYRING_SERVICE: &str = "zinq";
const KEYRING_USER: &str = "auth";

pub struct AuthManager {
    app_handle: AppHandle,
    api_client: ApiClient,
    user: Arc<RwLock<Option<User>>>,
    tokens: Arc<RwLock<Option<TokenPair>>>,
}

impl AuthManager {
    pub fn new(app_handle: AppHandle, mut api_client: ApiClient) -> Arc<Self> {
        let tokens: Arc<RwLock<Option<TokenPair>>> = Arc::new(RwLock::new(None));

        let tokens_for_provider = tokens.clone();
        api_client.set_token_provider(move || {
            tokens_for_provider
                .try_read()
                .ok()
                .and_then(|t| t.as_ref().map(|t| t.access_token.clone()))
        });

        Arc::new(Self {
            app_handle,
            api_client,
            user: Arc::new(RwLock::new(None)),
            tokens,
        })
    }

    pub async fn init(self: &Arc<Self>) {
        self.emit_status(AuthEventPayload {
            status: "initializing".into(),
            user: None,
        });

        let this = Arc::clone(self);
        tauri::async_runtime::spawn(async move {
            match this.load_tokens().await {
                Ok(Some(_)) => {
                    this.emit_status(AuthEventPayload {
                        status: "refreshing".into(),
                        user: None,
                    });

                    match this.do_refresh().await {
                        Ok(user) => {
                            this.emit_status(AuthEventPayload {
                                status: "authenticated".into(),
                                user: Some(user),
                            });
                        }
                        Err(_) => {
                            let _ = this.delete_tokens().await;
                            this.emit_status(AuthEventPayload {
                                status: "unauthenticated".into(),
                                user: None,
                            });
                        }
                    }
                }
                Ok(None) | Err(_) => {
                    this.emit_status(AuthEventPayload {
                        status: "unauthenticated".into(),
                        user: None,
                    });
                }
            }
        });
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<(), TauriAppError> {
        let body = LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        let tokens = self
            .api_client
            .post::<TokenPair, _>("/auth/sign-in", &body)
            .await?;

        self.save_tokens(&tokens).await.map_err(|e| TauriAppError {
            kind: ErrorKind::Unexpected,
            message: e,
            api_error: None,
        })?;

        self.fetch_and_emit_user().await
    }

    pub async fn register(
        &self,
        username: &str,
        email: &str,
        global_name: &str,
        password: &str,
    ) -> Result<(), TauriAppError> {
        let body = RegisterRequest {
            username: username.to_string(),
            email: email.to_string(),
            global_name: global_name.to_string(),
            password: password.to_string(),
        };

        let tokens = self
            .api_client
            .post::<TokenPair, _>("/auth/sign-up", &body)
            .await?;

        self.save_tokens(&tokens).await.map_err(|e| TauriAppError {
            kind: ErrorKind::Unexpected,
            message: e,
            api_error: None,
        })?;

        self.fetch_and_emit_user().await
    }

    pub async fn logout(&self) -> Result<(), String> {
        self.delete_tokens().await?;

        self.emit_status(AuthEventPayload {
            status: "unauthenticated".into(),
            user: None,
        });

        Ok(())
    }

    async fn fetch_and_emit_user(&self) -> Result<(), TauriAppError> {
        self.emit_status(AuthEventPayload {
            status: "loading_user".into(),
            user: None,
        });

        let user = self
            .api_client
            .get::<User>("/users/@me")
            .await?;
        *self.user.write().await = Some(user.clone());

        self.emit_status(AuthEventPayload {
            status: "authenticated".into(),
            user: Some(user),
        });

        Ok(())
    }

    async fn do_refresh(&self) -> Result<User, ClientError> {
        let refresh_token = self
            .tokens
            .read()
            .await
            .as_ref()
            .ok_or_else(|| {
                ClientError::UnexpectedStatus(
                    reqwest::StatusCode::UNAUTHORIZED,
                    "No refresh token available".into(),
                )
            })?
            .refresh_token
            .clone();

        let body = RefreshRequest { refresh_token };
        let new_tokens = self
            .api_client
            .post::<TokenPair, _>("/auth/refresh", &body)
            .await?;

        self.save_tokens(&new_tokens).await.map_err(|e| {
            ClientError::UnexpectedStatus(
                reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                e,
            )
        })?;

        let user = self
            .api_client
            .get::<User>("/users/@me")
            .await?;
        *self.user.write().await = Some(user.clone());
        Ok(user)
    }

    async fn save_tokens(&self, tokens: &TokenPair) -> Result<(), String> {
        self.app_handle
            .keyring()
            .set_password(KEYRING_SERVICE, KEYRING_USER, &tokens.refresh_token)
            .map_err(|e| e.to_string())?;
        *self.tokens.write().await = Some(tokens.clone());
        Ok(())
    }

    async fn load_tokens(&self) -> Result<Option<TokenPair>, String> {
        match self
            .app_handle
            .keyring()
            .get_password(KEYRING_SERVICE, KEYRING_USER)
        {
            Ok(Some(refresh_token)) => {
                let tokens = TokenPair {
                    access_token: String::new(),
                    refresh_token,
                    expires_in: 0,
                };
                *self.tokens.write().await = Some(tokens.clone());
                Ok(Some(tokens))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn delete_tokens(&self) -> Result<(), String> {
        self.app_handle
            .keyring()
            .delete_password(KEYRING_SERVICE, KEYRING_USER)
            .map_err(|e| e.to_string())?;
        *self.tokens.write().await = None;
        *self.user.write().await = None;
        Ok(())
    }

    fn emit_status(&self, payload: AuthEventPayload) {
        let _ = self.app_handle.emit("auth:status-changed", payload);
    }
}
