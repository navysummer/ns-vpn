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

#[tauri::command]
pub async fn fetch_ip_info() -> Result<IpInfo, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .no_proxy()
        .build()
        .map_err(|e| e.to_string())?;

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
