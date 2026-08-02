use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct MihomoClient {
    client: Client,
    base_url: String,
}

impl MihomoClient {
    pub fn new(api_port: u16) -> Self {
        Self {
            client: Client::new(),
            base_url: format!("http://127.0.0.1:{}", api_port),
        }
    }

    fn urlencode(s: &str) -> String {
        urlencoding::encode(s).into_owned()
    }

    pub async fn get_configs(&self) -> Result<serde_json::Value, String> {
        self.get("/configs").await
    }

    pub async fn patch_configs(&self, patch: serde_json::Value) -> Result<(), String> {
        self.patch("/configs", patch).await
    }

    pub async fn get_proxies(&self) -> Result<ProxiesResponse, String> {
        self.get("/proxies").await
    }

    pub async fn get_proxy_delay(&self, name: &str, url: &str, timeout: u64) -> Result<DelayResult, String> {
        let encoded = Self::urlencode(name);
        let path = format!("/proxies/{}/delay?url={}&timeout={}", encoded, url, timeout);
        self.get(&path).await
    }

    pub async fn select_proxy(&self, group: &str, name: &str) -> Result<(), String> {
        let body = serde_json::json!({ "name": name });
        self.put(&format!("/proxies/{}", Self::urlencode(group)), body).await
    }

    pub async fn get_connections(&self) -> Result<ConnectionsResponse, String> {
        let url = format!("{}/connections?interval=1000", self.base_url);
        let mut resp = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        if !resp.status().is_success() {
            return Err(format!("Connections API error: HTTP {}", resp.status()));
        }
        let mut buf: Vec<u8> = Vec::new();
        loop {
            let chunk = tokio::time::timeout(
                std::time::Duration::from_secs(5),
                resp.chunk(),
            ).await.map_err(|_| "connections timeout".to_string())?
                .map_err(|e| format!("chunk error: {}", e))?;
            match chunk {
                Some(bytes) => {
                    buf.extend_from_slice(&bytes);
                    if let Ok(conns) = serde_json::from_slice::<ConnectionsResponse>(&buf) {
                        return Ok(conns);
                    }
                }
                None => break,
            }
        }
        Err("connections: no valid JSON response".to_string())
    }

    pub async fn close_connection(&self, id: &str) -> Result<(), String> {
        self.delete(&format!("/connections/{}", id)).await
    }

    pub async fn close_all_connections(&self) -> Result<(), String> {
        self.delete("/connections").await
    }

    pub async fn get_rules(&self) -> Result<RulesResponse, String> {
        self.get("/rules").await
    }

    pub async fn get_traffic(&self) -> Result<Vec<TrafficEntry>, String> {
        let url = format!("{}/traffic", self.base_url);
        let mut resp = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        if !resp.status().is_success() {
            return Err(format!("Traffic API error: HTTP {}", resp.status()));
        }
        let mut buf: Vec<u8> = Vec::new();
        let mut entries = Vec::new();
        loop {
            let chunk = tokio::time::timeout(
                std::time::Duration::from_secs(3),
                resp.chunk(),
            ).await.map_err(|_| "traffic timeout".to_string())?
                .map_err(|e| format!("chunk error: {}", e))?;
            match chunk {
                Some(bytes) => {
                    buf.extend_from_slice(&bytes);
                    if let Some(pos) = buf.iter().position(|&b| b == b'\n') {
                        let line = &buf[..pos];
                        if let Ok(entry) = serde_json::from_slice::<TrafficEntry>(line) {
                            entries.push(entry);
                            break;
                        }
                    }
                    if buf.len() > 4096 {
                        break;
                    }
                }
                None => break,
            }
        }
        Ok(entries)
    }

    pub async fn get_version(&self) -> Result<String, String> {
        let url = format!("{}/version", self.base_url);
        let resp = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Version request failed: {}", e))?;
        let data: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        Ok(data["version"].as_str().unwrap_or("unknown").to_string())
    }

    pub async fn get_logs(&self) -> Result<Vec<LogEntry>, String> {
        self.get("/logs").await
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("API error: {}", text));
        }
        resp.json().await.map_err(|e| format!("Parse failed: {}", e))
    }

    async fn patch(&self, path: &str, body: serde_json::Value) -> Result<(), String> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.client.patch(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("API error: {}", text));
        }
        Ok(())
    }

    async fn put(&self, path: &str, body: serde_json::Value) -> Result<(), String> {
        let url = format!("{}{}", self.base_url, path);
        self.client.put(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        Ok(())
    }

    async fn delete(&self, path: &str) -> Result<(), String> {
        let url = format!("{}{}", self.base_url, path);
        self.client.delete(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxiesResponse {
    #[serde(default)]
    pub proxies: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelayResult {
    pub delay: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionsResponse {
    #[serde(rename = "uploadTotal", default)]
    pub upload_total: u64,
    #[serde(rename = "downloadTotal", default)]
    pub download_total: u64,
    #[serde(default)]
    pub memory: u64,
    #[serde(default)]
    pub connections: Vec<ConnectionInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionInfo {
    pub id: String,
    #[serde(default)]
    pub metadata: ConnectionMeta,
    #[serde(default)]
    pub upload: u64,
    #[serde(default)]
    pub download: u64,
    #[serde(default)]
    pub start: String,
    #[serde(default)]
    pub chains: Vec<String>,
    #[serde(default)]
    pub rule: String,
    #[serde(rename = "rulePayload", default)]
    pub rule_payload: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConnectionMeta {
    #[serde(default)]
    pub network: String,
    #[serde(rename = "type", default)]
    pub meta_type: String,
    #[serde(rename = "sourceIP", default)]
    pub src_ip: Option<String>,
    #[serde(rename = "destinationIP", default)]
    pub dst_ip: Option<String>,
    #[serde(rename = "sourcePort", default)]
    pub src_port: u16,
    #[serde(rename = "destinationPort", default)]
    pub dst_port: u16,
    #[serde(default)]
    pub host: String,
    #[serde(rename = "dnsMode", default)]
    pub dns_mode: String,
    #[serde(default)]
    pub process: String,
    #[serde(rename = "processPath", default)]
    pub process_path: String,
    #[serde(rename = "inboundName", default)]
    pub in_name: String,
    #[serde(rename = "inboundPort", default)]
    pub in_port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RulesResponse {
    #[serde(default)]
    pub rules: Vec<RuleInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuleInfo {
    #[serde(rename = "type", default)]
    pub rule_type: String,
    #[serde(default)]
    pub payload: String,
    #[serde(default)]
    pub proxy: String,
    #[serde(default)]
    pub matcher: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrafficEntry {
    pub up: u64,
    pub down: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    #[serde(rename = "type")]
    pub log_type: String,
    pub payload: String,
    pub time: String,
}
