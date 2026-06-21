mod models;
pub mod repositories;

pub use models::SyncStateRow;
use std::{path::PathBuf, str::FromStr};

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    ConnectOptions, SqlitePool,
};

pub async fn create_pool(app_dir: &PathBuf) -> Result<SqlitePool, sqlx::Error> {
    std::fs::create_dir_all(app_dir).ok();

    let db_path = app_dir.join("zinq.db");
    let db_url = format!("sqlite://{}", db_path.display());

    tracing::info!("Connecting to database at {}", db_url);

    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .busy_timeout(std::time::Duration::from_secs(30))
        .log_statements(tracing::log::LevelFilter::Trace);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    tracing::info!("Running migrations...");

    sqlx::migrate!("src/db/migrations").run(pool).await?;

    tracing::info!("Migrations completed successfully");

    Ok(())
}
