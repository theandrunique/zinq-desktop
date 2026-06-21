use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChatType {
    Dm,
    GroupDm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMember {
    pub user_id: String,
    pub username: String,
    pub global_name: String,
    pub avatar: Option<String>,
    pub permissions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: String,
    pub owner_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    pub last_message_id: Option<String>,
    pub permissions: String,
    pub created_at: DateTime<Utc>,
    pub members: Vec<ChatMember>,
}
