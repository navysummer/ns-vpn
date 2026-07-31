use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyGroupInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: String,
    pub now: Option<String>,
    pub all: Vec<ProxyNodeInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyNodeInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub delay: Option<u32>,
}

#[tauri::command]
pub async fn get_proxies(state: State<'_, AppState>) -> Result<Vec<ProxyGroupInfo>, String> {
    if let Some(client) = state.core_manager.client() {
        let resp = client.get_proxies().await?;
        let mut groups = Vec::new();
        if let Some(obj) = resp.proxies.as_object() {
            for (name, val) in obj {
                if let Some(group_type) = val.get("type").and_then(|v| v.as_str()) {
                    if group_type == "Selector" || group_type == "URLTest"
                        || group_type == "Fallback" || group_type == "LoadBalance"
                        || group_type == "Relay"
                    {
                        let now = val.get("now").and_then(|v| v.as_str()).map(|s| s.to_string());
                        let all_names = val.get("all").and_then(|v| v.as_array());
                        let mut nodes = Vec::new();
                        if let Some(arr) = all_names {
                            for node_val in arr {
                                // meow returns plain name strings in `all`
                                let node_name = node_val.as_str().unwrap_or("").to_string();
                                if node_name.is_empty() { continue; }
                                let node_type = obj.get(&node_name)
                                    .and_then(|v| v.get("type"))
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                nodes.push(ProxyNodeInfo {
                                    name: node_name,
                                    node_type,
                                    delay: None,
                                });
                            }
                        }
                        groups.push(ProxyGroupInfo {
                            name: name.clone(),
                            group_type: group_type.to_string(),
                            now,
                            all: nodes,
                        });
                    }
                }
            }
        }
        return Ok(groups);
    }
    Ok(Vec::new())
}

#[tauri::command]
pub async fn select_proxy(state: State<'_, AppState>, group: String, name: String) -> Result<(), String> {
    if let Some(client) = state.core_manager.client() {
        client.select_proxy(&group, &name).await?;
        return Ok(());
    }
    Err("Core not running".into())
}

#[tauri::command]
pub async fn test_delay(state: State<'_, AppState>, name: String, url: Option<String>) -> Result<u32, String> {
    if let Some(client) = state.core_manager.client() {
        let test_url = url.unwrap_or_else(|| "http://www.gstatic.com/generate_204".into());
        let result = client.get_proxy_delay(&name, &test_url, 5000).await?;
        return Ok(result.delay);
    }
    Ok(0)
}

#[tauri::command]
pub async fn change_mode(state: State<'_, AppState>, mode: String) -> Result<(), String> {
    if let Some(client) = state.core_manager.client() {
        let patch = serde_json::json!({ "mode": mode });
        client.patch_configs(patch).await?;
        return Ok(());
    }
    Err("Core not running".into())
}
