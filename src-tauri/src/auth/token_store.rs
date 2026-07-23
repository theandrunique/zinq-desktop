use anyhow::{Context};
use chrono::{DateTime, Utc};
use tauri::AppHandle;
use tauri_plugin_keyring::KeyringExt;

use crate::auth::{schemas::TokenPairSchema, types::TokenPair};

const KEYRING_SERVICE: &str = "zinq_desktop";

const KEYRING_USER_ACCESS: &str = "access_token";
const KEYRING_USER_REFRESH: &str = "refresh_token";
const KEYRING_USER_EXPIRES_AT: &str = "expires_at";

pub struct TokenStore {
    app_handle: AppHandle,
}

impl TokenStore {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn save_tokens(&self, tokens: &TokenPair) -> Result<(), anyhow::Error> {
        self.app_handle
            .keyring()
            .set_password(KEYRING_SERVICE, KEYRING_USER_ACCESS, &tokens.access_token)
            .with_context(|| "Failed to save token")?;

        self.app_handle
            .keyring()
            .set_password(KEYRING_SERVICE, KEYRING_USER_REFRESH, &tokens.refresh_token)
            .with_context(|| "Failed to save refresh token")?;

        self.app_handle
            .keyring()
            .set_password(KEYRING_SERVICE, KEYRING_USER_EXPIRES_AT, &tokens.expires_at.to_string())
            .with_context(|| "Failed to save expires in")?;

        Ok(())
    }

    pub fn load_tokens(&self) -> Result<Option<TokenPair>, anyhow::Error> {
        let access_token = match self.app_handle.keyring().get_password(KEYRING_SERVICE, KEYRING_USER_ACCESS)? {
            Some(t) => t,
            None => return Ok(None),
        };

        let refresh_token = match self.app_handle.keyring().get_password(KEYRING_SERVICE, KEYRING_USER_REFRESH)? {
            Some(t) => t,
            None => {
                tracing::warn!("Corrupted keyring: access token exists, but refresh token is missing. Clearing.");
                self.delete_tokens().ok();
                return Ok(None);
            },
        };

        let expires_at_str = match self.app_handle.keyring().get_password(KEYRING_SERVICE, KEYRING_USER_EXPIRES_AT)? {
            Some(t) => t,
            None => {
                tracing::warn!("Corrupted keyring: expires_at is missing. Clearing.");
                self.delete_tokens().ok();
                return Ok(None);
            },
        };

        let expires_at = match expires_at_str.parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            Err(error) => {
                tracing::warn!(%error, raw_value = %expires_at_str, "Error parsing expires at. Clearing corrupted tokens.");
                self.delete_tokens().ok();
                return Ok(None);
            },
        };

        tracing::trace!("Successfully loaded tokens from keyring");
        Ok(Some(TokenPair {
            access_token: access_token,
            refresh_token: refresh_token,
            expires_at: expires_at,
        }))
    }

    pub fn delete_tokens(&self) -> Result<(), anyhow::Error> {
        self.app_handle
            .keyring()
            .delete_password(KEYRING_SERVICE, KEYRING_USER_ACCESS)
            .with_context(|| "Failed to delete token")?;

        self.app_handle
            .keyring()
            .delete_password(KEYRING_SERVICE, KEYRING_USER_REFRESH)
            .with_context(|| "Failed to delete refresh token")?;

        self.app_handle
            .keyring()
            .delete_password(KEYRING_SERVICE, KEYRING_USER_EXPIRES_AT)
            .with_context(|| "Failed to delete expires at")?;

        Ok(())
    }
}
