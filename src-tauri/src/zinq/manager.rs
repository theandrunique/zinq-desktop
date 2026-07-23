use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};

use crate::auth::manager::AuthManager;
use crate::db::repositories::sync_state;
use crate::errors::AppError;
use crate::schemas::EventLog;
use crate::zinq::{
    event_processor::EventProcessor,
    socket_client::SocketClient,
    sync_manager::SyncManager,
};

pub struct ZinqManager {
    pool: SqlitePool,
    app_handle: AppHandle,
}

impl ZinqManager {
    pub fn new(app_handle: AppHandle, pool: SqlitePool) -> Self {
        Self { pool, app_handle }
    }

    pub async fn init(&self) -> Result<(), AppError> {
        tracing::info!("Zinq init started");

        let last_event_id = sync_state::get_value(&self.pool, "last_event_id")
            .await?
            .unwrap_or_else(|| "0".to_string());

        let token = self
            .app_handle
            .state::<AuthManager>()
            .get_access_token()
            .await
            .ok_or_else(|| AppError::Internal {
                message: "No access token available".into(),
            })?;

        let (_socket_client, mut event_rx) =
            SocketClient::connect("http://localhost:8000", &token)
                .await?;

        let sync_manager = SyncManager::new(self.pool.clone(), self.app_handle.clone());

        let sync_result = if last_event_id == "0" {
            sync_manager.full_sync().await
        } else {
            let can_incremental = sync_manager
                .check_can_incremental_sync(&last_event_id)
                .await
                .unwrap_or(false);
            if can_incremental {
                sync_manager.incremental_sync(&last_event_id).await
            } else {
                sync_manager.full_sync().await
            }
        };

        if let Err(e) = sync_result {
            tracing::error!("Sync failed: {:?}", e);
        }

        drain_buffer(&self.pool, &mut event_rx).await;

        let pool = self.pool.clone();
        tauri::async_runtime::spawn(async move {
            live_event_loop(pool, event_rx).await;
        });

        tracing::info!("Zinq init completed");
        Ok(())
    }
}

async fn drain_buffer(pool: &SqlitePool, rx: &mut tokio::sync::mpsc::UnboundedReceiver<EventLog>) {
    let processor = EventProcessor::new(pool.clone());
    let mut count = 0;

    while let Ok(event) = rx.try_recv() {
        if let Err(e) = processor.process_event(&event).await {
            tracing::error!("Failed to process buffered event {}: {e:?}", event.event_id);
        }
        count += 1;
    }

    if count > 0 {
        tracing::info!("Processed {count} buffered events");
    }
}

async fn live_event_loop(pool: SqlitePool, mut rx: tokio::sync::mpsc::UnboundedReceiver<EventLog>) {
    let processor = EventProcessor::new(pool);

    while let Some(event) = rx.recv().await {
        if let Err(e) = processor.process_event(&event).await {
            tracing::error!("Failed to process live event {}: {e:?}", event.event_id);
        }
    }
}
