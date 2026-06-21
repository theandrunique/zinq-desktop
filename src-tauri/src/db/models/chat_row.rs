use chrono::{DateTime, Utc};

use crate::schemas::{Chat, ChatMember};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ChatRow {
    pub id: String,
    pub owner_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub last_message_id: Option<String>,
    pub permissions: Option<String>,
    pub chat_type: String,
    pub created_at: DateTime<Utc>,
}

impl From<&Chat> for ChatRow {
    fn from(chat: &Chat) -> Self {
        Self {
            id: chat.id.clone(),
            owner_id: chat.owner_id.clone(),
            name: chat.name.clone(),
            description: chat.description.clone(),
            image: chat.image.clone(),
            last_message_id: chat.last_message_id.clone(),
            permissions: Some(chat.permissions.clone()),
            chat_type: match chat.chat_type {
                crate::schemas::ChatType::Dm => "DM".into(),
                crate::schemas::ChatType::GroupDm => "GROUP_DM".into(),
            },
            created_at: chat.created_at,
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ChatMemberRow {
    pub chat_id: String,
    pub user_id: String,
    pub username: String,
    pub global_name: String,
    pub avatar: Option<String>,
    pub permissions: Option<String>,
}

impl ChatMemberRow {
    pub fn from_member(chat_id: &str, member: &ChatMember) -> Self {
        Self {
            chat_id: chat_id.to_string(),
            user_id: member.user_id.clone(),
            username: member.username.clone(),
            global_name: member.global_name.clone(),
            avatar: member.avatar.clone(),
            permissions: member.permissions.clone(),
        }
    }
}
