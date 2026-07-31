use tauri::State;
use crate::AppState;

#[tauri::command]
pub async fn start_core(state: State<'_, AppState>) -> Result<(), String> {
    let config = state.config.read().clone();
    state.core_manager.start(&config).await?;
    *state.proxy_running.write() = true;
    // Persist proxy_running = true to config
    {
        let mut cfg = state.config.write();
        cfg.proxy_running = true;
        let _ = cfg.save();
    }
    Ok(())
}

#[tauri::command]
pub async fn stop_core(state: State<'_, AppState>) -> Result<(), String> {
    state.core_manager.stop().await?;
    *state.proxy_running.write() = false;
    // Persist proxy_running = false to config
    {
        let mut cfg = state.config.write();
        cfg.proxy_running = false;
        let _ = cfg.save();
    }
    Ok(())
}

#[tauri::command]
pub async fn auto_start_core(state: State<'_, AppState>) -> Result<bool, String> {
    let should_start = state.config.read().proxy_running;
    if should_start && !state.core_manager.is_running() {
        let config = state.config.read().clone();
        state.core_manager.set_skip_write(true);
        state.core_manager.start(&config).await?;
        *state.proxy_running.write() = true;
        log::info!("Auto-started core on app launch");
        return Ok(true);
    }
    Ok(should_start)
}

#[tauri::command]
pub async fn restart_core(state: State<'_, AppState>) -> Result<(), String> {
    let config = state.config.read().clone();
    state.core_manager.restart(&config).await?;
    *state.proxy_running.write() = true;
    Ok(())
}

#[tauri::command]
pub fn get_core_status(state: State<AppState>) -> Result<CoreStatus, String> {
    let running = state.core_manager.is_running();
    let config = state.config.read();
    Ok(CoreStatus {
        running,
        api_port: config.api_port,
        mixed_port: config.mixed_port,
    })
}

#[derive(serde::Serialize)]
pub struct CoreStatus {
    pub running: bool,
    pub api_port: u16,
    pub mixed_port: u16,
}
