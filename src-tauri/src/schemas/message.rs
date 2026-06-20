use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Attachment {
    pub id: String,
    pub message_id: String,
    pub chat_id: String,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub storage_key: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Message {
    pub id: String,
    pub chat_id: String,
    pub author_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub attachments: Vec<Attachment>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum MessageType {
    Default,
    Reply { referenced_message_id: String },
    MemberAdd { user_id: String },
    MemberRemove { user_id: String },
    MemberLeave { user_id: String },
    ChatNameUpdate { new_name: String },
    ChatImageUpdate { new_image: String },
    ChatPinnedMessage,
    ChatUnpinMessage,
    ChatCreate { chat_name: String },
    Forward,
}
