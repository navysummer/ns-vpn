pub mod commands;
pub mod config;
pub mod proxy;
pub mod system;

use config::AppConfig;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct AppState {
    pub config: Arc<RwLock<AppConfig>>,
    pub proxy_running: Arc<RwLock<bool>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        config: Arc::new(RwLock::new(AppConfig::default())),
        proxy_running: Arc::new(RwLock::new(false)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--flag1"]),
        ))
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::config::get_config,
            commands::config::update_config,
            commands::config::reset_config,
            commands::proxy::get_proxy_status,
            commands::proxy::set_system_proxy,
            commands::proxy::set_tun_mode,
            commands::proxy::restart_core,
            commands::proxy::stop_core,
            commands::system::get_version,
            commands::system::get_system_info,
            commands::system::open_app_dir,
            commands::system::select_config_file,
            commands::traffic::get_traffic,
            commands::logs::get_logs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}