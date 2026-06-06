use std::sync::Arc;

use tauri::{async_runtime::RwLock, AppHandle, Emitter};
use tauri_plugin_keyring::KeyringExt;

use crate::api_client::{ApiClient, ClientError};
use crate::auth::types::{AuthEventPayload, AuthEventStatus, LoginRequest, RefreshRequest, RegisterRequest, TokenPair};
use crate::errors::{ErrorKind, TauriAppError};
use crate::types::User;

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

        use std::sync::Weak;
        let manager: Arc<Self> = Arc::new_cyclic(|weak_self: &Weak<Self>| {
            let weak = weak_self.clone();
            api_client.set_refresh_provider(move || {
                let weak = weak.clone();
                Box::pin(async move {
                    if let Some(m) = weak.upgrade() {
                        if let Err(e) = m.refresh_session().await {
                            tracing::error!("Auto-refresh failed: {:?}", e);
                            return false;
                        }
                        tracing::info!("Auto-refresh succeeded");
                        return true;
                    }
                    false
                })
            });

            Self {
                app_handle,
                api_client,
                user: Arc::new(RwLock::new(None)),
                tokens,
            }
        });

        manager
    }

    pub async fn init(self: &Arc<Self>) {
        tracing::info!("Auth init started");
        self.emit_status(AuthEventPayload {
            status: AuthEventStatus::Initializing,
            user: None,
        });

        let this = Arc::clone(self);
        tauri::async_runtime::spawn(async move {
            match this.load_tokens().await {
                Ok(Some(refresh_token)) => {
                    tracing::info!("Refresh token found, attempting refresh");
                    this.emit_status(AuthEventPayload {
                        status: AuthEventStatus::Refreshing,
                        user: None,
                    });

                    match this.do_refresh(&refresh_token).await {
                        Ok(user) => {
                            tracing::info!("Init refresh succeeded");
                            this.emit_status(AuthEventPayload {
                                status: AuthEventStatus::Authenticated,
                                user: Some(user),
                            });
                        }
                        Err(_) => {
                            tracing::warn!("Init refresh failed, clearing tokens");
                            let _ = this.delete_tokens().await;
                            this.emit_status(AuthEventPayload {
                                status: AuthEventStatus::Unauthenticated,
                                user: None,
                            });
                        }
                    }
                }
                Ok(None) | Err(_) => {
                    tracing::info!("No refresh token found");
                    this.emit_status(AuthEventPayload {
                        status: AuthEventStatus::Unauthenticated,
                        user: None,
                    });
                }
            }
        });
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<(), TauriAppError> {
        tracing::info!(username, "Login attempt");

        let body = LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        let tokens = self
            .api_client
            .post::<TokenPair, _>("/auth/sign-in", &body)
            .await?;

        self.save_tokens(&tokens).await?;

        self.fetch_and_emit_user().await
    }

    pub async fn register(
        &self,
        username: &str,
        email: &str,
        global_name: &str,
        password: &str,
    ) -> Result<(), TauriAppError> {
        tracing::info!(username, email, "Register attempt");

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

        self.save_tokens(&tokens).await?;

        self.fetch_and_emit_user().await
    }

    pub async fn logout(&self) -> Result<(), TauriAppError> {
        tracing::info!("Logout initiated");

        if let Err(e) = self.delete_tokens().await {
            tracing::warn!("Failed to delete tokens during logout: {}", e.message);
        }

        self.emit_status(AuthEventPayload {
            status: AuthEventStatus::Unauthenticated,
            user: None,
        });

        Ok(())
    }

    /// Called by the ApiClient's auto-refresh mechanism on 401/AuthInvalidToken.
    /// On failure, emits "unauthenticated" and clears tokens.
    /// On success, emits "authenticated" with user.
    async fn refresh_session(&self) -> Result<(), TauriAppError> {
        let refresh_token = match self
            .tokens
            .read()
            .await
            .as_ref()
            .map(|t| t.refresh_token.clone())
        {
            Some(token) => token,
            None => {
                tracing::warn!("Refresh requested but no refresh token available");
                return Err(TauriAppError {
                    kind: ErrorKind::Unexpected,
                    message: "No refresh token available".into(),
                    api_error: None,
                });
            }
        };

        match self.do_refresh(&refresh_token).await {
            Ok(user) => {
                tracing::info!("Session refreshed successfully");
                self.emit_status(AuthEventPayload {
                    status: AuthEventStatus::Authenticated,
                    user: Some(user),
                });
                Ok(())
            }
            Err(e) => {
                tracing::error!("Session refresh failed: {:?}", e);
                let _ = self.delete_tokens().await;
                self.emit_status(AuthEventPayload {
                    status: AuthEventStatus::Unauthenticated,
                    user: None,
                });
                Err(TauriAppError {
                    kind: ErrorKind::Unexpected,
                    message: "Session expired, please log in again".into(),
                    api_error: None,
                })
            }
        }
    }

    async fn fetch_and_emit_user(&self) -> Result<(), TauriAppError> {
        self.emit_status(AuthEventPayload {
            status: AuthEventStatus::LoadingUser,
            user: None,
        });

        let user = self.api_client.get::<User>("/users/@me").await?;
        *self.user.write().await = Some(user.clone());

        tracing::info!(user_id = %user.id, username = %user.username, "User loaded");
        self.emit_status(AuthEventPayload {
            status: AuthEventStatus::Authenticated,
            user: Some(user),
        });

        Ok(())
    }

    async fn do_refresh(&self, refresh_token: &str) -> Result<User, ClientError> {
        tracing::debug!("Performing token refresh");

        let body = RefreshRequest {
            refresh_token: refresh_token.to_string(),
        };

        let new_tokens = self
            .api_client
            .post::<TokenPair, _>("/auth/refresh", &body)
            .await?;

        self.save_tokens(&new_tokens).await.map_err(|e| {
            ClientError::UnexpectedStatus(reqwest::StatusCode::INTERNAL_SERVER_ERROR, e.message)
        })?;

        let user = self.api_client.get::<User>("/users/@me").await?;
        *self.user.write().await = Some(user.clone());

        tracing::info!("Token refresh completed");
        Ok(user)
    }

    async fn save_tokens(&self, tokens: &TokenPair) -> Result<(), TauriAppError> {
        self.app_handle
            .keyring()
            .set_password(KEYRING_SERVICE, KEYRING_USER, &tokens.refresh_token)
            .map_err(|e| TauriAppError {
                kind: ErrorKind::Unexpected,
                message: format!("Failed to save credentials: {}", e),
                api_error: None,
            })?;

        *self.tokens.write().await = Some(tokens.clone());
        tracing::debug!("Tokens saved to keyring");
        Ok(())
    }

    async fn load_tokens(&self) -> Result<Option<String>, TauriAppError> {
        match self
            .app_handle
            .keyring()
            .get_password(KEYRING_SERVICE, KEYRING_USER)
        {
            Ok(Some(refresh_token)) => {
                tracing::debug!("Refresh token loaded from keyring");
                Ok(Some(refresh_token))
            }
            Ok(None) => {
                tracing::debug!("No refresh token in keyring");
                Ok(None)
            }
            Err(e) => {
                tracing::warn!("Failed to read keyring: {}", e);
                Err(TauriAppError {
                    kind: ErrorKind::Unexpected,
                    message: format!("Failed to read stored credentials: {}", e),
                    api_error: None,
                })
            }
        }
    }

    async fn delete_tokens(&self) -> Result<(), TauriAppError> {
        let _ = self
            .app_handle
            .keyring()
            .delete_password(KEYRING_SERVICE, KEYRING_USER);

        *self.tokens.write().await = None;
        *self.user.write().await = None;
        tracing::info!("Tokens cleared");
        Ok(())
    }

    fn emit_status(&self, payload: AuthEventPayload) {
        let _ = self.app_handle.emit("auth:status-changed", payload);
    }
}
