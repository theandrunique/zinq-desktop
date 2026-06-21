use std::sync::Arc;

use sqlx::SqlitePool;
use tauri::AppHandle;

use crate::{
    api_client::ApiClient,
    db::{
        models::{AttachmentRow, ChatMemberRow, ChatRow, MessageRow},
        repositories::{self, sync_state},
    },
    errors::{ErrorKind, TauriAppError},
    schemas::{Chat, EventLog, Message},
};

pub struct SyncManager {
    pool: SqlitePool,
    api_client: Arc<ApiClient>,
    _app_handle: AppHandle,
}

impl SyncManager {
    pub fn new(pool: SqlitePool, api_client: Arc<ApiClient>, app_handle: AppHandle) -> Self {
        Self {
            pool,
            api_client,
            _app_handle: app_handle,
        }
    }

    pub async fn get_latest_event_id(&self) -> Result<String, TauriAppError> {
        let events = self
            .api_client
            .get::<Vec<EventLog>>("/sync?limit=1")
            .await?;

        events
            .first()
            .map(|e| e.event_id.clone())
            .ok_or_else(|| TauriAppError {
                kind: ErrorKind::Unexpected,
                message: "No events returned from server".into(),
                api_error: None,
            })
    }

    pub async fn full_sync(&self) -> Result<(), TauriAppError> {
        tracing::info!("Starting full sync...");

        let latest_id = self.get_latest_event_id().await?;
        sync_state::set(&self.pool, "last_event_id", &latest_id)
            .await
            .map_err(|e| TauriAppError {
                kind: ErrorKind::Unexpected,
                message: format!("Failed to save last_event_id: {}", e),
                api_error: None,
            })?;

        let chats = self
            .api_client
            .get::<Vec<Chat>>("/users/@me/chats")
            .await?;

        for chat in &chats {
            let chat_row = ChatRow::from(chat);
            repositories::chat::upsert_chat(&self.pool, &chat_row)
                .await
                .map_err(|e| TauriAppError {
                    kind: ErrorKind::Unexpected,
                    message: format!("Failed to upsert chat {}: {}", chat.id, e),
                    api_error: None,
                })?;

            let members: Vec<ChatMemberRow> = chat
                .members
                .iter()
                .map(|m| ChatMemberRow::from_member(&chat.id, m))
                .collect();
            repositories::chat::replace_chat_members(&self.pool, &chat.id, &members)
                .await
                .map_err(|e| TauriAppError {
                    kind: ErrorKind::Unexpected,
                    message: format!("Failed to replace members for chat {}: {}", chat.id, e),
                    api_error: None,
                })?;

            if let Some(ref last_msg_id) = chat.last_message_id {
                let endpoint = format!(
                    "/chats/{}/messages?before={}&limit=50",
                    chat.id, last_msg_id
                );
                let messages = self
                    .api_client
                    .get::<Vec<Message>>(&endpoint)
                    .await?;

                for msg in &messages {
                    let msg_row = MessageRow::from(msg);
                    repositories::message::upsert_message(&self.pool, &msg_row)
                        .await
                        .map_err(|e| TauriAppError {
                            kind: ErrorKind::Unexpected,
                            message: format!("Failed to upsert message {}: {}", msg.id, e),
                            api_error: None,
                        })?;

                    for attachment in &msg.attachments {
                        let att_row = AttachmentRow::from(attachment);
                        repositories::message::upsert_attachment(&self.pool, &att_row)
                            .await
                            .map_err(|e| TauriAppError {
                                kind: ErrorKind::Unexpected,
                                message: format!("Failed to upsert attachment {}: {}", attachment.id, e),
                                api_error: None,
                            })?;
                    }
                }
            }
        }

        tracing::info!("Full sync completed");
        Ok(())
    }

    pub async fn incremental_sync(&self, last_event_id: &str) -> Result<(), TauriAppError> {
        tracing::info!("Starting incremental sync from {}", last_event_id);

        let endpoint = format!("/sync?after={}&limit=100", last_event_id);
        let events = self
            .api_client
            .get::<Vec<EventLog>>(&endpoint)
            .await?;

        for event in &events {
            sync_state::set(&self.pool, "last_event_id", &event.event_id)
                .await
                .map_err(|e| TauriAppError {
                    kind: ErrorKind::Unexpected,
                    message: format!("Failed to save last_event_id: {}", e),
                    api_error: None,
                })?;
        }

        tracing::info!("Incremental sync processed {} events", events.len());
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
