use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::schemas::{
    chat::{Chat, ChatMember},
    message::Message,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventLog {
    pub event_id: String,
    #[serde(rename = "type")]
    pub event_type: EventLogType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum EventLogType {
    MessageCreate { message: Message },
    MessageUpdate { message: Message },
    MessageDelete { message_id: String },
    ChatCreate { chat: Chat },
    ChatMemberAdd { chat_id: String, member: ChatMember },
    ChatMemberRemove { chat_id: String, member: ChatMember },
}
