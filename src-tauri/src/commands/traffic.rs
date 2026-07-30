use serde::Serialize;
use tauri::State;
use crate::AppState;

#[derive(Debug, Serialize, Clone)]
pub struct TrafficData {
    pub upload_speed: u64,
    pub download_speed: u64,
    pub upload_total: u64,
    pub download_total: u64,
    pub active_connections: u64,
}

#[tauri::command]
pub async fn get_traffic(state: State<'_, AppState>) -> Result<TrafficData, String> {
    if let Some(client) = state.core_manager.client() {
        match client.get_traffic().await {
            Ok(entries) => {
                let mut up_speed: u64 = 0;
                let mut down_speed: u64 = 0;
                for entry in &entries {
                    up_speed += entry.up;
                    down_speed += entry.down;
                }
                // Also get connection count for totals
                let conns = client.get_connections().await.ok();
                let (up_total, down_total, active) = match conns {
                    Some(c) => (c.upload_total, c.download_total, c.connections.len() as u64),
                    None => (0, 0, 0),
                };
                return Ok(TrafficData {
                    upload_speed: up_speed,
                    download_speed: down_speed,
                    upload_total: up_total,
                    download_total: down_total,
                    active_connections: active,
                });
            }
            Err(_) => {}
        }
    }
    Ok(TrafficData {
        upload_speed: 0,
        download_speed: 0,
        upload_total: 0,
        download_total: 0,
        active_connections: 0,
    })
}
