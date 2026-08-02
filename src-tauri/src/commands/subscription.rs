use std::path::PathBuf;
use tauri::State;
use crate::AppState;

#[tauri::command]
pub async fn write_config_only(
    state: State<'_, AppState>,
    content: String,
) -> Result<(), String> {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ns-vpn");

    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }

    let config_file = config_dir.join("config.yaml");
    let config = state.config.read().clone();

    let sub_yaml: serde_yaml_ng::Value = serde_yaml_ng::from_str(&content)
        .unwrap_or(serde_yaml_ng::Value::Null);

    let proxies = sub_yaml.get("proxies").cloned().unwrap_or(serde_yaml_ng::Value::Null);
    let mut proxy_groups = sub_yaml.get("proxy-groups").cloned().unwrap_or(serde_yaml_ng::Value::Null);
    let mut rules = sub_yaml.get("rules").cloned().unwrap_or(serde_yaml_ng::Value::Null);

    if proxy_groups.is_null() {
        if let Some(arr) = proxies.as_sequence() {
            let names: Vec<String> = arr.iter()
                .filter_map(|p| p.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()))
                .collect();
            if !names.is_empty() {
                proxy_groups = serde_yaml_ng::Value::Sequence(
                    vec![serde_yaml_ng::to_value(serde_json::json!({
                        "name": "Proxy",
                        "type": "select",
                        "proxies": names,
                    })).unwrap_or(serde_yaml_ng::Value::Null)]
                );
            }
        }
    }

    if rules.is_null() {
        rules = serde_yaml_ng::Value::Sequence(
            vec![serde_yaml_ng::Value::String("MATCH,Proxy".to_string())]
        );
    }

    let dns_fallback_filter = if config.dns.fallback_filter {
        serde_json::json!({
            "geoip": true,
            "geoip-code": "CN",
            "ipcidr": ["240.0.0.0/4"],
        })
    } else {
        serde_json::json!({ "geoip": false })
    };

    let meow_config = serde_json::json!({
        "mixed-port": config.mixed_port,
        "allow-lan": config.allow_lan,
        "bind-address": config.bind_address,
        "mode": config.mode,
        "log-level": config.log_level,
        "ipv6": config.ipv6,
        "external-controller": config.external_controller,
        "tun": {
            "enable": config.tun_mode,
            "stack": config.tun.stack,
            "dns-hijack": config.tun.dns_hijack,
            "auto-route": config.tun.auto_route,
            "strict-route": config.tun.strict_route,
        },
        "dns": {
            "enable": config.dns.enable,
            "listen": config.dns.listen,
            "enhanced-mode": config.dns.enhanced_mode,
            "fake-ip-range": config.dns.fake_ip_range,
            "nameserver": config.dns.nameserver,
            "fallback": config.dns.fallback,
            "fallback-filter": dns_fallback_filter,
        },
        "proxies": proxies,
        "proxy-groups": proxy_groups,
        "rules": rules,
    });

    let yaml = serde_yaml_ng::to_string(&meow_config).map_err(|e| e.to_string())?;
    std::fs::write(&config_file, yaml).map_err(|e| e.to_string())?;
    state.core_manager.set_skip_write(true);
    Ok(())
}

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

fn looks_like_clash_yaml(content: &str) -> bool {
    let first = content.trim_start().lines().next().unwrap_or("").trim();
    first == "proxies:" || first.starts_with("proxy-groups:") || first == "rules:" || first.starts_with("mixed-port:") || first.starts_with("port:")
}

fn convert_v2rayn_to_clash(content: &str) -> Result<String, String> {
    let text = content.trim();
    if looks_like_clash_yaml(text) {
        return Ok(text.to_string());
    }
    let decoded = if text.lines().all(|l| l.trim().is_empty() || l.starts_with("ss://") || l.starts_with("vmess://") || l.starts_with("trojan://") || l.starts_with("vless://") || l.starts_with("hysteria2://") || l.starts_with("hysteria://") || l.starts_with("snell://") || l.starts_with("anytls://") || l.starts_with("ssr://") || l.starts_with("tuic://") || l.starts_with("ssh://") || l.starts_with("naive+https://") || l.starts_with("wireguard://") || l.starts_with("wg://")) {
        text.to_string()
    } else {
        base64_decode(text)?
    };
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

fn parse_vmess_plain(rest: &str) -> Result<Option<serde_json::Value>, String> {
    let at_idx = rest.find("@").ok_or("Invalid vmess URI")?;
    let uuid = urldecode(&rest[..at_idx])?;
    let server_part = &rest[at_idx + 1..];

    let sp: Vec<&str> = server_part.rsplitn(2, ":").collect();
    if sp.len() < 2 {
        return Ok(None);
    }
    let server = sp[1].trim_start_matches('[').trim_end_matches(']');
    let port_and_params = sp[0];
    let semi_idx = port_and_params.find(';').or_else(|| port_and_params.find('?'));
    let port = if let Some(idx) = semi_idx {
        port_and_params[..idx].parse::<u16>().unwrap_or(443)
    } else {
        port_and_params.parse::<u16>().unwrap_or(443)
    };

    let params_str = if let Some(idx) = semi_idx {
        &port_and_params[idx + 1..]
    } else {
        ""
    };
    let params = parse_query_params(params_str);

    let security = params.get("security").map(|s| s.as_str()).unwrap_or("none");
    let sni = params.get("sni").map(|s| s.as_str()).unwrap_or("");
    let net = params.get("type").map(|s| s.as_str()).unwrap_or("tcp");
    let host = params.get("host").map(|s| s.as_str()).unwrap_or("");
    let path = params.get("path").map(|s| s.as_str()).unwrap_or("");

    let default_name = format!("vmess-{}:{}", server, port);
    let name = params.get("remarks").map(|s| s.as_str()).unwrap_or(&default_name);

    let mut node = serde_json::json!({
        "name": name,
        "type": "vmess",
        "server": server,
        "port": port,
        "uuid": uuid,
        "alterId": params.get("alterId").and_then(|s| s.parse::<u16>().ok()).unwrap_or(0),
        "cipher": "auto",
    });

    if security == "tls" {
        node["tls"] = serde_json::json!(true);
        if !sni.is_empty() {
            node["servername"] = serde_json::json!(sni);
        }
        node["skip-cert-verify"] = serde_json::json!(true);
    }

    match net {
        "ws" => {
            node["network"] = serde_json::json!("ws");
            let mut ws_opts = serde_json::json!({});
            if !path.is_empty() {
                ws_opts["path"] = serde_json::json!(path);
            }
            if !host.is_empty() {
                ws_opts["headers"] = serde_json::json!({ "Host": host });
            }
            node["ws-opts"] = ws_opts;
        }
        "grpc" => {
            node["network"] = serde_json::json!("grpc");
            let mut grpc_opts = serde_json::json!({});
            let service_name = params.get("serviceName").map(|s| s.as_str()).unwrap_or("");
            if !service_name.is_empty() {
                grpc_opts["grpc-service-name"] = serde_json::json!(service_name);
            }
            node["grpc-opts"] = grpc_opts;
        }
        "h2" | "http" => {
            node["network"] = serde_json::json!("h2");
            let mut h2_opts = serde_json::json!({});
            if !path.is_empty() {
                h2_opts["path"] = serde_json::json!(path);
            }
            if !host.is_empty() {
                h2_opts["host"] = serde_json::json!([host]);
            }
            node["h2-opts"] = h2_opts;
        }
        _ => {
            if !net.is_empty() && net != "tcp" {
                node["network"] = serde_json::json!(net);
            }
        }
    }

    Ok(Some(node))
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
            let at_parts: Vec<&str> = rest.splitn(2, "@").collect();
            if at_parts.len() != 2 {
                return Ok(None);
            }
            let userinfo = at_parts[0].split('#').next().unwrap_or(at_parts[0]);
            let host_port = at_parts[1].split('#').next().unwrap_or(at_parts[1]);
            let method_pass = base64_decode(userinfo)?;
            let mp: Vec<&str> = method_pass.splitn(2, ":").collect();
            if mp.len() != 2 {
                return Ok(None);
            }
            let hp: Vec<&str> = host_port.rsplitn(2, ":").collect();
            if hp.len() != 2 {
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
            if rest.contains('@') {
                return parse_vmess_plain(rest);
            }
            let clean = rest.split('#').next().unwrap_or(rest);
            let decoded = base64_decode(clean)?;
            let v: serde_json::Value = serde_json::from_str(&decoded).map_err(|e| e.to_string())?;
            let add = v["add"].as_str().unwrap_or("");
            let port = v["port"].as_str().unwrap_or("443");
            let id = v["id"].as_str().unwrap_or("");
            let aid = v["aid"].as_str().unwrap_or("0");
            let net = v["net"].as_str().unwrap_or("tcp");
            let tls = v["tls"].as_str().unwrap_or("");
            let sni = v["sni"].as_str().unwrap_or("");
            let path = v["path"].as_str().unwrap_or("");
            let host = v["host"].as_str().unwrap_or("");
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
            });

            if tls == "tls" {
                node["tls"] = serde_json::json!(true);
                if !sni.is_empty() {
                    node["servername"] = serde_json::json!(sni);
                }
                node["skip-cert-verify"] = serde_json::json!(true);
            }

            match net {
                "ws" => {
                    node["network"] = serde_json::json!("ws");
                    let mut ws_opts = serde_json::json!({});
                    if !path.is_empty() {
                        ws_opts["path"] = serde_json::json!(path);
                    }
                    if !host.is_empty() {
                        ws_opts["headers"] = serde_json::json!({ "Host": host });
                    }
                    node["ws-opts"] = ws_opts;
                }
                "grpc" => {
                    node["network"] = serde_json::json!("grpc");
                    let mut grpc_opts = serde_json::json!({});
                    let service_name = v["path"].as_str().unwrap_or("");
                    if !service_name.is_empty() {
                        grpc_opts["grpc-service-name"] = serde_json::json!(service_name);
                    }
                    node["grpc-opts"] = grpc_opts;
                }
                "h2" | "http" => {
                    node["network"] = serde_json::json!("h2");
                    let mut h2_opts = serde_json::json!({});
                    if !path.is_empty() {
                        h2_opts["path"] = serde_json::json!(path);
                    }
                    if !host.is_empty() {
                        h2_opts["host"] = serde_json::json!([host]);
                    }
                    node["h2-opts"] = h2_opts;
                }
                _ => {
                    if !net.is_empty() && net != "tcp" {
                        node["network"] = serde_json::json!(net);
                    }
                }
            }

            Ok(Some(node))
        }
        "trojan" => {
            let at_parts: Vec<&str> = rest.splitn(2, "@").collect();
            if at_parts.len() != 2 {
                return Ok(None);
            }
            let password = urldecode(at_parts[0])?;
            let host_port = at_parts[1].split('#').next().unwrap_or(at_parts[1]);
            let hp: Vec<&str> = host_port.rsplitn(2, ":").collect();
            if hp.len() != 2 {
                return Ok(None);
            }
            let hp_query = hp[0].find('?').map(|i| &hp[0][..i]).unwrap_or(hp[0]);
            Ok(Some(serde_json::json!({
                "name": format!("trojan-{}:{}", hp[1], hp_query),
                "type": "trojan",
                "server": hp[1],
                "port": hp_query.parse::<u16>().unwrap_or(443),
                "password": password,
            })))
        }
        "vless" => {
            let at_idx = rest.find("@").ok_or("Invalid vless URI")?;
            let userinfo = &rest[..at_idx];
            let server_part = &rest[at_idx + 1..];

            let password = urldecode(userinfo)?;
            let sp: Vec<&str> = server_part.rsplitn(2, ":").collect();
            if sp.len() < 2 {
                return Ok(None);
            }
            let server = sp[1].trim_start_matches('[').trim_end_matches(']');
            let port_and_params = sp[0];
            let semi_idx = port_and_params.find(';').or_else(|| port_and_params.find('?'));
            let port_str = if let Some(idx) = semi_idx {
                &port_and_params[..idx]
            } else {
                port_and_params
            };
            let port = port_str.parse::<u16>().unwrap_or(443);

            let params_str = if let Some(idx) = semi_idx {
                &port_and_params[idx + 1..]
            } else {
                ""
            };
            let params = parse_query_params(params_str.split('#').next().unwrap_or(params_str));

            let flow = params.get("flow").map(|s| s.as_str()).unwrap_or("");
            let encryption = params.get("encryption").map(|s| s.as_str()).unwrap_or("none");
            let sni = params.get("sni").map(|s| s.as_str()).unwrap_or("");
            let fp = params.get("fp").map(|s| s.as_str()).unwrap_or("");
            let _alpn = params.get("alpn").map(|s| s.as_str()).unwrap_or("");
            let _pfx = params.get("pfx").map(|s| s.as_str()).unwrap_or("");
            let _spx = params.get("spx").map(|s| s.as_str()).unwrap_or("");
            let security = params.get("security").map(|s| s.as_str()).unwrap_or("none");

            let default_name = format!("vless-{}:{}", server, port);
            let name = params.get("remarks").map(|s| s.as_str()).unwrap_or(&default_name);

            let mut node = serde_json::json!({
                "name": name,
                "type": "vless",
                "server": server,
                "port": port,
                "uuid": password,
                "udp": true,
            });

            if !flow.is_empty() {
                node["flow"] = serde_json::json!(flow);
            }
            if encryption != "none" {
                node["cipher"] = serde_json::json!(encryption);
            }

            match security {
                "tls" => {
                    node["tls"] = serde_json::json!(true);
                    if !sni.is_empty() { node["servername"] = serde_json::json!(sni); }
                    if !fp.is_empty() { node["client-fingerprint"] = serde_json::json!(fp); }
                    node["skip-cert-verify"] = serde_json::json!(true);
                }
                "reality" => {
                    node["tls"] = serde_json::json!(true);
                    node["reality"] = serde_json::json!(true);
                    if !sni.is_empty() { node["servername"] = serde_json::json!(sni); }
                    let pbk = params.get("pbk").map(|s| s.as_str()).unwrap_or("");
                    let sid = params.get("sid").map(|s| s.as_str()).unwrap_or("");
                    if !pbk.is_empty() { node["public-key"] = serde_json::json!(pbk); }
                    if !sid.is_empty() { node["short-id"] = serde_json::json!(sid); }
                    if !fp.is_empty() { node["client-fingerprint"] = serde_json::json!(fp); }
                }
                _ => {}
            }

            let net = params.get("type").map(|s| s.as_str()).unwrap_or("tcp");
            match net {
                "ws" => {
                    node["network"] = serde_json::json!("ws");
                    let mut ws_opts = serde_json::json!({});
                    let path = params.get("path").map(|s| s.as_str()).unwrap_or("/");
                    let host = params.get("host").map(|s| s.as_str()).unwrap_or("");
                    ws_opts["path"] = serde_json::json!(path);
                    if !host.is_empty() {
                        ws_opts["headers"] = serde_json::json!({ "Host": host });
                    }
                    node["ws-opts"] = ws_opts;
                }
                "grpc" => {
                    node["network"] = serde_json::json!("grpc");
                    let service_name = params.get("serviceName").map(|s| s.as_str()).unwrap_or("");
                    node["grpc-opts"] = serde_json::json!({ "grpc-service-name": service_name });
                }
                "h2" | "http" => {
                    node["network"] = serde_json::json!("h2");
                    let mut h2_opts = serde_json::json!({});
                    let path = params.get("path").map(|s| s.as_str()).unwrap_or("/");
                    let host = params.get("host").map(|s| s.as_str()).unwrap_or("");
                    h2_opts["path"] = serde_json::json!(path);
                    if !host.is_empty() {
                        h2_opts["host"] = serde_json::json!([host]);
                    }
                    node["h2-opts"] = h2_opts;
                }
                "httpupgrade" => {
                    node["network"] = serde_json::json!("httpupgrade");
                    let mut opts = serde_json::json!({});
                    let path = params.get("path").map(|s| s.as_str()).unwrap_or("/");
                    let host = params.get("host").map(|s| s.as_str()).unwrap_or("");
                    opts["path"] = serde_json::json!(path);
                    if !host.is_empty() {
                        opts["headers"] = serde_json::json!({ "Host": host });
                    }
                    node["httpupgrade-opts"] = opts;
                }
                _ => {
                    if !net.is_empty() && net != "tcp" {
                        node["network"] = serde_json::json!(net);
                    }
                }
            }

            Ok(Some(node))
        }
        "hysteria2" => {
            let at_idx = rest.find("@").ok_or("Invalid hysteria2 URI")?;
            let password = urldecode(&rest[..at_idx])?;
            let server_part = &rest[at_idx + 1..];

            let semi_idx = server_part.find(';').or_else(|| server_part.find('?'));
            let server_port = if let Some(idx) = semi_idx {
                &server_part[..idx]
            } else {
                server_part
            };
            let sp: Vec<&str> = server_port.rsplitn(2, ":").collect();
            if sp.len() < 2 {
                return Ok(None);
            }
            let server = sp[1].trim_start_matches('[').trim_end_matches(']');
            let port = sp[0].parse::<u16>().unwrap_or(443);

            let params_str = if let Some(idx) = semi_idx {
                &server_part[idx + 1..]
            } else {
                ""
            };
            let params = parse_query_params(params_str.split('#').next().unwrap_or(params_str));

            let sni = params.get("sni").map(|s| s.as_str()).unwrap_or("");
            let insecure = params.get("insecure").map(|s| s.as_str()).unwrap_or("0") == "1";
            let obfs = params.get("obfs").map(|s| s.as_str()).unwrap_or("");
            let obfs_password = params.get("obfs-password").map(|s| s.as_str()).unwrap_or("");

            let default_name = format!("hy2-{}:{}", server, port);
            let name = params.get("remarks").map(|s| s.as_str()).unwrap_or(&default_name);

            let mut node = serde_json::json!({
                "name": name,
                "type": "hysteria2",
                "server": server,
                "port": port,
                "password": password,
                "skip-cert-verify": insecure,
            });

            if !sni.is_empty() {
                node["sni"] = serde_json::json!(sni);
            }
            if obfs == "password" && !obfs_password.is_empty() {
                node["obfs"] = serde_json::json!("password");
                node["obfs-password"] = serde_json::json!(obfs_password);
            }

            Ok(Some(node))
        }
        "snell" => {
            let at_idx = rest.find("@").ok_or("Invalid snell URI")?;
            let password = urldecode(&rest[..at_idx])?;
            let server_part = &rest[at_idx + 1..];

            let semi_idx = server_part.find(';').or_else(|| server_part.find('?'));
            let server_port = if let Some(idx) = semi_idx {
                &server_part[..idx]
            } else {
                server_part
            };
            let sp: Vec<&str> = server_port.rsplitn(2, ":").collect();
            if sp.len() < 2 {
                return Ok(None);
            }
            let server = sp[1].trim_start_matches('[').trim_end_matches(']');
            let port = sp[0].parse::<u16>().unwrap_or(443);

            let params_str = if let Some(idx) = semi_idx {
                &server_part[idx + 1..]
            } else {
                ""
            };
            let params = parse_query_params(params_str.split('#').next().unwrap_or(params_str));

            let snell_version = params.get("snell").map(|s| s.as_str()).unwrap_or("4");
            let obfs_type = params.get("obfs").map(|s| s.as_str()).unwrap_or("");
            let obfs_host = params.get("obfs-host").map(|s| s.as_str()).unwrap_or("");

            let default_name = format!("snell-{}:{}", server, port);
            let name = params.get("remarks").map(|s| s.as_str()).unwrap_or(&default_name);

            let mut node = serde_json::json!({
                "name": name,
                "type": "snell",
                "server": server,
                "port": port,
                "psk": password,
                "version": snell_version.parse::<u8>().unwrap_or(4),
            });

            if !obfs_type.is_empty() {
                let mut obfs_opts = serde_json::json!({
                    "type": obfs_type,
                });
                if !obfs_host.is_empty() {
                    obfs_opts["host"] = serde_json::json!(obfs_host);
                }
                node["obfs-opts"] = obfs_opts;
            }

            Ok(Some(node))
        }
        "anytls" => {
            let at_idx = rest.find("@").ok_or("Invalid anytls URI")?;
            let password = urldecode(&rest[..at_idx])?;
            let server_part = &rest[at_idx + 1..];

            let semi_idx = server_part.find(';').or_else(|| server_part.find('?'));
            let server_port = if let Some(idx) = semi_idx {
                &server_part[..idx]
            } else {
                server_part
            };
            let sp: Vec<&str> = server_port.rsplitn(2, ":").collect();
            if sp.len() < 2 {
                return Ok(None);
            }
            let server = sp[1].trim_start_matches('[').trim_end_matches(']');
            let port = sp[0].parse::<u16>().unwrap_or(443);

            let params_str = if let Some(idx) = semi_idx {
                &server_part[idx + 1..]
            } else {
                ""
            };
            let params = parse_query_params(params_str.split('#').next().unwrap_or(params_str));

            let sni = params.get("sni").map(|s| s.as_str()).unwrap_or("");
            let fp = params.get("fp").map(|s| s.as_str()).unwrap_or("chrome");
            let insecure = params.get("insecure").map(|s| s.as_str()).unwrap_or("0") == "1";

            let default_name = format!("anytls-{}:{}", server, port);
            let name = params.get("remarks").map(|s| s.as_str()).unwrap_or(&default_name);

            let mut node = serde_json::json!({
                "name": name,
                "type": "anytls",
                "server": server,
                "port": port,
                "password": password,
                "client-fingerprint": fp,
                "skip-cert-verify": insecure,
            });

            if !sni.is_empty() {
                node["sni"] = serde_json::json!(sni);
            }

            Ok(Some(node))
        }
        "ssr" => {
            let decoded = base64_decode(rest)?;
            let parts: Vec<&str> = decoded.splitn(6, ":").collect();
            if parts.len() < 6 {
                return Ok(None);
            }
            let server = parts[0];
            let port = parts[1].parse::<u16>().unwrap_or(443);
            let protocol = parts[2];
            let method = parts[3];
            let obfs = parts[4];
            let rest2 = parts[5];
            let at_idx = rest2.rfind('/').unwrap_or(rest2.len());
            let password = urldecode(&rest2[..at_idx])?;
            let params_str = if at_idx < rest2.len() { &rest2[at_idx..] } else { "" };
            let params = parse_query_params(params_str);

            let default_name = format!("ssr-{}:{}", server, port);
            let name = params.get("remarks").map(|s| s.as_str()).unwrap_or(&default_name);

            let mut node = serde_json::json!({
                "name": name,
                "type": "ss",
                "server": server,
                "port": port,
                "cipher": method,
                "password": password,
            });
            if !protocol.is_empty() {
                node["ssr-protocol"] = serde_json::json!(protocol);
            }
            if !obfs.is_empty() {
                node["ssr-obfs"] = serde_json::json!(obfs);
            }
            Ok(Some(node))
        }
        "hysteria" => {
            let at_idx = rest.find("@").ok_or("Invalid hysteria URI")?;
            let auth = urldecode(&rest[..at_idx])?;
            let server_part = &rest[at_idx + 1..];

            let semi_idx = server_part.find(';').or_else(|| server_part.find('?'));
            let server_port = if let Some(idx) = semi_idx {
                &server_part[..idx]
            } else {
                server_part
            };
            let sp: Vec<&str> = server_port.rsplitn(2, ":").collect();
            if sp.len() < 2 {
                return Ok(None);
            }
            let server = sp[1].trim_start_matches('[').trim_end_matches(']');
            let port = sp[0].parse::<u16>().unwrap_or(443);

            let params_str = if let Some(idx) = semi_idx {
                &server_part[idx + 1..]
            } else {
                ""
            };
            let params = parse_query_params(params_str.split('#').next().unwrap_or(params_str));

            let protocol = params.get("protocol").map(|s| s.as_str()).unwrap_or("udp");
            let up_mbps = params.get("up").and_then(|s| s.parse::<u64>().ok()).unwrap_or(10);
            let down_mbps = params.get("down").and_then(|s| s.parse::<u64>().ok()).unwrap_or(50);
            let insecure = params.get("insecure").map(|s| s.as_str()).unwrap_or("0") == "1";

            let default_name = format!("hysteria-{}:{}", server, port);
            let name = params.get("remarks").map(|s| s.as_str()).unwrap_or(&default_name);

            Ok(Some(serde_json::json!({
                "name": name,
                "type": "hysteria2",
                "server": server,
                "port": port,
                "password": auth,
                "protocol": protocol,
                "up": format!("{} Mbps", up_mbps),
                "down": format!("{} Mbps", down_mbps),
                "skip-cert-verify": insecure,
            })))
        }
        "tuic" => {
            let at_idx = rest.find("@").ok_or("Invalid tuic URI")?;
            let userinfo = &rest[..at_idx];
            let server_part = &rest[at_idx + 1..];
            let hp: Vec<&str> = server_part.split(['?', ';', '#']).next().unwrap_or(server_part).rsplitn(2, ":").collect();
            if hp.len() < 2 { return Ok(None); }
            let server = hp[1].trim_start_matches('[').trim_end_matches(']');
            let port = hp[0].parse::<u16>().unwrap_or(443);

            let semi_idx = server_part.find('?').or_else(|| server_part.find(';'));
            let params_str = semi_idx.map(|i| &server_part[i + 1..]).unwrap_or("");
            let params = parse_query_params(params_str.split('#').next().unwrap_or(params_str));

            let uuid_pass: Vec<&str> = userinfo.splitn(2, ":").collect();
            let uuid = uuid_pass[0];
            let password = uuid_pass.get(1).unwrap_or(&"");

            let default_name = format!("tuic-{}:{}", server, port);
            let name = params.get("remarks").map(|s| s.as_str()).unwrap_or(&default_name);

            let mut node = serde_json::json!({
                "name": name,
                "type": "direct",
                "server": server,
                "port": port,
                "tuic-uuid": uuid,
                "tuic-password": password,
            });
            if let Some(cc) = params.get("congestion_control") {
                node["tuic-congestion-control"] = serde_json::json!(cc);
            }
            Ok(Some(node))
        }
        "ssh" => {
            let at_idx = rest.find("@").ok_or("Invalid ssh URI")?;
            let user = urldecode(&rest[..at_idx])?;
            let server_part = &rest[at_idx + 1..];
            let hp: Vec<&str> = server_part.rsplitn(2, ":").collect();
            if hp.len() < 2 { return Ok(None); }
            let server = hp[1].trim_start_matches('[').trim_end_matches(']');
            let port = hp[0].parse::<u16>().unwrap_or(22);
            let name = format!("ssh-{}:{}", server, port);
            Ok(Some(serde_json::json!({
                "name": name,
                "type": "direct",
                "server": server,
                "port": port,
                "ssh-user": user,
            })))
        }
        "naive+https" => {
            let user_pass = &rest[..rest.find('@').unwrap_or(0)];
            let server_part = &rest[rest.find('@').map(|i| i + 1).unwrap_or(0)..];
            let hp: Vec<&str> = server_part.rsplitn(2, ":").collect();
            let server = hp.last().map(|s| s.trim_start_matches('[').trim_end_matches(']')).unwrap_or(server_part);
            let port = hp.get(0).and_then(|s| s.parse::<u16>().ok()).unwrap_or(443);
            let default_name = format!("naive-{}:{}", server, port);
            let name = default_name.clone();
            Ok(Some(serde_json::json!({
                "name": name,
                "type": "direct",
                "server": server,
                "port": port,
                "naive-user": user_pass,
            })))
        }
        "wireguard" | "wg" => {
            let hp: Vec<&str> = rest.rsplitn(2, ":").collect();
            if hp.len() < 2 { return Ok(None); }
            let server = hp[1].trim_start_matches('[').trim_end_matches(']');
            let port = hp[0].parse::<u16>().unwrap_or(51820);
            let default_name = format!("wg-{}:{}", server, port);
            let name = default_name.clone();
            Ok(Some(serde_json::json!({
                "name": name,
                "type": "direct",
                "server": server,
                "port": port,
                "wireguard": true,
            })))
        }
        _ => Ok(None),
    }
}

fn urldecode(input: &str) -> Result<String, String> {
    let decoded = input.replace("%40", "@").replace("%3A", ":").replace("%2F", "/")
        .replace("%3F", "?").replace("%3D", "=").replace("%26", "&").replace("%23", "#")
        .replace("%25", "%");
    Ok(decoded)
}

fn parse_query_params(query: &str) -> std::collections::HashMap<String, String> {
    let mut params = std::collections::HashMap::new();
    let cleaned = query.trim_start_matches('?').trim_start_matches(';');
    for part in cleaned.split('&') {
        if let Some((k, v)) = part.split_once('=') {
            let k = urldecode(k).unwrap_or_default();
            let v = urldecode(v).unwrap_or_default();
            params.insert(k, v);
        }
    }
    params
}

fn convert_singbox_to_clash(content: &str) -> Result<String, String> {
    let v: serde_json::Value = serde_json::from_str(content).map_err(|e| e.to_string())?;

    let outbounds = v["outbounds"].as_array().ok_or("No outbounds found")?;

    let mut proxies = Vec::new();
    let mut proxy_names = Vec::new();
    let mut proxy_groups = Vec::new();

    for ob in outbounds {
        let ob_type = ob["type"].as_str().unwrap_or("");
        let tag = ob["tag"].as_str().unwrap_or("");

        // Handle group types (selector, urltest, fallback, loadbalance)
        if ob_type == "selector" || ob_type == "urltest" || ob_type == "fallback" || ob_type == "loadbalance" {
            let clash_group_type = match ob_type {
                "selector" => "select",
                "urltest" => "url-test",
                "fallback" => "fallback",
                "loadbalance" => "load-balance",
                _ => "select",
            };
            let outbounds_arr = ob["outbounds"].as_array();
            let members: Vec<String> = if let Some(arr) = outbounds_arr {
                arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
            } else {
                Vec::new()
            };
            proxy_groups.push(serde_json::json!({
                "name": tag,
                "type": clash_group_type,
                "proxies": members,
            }));
            continue;
        }

        let clash_type = match ob_type {
            "shadowsocks" => "ss",
            "vmess" => "vmess",
            "vless" => "vless",
            "trojan" => "trojan",
            "hysteria2" => "hysteria2",
            "hysteria" => "hysteria",
            "shadowsocksr" => "ssr",
            "wireguard" => "wireguard",
            "tor" => "tor",
            "ssh" => "ssh",
            _ => ob_type,
        };

        let mut node = serde_json::json!({
            "name": tag,
            "type": clash_type,
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

        // TLS settings
        if ob["tls"]["enabled"] == true || ob["tls"]["enabled"] == serde_json::Value::Null && ob["tls"].is_object() {
            node["tls"] = serde_json::json!(true);
            if let Some(server_name) = ob["tls"]["server_name"].as_str() {
                node["servername"] = serde_json::json!(server_name);
            }
            if ob["tls"]["insecure"] == true {
                node["skip-cert-verify"] = serde_json::json!(true);
            }
            if let Some(alpn) = ob["tls"]["alpn"].as_array() {
                let alpn_strs: Vec<&str> = alpn.iter().filter_map(|a| a.as_str()).collect();
                if !alpn_strs.is_empty() {
                    node["alpn"] = serde_json::json!(alpn_strs);
                }
            }
            if let Some(fingerprint) = ob["tls"]["utls"]["fingerprint"].as_str() {
                node["client-fingerprint"] = serde_json::json!(fingerprint);
            }
            // Reality
            if ob["tls"]["reality"]["enabled"] == true {
                node["reality"] = serde_json::json!(true);
                if let Some(public_key) = ob["tls"]["reality"]["public_key"].as_str() {
                    node["public-key"] = serde_json::json!(public_key);
                }
                if let Some(short_id) = ob["tls"]["reality"]["short_id"].as_str() {
                    node["short-id"] = serde_json::json!(short_id);
                }
            }
        }

        // Transport settings
        if let Some(transport) = ob["transport"].as_object() {
            if let Some(transport_type) = transport.get("type").and_then(|t| t.as_str()) {
                node["network"] = serde_json::json!(transport_type);
                match transport_type {
                    "ws" => {
                        let mut ws_opts = serde_json::json!({});
                        if let Some(path) = transport.get("path").and_then(|p| p.as_str()) {
                            ws_opts["path"] = serde_json::json!(path);
                        }
                        if let Some(headers_val) = transport.get("headers") {
                            if let Some(headers) = headers_val.as_object() {
                                if let Some(host) = headers.get("Host").and_then(|h| h.as_str()) {
                                    ws_opts["headers"] = serde_json::json!({ "Host": host });
                                }
                            }
                        }
                        node["ws-opts"] = ws_opts;
                    }
                    "grpc" => {
                        let mut grpc_opts = serde_json::json!({});
                        if let Some(service_name) = transport.get("service_name").and_then(|s| s.as_str()) {
                            grpc_opts["grpc-service-name"] = serde_json::json!(service_name);
                        }
                        node["grpc-opts"] = grpc_opts;
                    }
                    "http" | "h2" => {
                        let mut h2_opts = serde_json::json!({});
                        if let Some(path) = transport.get("path").and_then(|p| p.as_str()) {
                            h2_opts["path"] = serde_json::json!(path);
                        }
                        if let Some(host_val) = transport.get("host") {
                            if let Some(host_str) = host_val.as_str() {
                                h2_opts["host"] = serde_json::json!([host_str]);
                            } else if let Some(host_arr) = host_val.as_array() {
                                h2_opts["host"] = serde_json::Value::Array(host_arr.clone());
                            }
                        }
                        node["h2-opts"] = h2_opts;
                    }
                    "quic" => {
                        let mut quic_opts = serde_json::json!({});
                        if let Some(security) = transport.get("security").and_then(|s| s.as_str()) {
                            quic_opts["quic-security"] = serde_json::json!(security);
                        }
                        if let Some(key) = transport.get("key").and_then(|k| k.as_str()) {
                            quic_opts["quic-key"] = serde_json::json!(key);
                        }
                        if let Some(obfs) = transport.get("obfs").and_then(|o| o.as_object()) {
                            if let Some(obfs_type) = obfs.get("type").and_then(|t| t.as_str()) {
                                quic_opts["obfs-type"] = serde_json::json!(obfs_type);
                            }
                            if let Some(obfs_pass) = obfs.get("password").and_then(|p| p.as_str()) {
                                quic_opts["obfs-password"] = serde_json::json!(obfs_pass);
                            }
                        }
                        node["quic-opts"] = quic_opts;
                    }
                    "httpupgrade" => {
                        let mut opts = serde_json::json!({});
                        if let Some(path) = transport.get("path").and_then(|p| p.as_str()) {
                            opts["path"] = serde_json::json!(path);
                        }
                        if let Some(headers_val) = transport.get("headers") {
                            if let Some(headers) = headers_val.as_object() {
                                if let Some(host) = headers.get("Host").and_then(|h| h.as_str()) {
                                    opts["headers"] = serde_json::json!({ "Host": host });
                                }
                            }
                        }
                        node["httpupgrade-opts"] = opts;
                    }
                    _ => {}
                }
            }
        }

        // Obfs settings (sing-box uses "obfs" field directly)
        if let Some(obfs) = ob["obfs"].as_object() {
            if let Some(obfs_type) = obfs.get("type").and_then(|t| t.as_str()) {
                node["obfs"] = serde_json::json!(obfs_type);
                if let Some(obfs_pass) = obfs.get("password").and_then(|p| p.as_str()) {
                    node["obfs-password"] = serde_json::json!(obfs_pass);
                }
                if let Some(obfs_host) = obfs.get("host").and_then(|h| h.as_str()) {
                    node["obfs-opts"] = serde_json::json!({
                        "type": obfs_type,
                        "host": obfs_host,
                    });
                }
            }
        }

        // Hysteria2 specific fields
        if clash_type == "hysteria2" || clash_type == "hysteria" {
            if let Some(up_mbps) = ob["up_mbps"].as_u64() {
                node["up"] = serde_json::json!(format!("{} mbps", up_mbps));
            }
            if let Some(down_mbps) = ob["down_mbps"].as_u64() {
                node["down"] = serde_json::json!(format!("{} mbps", down_mbps));
            }
            if let Some(sni) = ob["sni"].as_str() {
                if !node.as_object().unwrap().contains_key("sni") {
                    node["sni"] = serde_json::json!(sni);
                }
            }
        }

        proxies.push(node.clone());
        proxy_names.push(tag.to_string());
    }

    if proxies.is_empty() {
        return Err("No valid proxy nodes found in sing-box config".into());
    }

    // If no groups were extracted, create a default one
    if proxy_groups.is_empty() {
        proxy_groups.push(serde_json::json!({
            "name": "Proxy",
            "type": "select",
            "proxies": proxy_names,
        }));
    }

    // Extract route rules if present
    let mut rules: Vec<String> = Vec::new();
    if let Some(route) = v.get("route") {
        if let Some(route_rules) = route.get("rules").and_then(|r| r.as_array()) {
            for rule in route_rules {
                if let Some(clash_rule) = convert_singbox_route_rule(rule) {
                    rules.push(clash_rule);
                }
            }
        }
    }
    if rules.is_empty() {
        rules.push("MATCH,Proxy".to_string());
    }

    let config = serde_json::json!({
        "proxies": proxies,
        "proxy-groups": proxy_groups,
        "rules": rules,
    });

    serde_yaml_ng::to_string(&config).map_err(|e| e.to_string())
}

fn convert_singbox_route_rule(rule: &serde_json::Value) -> Option<String> {
    let rule_type = rule["type"].as_str()?;
    let outbound = rule["outbound"].as_str()?;

    // Map common sing-box rule types to Clash rules
    match rule_type {
        "simple" => {
            // simple rules can contain domain or ip_cidr
            if let Some(domain) = rule.get("domain").and_then(|d| d.as_str()) {
                return Some(format!("DOMAIN,{},{}", domain, outbound));
            }
            if let Some(ip_cidr) = rule.get("ip_cidr").and_then(|i| i.as_str()) {
                return Some(format!("IP-CIDR,{},{}", ip_cidr, outbound));
            }
            if let Some(ip_is_private) = rule.get("ip_is_private").and_then(|i| i.as_bool()) {
                if ip_is_private {
                    return Some(format!("GEOIP,private,{}", outbound));
                }
            }
            None
        }
        "domain" => {
            if let Some(domain) = rule.get("domain").and_then(|d| d.as_str()) {
                let rule_prefix = if rule.get("domain_suffix").is_some() || rule.get("domain_keyword").is_some() {
                    "DOMAIN-SUFFIX"
                } else {
                    "DOMAIN"
                };
                return Some(format!("{},{},{}", rule_prefix, domain, outbound));
            }
            if let Some(domain_suffix) = rule.get("domain_suffix").and_then(|d| d.as_str()) {
                return Some(format!("DOMAIN-SUFFIX,{},{}", domain_suffix, outbound));
            }
            if let Some(domain_keyword) = rule.get("domain_keyword").and_then(|d| d.as_str()) {
                return Some(format!("DOMAIN-KEYWORD,{},{}", domain_keyword, outbound));
            }
            if let Some(domain_suffixes) = rule.get("domain_suffix").and_then(|d| d.as_array()) {
                let first = domain_suffixes.first()?.as_str()?;
                return Some(format!("DOMAIN-SUFFIX,{},{}", first, outbound));
            }
            if let Some(domain_keywords) = rule.get("domain_keyword").and_then(|d| d.as_array()) {
                let first = domain_keywords.first()?.as_str()?;
                return Some(format!("DOMAIN-KEYWORD,{},{}", first, outbound));
            }
            None
        }
        "ip_cidr" | "ip" => {
            if let Some(ip_cidr) = rule.get("ip_cidr").and_then(|i| i.as_str()) {
                return Some(format!("IP-CIDR,{},{}", ip_cidr, outbound));
            }
            if let Some(ip_is_private) = rule.get("ip_is_private").and_then(|i| i.as_bool()) {
                if ip_is_private {
                    return Some(format!("GEOIP,private,{}", outbound));
                }
            }
            None
        }
        "geoip" => {
            let code = rule.get("code").and_then(|c| c.as_str()).unwrap_or("cn");
            return Some(format!("GEOIP,{},{}", code, outbound));
        }
        "protocol" => {
            let _proto = rule.get("protocol").and_then(|p| p.as_str())?;
            return Some(format!("MATCH,{}", outbound));
        }
        "process" => {
            let process = rule.get("name").and_then(|n| n.as_str())
                .or_else(|| rule.get("path").and_then(|p| p.as_str()))?;
            return Some(format!("PROCESS-NAME,{},{}", process, outbound));
        }
        _ => None,
    }
}

#[tauri::command]
pub fn convert_content(content: String, format: String) -> Result<String, String> {
    match format.as_str() {
        "clash" | "yaml" => Ok(content),
        "v2rayn" => convert_v2rayn_to_clash(&content),
        "singbox" => convert_singbox_to_clash(&content),
        _ => Err(format!("Unsupported format: {}", format)),
    }
}

#[tauri::command]
pub fn validate_content(content: String, format: String) -> Result<String, String> {
    match format.as_str() {
        "clash" | "yaml" => {
            let v: serde_yaml_ng::Value = serde_yaml_ng::from_str(&content)
                .map_err(|e| format!("YAML parse failed: {}", e))?;
            if v.get("proxies").is_none() {
                return Err("Missing required field: proxies".into());
            }
            if let Some(proxies) = v.get("proxies").and_then(|p| p.as_sequence()) {
                if proxies.is_empty() {
                    return Err("proxies list is empty".into());
                }
            }
            Ok(content)
        }
        "v2rayn" => {
            convert_v2rayn_to_clash(&content)?;
            Ok(content)
        }
        "singbox" => {
            convert_singbox_to_clash(&content)?;
            Ok(content)
        }
        _ => Err(format!("Unsupported format: {}", format)),
    }
}

#[cfg(test)]
mod temp_wco_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn wco_clash_no_groups() {
        let content = std::fs::read_to_string("/tmp/clash.yaml").unwrap();
        let sub_yaml: serde_yaml_ng::Value = serde_yaml_ng::from_str(&content).unwrap_or(serde_yaml_ng::Value::Null);
        let proxies = sub_yaml.get("proxies").cloned().unwrap_or(serde_yaml_ng::Value::Null);
        let mut proxy_groups = sub_yaml.get("proxy-groups").cloned().unwrap_or(serde_yaml_ng::Value::Null);
        let mut rules = sub_yaml.get("rules").cloned().unwrap_or(serde_yaml_ng::Value::Null);

        if proxy_groups.is_null() {
            if let Some(arr) = proxies.as_sequence() {
                let names: Vec<String> = arr.iter()
                    .filter_map(|p| p.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()))
                    .collect();
                if !names.is_empty() {
                    proxy_groups = serde_yaml_ng::Value::Sequence(
                        vec![serde_yaml_ng::to_value(serde_json::json!({
                            "name": "Proxy",
                            "type": "select",
                            "proxies": names,
                        })).unwrap_or(serde_yaml_ng::Value::Null)]
                    );
                }
            }
        }
        if rules.is_null() {
            rules = serde_yaml_ng::Value::Sequence(
                vec![serde_yaml_ng::to_value(serde_json::json!(["MATCH", "Proxy"]))
                    .unwrap_or(serde_yaml_ng::Value::Null)]
            );
        }
        let meow_config = serde_json::json!({
            "mixed-port": 7890,
            "proxies": proxies,
            "proxy-groups": proxy_groups,
            "rules": rules,
        });
        let yaml = serde_yaml_ng::to_string(&meow_config).unwrap();
        println!("OUTPUT LEN: {}", yaml.len());
        println!("OUTPUT HEAD:\n{}", yaml.chars().take(400).collect::<String>());
        let back: serde_yaml_ng::Value = serde_yaml_ng::from_str(&yaml).unwrap();
        println!("ROUNDTRIP proxies: {}", back.get("proxies").and_then(|p| p.as_sequence()).map(|a| a.len()).unwrap_or(0));
        println!("ROUNDTRIP groups: {}", back.get("proxy-groups").and_then(|p| p.as_sequence()).map(|a| a.len()).unwrap_or(0));
    }
}
