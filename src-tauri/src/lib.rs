pub mod commands;
pub mod config;
pub mod core;
pub mod system;

use config::AppConfig;
use core::manager::CoreManager;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct AppState {
    pub config: Arc<RwLock<AppConfig>>,
    pub proxy_running: Arc<RwLock<bool>>,
    pub core_manager: CoreManager,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = AppConfig::load();
    let app_state = AppState {
        config: Arc::new(RwLock::new(config)),
        proxy_running: Arc::new(RwLock::new(false)),
        core_manager: CoreManager::new(),
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
            commands::core_cmd::start_core,
            commands::core_cmd::stop_core,
            commands::core_cmd::restart_core,
            commands::core_cmd::get_core_status,
            commands::proxy::get_proxy_status,
            commands::proxy::set_system_proxy,
            commands::proxy::set_tun_mode,
            commands::system::get_version,
            commands::system::get_system_info,
            commands::system::open_app_dir,
            commands::system::select_config_file,
            commands::traffic::get_traffic,
            commands::logs::get_logs,
            commands::proxies::get_proxies,
            commands::proxies::select_proxy,
            commands::proxies::test_delay,
            commands::connections::get_connections,
            commands::connections::close_connection,
            commands::connections::close_all_connections,
            commands::rules::get_rules,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
