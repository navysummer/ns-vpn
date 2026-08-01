let invoke: ((cmd: string, args?: Record<string, unknown>) => Promise<unknown>) | null = null;

async function getInvoke() {
  if (!invoke) {
    const core = await import("@tauri-apps/api/core");
    invoke = core.invoke;
  }
  return invoke;
}

export async function tauriInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const fn = await getInvoke();
  return fn(cmd, args) as Promise<T>;
}

// ---- Config ----
export interface AppConfig {
  mixed_port: number;
  socks_port: number;
  port: number;
  redir_port: number;
  tproxy_port: number;
  api_port: number;
  allow_lan: boolean;
  bind_address: string;
  mode: string;
  log_level: string;
  ipv6: boolean;
  external_controller: string;
  tun: { enable: boolean; stack: string; dns_hijack: string[]; auto_route: boolean; strict_route: boolean };
  dns: { enable: boolean; listen: string; enhanced_mode: string; fake_ip_range: string; nameserver: string[]; fallback: string[]; fallback_filter: boolean };
  language: string;
  theme: string;
  system_proxy: boolean;
  tun_mode: boolean;
  start_at_boot: boolean;
  silent_start: boolean;
  unified_delay: boolean;
  tray_click_action: string;
  copy_env_type: string;
  startup_page: string;
  lite_mode: boolean;
  core_path: string;
  proxy_running: boolean;
}

export const getConfig = () => tauriInvoke<AppConfig>("get_config");
export const updateConfig = (config: AppConfig) => tauriInvoke("update_config", { config });
export const resetConfig = () => tauriInvoke("reset_config");

// ---- Core ----
export interface CoreStatus {
  running: boolean;
  api_port: number;
  mixed_port: number;
}

export const startCore = () => tauriInvoke("start_core");
export const stopCore = () => tauriInvoke("stop_core");
export const restartCore = () => tauriInvoke("restart_core");
export const getCoreStatus = () => tauriInvoke<CoreStatus>("get_core_status");
export const getCoreVersion = () => tauriInvoke<string>("get_core_version");
export const autoStartCore = () => tauriInvoke<boolean>("auto_start_core");

// ---- Core Download ----
export interface CoreInfo {
  versions: string[];
  default: string;
  hasCore: boolean;
  corePath: string;
}

export interface CoreVersionInfo {
  version: string;
  is_default: boolean;
}

export const downloadCore = (channel?: string) =>
  tauriInvoke<string>("download_core", { channel });
export const checkCoreInstalled = () => tauriInvoke<CoreInfo>("check_core_installed");
export const listCoreVersions = () => tauriInvoke<CoreVersionInfo[]>("list_core_versions");
export const installCoreVersion = (version: string) =>
  tauriInvoke<string>("install_core_version", { version });
export const uninstallCoreVersion = (version: string) =>
  tauriInvoke("uninstall_core_version", { version });
export const setCoreDefaultVersion = (version: string) =>
  tauriInvoke("set_core_default_version", { version });
export const getCoreDefaultVersion = () =>
  tauriInvoke<string>("get_core_default_version");
export const installCoreWithProgress = () =>
  tauriInvoke("install_core_with_progress");

// ---- Proxy ----
export interface ProxyStatus {
  running: boolean;
  system_proxy: boolean;
  tun_mode: boolean;
  port: number;
  api_port: number;
}

export const getProxyStatus = () => tauriInvoke<ProxyStatus>("get_proxy_status");
export const setSystemProxy = (enable: boolean) => tauriInvoke("set_system_proxy", { enable });
export const setTunMode = (enable: boolean) => tauriInvoke("set_tun_mode", { enable });

// ---- System ----
export interface SystemInfo {
  version: string;
  os: string;
  arch: string;
  core_version: string | null;
}

export const getVersion = () => tauriInvoke<string>("get_version");
export const getSystemInfo = () => tauriInvoke<SystemInfo>("get_system_info");
export const openAppDir = () => tauriInvoke("open_app_dir");

// ---- IP Info ----
export interface IpInfo {
  ip: string;
  country: string;
  asn: string;
  isp: string;
  org: string;
  city: string;
  timezone: string;
}

export const fetchIpInfo = (proxyUrl?: string) => tauriInvoke<IpInfo>("fetch_ip_info", proxyUrl ? { proxy_url: proxyUrl } : undefined);

// ---- Traffic ----
export interface TrafficData {
  upload_speed: number;
  download_speed: number;
  upload_total: number;
  download_total: number;
  active_connections: number;
}

export const getTraffic = () => tauriInvoke<TrafficData>("get_traffic");

// ---- Logs ----
export interface LogEntry {
  time: string;
  level: string;
  payload: string;
  type: string;
}

export const getLogs = () => tauriInvoke<LogEntry[]>("get_logs");

// ---- Proxies ----
export interface ProxyNodeInfo {
  name: string;
  type: string;
  delay?: number;
}

export interface ProxyGroupInfo {
  name: string;
  type: string;
  now?: string;
  all: ProxyNodeInfo[];
}

export const getProxies = () => tauriInvoke<ProxyGroupInfo[]>("get_proxies");
export const selectProxy = (group: string, name: string) =>
  tauriInvoke("select_proxy", { group, name });
export const testDelay = (name: string, url?: string) =>
  tauriInvoke<number>("test_delay", { name, url });
export const changeMode = (mode: string) =>
  tauriInvoke("change_mode", { mode });

// ---- Connections ----
export interface ConnectionMeta {
  network: string;
  type: string;
  source: string;
  destination: string;
  host: string;
}

export interface ConnectionInfo {
  id: string;
  metadata: ConnectionMeta;
  upload: number;
  download: number;
  start: string;
  chains: string[];
  rule: string;
  rule_payload: string;
  source: string;
  destination: string;
  type: string;
  network: string;
}

export interface ConnectionsData {
  connections: ConnectionInfo[];
  upload_total: number;
  download_total: number;
}

export const getConnections = () => tauriInvoke<ConnectionsData>("get_connections");
export const closeConnection = (id: string) => tauriInvoke("close_connection", { id });
export const closeAllConnections = () => tauriInvoke("close_all_connections");

// ---- Rules ----
export interface RuleInfo {
  type: string;
  payload: string;
  proxy: string;
  matcher: string;
}

export const getRules = () => tauriInvoke<RuleInfo[]>("get_rules");

// ---- Subscription ----
export const applySubscription = (content: string, format: string) =>
  tauriInvoke("apply_subscription", { content, format });
export const fetchSubscriptionUrl = (url: string) =>
  tauriInvoke<string>("fetch_subscription_url", { url });
export const convertContent = (content: string, format: string) =>
  tauriInvoke<string>("convert_content", { content, format });
export const writeConfigOnly = (content: string) =>
  tauriInvoke("write_config_only", { content });
