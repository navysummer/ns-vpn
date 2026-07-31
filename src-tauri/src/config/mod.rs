use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILE: &str = "ns-vpn.yaml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default = "default_mixed_port")]
    pub mixed_port: u16,
    #[serde(default = "default_socks_port")]
    pub socks_port: u16,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_redir_port")]
    pub redir_port: u16,
    #[serde(default = "default_tproxy_port")]
    pub tproxy_port: u16,
    #[serde(default = "default_api_port")]
    pub api_port: u16,
    #[serde(default)]
    pub allow_lan: bool,
    #[serde(default = "default_bind_address")]
    pub bind_address: String,
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default)]
    pub ipv6: bool,
    #[serde(default = "default_external_controller")]
    pub external_controller: String,
    #[serde(default)]
    pub tun: TunConfig,
    #[serde(default)]
    pub dns: DnsConfig,
    // App-level settings
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default)]
    pub system_proxy: bool,
    #[serde(default)]
    pub tun_mode: bool,
    #[serde(default)]
    pub start_at_boot: bool,
    #[serde(default)]
    pub silent_start: bool,
    #[serde(default)]
    pub unified_delay: bool,
    #[serde(default = "default_tray_click_action")]
    pub tray_click_action: String,
    #[serde(default = "default_copy_env_type")]
    pub copy_env_type: String,
    #[serde(default = "default_startup_page")]
    pub startup_page: String,
    #[serde(default)]
    pub lite_mode: bool,
    #[serde(default = "default_core_path")]
    pub core_path: String,
    #[serde(default)]
    pub proxy_running: bool,
}

fn default_mixed_port() -> u16 { 7890 }
fn default_socks_port() -> u16 { 7891 }
fn default_port() -> u16 { 7892 }
fn default_redir_port() -> u16 { 7893 }
fn default_tproxy_port() -> u16 { 7894 }
fn default_api_port() -> u16 { 9090 }
fn default_bind_address() -> String { "*".into() }
fn default_mode() -> String { "rule".into() }
fn default_log_level() -> String { "info".into() }
fn default_external_controller() -> String { "127.0.0.1:9090".into() }
fn default_language() -> String { "zh-CN".into() }
fn default_theme() -> String { "dark".into() }
fn default_tray_click_action() -> String { "show".into() }
fn default_copy_env_type() -> String { "bash".into() }
fn default_startup_page() -> String { "dashboard".into() }
fn default_core_path() -> String { "mihomo".into() }

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            mixed_port: default_mixed_port(),
            socks_port: default_socks_port(),
            port: default_port(),
            redir_port: default_redir_port(),
            tproxy_port: default_tproxy_port(),
            api_port: default_api_port(),
            allow_lan: false,
            bind_address: default_bind_address(),
            mode: default_mode(),
            log_level: default_log_level(),
            ipv6: false,
            external_controller: default_external_controller(),
            tun: TunConfig::default(),
            dns: DnsConfig::default(),
            language: default_language(),
            theme: default_theme(),
            system_proxy: false,
            tun_mode: false,
            start_at_boot: false,
            silent_start: false,
            unified_delay: false,
            tray_click_action: default_tray_click_action(),
            copy_env_type: default_copy_env_type(),
            startup_page: default_startup_page(),
            lite_mode: false,
            core_path: default_core_path(),
            proxy_running: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TunConfig {
    #[serde(default)]
    pub enable: bool,
    #[serde(default = "default_tun_stack")]
    pub stack: String,
    #[serde(default = "default_tun_dns_hijack")]
    pub dns_hijack: Vec<String>,
    #[serde(default)]
    pub auto_route: bool,
    #[serde(default)]
    pub strict_route: bool,
}

fn default_tun_stack() -> String { "mixed".into() }
fn default_tun_dns_hijack() -> Vec<String> { vec!["any:53".into()] }

impl Default for TunConfig {
    fn default() -> Self {
        Self {
            enable: false,
            stack: default_tun_stack(),
            dns_hijack: default_tun_dns_hijack(),
            auto_route: true,
            strict_route: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnsConfig {
    #[serde(default = "default_true")]
    pub enable: bool,
    #[serde(default = "default_listen")]
    pub listen: String,
    #[serde(default = "default_enhanced_mode")]
    pub enhanced_mode: String,
    #[serde(default)]
    pub fake_ip_range: String,
    #[serde(default = "default_nameservers")]
    pub nameserver: Vec<String>,
    #[serde(default = "default_fallback_nameservers")]
    pub fallback: Vec<String>,
    #[serde(default = "default_true")]
    pub fallback_filter: bool,
}

fn default_true() -> bool { true }
fn default_listen() -> String { "0.0.0.0:1053".into() }
fn default_enhanced_mode() -> String { "fake-ip".into() }
fn default_nameservers() -> Vec<String> { vec!["223.5.5.5".into(), "119.29.29.29".into()] }
fn default_fallback_nameservers() -> Vec<String> { vec!["8.8.8.8".into(), "1.1.1.1".into()] }

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            enable: true,
            listen: default_listen(),
            enhanced_mode: default_enhanced_mode(),
            fake_ip_range: "198.18.0.1/16".into(),
            nameserver: default_nameservers(),
            fallback: default_fallback_nameservers(),
            fallback_filter: true,
        }
    }
}

impl AppConfig {
    fn config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."));
        config_dir.join("ns-vpn").join(CONFIG_FILE)
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        match std::fs::read_to_string(&path) {
            Ok(content) => serde_yaml_ng::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let content = serde_yaml_ng::to_string(self).map_err(|e| e.to_string())?;
        std::fs::write(path, content).map_err(|e| e.to_string())
    }
}
