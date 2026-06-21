use sqlx::SqlitePool;
use tauri::AppHandle;

use crate::{errors::TauriAppError, schemas::{EventLog, EventLogType}};

pub struct EventProcessor {
    pool: SqlitePool,
    app_handle: AppHandle,
}

impl EventProcessor {
    pub fn new(pool: SqlitePool, app_handle: AppHandle) -> Self {
        Self {
            pool,
            app_handle,
        }
    }

    pub async fn process_event(&self, event: &EventLog) -> Result<(), TauriAppError> {
        tracing::info!("Processing event: {:?}", event);

        match &event.event_type {
            EventLogType::MessageCreate { message } => {

            }
            EventLogType::MessageUpdate { message } => {

            }
            EventLogType::MessageDelete { message_id } => {

            }
            EventLogType::ChatCreate { chat } => {

            }
            EventLogType::ChatMemberAdd { chat_id, member } => {

            }
            EventLogType::ChatMemberRemove { chat_id, member } => {

            }
        }
        Ok(())
    }
}
