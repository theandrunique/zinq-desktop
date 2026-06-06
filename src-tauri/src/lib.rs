mod api_client;
mod auth;
mod errors;
mod logging;

use api_client::ApiClient;
use tauri::Manager;

use crate::auth::auth_manager::AuthManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_keyring::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).ok();

            logging::init_logging(&app_data_dir);

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
