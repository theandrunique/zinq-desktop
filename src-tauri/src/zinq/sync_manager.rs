use std::sync::Arc;

use sqlx::SqlitePool;
use tauri::AppHandle;

use crate::{
    api_client::ApiClient,
    errors::{ErrorKind, TauriAppError},
    schemas::EventLog,
};

pub struct SyncManager {
    pool: SqlitePool,
    api_client: Arc<ApiClient>,
    app_handle: AppHandle,
}

impl SyncManager {
    pub fn new(pool: SqlitePool, api_client: Arc<ApiClient>, app_handle: AppHandle) -> Self {
        Self {
            pool,
            api_client,
            app_handle,
        }
    }

    pub async fn full_sync(&self) -> Result<(), anyhow::Error> {
        tracing::info!("Starting full sync...");
        Ok(())
    }

    pub async fn incremental_sync(&self) -> Result<(), anyhow::Error> {
        tracing::info!("Starting incremental sync...");
        Ok(())
    }

    pub async fn check_can_incremental_sync(
        &self,
        last_event_id: &str,
    ) -> Result<bool, TauriAppError> {
        if last_event_id.is_empty() {
            return Ok(false);
        }

        let Some(before) = decrement_snowflake(last_event_id) else {
            return Err(TauriAppError {
                kind: ErrorKind::Unexpected,
                message: format!("Invalid snowflake ID: {}", last_event_id),
                api_error: None,
            });
        };

        let endpoint = format!("/sync?after={}&limit=1", before);
        let result = self
            .api_client
            .get::<Vec<EventLog>>(&endpoint)
            .await?;

        Ok(result.first().map_or(false, |e| e.event_id == last_event_id))
    }
}

fn decrement_snowflake(id: &str) -> Option<String> {
    let n: i64 = id.parse().ok()?;
    Some((n - 1).to_string())
}
