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
    let up_speed: u64;
    let down_speed: u64;
    let up_total: u64;
    let down_total: u64;
    let active_connections: u64;

    if let Some(client) = state.core_manager.client() {
        let traffic = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            client.get_traffic(),
        ).await;
        let (speed_up, speed_down) = match traffic {
            Ok(Ok(entries)) => {
                (entries.iter().map(|e| e.up).sum(), entries.iter().map(|e| e.down).sum())
            }
            _ => (0, 0),
        };
        up_speed = speed_up;
        down_speed = speed_down;

        let conns = client.get_connections().await.ok();
        match conns {
            Some(c) => {
                up_total = c.upload_total;
                down_total = c.download_total;
                active_connections = c.connections.len() as u64;
            }
            None => {
                up_total = 0;
                down_total = 0;
                active_connections = 0;
            }
        }
    } else {
        up_speed = 0;
        down_speed = 0;
        up_total = 0;
        down_total = 0;
        active_connections = 0;
    }

    Ok(TrafficData {
        upload_speed: up_speed,
        download_speed: down_speed,
        upload_total: up_total,
        download_total: down_total,
        active_connections,
    })
}
