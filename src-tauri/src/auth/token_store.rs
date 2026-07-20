use anyhow::{Context};
use tauri::AppHandle;
use tauri_plugin_keyring::KeyringExt;

const KEYRING_SERVICE: &str = "zinq";
const KEYRING_USER: &str = "auth";

pub struct TokenStore {
    app_handle: AppHandle,
}

impl TokenStore {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    pub async fn save_refresh_token(&self, token: &str) -> Result<(), anyhow::Error> {
        self.app_handle
            .keyring()
            .set_password(KEYRING_SERVICE, KEYRING_USER, token)
            .with_context(|| "Failed to save token")
    }

    pub async fn load_refresh_token(&self) -> Result<Option<String>, anyhow::Error> {
        self.app_handle
            .keyring()
            .get_password(KEYRING_SERVICE, KEYRING_USER)
            .with_context(|| "Failed to load token")
    }

    pub async fn delete_tokens(&self) -> Result<(), anyhow::Error> {
        self.app_handle
            .keyring()
            .delete_password(KEYRING_SERVICE, KEYRING_USER)
            .with_context(|| "Failed to delete token")
    }
}
