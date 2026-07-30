use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuleInfo {
    #[serde(rename = "type")]
    pub rule_type: String,
    pub payload: String,
    pub proxy: String,
    pub matcher: String,
}

#[tauri::command]
pub async fn get_rules(state: State<'_, AppState>) -> Result<Vec<RuleInfo>, String> {
    if let Some(client) = state.core_manager.client() {
        let resp = client.get_rules().await?;
        return Ok(resp.rules.into_iter().map(|r| RuleInfo {
            rule_type: r.rule_type,
            payload: r.payload,
            proxy: r.proxy,
            matcher: r.matcher,
        }).collect());
    }
    Ok(Vec::new())
}
