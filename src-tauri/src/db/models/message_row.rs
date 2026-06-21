use chrono::{DateTime, Utc};

use crate::schemas::{Attachment, Message};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MessageRow {
    pub id: String,
    pub chat_id: String,
    pub author_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
    pub message_type: String,
}

impl From<&Message> for MessageRow {
    fn from(msg: &Message) -> Self {
        Self {
            id: msg.id.clone(),
            chat_id: msg.chat_id.clone(),
            author_id: msg.author_id.clone(),
            content: msg.content.clone(),
            created_at: msg.created_at,
            edited_at: msg.edited_at,
            message_type: serde_json::to_string(&msg.message_type).unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AttachmentRow {
    pub id: String,
    pub message_id: String,
    pub chat_id: String,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub storage_key: String,
    pub created_at: DateTime<Utc>,
}

impl From<&Attachment> for AttachmentRow {
    fn from(a: &Attachment) -> Self {
        Self {
            id: a.id.clone(),
            message_id: a.message_id.clone(),
            chat_id: a.chat_id.clone(),
            filename: a.filename.clone(),
            content_type: a.content_type.clone(),
            size: a.size,
            storage_key: a.storage_key.clone(),
            created_at: a.created_at,
        }
    }
}
