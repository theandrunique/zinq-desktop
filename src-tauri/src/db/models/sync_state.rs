#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SyncStateRow {
    pub key: String,
    pub value: String,
    pub updated_at: i64,
}
