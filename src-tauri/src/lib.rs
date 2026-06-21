mod api_client;
mod auth;
mod db;
mod errors;
mod logging;
mod schemas;
mod types;
mod zinq;

use std::sync::Arc;

use api_client::ApiClient;
use tauri::Manager;

use crate::auth::auth_manager::AuthManager;
use crate::zinq::manager::ZinqManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_keyring::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).ok();

            logging::init_logging(&app_data_dir);

            let pool = tauri::async_runtime::block_on(async {
                let pool = db::create_pool(&app_data_dir)
                    .await
                    .expect("Failed to create database pool");
                db::run_migrations(&pool)
                    .await
                    .expect("Failed to run migrations");
                pool
            });

            let api_client = Arc::new(ApiClient::new("http://localhost:8000".into()));
            let auth_manager = AuthManager::new(app.handle().clone(), api_client.clone());
            let zinq_manager = ZinqManager::new(app.handle().clone(), pool.clone(), api_client);
            app.manage(auth_manager);
            app.manage(zinq_manager);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            auth::commands::auth_init,
            auth::commands::auth_login,
            auth::commands::auth_register,
            auth::commands::auth_logout,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
