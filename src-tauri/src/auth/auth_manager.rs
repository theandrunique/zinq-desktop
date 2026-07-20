use tauri::{async_runtime::RwLock, AppHandle, Emitter, Manager};

use crate::api_client::ApiClient;
use crate::auth::schemas::{LoginRequestSchema, RefreshRequestSchema, RegisterRequestSchema, TokenPairSchema};
use crate::auth::token_store::TokenStore;
use crate::auth::types::{AuthEventPayload, AuthEventStatus};
use crate::errors::AppError;
use crate::schemas::UserPrivate;

pub struct AuthManager {
    app_handle: AppHandle,
    token_store: TokenStore,
    tokens: RwLock<Option<TokenPairSchema>>,
    user: RwLock<Option<UserPrivate>>,
}

impl AuthManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            token_store: TokenStore::new(app_handle.clone()),
            app_handle,
            tokens: RwLock::new(None),
            user: RwLock::new(None),
        }
    }

    pub fn get_access_token(&self) -> Option<String> {
        self.tokens
            .try_read()
            .ok()?
            .as_ref()
            .map(|t| t.access_token.clone())
    }

    pub async fn init(&self) {
        tracing::info!("Auth init started");
        self.emit(AuthEventStatus::Initializing, None);

        match self.token_store.load_refresh_token().await {
            Ok(Some(token)) => {
                self.emit(AuthEventStatus::Refreshing, None);

                match self.do_refresh(&token).await {
                    Ok(user) => self.emit(AuthEventStatus::Authenticated, Some(user)),
                    Err(_) => {
                        self.token_store.delete_tokens().await.ok();
                        *self.tokens.write().await = None;
                        *self.user.write().await = None;
                        self.emit(AuthEventStatus::Unauthenticated, None);
                    }
                }
            }
            _ => {
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

        let tokens = self
            .api()
            .post::<TokenPairSchema, _>(
                "/auth/sign-in",
                &LoginRequestSchema {
                    username: username.into(),
                    password: password.into(),
                },
            )
            .await?;

        self.token_store
            .save_refresh_token(&tokens.refresh_token)
            .await?;
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

        let tokens = self
            .api()
            .post::<TokenPairSchema, _>(
                "/auth/sign-up",
                &RegisterRequestSchema {
                    username: username.into(),
                    email: email.into(),
                    global_name: global_name.into(),
                    password: password.into(),
                },
            )
            .await?;

        self.token_store
            .save_refresh_token(&tokens.refresh_token)
            .await?;
        *self.tokens.write().await = Some(tokens);
        self.fetch_and_emit_user().await
    }

    pub async fn logout(&self) -> Result<(), AppError> {
        tracing::info!("Logout initiated");

        self.token_store.delete_tokens().await.ok();
        *self.tokens.write().await = None;
        *self.user.write().await = None;
        self.emit(AuthEventStatus::Unauthenticated, None);

        Ok(())
    }

    pub async fn refresh(&self) -> Result<(), AppError> {
        let refresh_token = self
            .tokens
            .read()
            .await
            .as_ref()
            .map(|t| t.refresh_token.clone())
            .ok_or_else(|| AppError::Internal {
                message: "No refresh token available".into(),
            })?;

        match self.do_refresh(&refresh_token).await {
            Ok(user) => {
                self.emit(AuthEventStatus::Authenticated, Some(user));
                Ok(())
            }
            Err(e) => {
                self.token_store.delete_tokens().await.ok();
                *self.tokens.write().await = None;
                *self.user.write().await = None;
                self.emit(AuthEventStatus::Unauthenticated, None);
                Err(e)
            }
        }
    }

    async fn do_refresh(&self, refresh_token: &str) -> Result<UserPrivate, AppError> {
        tracing::debug!("Performing token refresh");

        let new_tokens = self
            .api()
            .post::<TokenPairSchema, _>(
                "/auth/refresh",
                &RefreshRequestSchema {
                    refresh_token: refresh_token.into(),
                },
            )
            .await?;

        self.token_store
            .save_refresh_token(&new_tokens.refresh_token)
            .await?;
        *self.tokens.write().await = Some(new_tokens);

        self.emit(AuthEventStatus::LoadingUser, None);

        let user = self.api().get::<UserPrivate>("/users/@me").await?;
        *self.user.write().await = Some(user.clone());

        tracing::info!(user_id = %user.id, username = %user.username, "User loaded");

        Ok(user)
    }

    async fn fetch_and_emit_user(&self) -> Result<(), AppError> {
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
