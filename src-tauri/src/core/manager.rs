use crate::config::AppConfig;
use crate::core::client::MihomoClient;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct CoreManager {
    client: RwLock<Option<Arc<MihomoClient>>>,
    api_port: RwLock<u16>,
    skip_write: RwLock<bool>,
    running: RwLock<bool>,
    shutdown_tx: RwLock<Option<tokio::sync::watch::Sender<bool>>>,
}

impl CoreManager {
    pub fn new() -> Self {
        Self {
            client: RwLock::new(None),
            api_port: RwLock::new(9090),
            skip_write: RwLock::new(false),
            running: RwLock::new(false),
            shutdown_tx: RwLock::new(None),
        }
    }

    pub fn set_skip_write(&self, skip: bool) {
        *self.skip_write.write() = skip;
    }

    pub fn client(&self) -> Option<Arc<MihomoClient>> {
        self.client.read().clone()
    }

    pub fn is_running(&self) -> bool {
        *self.running.read()
    }

    pub async fn start(&self, config: &AppConfig) -> Result<(), String> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("ns-vpn");
        std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

        let config_path = config_dir.join("config.yaml");

        let should_skip = *self.skip_write.write();
        if !should_skip {
            self.write_config(config, &config_path).await?;
        }
        *self.skip_write.write() = false;

        ensure_geodata().await?;

        let api_port = config.api_port;
        let config_path_str = config_path.to_string_lossy().to_string();

        let (shutdown_tx, mut shutdown_rx) = tokio::sync::watch::channel(false);

        let api_port_clone = api_port;
        let running_flag = Arc::new(RwLock::new(false));

        let client = Arc::new(MihomoClient::new(api_port));

        let running_for_task = Arc::clone(&running_flag);

        tokio::spawn(async move {
            log::info!("Starting meow kernel with config: {}", config_path_str);
            match run_meow_kernel(
                &config_path_str,
                api_port_clone,
                &mut shutdown_rx,
                &running_for_task,
            ).await {
                Ok(()) => {
                    log::info!("meow kernel exited normally");
                }
                Err(e) => {
                    log::error!("meow kernel error: {}", e);
                    *running_for_task.write() = false;
                }
            }
        });

        *self.api_port.write() = api_port;
        *self.running.write() = true;
        *self.client.write() = Some(client);
        *self.shutdown_tx.write() = Some(shutdown_tx);

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        log::info!("Core started on port {}", api_port);
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), String> {
        if let Some(tx) = self.shutdown_tx.write().take() {
            let _ = tx.send(true);
        }

        *self.running.write() = false;
        *self.client.write() = None;

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        log::info!("Core stopped");
        Ok(())
    }

    pub async fn restart(&self, config: &AppConfig) -> Result<(), String> {
        self.stop().await?;
        self.start(config).await
    }

    async fn write_config(&self, config: &AppConfig, config_path: &std::path::Path) -> Result<(), String> {
        let existing = if config_path.exists() {
            std::fs::read_to_string(config_path).ok()
                .and_then(|c| serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&c).ok())
        } else {
            None
        };

        let proxies = existing.as_ref().and_then(|e| e.get("proxies")).cloned()
            .unwrap_or(serde_yaml_ng::Value::Null);
        let proxy_groups = existing.as_ref().and_then(|e| e.get("proxy-groups")).cloned()
            .unwrap_or(serde_yaml_ng::Value::Null);
        let rules = existing.as_ref().and_then(|e| e.get("rules")).cloned()
            .unwrap_or(serde_yaml_ng::Value::Null);

        let dns_fallback_filter = if config.dns.fallback_filter {
            serde_json::json!({
                "geoip": true,
                "geoip-code": "CN",
                "ipcidr": ["240.0.0.0/4"],
            })
        } else {
            serde_json::json!({
                "geoip": false,
            })
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
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(config_path, yaml).map_err(|e| e.to_string())
    }
}

async fn ensure_geodata() -> Result<(), String> {
    let mmdb_path = meow_config::default_geoip_path();
    let asn_path = meow_config::default_asn_path();
    let geosite_path = meow_config::default_geosite_path();

    let targets: Vec<(&str, &std::path::Path)> = vec![
        ("Country.mmdb", mmdb_path.as_path()),
        ("GeoLite2-ASN.mmdb", asn_path.as_path()),
        ("geosite.dat", geosite_path.as_path()),
    ];

    for (label, path) in targets {
        if path.exists() {
            continue;
        }
        let url = match label {
            "Country.mmdb" => "https://github.com/MetaCubeX/meta-rules-dat/releases/latest/download/country.mmdb",
            "GeoLite2-ASN.mmdb" => "https://github.com/P3TERX/GeoLite.mmdb/releases/latest/download/GeoLite2-ASN.mmdb",
            _ => "https://github.com/MetaCubeX/meta-rules-dat/releases/latest/download/geosite.dat",
        };
        log::info!("geodata: downloading {} to {}", label, path.display());
        meow_config::geodata::download_and_replace(url, path, None)
            .await
            .map_err(|e| format!("failed to download geodata {label}: {e}"))?;
    }

    Ok(())
}

async fn run_meow_kernel(
    config_path: &str,
    api_port: u16,
    shutdown_rx: &mut tokio::sync::watch::Receiver<bool>,
    running_flag: &Arc<RwLock<bool>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use meow_config::load_config;
    use meow_config::ListenerType;
    use meow_tunnel::tunnel::Tunnel;
    use meow_listener::SnifferRuntime;
    use meow_api::ApiServer;
    use dashmap::DashMap;
    use parking_lot::RwLock as PLRwLock;
    use std::net::IpAddr;

    let config = load_config(config_path).await
        .map_err(|e| format!("Failed to load config: {}", e))?;

    let raw_config = Arc::new(PLRwLock::new(config.raw.clone()));

    let proxy_providers: Arc<DashMap<String, Arc<meow_config::proxy_provider::ProxyProvider>>> = {
        let map = DashMap::new();
        for (name, provider) in config.proxy_providers {
            map.insert(name, provider);
        }
        Arc::new(map)
    };

    let rule_providers = Arc::new(PLRwLock::new(config.rule_providers));

    let tunnel = Tunnel::new(Arc::clone(&config.dns.resolver));
    tunnel.set_mode(config.general.mode);
    tunnel.update_proxies(config.proxies);
    tunnel.update_rules(config.rules);
    tunnel.spawn_background_tasks();

    if let Some(listen_addr) = config.dns.listen_addr {
        use meow_dns::DnsServer;
        let dns_server = DnsServer::new(Arc::clone(&config.dns.resolver), listen_addr);
        tokio::spawn(async move {
            if let Err(e) = dns_server.run().await {
                log::error!("DNS server error: {}", e);
            }
        });
    }

    let sniffer_runtime = Arc::new(SnifferRuntime::new(config.sniffer));
    let auth = config.auth;

    for nl in &config.listeners.named {
        let ip: IpAddr = nl.listen.parse().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
        let addr = std::net::SocketAddr::new(ip, nl.port);
        let name = nl.name.clone();
        match nl.listener_type {
            ListenerType::Mixed | ListenerType::Http | ListenerType::Socks5 => {
                let listener = meow_listener::MixedListener::new(tunnel.clone(), addr, name.clone())
                    .with_sniffer(Arc::clone(&sniffer_runtime))
                    .with_auth(Arc::clone(&auth))
                    .with_max_connections(nl.max_connections);
                tokio::spawn(async move {
                    if let Err(e) = listener.run().await {
                        log::error!("Listener '{}' error: {}", name, e);
                    }
                });
            }
            ListenerType::TProxy => {
                log::warn!("TProxy listener '{}' requires feature 'listener-tproxy' which is not enabled", name);
            }
        }
    }

    let api_addr: std::net::SocketAddr = format!("0.0.0.0:{}", api_port).parse()?;
    let log_tx = crate::get_log_tx();

    let api_server = ApiServer::new(
        tunnel.clone(),
        api_addr,
        config.api.secret.clone(),
        config_path.to_string(),
        Arc::clone(&raw_config),
        log_tx,
        Arc::clone(&proxy_providers),
        Arc::clone(&rule_providers),
        config.listeners.named.clone(),
        config.api.external_ui.clone(),
    );

    *running_flag.write() = true;

    tokio::select! {
        result = api_server.run() => {
            if let Err(e) = result {
                log::error!("API server stopped with error: {}", e);
            }
        }
        _ = shutdown_rx.changed() => {
            log::info!("Shutdown signal received");
        }
    }

    *running_flag.write() = false;
    Ok(())
}
