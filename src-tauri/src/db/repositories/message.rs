use sqlx::SqlitePool;

use crate::db::models::{AttachmentRow, MessageRow};

pub async fn upsert_message(pool: &SqlitePool, msg: &MessageRow) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO messages (id, chat_id, author_id, content, created_at, edited_at, type)
         VALUES (?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
            content = excluded.content,
            edited_at = excluded.edited_at,
            type = excluded.type",
    )
    .bind(&msg.id)
    .bind(&msg.chat_id)
    .bind(&msg.author_id)
    .bind(&msg.content)
    .bind(&msg.created_at)
    .bind(&msg.edited_at)
    .bind(&msg.message_type)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_message(pool: &SqlitePool, message_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM attachments WHERE message_id = ?")
        .bind(message_id)
        .execute(pool)
        .await?;

    sqlx::query("DELETE FROM messages WHERE id = ?")
        .bind(message_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn upsert_attachment(pool: &SqlitePool, attachment: &AttachmentRow) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO attachments (id, message_id, chat_id, filename, content_type, size, storage_key, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
            filename = excluded.filename,
            content_type = excluded.content_type,
            size = excluded.size,
            storage_key = excluded.storage_key",
    )
    .bind(&attachment.id)
    .bind(&attachment.message_id)
    .bind(&attachment.chat_id)
    .bind(&attachment.filename)
    .bind(&attachment.content_type)
    .bind(&attachment.size)
    .bind(&attachment.storage_key)
    .bind(&attachment.created_at)
    .execute(pool)
    .await?;
    Ok(())
}
