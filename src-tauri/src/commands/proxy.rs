use serde::{Deserialize, Serialize};
use tauri::State;
use crate::system::proxy as sys_proxy;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyStatus {
    pub running: bool,
    pub system_proxy: bool,
    pub tun_mode: bool,
    pub port: u16,
    pub api_port: u16,
}

#[tauri::command]
pub fn get_proxy_status(state: State<AppState>) -> Result<ProxyStatus, String> {
    let config = state.config.read();
    let running = state.core_manager.is_running();
    Ok(ProxyStatus {
        running,
        system_proxy: config.system_proxy,
        tun_mode: config.tun_mode,
        port: config.mixed_port,
        api_port: config.api_port,
    })
}

#[tauri::command]
pub fn set_system_proxy(state: State<AppState>, enable: bool) -> Result<(), String> {
    let config = state.config.read();
    let port = config.mixed_port;
    drop(config);

    if enable {
        sys_proxy::set_system_proxy("127.0.0.1", port)?;
    } else {
        sys_proxy::unset_system_proxy()?;
    }

    state.config.write().system_proxy = enable;
    state.config.read().save()
}

#[tauri::command]
pub fn set_tun_mode(state: State<AppState>, enable: bool) -> Result<(), String> {
    state.config.write().tun_mode = enable;
    state.config.read().save()
}
