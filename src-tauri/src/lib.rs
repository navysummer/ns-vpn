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

use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::OnceLock;

#[derive(Clone)]
pub struct LogEntry {
    pub time: String,
    pub level: String,
    pub payload: String,
}

static LOG_HISTORY: OnceLock<Mutex<VecDeque<LogEntry>>> = OnceLock::new();
const LOG_HISTORY_CAP: usize = 500;

fn record_log(level: &str, payload: String) {
    if let Some(history) = LOG_HISTORY.get() {
        let mut history = history.lock().unwrap();
        history.push_back(LogEntry {
            time: chrono::Utc::now().to_rfc3339(),
            level: level.to_string(),
            payload,
        });
        while history.len() > LOG_HISTORY_CAP {
            history.pop_front();
        }
    }
}

fn get_log_history() -> Vec<LogEntry> {
    LOG_HISTORY
        .get()
        .map(|h| h.lock().unwrap().iter().cloned().collect())
        .unwrap_or_default()
}

struct HistoryLayer;

impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for HistoryLayer {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let level = match *event.metadata().level() {
            tracing::Level::TRACE | tracing::Level::DEBUG => "debug",
            tracing::Level::INFO => "info",
            tracing::Level::WARN => "warning",
            tracing::Level::ERROR => "error",
        };
        let mut visitor = PayloadVisitor(String::new());
        event.record(&mut visitor);
        record_log(level, visitor.0);
    }
}

struct PayloadVisitor(String);

impl tracing::field::Visit for PayloadVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.0 = format!("{value:?}");
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.0 = value.to_string();
        }
    }
}

static LOG_TX: OnceLock<tokio::sync::broadcast::Sender<meow_api::log_stream::LogMessage>> =
    OnceLock::new();

pub fn get_log_tx() -> tokio::sync::broadcast::Sender<meow_api::log_stream::LogMessage> {
    LOG_TX
        .get()
        .cloned()
        .unwrap_or_else(|| tokio::sync::broadcast::channel(128).0)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use meow_api::log_stream::LogBroadcastLayer;
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::prelude::*;

    let _ = LOG_HISTORY.set(Mutex::new(VecDeque::new()));
    let (log_tx, _) = tokio::sync::broadcast::channel::<meow_api::log_stream::LogMessage>(128);
    let _ = LOG_TX.set(log_tx.clone());
    let log_layer = LogBroadcastLayer { tx: log_tx }.with_filter(LevelFilter::TRACE);
    tracing_subscriber::registry()
        .with(log_layer)
        .with(HistoryLayer)
        .init();
    tracing_log::LogTracer::init().ok();

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
            commands::core_cmd::auto_start_core,
            commands::core_download::download_core,
            commands::core_download::check_core_installed,
            commands::core_download::list_core_versions,
            commands::core_download::install_core_version,
            commands::core_download::uninstall_core_version,
            commands::core_download::set_core_default_version,
            commands::core_download::get_core_default_version,
            commands::core_download::install_core_with_progress,
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
            commands::proxies::change_mode,
            commands::connections::get_connections,
            commands::connections::close_connection,
            commands::connections::close_all_connections,
            commands::rules::get_rules,
            commands::subscription::apply_subscription,
            commands::subscription::fetch_subscription_url,
            commands::subscription::convert_content,
            commands::subscription::write_config_only,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
