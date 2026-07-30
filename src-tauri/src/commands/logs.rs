use serde::Serialize;
use tauri::State;
use crate::AppState;

#[derive(Debug, Serialize, Clone)]
pub struct LogEntry {
    pub time: String,
    pub level: String,
    pub payload: String,
    #[serde(rename = "type")]
    pub log_type: String,
}

#[tauri::command]
pub async fn get_logs(state: State<'_, AppState>) -> Result<Vec<LogEntry>, String> {
    if let Some(client) = state.core_manager.client() {
        match client.get_logs().await {
            Ok(entries) => {
                return Ok(entries.into_iter().map(|e| LogEntry {
                    time: e.time,
                    level: e.log_type.clone(),
                    payload: e.payload,
                    log_type: e.log_type,
                }).collect());
            }
            Err(_) => {}
        }
    }
    Ok(Vec::new())
}
