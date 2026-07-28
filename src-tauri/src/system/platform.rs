/// Platform-specific system utilities

#[allow(dead_code)]
#[cfg(target_os = "windows")]
pub fn get_system_proxy() -> Result<(String, u16), String> {
    Ok(("127.0.0.1".to_string(), 7890))
}

#[allow(dead_code)]
#[cfg(target_os = "macos")]
pub fn get_system_proxy() -> Result<(String, u16), String> {
    Ok(("127.0.0.1".to_string(), 7890))
}

#[allow(dead_code)]
#[cfg(target_os = "linux")]
pub fn get_system_proxy() -> Result<(String, u16), String> {
    Ok(("127.0.0.1".to_string(), 7890))
}

#[allow(dead_code)]
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn get_system_proxy() -> Result<(String, u16), String> {
    Err("Unsupported platform".to_string())
}