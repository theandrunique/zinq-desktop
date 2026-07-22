use anyhow::{Context};
use tauri::AppHandle;
use tauri_plugin_keyring::KeyringExt;

use crate::auth::{schemas::TokenPairSchema, types::TokenPair};

const KEYRING_SERVICE: &str = "zinq";

const KEYRING_USER_ACCESS: &str = "access_token";
const KEYRING_USER_REFRESH: &str = "refresh_token";
const KEYRING_USER_EXPIRES_IN: &str = "expires_at";

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
            .set_password(KEYRING_SERVICE, KEYRING_USER_EXPIRES_IN, &tokens.expires_at.to_string())
            .with_context(|| "Failed to save expires in")?;

        Ok(())
    }

    pub fn load_tokens(&self) -> Result<Option<TokenPair>, anyhow::Error> {
        let access_token = self.app_handle
            .keyring()
            .get_password(KEYRING_SERVICE, KEYRING_USER_ACCESS)
            .with_context(|| "Failed to load token")?;

        let refresh_token = self.app_handle
            .keyring()
            .get_password(KEYRING_SERVICE, KEYRING_USER_REFRESH)
            .with_context(|| "Failed to load refresh token")?;

        let expires_at = self.app_handle
            .keyring()
            .get_password(KEYRING_SERVICE, KEYRING_USER_EXPIRES_IN)
            .with_context(|| "Failed to load expires in")?;

        if access_token.is_none() || refresh_token.is_none() || expires_at.is_none() {
            return Ok(None);
        }

        Ok(Some(TokenPair {
            access_token: access_token.unwrap(),
            refresh_token: refresh_token.unwrap(),
            expires_at: expires_at.unwrap().parse().expect("Failed to parse expires_at"),
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
            .delete_password(KEYRING_SERVICE, KEYRING_USER_EXPIRES_IN)
            .with_context(|| "Failed to delete expires at")?;

        Ok(())
    }
}
