use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub theme_mode: String,
    pub language: String,
    pub system_proxy: bool,
    pub system_proxy_port: u16,
    pub tun_mode: bool,
    pub mixed_port: u16,
    pub api_port: u16,
    pub allow_lan: bool,
    pub log_level: String,
    pub start_at_boot: bool,
    pub enable_auto_launch: bool,
    pub core_path: String,
    pub config_path: String,
    pub subscription_url: String,
    pub auto_update_interval: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme_mode: "auto".to_string(),
            language: "zh-CN".to_string(),
            system_proxy: false,
            system_proxy_port: 7890,
            tun_mode: false,
            mixed_port: 7890,
            api_port: 9090,
            allow_lan: false,
            log_level: "info".to_string(),
            start_at_boot: false,
            enable_auto_launch: false,
            core_path: String::new(),
            config_path: String::new(),
            subscription_url: String::new(),
            auto_update_interval: 0,
        }
    }
}