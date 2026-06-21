use sqlx::SqlitePool;

use crate::db::models::SyncStateRow;

pub async fn get(pool: &SqlitePool, key: &str) -> Result<Option<SyncStateRow>, sqlx::Error> {
    sqlx::query_as::<_, SyncStateRow>("SELECT key, value, updated_at FROM sync_state WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await
}

pub async fn get_value(pool: &SqlitePool, key: &str) -> Result<Option<String>, sqlx::Error> {
    let row = get(pool, key).await?;
    Ok(row.map(|r| r.value))
}

pub async fn set(pool: &SqlitePool, key: &str, value: &str) -> Result<(), sqlx::Error> {
    let updated_at = chrono::Utc::now().timestamp();
    sqlx::query(
        "INSERT INTO sync_state (key, value, updated_at)
         VALUES (?, ?, ?)
         ON CONFLICT(key) DO UPDATE SET
            value = excluded.value,
            updated_at = excluded.updated_at",
    )
    .bind(key)
    .bind(value)
    .bind(updated_at)
    .execute(pool)
    .await?;
    Ok(())
}
