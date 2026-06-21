use std::sync::Arc;

use sqlx::SqlitePool;
use tauri::AppHandle;

use crate::{
    api_client::ApiClient,
    db::{self, repositories::sync_state},
    errors::TauriAppError,
};

pub struct ZinqManager {
    api_client: Arc<ApiClient>,
    pool: SqlitePool,
    app_handle: AppHandle,
}

impl ZinqManager {
    pub fn new(app_handle: AppHandle, pool: SqlitePool, api_client: Arc<ApiClient>) -> Self {
        Self {
            api_client,
            pool,
            app_handle,
        }
    }

    pub async fn init(&self) -> Result<(), anyhow::Error> {
        tracing::info!("Initializing zinq manager...");

        let value = sync_state::get_value(&self.pool, "last_event_id")
            .await
            .map_err(anyhow::Error::from)?
            .unwrap_or("0".to_string());

        if value == "0" {
            tracing::info!("Performing full sync...");
        } else {
            tracing::info!("Performing incremental sync...");
        }

        Ok(())
    }
}
