use serde::Serialize;
use tauri::{State, AppHandle, Emitter};
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub is_default: bool,
}

#[tauri::command]
pub async fn download_core(_state: State<'_, AppState>, _channel: Option<String>) -> Result<String, String> {
    Ok("meow-rs SDK embedded".into())
}

#[tauri::command]
pub async fn check_core_installed() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "versions": ["0.19.0"],
        "default": "0.19.0",
        "hasCore": true,
        "corePath": "meow-rs-embedded",
    }))
}

#[tauri::command]
pub async fn list_core_versions() -> Result<Vec<VersionInfo>, String> {
    Ok(vec![VersionInfo {
        version: "0.19.0".into(),
        is_default: true,
    }])
}

#[tauri::command]
pub async fn install_core_version(_version: String) -> Result<String, String> {
    Ok("meow-rs SDK embedded".into())
}

#[tauri::command]
pub async fn uninstall_core_version(_version: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn set_core_default_version(_version: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn get_core_default_version() -> Result<String, String> {
    Ok("0.19.0".into())
}

#[derive(Debug, Serialize, Clone)]
pub struct CoreInstallProgress {
    pub status: String,
    pub progress: f64,
    pub message: String,
}

#[tauri::command]
pub async fn install_core_with_progress(app: AppHandle, _state: State<'_, AppState>) -> Result<(), String> {
    let _ = app.emit("core-install-status", CoreInstallProgress {
        status: "done".into(),
        progress: 1.0,
        message: "Core is embedded (meow-rs SDK)".into(),
    });
    Ok(())
}
