use chrono::Utc;
use tauri::{async_runtime::RwLock, AppHandle, Emitter, Manager};
use tokio::sync::Mutex;

use crate::api_client::ApiClient;
use crate::auth::schemas::{LoginRequestSchema, RefreshRequestSchema, RegisterRequestSchema, TokenPairSchema};
use crate::auth::token_store::TokenStore;
use crate::auth::types::{AuthEventPayload, AuthEventStatus, TokenPair};
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

    pub async fn get_access_token(&self) -> Option<String> {
        let now = Utc::now();

        let needs_refresh = {
            let tokens = self.tokens.read().await;
            match &*tokens {
                Some(t) if t.expires_at >= now => {
                    tracing::trace!(%now, expires_at = %t.expires_at, "No need for refresh");
                    return Some(t.access_token.clone())
                },
                Some(_) => true,
                None => false,
            }
        };

        if !needs_refresh {
            return None;
        }

        tracing::info!("Token expired, refreshing");
        let _guard = self.refresh_lock.lock().await;

        let old_refresh_token = {
            let tokens = self.tokens.read().await;
            match &*tokens {
                Some(t) if t.expires_at >= Utc::now() => {
                    tracing::trace!("Token already refreshed by another request");
                    return Some(t.access_token.clone());
                }
                Some(t) => t.refresh_token.clone(),
                None => return None,
            }
        };

        match self.refresh_token(&old_refresh_token).await {
            Ok(new_tokens) => {
                tracing::info!("Token successfully refreshed");

                {
                    let mut tokens = self.tokens.write().await;
                    *tokens = Some(new_tokens.clone());
                }

                self.token_store.save_tokens(&new_tokens)
                    .expect("Failed to save tokens");

                Some(new_tokens.access_token)
            }
            Err(err) => {
                tracing::error!("Failed to refresh token: {:?}", err);
                {
                    let mut tokens = self.tokens.write().await;
                    *tokens = None;
                }
                self.emit(AuthEventStatus::Unauthenticated, None);
                None
            }
        }
    }

    pub async fn init(&self) {
        tracing::info!("Auth init started");
        self.emit(AuthEventStatus::Initializing, None);

        match self.token_store.load_tokens() {
            Ok(Some(tokens)) => {
                let access_preview: String = tokens.access_token.chars().take(15).collect();
                let refresh_preview: String = tokens.refresh_token.chars().take(15).collect();
                tracing::trace!(access_token = %access_preview, refresh_token = %refresh_preview, expires_at = %tokens.expires_at, "Session found");
                self.emit(AuthEventStatus::Refreshing, None);

                *self.tokens.write().await = Some(tokens);
                match self.fetch_and_emit_user().await {
                    Ok(_) => {
                        tracing::info!("The session was successfully restored");
                    },
                    Err(err) =>  {
                        tracing::error!("Error while fetching user: {:?}", err);
                        self.emit(AuthEventStatus::Unauthenticated, None);
                    }
                }
            }
            _ => {
                tracing::trace!("Session not found");
                self.emit(AuthEventStatus::Unauthenticated, None);
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

        self.token_store.save_tokens(&tokens)?;

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

        self.token_store.delete_tokens()?;
        *self.tokens.write().await = None;
        *self.user.write().await = None;
        self.emit(AuthEventStatus::Unauthenticated, None);

        Ok(())
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
        self.emit(AuthEventStatus::LoadingUser, None);

        let user = self.api().get::<UserPrivate>("/users/@me").await?;
        *self.user.write().await = Some(user.clone());

        tracing::info!(user_id = %user.id, username = %user.username, "User loaded");
        self.emit(AuthEventStatus::Authenticated, Some(user));

        Ok(())
    }

    fn api(&self) -> tauri::State<'_, ApiClient> {
        self.app_handle.state::<ApiClient>()
    }

    fn emit(&self, status: AuthEventStatus, user: Option<UserPrivate>) {
        let _ = self
            .app_handle
            .emit("auth:status-changed", AuthEventPayload { status, user });
    }
}
