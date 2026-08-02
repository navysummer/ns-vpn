/// Local system proxy implementation
/// Uses platform-specific shell commands instead of external crates

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
use std::process::Command;

/// Set system proxy (HTTP/HTTPS)
pub fn set_system_proxy(host: &str, port: u16) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let service = find_network_service()?;
        Command::new("networksetup")
            .args(["-setwebproxy", &service, host, &port.to_string()])
            .output()
            .map_err(|e| format!("Failed to set HTTP proxy: {}", e))?;
        Command::new("networksetup")
            .args(["-setsecurewebproxy", &service, host, &port.to_string()])
            .output()
            .map_err(|e| format!("Failed to set HTTPS proxy: {}", e))?;
        Command::new("networksetup")
            .args(["-setwebproxystate", &service, "on"])
            .output()
            .map_err(|e| format!("Failed to enable HTTP proxy: {}", e))?;
        Command::new("networksetup")
            .args(["-setsecurewebproxystate", &service, "on"])
            .output()
            .map_err(|e| format!("Failed to enable HTTPS proxy: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        let proxy_url = format!("{}:{}", host, port);

        let output = Command::new("reg")
            .args([
                "add", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
                "/v", "ProxyEnable", "/t", "REG_DWORD", "/d", "1", "/f",
            ])
            .output()
            .map_err(|e| format!("Failed to enable proxy: {}", e))?;
        if !output.status.success() {
            return Err("Failed to enable system proxy".to_string());
        }

        let output = Command::new("reg")
            .args([
                "add", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
                "/v", "ProxyServer", "/t", "REG_SZ", "/d", &proxy_url, "/f",
            ])
            .output()
            .map_err(|e| format!("Failed to set proxy server: {}", e))?;
        if !output.status.success() {
            return Err("Failed to set proxy server address".to_string());
        }
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy", "mode", "'manual'"])
            .output()
            .map_err(|e| format!("Failed to set proxy mode: {}", e))?;
        Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.http", "host", &format!("'{}'", host)])
            .output()
            .map_err(|e| format!("Failed to set HTTP proxy host: {}", e))?;
        Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.http", "port", &port.to_string()])
            .output()
            .map_err(|e| format!("Failed to set HTTP proxy port: {}", e))?;
        Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.https", "host", &format!("'{}'", host)])
            .output()
            .map_err(|e| format!("Failed to set HTTPS proxy host: {}", e))?;
        Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.https", "port", &port.to_string()])
            .output()
            .map_err(|e| format!("Failed to set HTTPS proxy port: {}", e))?;
    }

    #[cfg(any(target_os = "ios", target_os = "android"))]
    {
        let _ = (host, port);
    }

    Ok(())
}

/// Unset system proxy
pub fn unset_system_proxy() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let service = find_network_service()?;
        Command::new("networksetup")
            .args(["-setwebproxystate", &service, "off"])
            .output()
            .map_err(|e| format!("Failed to disable HTTP proxy: {}", e))?;
        Command::new("networksetup")
            .args(["-setsecurewebproxystate", &service, "off"])
            .output()
            .map_err(|e| format!("Failed to disable HTTPS proxy: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("reg")
            .args([
                "add", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
                "/v", "ProxyEnable", "/t", "REG_DWORD", "/d", "0", "/f",
            ])
            .output()
            .map_err(|e| format!("Failed to disable proxy: {}", e))?;
        if !output.status.success() {
            return Err("Failed to disable system proxy".to_string());
        }
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy", "mode", "'none'"])
            .output()
            .map_err(|e| format!("Failed to disable proxy: {}", e))?;
    }

    Ok(())
}

/// Find the active network service on macOS
#[cfg(target_os = "macos")]
fn find_network_service() -> Result<String, String> {
    let output = Command::new("networksetup")
        .args(["-listallnetworkservices"])
        .output()
        .map_err(|e| format!("Failed to list network services: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let services: Vec<&str> = stdout
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with("An asterisk") && !l.starts_with("networksetup"))
        .collect();

    for preferred in &["Wi-Fi", "Ethernet", "USB 10/100/1000 LAN", "Thunderbolt Ethernet"] {
        if let Some(&s) = services.iter().find(|s| s.contains(preferred)) {
            return Ok(s.to_string());
        }
    }

    services
        .first()
        .map(|s| s.to_string())
        .ok_or_else(|| "No network service found".to_string())
}