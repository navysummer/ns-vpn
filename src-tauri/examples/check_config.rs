fn main() {
    let path = std::env::args().nth(1).expect("usage: check_config <path>");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        match meow_config::load_config(&path).await {
            Ok(cfg) => {
                println!("CONFIG OK");
                println!("mode: {:?}", cfg.general.mode);
                println!("proxies: {}", cfg.proxies.len());
                println!("rules: {}", cfg.rules.len());
                println!("listeners: {:?}", cfg.listeners.named);
            }
            Err(e) => {
                println!("CONFIG ERROR: {:?}", e);
            }
        }
    });
}
