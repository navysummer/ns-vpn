use std::process::{Command, Stdio};

use crate::config::AppConfig;

#[allow(dead_code)]
pub struct ProxyManager {
    process: Option<std::process::Child>,
}

#[allow(dead_code)]
impl ProxyManager {
    pub fn new() -> Self {
        Self { process: None }
    }

    /// Start the mihomo core with the given config
    pub fn start(&mut self, config: &AppConfig) -> Result<(), String> {
        if self.process.is_some() {
            return Err("Core is already running".to_string());
        }

        let core_path = if config.core_path.is_empty() {
            "mihomo".to_string()
        } else {
            config.core_path.clone()
        };

        let child = Command::new(&core_path)
            .arg("-d")
            .arg(&config.config_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start core: {}", e))?;

        self.process = Some(child);
        log::info!("Core started successfully");
        Ok(())
    }

    /// Stop the mihomo core
    pub fn stop(&mut self) -> Result<(), String> {
        if let Some(mut child) = self.process.take() {
            child.kill().map_err(|e| format!("Failed to stop core: {}", e))?;
            child.wait().map_err(|e| format!("Failed to wait for core: {}", e))?;
            log::info!("Core stopped");
        }
        Ok(())
    }

    /// Restart the mihomo core
    pub fn restart(&mut self, config: &AppConfig) -> Result<(), String> {
        self.stop()?;
        self.start(config)
    }

    /// Check if the core is running
    pub fn is_running(&mut self) -> bool {
        if let Some(ref mut child) = self.process {
            match child.try_wait() {
                Ok(Some(_)) => {
                    self.process = None;
                    false
                }
                Ok(None) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }
}