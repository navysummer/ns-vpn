use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub version: String,
    pub os: String,
    pub arch: String,
    pub core_version: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IpInfo {
    pub ip: String,
    pub country: String,
    pub asn: String,
    pub isp: String,
    pub org: String,
    pub city: String,
    pub timezone: String,
}

#[derive(Debug, Serialize)]
pub struct ConfigFileInfo {
    pub path: String,
    pub size: u64,
    pub modified: String,
}

#[tauri::command]
pub async fn fetch_ip_info(proxy_url: Option<String>) -> Result<IpInfo, String> {
    let mut builder = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10));
    if let Some(ref url) = proxy_url {
        let proxy = reqwest::Proxy::http(url).map_err(|e| e.to_string())?;
        builder = builder.proxy(proxy);
    } else {
        builder = builder.no_proxy();
    }
    let client = builder.build().map_err(|e| e.to_string())?;

    let resp = client.get("http://ip-api.com/json/")
        .send()
        .await
        .map_err(|e| format!("IP request failed: {}", e))?;
    let data: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

    Ok(IpInfo {
        ip: data["query"].as_str().unwrap_or("").to_string(),
        country: data["country"].as_str().unwrap_or("").to_string(),
        asn: data["as"].as_str().unwrap_or("").to_string(),
        isp: data["isp"].as_str().unwrap_or("").to_string(),
        org: data["org"].as_str().unwrap_or("").to_string(),
        city: format!("{}, {}",
            data["city"].as_str().unwrap_or(""),
            data["regionName"].as_str().unwrap_or("")),
        timezone: data["timezone"].as_str().unwrap_or("").to_string(),
    })
}

#[tauri::command]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub fn get_system_info() -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        core_version: None,
    })
}

#[tauri::command]
pub fn open_app_dir() -> Result<(), String> {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("ns-vpn");
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }
    opener::open(config_dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn select_config_file() -> Result<String, String> {
    Ok(String::new())
}

#[tauri::command]
pub fn get_config_file_info() -> Result<ConfigFileInfo, String> {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("ns-vpn");
    let config_file = config_dir.join("config.yaml");
    let meta = std::fs::metadata(&config_file).map_err(|e| e.to_string())?;
    use std::time::UNIX_EPOCH;
    let modified = meta.modified().unwrap_or(UNIX_EPOCH);
    let duration = modified.duration_since(UNIX_EPOCH).unwrap_or_default();
    let datetime = chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
        .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default();
    Ok(ConfigFileInfo {
        path: config_file.to_string_lossy().to_string(),
        size: meta.len(),
        modified: datetime,
    })
}

#[tauri::command]
pub fn get_log_dir() -> Result<String, String> {
    let log_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("ns-vpn")
        .join("logs");
    std::fs::create_dir_all(&log_dir).map_err(|e| e.to_string())?;
    Ok(log_dir.to_string_lossy().to_string())
}

#[derive(Debug, Serialize)]
pub struct DiagnosticsData {
    pub app_version: String,
    pub os: String,
    pub arch: String,
    pub core_version: Option<String>,
    pub config_file: Option<ConfigFileInfo>,
    pub proxy_running: bool,
    pub rules_count: usize,
    pub connections_count: usize,
    pub memory: u64,
}

#[tauri::command]
pub async fn export_diagnostics(state: tauri::State<'_, crate::AppState>) -> Result<DiagnosticsData, String> {
    let core_version = if let Some(client) = state.core_manager.client() {
        client.get_version().await.ok()
    } else {
        None
    };
    let config_file = get_config_file_info().ok();
    let rules_count = 0;
    let connections_count = 0;
    let memory = 0;
    Ok(DiagnosticsData {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        core_version,
        config_file,
        proxy_running: *state.proxy_running.read(),
        rules_count,
        connections_count,
        memory,
    })
}
