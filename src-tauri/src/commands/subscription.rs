use std::path::PathBuf;
use tauri::State;
use crate::AppState;

#[tauri::command]
pub async fn apply_subscription(
    state: State<'_, AppState>,
    content: String,
    format: String,
) -> Result<(), String> {
    let config = state.config.read().clone();
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ns-vpn");

    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }

    let config_file = config_dir.join("config.yaml");

    let final_content = match format.as_str() {
        "clash" | "yaml" => content,
        "v2rayn" => convert_v2rayn_to_clash(&content)?,
        "singbox" => convert_singbox_to_clash(&content)?,
        "openvpn" => {
            let yaml = convert_openvpn_to_clash(&content)?;
            std::fs::write(&config_file, yaml).map_err(|e| e.to_string())?;
            state.core_manager.stop().await.ok();
            state.core_manager.set_skip_write(true);
            state.core_manager.start(&config).await?;
            return Ok(());
        }
        _ => content,
    };

    std::fs::write(&config_file, final_content).map_err(|e| e.to_string())?;

    state.core_manager.stop().await.ok();
    state.core_manager.set_skip_write(true);
    state.core_manager.start(&config).await?;

    Ok(())
}

#[tauri::command]
pub async fn fetch_subscription_url(url: String) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.get(&url)
        .header("User-Agent", "clash-verge/v2.0")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    resp.text().await.map_err(|e| format!("Read body failed: {}", e))
}

fn convert_v2rayn_to_clash(content: &str) -> Result<String, String> {
    let decoded = base64_decode(content.trim())?;
    let lines: Vec<&str> = decoded.lines().filter(|l| !l.trim().is_empty()).collect();

    let mut proxies = Vec::new();
    for line in &lines {
        if let Some(node) = parse_v2rayn_line(line)? {
            proxies.push(node);
        }
    }

    if proxies.is_empty() {
        return Err("No valid proxy nodes found in v2rayn config".into());
    }

    let config = serde_json::json!({
        "proxies": proxies,
        "proxy-groups": [
            {
                "name": "Proxy",
                "type": "select",
                "proxies": proxies.iter().map(|p| p["name"].as_str().unwrap_or("")).collect::<Vec<_>>(),
            }
        ],
        "rules": ["MATCH,Proxy"],
    });

    serde_yaml_ng::to_string(&config).map_err(|e| e.to_string())
}

fn base64_decode(input: &str) -> Result<String, String> {
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(input)
        .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(input))
        .map_err(|e| format!("Base64 decode failed: {}", e))?;
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode failed: {}", e))
}

fn parse_v2rayn_line(line: &str) -> Result<Option<serde_json::Value>, String> {
    if !line.contains("://") {
        return Ok(None);
    }

    let parts: Vec<&str> = line.splitn(2, "://").collect();
    if parts.len() != 2 {
        return Ok(None);
    }

    let scheme = parts[0];
    let rest = parts[1];

    match scheme {
        "ss" => {
            let decoded = base64_decode(rest)?;
            let at_parts: Vec<&str> = decoded.rsplitn(2, "@").collect();
            if at_parts.len() != 2 {
                return Ok(None);
            }
            let method_pass = at_parts[1];
            let host_port = at_parts[0];
            let hp: Vec<&str> = host_port.rsplitn(2, ":").collect();
            if hp.len() != 2 {
                return Ok(None);
            }
            let mp: Vec<&str> = method_pass.splitn(2, ":").collect();
            if mp.len() != 2 {
                return Ok(None);
            }
            Ok(Some(serde_json::json!({
                "name": format!("ss-{}:{}", hp[1], hp[0]),
                "type": "ss",
                "server": hp[1],
                "port": hp[0].parse::<u16>().unwrap_or(443),
                "cipher": mp[0],
                "password": mp[1],
            })))
        }
        "vmess" => {
            let decoded = base64_decode(rest)?;
            let v: serde_json::Value = serde_json::from_str(&decoded).map_err(|e| e.to_string())?;
            let add = v["add"].as_str().unwrap_or("");
            let port = v["port"].as_str().unwrap_or("443");
            let id = v["id"].as_str().unwrap_or("");
            let aid = v["aid"].as_str().unwrap_or("0");
            let net = v["net"].as_str().unwrap_or("tcp");
            let tls = v["tls"].as_str().unwrap_or("");
            let default_name = format!("vmess-{}:{}", add, port);
            let name = v["ps"].as_str().unwrap_or(&default_name);

            let mut node = serde_json::json!({
                "name": name,
                "type": "vmess",
                "server": add,
                "port": port.parse::<u16>().unwrap_or(443),
                "uuid": id,
                "alterId": aid.parse::<u16>().unwrap_or(0),
                "cipher": "auto",
                "network": net,
            });

            if tls == "tls" {
                node["tls"] = serde_json::json!(true);
            }

            Ok(Some(node))
        }
        "trojan" => {
            let decoded = base64_decode(rest)?;
            let parts: Vec<&str> = decoded.rsplitn(2, "@").collect();
            if parts.len() != 2 {
                return Ok(None);
            }
            let password = parts[1];
            let host_port: Vec<&str> = parts[0].rsplitn(2, ":").collect();
            if host_port.len() != 2 {
                return Ok(None);
            }
            Ok(Some(serde_json::json!({
                "name": format!("trojan-{}:{}", host_port[1], host_port[0]),
                "type": "trojan",
                "server": host_port[1],
                "port": host_port[0].parse::<u16>().unwrap_or(443),
                "password": password,
            })))
        }
        _ => Ok(None),
    }
}

fn convert_singbox_to_clash(content: &str) -> Result<String, String> {
    let v: serde_json::Value = serde_json::from_str(content).map_err(|e| e.to_string())?;

    let outbounds = v["outbounds"].as_array().ok_or("No outbounds found")?;

    let mut proxies = Vec::new();
    let mut proxy_names = Vec::new();

    for ob in outbounds {
        let ob_type = ob["type"].as_str().unwrap_or("");
        let tag = ob["tag"].as_str().unwrap_or("");

        if ob_type == "selector" || ob_type == "urltest" || ob_type == "fallback" || ob_type == "loadbalance" {
            continue;
        }

        let mut node = serde_json::json!({
            "name": tag,
            "type": ob_type,
        });

        if let Some(server) = ob["server"].as_str() {
            node["server"] = serde_json::json!(server);
        }
        if let Some(port) = ob["server_port"].as_u64() {
            node["port"] = serde_json::json!(port);
        }
        if let Some(uuid) = ob["uuid"].as_str() {
            node["uuid"] = serde_json::json!(uuid);
        }
        if let Some(password) = ob["password"].as_str() {
            node["password"] = serde_json::json!(password);
        }
        if let Some(method) = ob["method"].as_str() {
            node["cipher"] = serde_json::json!(method);
        }

        proxies.push(node.clone());
        proxy_names.push(tag.to_string());
    }

    if proxies.is_empty() {
        return Err("No valid proxy nodes found in sing-box config".into());
    }

    let config = serde_json::json!({
        "proxies": proxies,
        "proxy-groups": [
            {
                "name": "Proxy",
                "type": "select",
                "proxies": proxy_names,
            }
        ],
        "rules": ["MATCH,Proxy"],
    });

    serde_yaml_ng::to_string(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn convert_content(content: String, format: String) -> Result<String, String> {
    match format.as_str() {
        "clash" | "yaml" => Ok(content),
        "v2rayn" => convert_v2rayn_to_clash(&content),
        "singbox" => convert_singbox_to_clash(&content),
        "openvpn" => convert_openvpn_to_clash(&content),
        _ => Err(format!("Unsupported format: {}", format)),
    }
}

fn convert_openvpn_to_clash(content: &str) -> Result<String, String> {
    let mut server = None;
    let mut port = None;
    let mut proto = "udp";

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("remote ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                server = Some(parts[1].to_string());
                if parts.len() >= 3 {
                    if let Ok(p) = parts[2].parse::<u16>() {
                        port = Some(p);
                    }
                }
            }
        } else if line.starts_with("proto ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                proto = parts[1];
            }
        }
    }

    let server = server.ok_or("No 'remote' directive found in OpenVPN config")?;
    let port = port.unwrap_or(if proto == "tcp" { 443 } else { 1194 });

    let config = serde_json::json!({
        "proxies": [{
            "name": format!("openvpn-{}", server),
            "type": "ss",
            "server": server,
            "port": port,
            "cipher": "aes-256-gcm",
            "password": "",
        }],
        "proxy-groups": [{
            "name": "Proxy",
            "type": "select",
            "proxies": [format!("openvpn-{}", server)],
        }],
        "rules": ["MATCH,Proxy"],
    });

    serde_yaml_ng::to_string(&config).map_err(|e| e.to_string())
}
