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

#[tauri::command]
pub async fn get_connections(state: State<'_, AppState>) -> Result<ConnectionsResponse, String> {
    if let Some(client) = state.core_manager.client() {
        let resp = client.get_connections().await?;
        return Ok(ConnectionsResponse {
            connections: resp.connections.into_iter().map(|c| ConnectionInfo {
                id: c.id,
                metadata: ConnectionMeta {
                    network: c.metadata.network,
                    meta_type: c.metadata.meta_type,
                    source: c.metadata.source,
                    destination: c.metadata.destination,
                    host: c.metadata.host,
                },
                upload: c.upload,
                download: c.download,
                start: c.start,
                chains: c.chains,
                rule: c.rule,
                rule_payload: c.rule_payload,
                source: c.source,
                destination: c.destination,
                conn_type: c.conn_type,
                network: c.network,
            }).collect(),
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
