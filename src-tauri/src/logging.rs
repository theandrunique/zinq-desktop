use std::{path::Path, sync::OnceLock};

use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

pub fn init_logging(app_data_dir: &Path) {
    let logs_dir = app_data_dir.join("logs");
    std::fs::create_dir_all(&logs_dir).ok();

    let file_appender = RollingFileAppender::new(Rotation::DAILY, &logs_dir, "zinq");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    LOG_GUARD.set(guard).ok();

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true);

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_target(true);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("trace,reqwest=warn,hyper_util=warn,tokio_tungstenite=warn,tungstenite=warn,keyring=warn"));

    if tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(stdout_layer)
        .try_init()
        .is_err()
    {
        eprintln!("Failed to initialize tracing subscriber");
    } else {
        tracing::info!(logs_dir = %logs_dir.display(), "Logging initialized");
    }
}
