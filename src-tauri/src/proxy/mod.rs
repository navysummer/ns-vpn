/// Proxy core management module
/// Handles the mihomo core lifecycle (start, stop, restart)

#[allow(dead_code)]
pub mod manager;

#[allow(dead_code)]
pub use manager::ProxyManager;

#[allow(dead_code)]
pub struct ProxyCore {
    pub running: bool,
    pub pid: Option<u32>,
}

#[allow(dead_code)]
impl ProxyCore {
    pub fn new() -> Self {
        Self {
            running: false,
            pid: None,
        }
    }
}