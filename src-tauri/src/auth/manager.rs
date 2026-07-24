use chrono::Utc;
use tauri::{async_runtime::RwLock, AppHandle, Emitter, Manager};
use tokio::sync::Mutex;

use crate::api_client::{ApiClient, TokenProviderError};
use crate::auth::schemas::{LoginRequestSchema, RefreshRequestSchema, RegisterRequestSchema, TokenPairSchema};
use crate::auth::token_store::TokenStore;
use crate::auth::types::{AuthEventStatus, TokenPair};
use crate::errors::AppError;
use crate::schemas::UserPrivate;

pub struct AuthManager {
    app_handle: AppHandle,
    token_store: TokenStore,
    tokens: RwLock<Option<TokenPair>>,
    user: RwLock<Option<UserPrivate>>,
    refresh_lock: Mutex<()>,
}

impl AuthManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            token_store: TokenStore::new(app_handle.clone()),
            app_handle,
            tokens: RwLock::new(None),
            user: RwLock::new(None),
            refresh_lock: Mutex::new(()),
        }
    }

    pub async fn init(&self) {
        tracing::info!("Auth initialization started");
        self.emit(AuthEventStatus::Initializing);

        match self.token_store.load_tokens() {
            Ok(Some(tokens)) => {
                tracing::trace!(
                    access_token = %tokens.access_token.chars().take(15).collect::<String>(),
                    refresh_token = %tokens.refresh_token.chars().take(15).collect::<String>(),
                    expires_at = %tokens.expires_at,
                    "Session loaded from keystore"
                );

                *self.tokens.write().await = Some(tokens);

                match self.fetch_and_emit_user().await {
                    Ok(_) => {
                        tracing::info!("Session successfully restored");
                    },
                    Err(AppError::Network { message }) => {
                        tracing::warn!(%message, "Network error during init");
                        self.emit(AuthEventStatus::NetworkError);
                    },
                    Err(err) =>  {
                        tracing::error!(%err, "Error while fetching user");
                        self.invalidate_session("init_fetch_failed").await;
                    }
                }
            }
            Ok(None) => {
                tracing::info!("No session was found (unauthenticated)");
                self.emit(AuthEventStatus::Unauthenticated);
            }
            Err(err) => {
                tracing::warn!(%err, "Failed to load tokens from keystore");
                self.emit(AuthEventStatus::Unauthenticated);
            }
        }
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(), AppError> {
        tracing::info!(username, "Login attempt");

        let tokens_response = self
            .api()
            .raw_post::<TokenPairSchema, _>(
                "/auth/sign-in",
                &LoginRequestSchema {
                    username: username.into(),
                    password: password.into(),
                },
            )
            .await?;

        let tokens = TokenPair::from_response(tokens_response);

        match self.token_store.save_tokens(&tokens) {
            Ok(_) => {}
            Err(err) => {
                tracing::warn!(%err, "Failed to save tokens to keystore. Session will not be saved.");
            }
        }

        *self.tokens.write().await = Some(tokens);

        self.fetch_and_emit_user().await
    }

    pub async fn register(
        &self,
        username: &str,
        email: &str,
        global_name: &str,
        password: &str,
    ) -> Result<(), AppError> {
        tracing::info!(username, email, "Register attempt");

        self.api()
            .raw_post::<UserPrivate, _>(
                "/auth/sign-up",
                &RegisterRequestSchema {
                    username: username.into(),
                    email: email.into(),
                    global_name: global_name.into(),
                    password: password.into(),
                },
            )
            .await?;

        self.login(username, password).await
    }

    pub async fn logout(&self) -> Result<(), AppError> {
        tracing::info!("Logout initiated");
        self.invalidate_session("user_logout").await;
        Ok(())
    }
}

impl AuthManager {
    pub async fn get_access_token(&self, force_refresh: bool) -> Result<Option<String>, TokenProviderError> {
        let now = Utc::now();

        {
            let tokens = self.tokens.read().await;
            match &*tokens {
                Some(t) if t.expires_at >= now && !force_refresh => {
                    tracing::trace!(%now, expires_at = %t.expires_at, "No need for refresh");
                    return Ok(Some(t.access_token.clone()))
                },
                None => return Ok(None),
                Some(_) => { },
            }
        }

        let _guard = self.refresh_lock.lock().await;

        let old_refresh_token = {
            let tokens = self.tokens.read().await;
            match &*tokens {
                Some(t) if t.expires_at >= Utc::now() => {
                    tracing::trace!("Token already refreshed by another request");
                    return Ok(Some(t.access_token.clone()));
                }
                None => return Ok(None),
                Some(t) => t.refresh_token.clone(),
            }
        };

        tracing::info!("Token expired, refreshing");
        match self.refresh_token(&old_refresh_token).await {
            Ok(new_tokens) => {
                tracing::info!("Token successfully refreshed");

                {
                    let mut tokens = self.tokens.write().await;
                    *tokens = Some(new_tokens.clone());
                }

                match self.token_store.save_tokens(&new_tokens) {
                    Ok(_) => {}
                    Err(err) => {
                        tracing::error!(%err, "Failed to save tokens new refreshed tokens, session would not be saved");
                    }
                }

                Ok(Some(new_tokens.access_token))
            }
            Err(AppError::Network { message }) => {
                tracing::warn!(%message, "Network error during refresh");
                Err(TokenProviderError::Network { message })
            }
            Err(err) => {
                tracing::error!(%err, "Failed to refresh token");
                {
                    let mut tokens = self.tokens.write().await;
                    *tokens = None;
                }
                self.emit(AuthEventStatus::Unauthenticated);
                Ok(None)
            }
        }
    }


    async fn refresh_token(&self, refresh_token: &str) -> Result<TokenPair, AppError> {
        let new_tokens = self
            .api()
            .raw_post::<TokenPairSchema, _>(
                "/auth/refresh",
                &RefreshRequestSchema {
                    refresh_token: refresh_token.into(),
                },
            )
            .await?;

        Ok(TokenPair::from_response(new_tokens))
    }

    async fn fetch_and_emit_user(&self) -> Result<(), AppError> {
        tracing::trace!("Fetching userinfo");
        self.emit(AuthEventStatus::LoadingUser);

        let user = self.api().get::<UserPrivate>("/users/@me").await?;
        *self.user.write().await = Some(user.clone());

        tracing::info!(user_id = %user.id, username = %user.username, "User info fetched");
        self.emit(AuthEventStatus::Authenticated { user });

        Ok(())
    }

    fn api(&self) -> tauri::State<'_, ApiClient> {
        self.app_handle.state::<ApiClient>()
    }

    fn emit(&self, status: AuthEventStatus) {
        let _ = self
            .app_handle
            .emit("auth:status-changed", status);
    }

    async fn invalidate_session(&self, reason: &str) {
        tracing::info!(reason, "Invalidating user session.");
        self.token_store.delete_tokens().ok();
        *self.tokens.write().await = None;
        *self.user.write().await = None;
        self.emit(AuthEventStatus::Unauthenticated);
    }
}
