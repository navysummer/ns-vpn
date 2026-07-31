use std::net::TcpListener;
use std::sync::Arc;
use std::time::Duration;

fn port_in_use(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_err()
}

fn main() {
    let path = std::env::args().nth(1).expect("usage: run_kernel <path>");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let (tx, mut rx) = tokio::sync::watch::channel(false);
        let running = Arc::new(parking_lot::RwLock::new(false));

        let running_for_task = running.clone();
        let t = tokio::spawn(async move {
            match run(&path, &mut rx, &running_for_task).await {
                Ok(()) => println!("kernel exited normally"),
                Err(e) => println!("kernel error: {:?}", e),
            }
        });

        for i in 0..50 {
            tokio::time::sleep(Duration::from_millis(200)).await;
            println!(
                "[{}] running={} mixed7890={} api9090={} dns1053={}",
                i,
                *running.read(),
                port_in_use(7890),
                port_in_use(9090),
                port_in_use(1053)
            );
            if *running.read() && port_in_use(7890) && port_in_use(9090) {
                println!("SUCCESS: all ports bound");
                break;
            }
        }

        let _ = tx.send(true);
        let _ = tokio::time::timeout(Duration::from_secs(5), t).await;
    });
}

async fn run(
    config_path: &str,
    shutdown_rx: &mut tokio::sync::watch::Receiver<bool>,
    running_flag: &Arc<parking_lot::RwLock<bool>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use meow_api::log_stream;
    use meow_api::ApiServer;
    use meow_config::load_config;
    use meow_config::ListenerType;
    use meow_listener::SnifferRuntime;
    use meow_tunnel::tunnel::Tunnel;
    use dashmap::DashMap;
    use parking_lot::RwLock as PLRwLock;
    use std::net::IpAddr;

    let config = load_config(config_path)
        .await
        .map_err(|e| format!("Failed to load config: {}", e))?;
    println!("loaded config: {} proxies, {} rules", config.proxies.len(), config.rules.len());

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
                println!("DNS server error: {}", e);
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
                        println!("Listener '{}' error: {}", name, e);
                    }
                });
            }
            ListenerType::TProxy => {
                println!("TProxy not supported");
            }
        }
    }

    let api_addr: std::net::SocketAddr = "0.0.0.0:9090".parse()?;
    let (log_tx, _log_rx) = tokio::sync::broadcast::channel::<log_stream::LogMessage>(1024);

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
                println!("API server stopped with error: {}", e);
            }
        }
        _ = shutdown_rx.changed() => {
            println!("Shutdown signal received");
        }
    }

    *running_flag.write() = false;
    Ok(())
}
