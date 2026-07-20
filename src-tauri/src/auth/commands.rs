use tauri::State;

use super::auth_manager::AuthManager;
use crate::errors::AppError;

#[tauri::command]
pub async fn auth_init(state: State<'_, AuthManager>) -> Result<(), AppError> {
    tracing::trace!("auth_init command called");
    state.init().await;
    Ok(())
}

#[tauri::command]
pub async fn auth_login(
    state: State<'_, AuthManager>,
    username: String,
    password: String,
) -> Result<(), AppError> {
    tracing::trace!("auth_login command called");
    state.login(&username, &password).await
}

#[tauri::command]
pub async fn auth_register(
    state: State<'_, AuthManager>,
    username: String,
    email: String,
    global_name: String,
    password: String,
) -> Result<(), AppError> {
    tracing::trace!("auth_register command called");
    state
        .register(&username, &email, &global_name, &password)
        .await
}

#[tauri::command]
pub async fn auth_logout(state: State<'_, AuthManager>) -> Result<(), AppError> {
    tracing::trace!("auth_logout command called");
    state.logout().await
}
