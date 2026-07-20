use tauri::State;

use crate::errors::AppError;
use crate::zinq::manager::ZinqManager;

#[tauri::command]
pub async fn zinq_init(state: State<'_, ZinqManager>) -> Result<(), AppError> {
    tracing::trace!("zinq_init command called");
    state.init().await
}
