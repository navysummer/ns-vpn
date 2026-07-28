/// System utilities module

#[allow(dead_code)]
pub mod platform;
pub mod proxy;

use serde::Serialize;

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub hostname: String,
    pub kernel_version: String,
}

#[allow(dead_code)]
impl SystemInfo {
    pub fn new() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            hostname: hostname::get()
                .map(|h| h.to_string_lossy().to_string())
                .unwrap_or_else(|_| "unknown".to_string()),
            kernel_version: String::new(),
        }
    }
}