use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChatType {
    Dm,
    GroupDm,
}

#[derive(Serialize)]
pub struct ChatMember {
    pub user_id: String,
    pub username: String,
    pub global_name: String,
    pub avatar: Option<String>,
    pub permissions: Option<String>,
}

#[derive(Serialize)]
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
