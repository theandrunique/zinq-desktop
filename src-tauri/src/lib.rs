mod api_client;
mod auth;
mod db;
mod errors;
mod logging;
mod schemas;
mod zinq;

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

            let api_client = ApiClient::new("http://localhost:8000".into());
            let auth = AuthManager::new(app.handle().clone());

            api_client.set_token_provider({
                let h = app.handle().clone();
                move || h.state::<AuthManager>().get_access_token()
            });

            api_client.set_refresh_provider({
                let h = app.handle().clone();
                move || {
                    let h = h.clone();
                    Box::pin(async move {
                        h.state::<AuthManager>().refresh().await.is_ok()
                    })
                }
            });

            let zinq = ZinqManager::new(app.handle().clone(), pool);

            app.manage(api_client);
            app.manage(auth);
            app.manage(zinq);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            auth::commands::auth_init,
            auth::commands::auth_login,
            auth::commands::auth_register,
            auth::commands::auth_logout,
            zinq::commands::zinq_init,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
