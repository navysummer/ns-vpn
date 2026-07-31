use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionMeta {
    pub network: String,
    #[serde(rename = "type")]
    pub meta_type: String,
    pub source: String,
    pub destination: String,
    pub host: String,
}

#[derive(Debug, Serialize)]
pub struct ConnectionsResponse {
    pub connections: Vec<ConnectionInfo>,
    pub upload_total: u64,
    pub download_total: u64,
}

fn fmt_addr(ip: &Option<String>, port: u16) -> String {
    match ip {
        Some(ip) if !ip.is_empty() => format!("{}:{}", ip, port),
        _ => if port > 0 { format!(":{}", port) } else { String::new() },
    }
}

#[tauri::command]
pub async fn get_connections(state: State<'_, AppState>) -> Result<ConnectionsResponse, String> {
    if let Some(client) = state.core_manager.client() {
        let resp = client.get_connections().await?;
        let conns = resp.connections.into_iter().map(|c| {
            let source = fmt_addr(&c.metadata.src_ip, c.metadata.src_port);
            let destination = if !c.metadata.host.is_empty() {
                if c.metadata.dst_port > 0 {
                    format!("{}:{}", c.metadata.host, c.metadata.dst_port)
                } else {
                    c.metadata.host.clone()
                }
            } else {
                fmt_addr(&c.metadata.dst_ip, c.metadata.dst_port)
            };
            ConnectionInfo {
                metadata: ConnectionMeta {
                    network: c.metadata.network.clone(),
                    meta_type: c.metadata.meta_type.clone(),
                    source: source.clone(),
                    destination: destination.clone(),
                    host: c.metadata.host.clone(),
                },
                upload: c.upload,
                download: c.download,
                start: c.start,
                chains: c.chains,
                rule: c.rule,
                rule_payload: c.rule_payload,
                source,
                destination,
                conn_type: c.metadata.meta_type,
                network: c.metadata.network,
                id: c.id,
            }
        }).collect();
        return Ok(ConnectionsResponse {
            connections: conns,
            upload_total: resp.upload_total,
            download_total: resp.download_total,
        });
    }
    Ok(ConnectionsResponse {
        connections: Vec::new(),
        upload_total: 0,
        download_total: 0,
    })
}

#[tauri::command]
pub async fn close_connection(state: State<'_, AppState>, id: String) -> Result<(), String> {
    if let Some(client) = state.core_manager.client() {
        client.close_connection(&id).await?;
        return Ok(());
    }
    Err("Core not running".into())
}

#[tauri::command]
pub async fn close_all_connections(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(client) = state.core_manager.client() {
        client.close_all_connections().await?;
        return Ok(());
    }
    Err("Core not running".into())
}
