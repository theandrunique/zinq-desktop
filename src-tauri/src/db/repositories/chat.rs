use sqlx::SqlitePool;

use crate::db::models::{ChatMemberRow, ChatRow};

pub async fn upsert_chat(pool: &SqlitePool, chat: &ChatRow) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO chats (id, owner_id, name, description, image, last_message_id, permissions, type, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
            owner_id = COALESCE(excluded.owner_id, chats.owner_id),
            name = COALESCE(excluded.name, chats.name),
            description = COALESCE(excluded.description, chats.description),
            image = COALESCE(excluded.image, chats.image),
            last_message_id = COALESCE(excluded.last_message_id, chats.last_message_id),
            permissions = COALESCE(excluded.permissions, chats.permissions)",
    )
    .bind(&chat.id)
    .bind(&chat.owner_id)
    .bind(&chat.name)
    .bind(&chat.description)
    .bind(&chat.image)
    .bind(&chat.last_message_id)
    .bind(&chat.permissions)
    .bind(&chat.chat_type)
    .bind(&chat.created_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn replace_chat_members(
    pool: &SqlitePool,
    chat_id: &str,
    members: &[ChatMemberRow],
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM chat_members WHERE chat_id = ?")
        .bind(chat_id)
        .execute(pool)
        .await?;

    for member in members {
        sqlx::query(
            "INSERT INTO chat_members (chat_id, user_id, username, global_name, avatar, permissions)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&member.chat_id)
        .bind(&member.user_id)
        .bind(&member.username)
        .bind(&member.global_name)
        .bind(&member.avatar)
        .bind(&member.permissions)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn update_last_message_id(
    pool: &SqlitePool,
    chat_id: &str,
    last_message_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE chats SET last_message_id = ? WHERE id = ?")
        .bind(last_message_id)
        .bind(chat_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn insert_chat_member(
    pool: &SqlitePool,
    member: &ChatMemberRow,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO chat_members (chat_id, user_id, username, global_name, avatar, permissions)
         VALUES (?, ?, ?, ?, ?, ?)
         ON CONFLICT(chat_id, user_id) DO UPDATE SET
            username = excluded.username,
            global_name = excluded.global_name,
            avatar = COALESCE(excluded.avatar, chat_members.avatar),
            permissions = COALESCE(excluded.permissions, chat_members.permissions)",
    )
    .bind(&member.chat_id)
    .bind(&member.user_id)
    .bind(&member.username)
    .bind(&member.global_name)
    .bind(&member.avatar)
    .bind(&member.permissions)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_chat_member(
    pool: &SqlitePool,
    chat_id: &str,
    user_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM chat_members WHERE chat_id = ? AND user_id = ?")
        .bind(chat_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}
