use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};

use crate::api_client::ApiClient;
use crate::db::{
    models::{AttachmentRow, ChatMemberRow, ChatRow, MessageRow},
    repositories::{self, sync_state},
};
use crate::errors::AppError;
use crate::schemas::{Chat, EventLog, Message};

pub struct SyncManager {
    pool: SqlitePool,
    app_handle: AppHandle,
}

impl SyncManager {
    pub fn new(pool: SqlitePool, app_handle: AppHandle) -> Self {
        Self { pool, app_handle }
    }

    fn api(&self) -> tauri::State<'_, ApiClient> {
        self.app_handle.state::<ApiClient>()
    }

    pub async fn get_latest_event_id(&self) -> Result<String, AppError> {
        let events = self
            .api()
            .get::<Vec<EventLog>>("/sync?limit=1")
            .await?;

        events
            .first()
            .map(|e| e.event_id.clone())
            .ok_or_else(|| AppError::Internal {
                message: "No events returned from server".into(),
            })
    }

    pub async fn full_sync(&self) -> Result<(), AppError> {
        tracing::info!("Starting full sync...");

        let latest_id = self.get_latest_event_id().await?;
        sync_state::set(&self.pool, "last_event_id", &latest_id).await?;

        let chats = self.api().get::<Vec<Chat>>("/users/@me/chats").await?;

        for chat in &chats {
            let chat_row = ChatRow::from(chat);
            repositories::chat::upsert_chat(&self.pool, &chat_row).await?;

            let members: Vec<ChatMemberRow> = chat
                .members
                .iter()
                .map(|m| ChatMemberRow::from_member(&chat.id, m))
                .collect();
            repositories::chat::replace_chat_members(&self.pool, &chat.id, &members).await?;

            if let Some(ref last_msg_id) = chat.last_message_id {
                let endpoint = format!(
                    "/chats/{}/messages?before={}&limit=50",
                    chat.id, last_msg_id
                );
                let messages = self.api().get::<Vec<Message>>(&endpoint).await?;

                for msg in &messages {
                    let msg_row = MessageRow::from(msg);
                    repositories::message::upsert_message(&self.pool, &msg_row).await?;

                    for attachment in &msg.attachments {
                        let att_row = AttachmentRow::from(attachment);
                        repositories::message::upsert_attachment(&self.pool, &att_row).await?;
                    }
                }
            }
        }

        tracing::info!("Full sync completed");
        Ok(())
    }

    pub async fn incremental_sync(&self, last_event_id: &str) -> Result<(), AppError> {
        tracing::info!("Starting incremental sync from {}", last_event_id);

        let endpoint = format!("/sync?after={}&limit=100", last_event_id);
        let events = self.api().get::<Vec<EventLog>>(&endpoint).await?;

        for event in &events {
            sync_state::set(&self.pool, "last_event_id", &event.event_id).await?;
        }

        tracing::info!("Incremental sync processed {} events", events.len());
        Ok(())
    }

    pub async fn check_can_incremental_sync(
        &self,
        last_event_id: &str,
    ) -> Result<bool, AppError> {
        if last_event_id.is_empty() {
            return Ok(false);
        }

        let Some(before) = decrement_snowflake(last_event_id) else {
            return Err(AppError::Internal {
                message: format!("Invalid snowflake ID: {}", last_event_id),
            });
        };

        let endpoint = format!("/sync?after={}&limit=1", before);
        let result = self.api().get::<Vec<EventLog>>(&endpoint).await?;

        Ok(result.first().is_some_and(|e| e.event_id == last_event_id))
    }
}

fn decrement_snowflake(id: &str) -> Option<String> {
    let n: i64 = id.parse().ok()?;
    Some((n - 1).to_string())
}
