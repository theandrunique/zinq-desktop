use sqlx::SqlitePool;

use crate::{
    db::{
        models::{AttachmentRow, ChatMemberRow, ChatRow, MessageRow},
        repositories,
    },
    schemas::{EventLog, EventLogType},
};

pub struct EventProcessor {
    pool: SqlitePool,
}

impl EventProcessor {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn process_event(&self, event: &EventLog) -> Result<(), sqlx::Error> {
        tracing::debug!("Processing event: {}", event.event_id);

        match &event.event_type {
            EventLogType::MessageCreate { message } => {
                let msg_row = MessageRow::from(message);
                repositories::message::upsert_message(&self.pool, &msg_row).await?;

                for attachment in &message.attachments {
                    let att_row = AttachmentRow::from(attachment);
                    repositories::message::upsert_attachment(&self.pool, &att_row).await?;
                }

                repositories::chat::update_last_message_id(
                    &self.pool,
                    &message.chat_id,
                    &message.id,
                )
                .await?;
            }
            EventLogType::MessageUpdate { message } => {
                let msg_row = MessageRow::from(message);
                repositories::message::upsert_message(&self.pool, &msg_row).await?;
            }
            EventLogType::MessageDelete { message_id } => {
                repositories::message::delete_message(&self.pool, message_id).await?;
            }
            EventLogType::ChatCreate { chat } => {
                let chat_row = ChatRow::from(chat);
                repositories::chat::upsert_chat(&self.pool, &chat_row).await?;

                let members: Vec<ChatMemberRow> = chat
                    .members
                    .iter()
                    .map(|m| ChatMemberRow::from_member(&chat.id, m))
                    .collect();
                repositories::chat::replace_chat_members(&self.pool, &chat.id, &members).await?;
            }
            EventLogType::ChatMemberAdd { chat_id, member } => {
                let member_row = ChatMemberRow::from_member(chat_id, member);
                repositories::chat::insert_chat_member(&self.pool, &member_row).await?;
            }
            EventLogType::ChatMemberRemove { chat_id, member } => {
                repositories::chat::delete_chat_member(&self.pool, chat_id, &member.user_id).await?;
            }
        }

        Ok(())
    }
}
