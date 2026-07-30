use tauri::State;
use crate::config::AppConfig;
use crate::AppState;

#[tauri::command]
pub fn get_config(state: State<AppState>) -> Result<AppConfig, String> {
    let config = state.config.read();
    Ok(config.clone())
}

#[tauri::command]
pub fn update_config(state: State<AppState>, config: AppConfig) -> Result<(), String> {
    config.save()?;
    *state.config.write() = config;
    Ok(())
}

#[tauri::command]
pub fn reset_config(state: State<AppState>) -> Result<(), String> {
    let default = AppConfig::default();
    default.save()?;
    *state.config.write() = default;
    Ok(())
}
