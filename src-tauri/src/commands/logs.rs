use serde::Serialize;
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
pub async fn get_logs(_state: tauri::State<'_, AppState>) -> Result<Vec<LogEntry>, String> {
    Ok(crate::get_log_history()
        .into_iter()
        .map(|e| LogEntry {
            time: e.time,
            level: e.level.clone(),
            payload: e.payload,
            log_type: e.level,
        })
        .collect())
}
