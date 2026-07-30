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
        let path = format!("/delay?name={}&url={}&timeout={}", name, url, timeout);
        self.get(&path).await
    }

    pub async fn select_proxy(&self, group: &str, name: &str) -> Result<(), String> {
        let body = serde_json::json!({ "name": name });
        self.put(&format!("/proxies/{}", group), body).await
    }

    pub async fn get_connections(&self) -> Result<ConnectionsResponse, String> {
        self.get("/connections").await
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
        self.get("/traffic").await
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
        resp.json().await.map_err(|e| format!("Parse failed: {}", e))
    }

    async fn patch(&self, path: &str, body: serde_json::Value) -> Result<(), String> {
        let url = format!("{}{}", self.base_url, path);
        self.client.patch(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
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
    #[serde(default)]
    pub connections: Vec<ConnectionInfo>,
    #[serde(default)]
    pub upload_total: u64,
    #[serde(default)]
    pub download_total: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionInfo {
    pub id: String,
    pub metadata: ConnectionMeta,
    pub upload: u64,
    pub download: u64,
    pub start: String,
    pub chains: Vec<String>,
    pub rule: String,
    pub rule_payload: String,
    pub source: String,
    pub destination: String,
    #[serde(rename = "type")]
    pub conn_type: String,
    pub network: String,
    pub host: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionMeta {
    pub network: String,
    #[serde(rename = "type")]
    pub meta_type: String,
    pub source: String,
    pub destination: String,
    pub host: String,
    pub dns_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RulesResponse {
    #[serde(default)]
    pub rules: Vec<RuleInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuleInfo {
    #[serde(rename = "type")]
    pub rule_type: String,
    pub payload: String,
    pub proxy: String,
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
