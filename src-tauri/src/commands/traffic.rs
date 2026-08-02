use serde::Serialize;
use std::sync::Mutex;
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

static LAST_TOTALS: Mutex<Option<(u64, u64, std::time::Instant)>> = Mutex::new(None);

#[tauri::command]
pub async fn get_traffic(state: State<'_, AppState>) -> Result<TrafficData, String> {
    if let Some(client) = state.core_manager.client() {
        let conns = client.get_connections().await.map_err(|e| e.to_string())?;
        let up_total = conns.upload_total;
        let down_total = conns.download_total;
        let active = conns.connections.len() as u64;

        let (up_speed, down_speed) = {
            let mut lock = LAST_TOTALS.lock().unwrap();
            if let Some((prev_up, prev_down, instant)) = *lock {
                let elapsed = instant.elapsed().as_secs_f64();
                if elapsed > 0.0 {
                    let speed_up = ((up_total.saturating_sub(prev_up)) as f64 / elapsed) as u64;
                    let speed_down = ((down_total.saturating_sub(prev_down)) as f64 / elapsed) as u64;
                    *lock = Some((up_total, down_total, std::time::Instant::now()));
                    (speed_up, speed_down)
                } else {
                    *lock = Some((up_total, down_total, std::time::Instant::now()));
                    (0, 0)
                }
            } else {
                *lock = Some((up_total, down_total, std::time::Instant::now()));
                (0, 0)
            }
        };

        Ok(TrafficData {
            upload_speed: up_speed,
            download_speed: down_speed,
            upload_total: up_total,
            download_total: down_total,
            active_connections: active,
        })
    } else {
        *LAST_TOTALS.lock().unwrap() = None;
        Ok(TrafficData {
            upload_speed: 0,
            download_speed: 0,
            upload_total: 0,
            download_total: 0,
            active_connections: 0,
        })
    }
}
