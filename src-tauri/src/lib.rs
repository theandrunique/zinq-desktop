#![allow(dead_code)]

mod api_client;
mod zinq;
mod auth;
mod errors;
mod logging;
mod types;
mod schemas;
mod db;

use api_client::ApiClient;
use tauri::Manager;

use crate::auth::auth_manager::AuthManager;

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

            let _ = tauri::async_runtime::block_on(async {
                let pool = db::create_pool(&app_data_dir)
                    .await
                    .expect("Failed to create database pool");
                db::run_migrations(&pool)
                    .await
                    .expect("Failed to run migrations");
            });

            let api_client = ApiClient::new("http://localhost:8000".into());
            let auth_manager = AuthManager::new(app.handle().clone(), api_client);
            app.manage(auth_manager);

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
