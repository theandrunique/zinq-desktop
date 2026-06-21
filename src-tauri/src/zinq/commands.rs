use tauri::State;
use crate::zinq::manager::ZinqManager;

#[tauri::command]
pub async fn zinq_init(state: State<'_, ZinqManager>) -> Result<(), String> {
    tracing::info!("zinq_init command called");
    state.init().await.map_err(|e| e.to_string())
}
