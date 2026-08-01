import { defineStore } from "pinia";
import { ref, computed, watch } from "vue";
import i18n from "@/locales";
import yaml from "js-yaml";
import {
  getConfig, updateConfig, type AppConfig,
  getTraffic, type TrafficData,
  getProxies, type ProxyGroupInfo,
  getConnections, type ConnectionInfo,
  getRules, type RuleInfo,
  getLogs, type LogEntry,
  startCore, stopCore, getCoreStatus, autoStartCore,
  setSystemProxy, setTunMode,
  selectProxy as tauriSelectProxy,
  testDelay as tauriTestDelay,
  closeConnection as tauriCloseConnection,
  closeAllConnections as tauriCloseAllConnections,
  changeMode as tauriChangeMode,
} from "@/utils/tauri";

const STORAGE_KEY = "ns-vpn-settings";

function loadLocalSettings() {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    return saved ? JSON.parse(saved) : null;
  } catch { return null; }
}

function saveLocalSettings(data: Record<string, unknown>) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
}

export const useAppStore = defineStore("app", () => {
  const local = loadLocalSettings();

  // ---- Local UI settings (persisted to localStorage) ----
  const theme = ref<"dark" | "light" | "auto">(local?.theme ?? "dark");
  const language = ref(local?.language ?? "zh-CN");
  const sidebarCollapsed = ref(false);
  const logFontSize = ref(local?.logFontSize ?? 14);
  const windowScale = ref(local?.windowScale ?? false);
  const accentColor = ref(local?.accentColor ?? "#4f8ef7");
const bgColor = ref(local?.bgColor ?? "default");

  // ---- Backend-synced settings (persisted to YAML via Rust) ----
  const mixedPort = ref(local?.mixedPort ?? 7890);
  const apiPort = ref(local?.apiPort ?? 9090);
  const allowLan = ref(local?.allowLan ?? false);
  const bindAddress = ref(local?.bindAddress ?? "*");
  const proxyMode = ref(local?.proxyMode ?? "rule");
  const logLevel = ref(local?.logLevel ?? "info");
  const ipv6 = ref(local?.ipv6 ?? false);
  const systemProxy = ref(local?.systemProxy ?? false);
  const tunMode = ref(local?.tunMode ?? false);
  const corePath = ref(local?.corePath ?? "mihomo");
  const startAtBoot = ref(local?.startAtBoot ?? false);
  const silentStart = ref(local?.silentStart ?? false);
  const unifiedDelay = ref(local?.unifiedDelay ?? false);
  const trayClickAction = ref(local?.trayClickAction ?? "show");
  const copyEnvType = ref(local?.copyEnvType ?? "bash");
  const startupPage = ref(local?.startupPage ?? "dashboard");
  const liteMode = ref(local?.liteMode ?? false);
  const autoUpdate = ref(local?.autoUpdate ?? true);
  const checkUpdateFreq = ref(local?.checkUpdateFreq ?? "daily");
  const adminMode = ref(local?.adminMode ?? false);
  const disableQuic = ref(local?.disableQuic ?? false);
  const dnsEnable = ref(local?.dnsEnable ?? true);
  const dnsListen = ref(local?.dnsListen ?? "0.0.0.0:1053");
  const dnsEnhancedMode = ref(local?.dnsEnhancedMode ?? "fake-ip");
  const fakeIpRange = ref(local?.fakeIpRange ?? "198.18.0.1/16");
  const nameservers = ref(local?.nameservers ?? ["223.5.5.5", "119.29.29.29"]);
  const fallbackNameservers = ref(local?.fallbackNameservers ?? ["8.8.8.8", "1.1.1.1"]);
  const overrideMerge = ref(local?.overrideMerge ?? true);
  const overrideMergeContent = ref(local?.overrideMergeContent ?? "# override config");
  const overrideScript = ref(local?.overrideScript ?? false);
  const overrideScriptContent = ref(local?.overrideScriptContent ?? "// script");
  const tcpConcurrent = ref(local?.tcpConcurrent ?? true);
  const globalClientFingerprint = ref(local?.globalClientFingerprint ?? "chrome");
  const findProcess = ref(local?.findProcess ?? "strict");
  const snifferEnabled = ref(local?.snifferEnabled ?? true);
  const snifferOverrideDestination = ref(local?.snifferOverrideDestination ?? true);
  const secret = ref(local?.secret ?? "");
  const serviceMode = ref(local?.serviceMode ?? "service");
  const configPath = ref(local?.configPath ?? "");

  // ---- Runtime state (from backend) ----
  const proxyRunning = ref(false);
  const traffic = ref<TrafficData>({
    upload_speed: 0, download_speed: 0, upload_total: 0, download_total: 0, active_connections: 0,
  });
  const proxyGroups = ref<ProxyGroupInfo[]>([]);
  const connections = ref<ConnectionInfo[]>([]);
  const connectionUploadTotal = ref(0);
  const connectionDownloadTotal = ref(0);
  const rules = ref<RuleInfo[]>([]);
  const logs = ref<LogEntry[]>([]);
  const coreVersion = ref("v1.18.0");
  const rulesCount = ref(0);

  // ---- Subscription data (parsed from content on apply) ----
  interface SubProxyGroup {
    name: string;
    type: string;
    now?: string;
    all: { name: string; type: string; delay?: number }[];
  }
  interface SubRule {
    type: string;
    payload: string;
    proxy: string;
    behavior: string;
  }
const subProxyGroups = ref<SubProxyGroup[]>([]);
const subRules = ref<SubRule[]>([]);
const activeSubId = ref<string | null>(null);
const activeSubName = ref<string>("");
const activeSubUrl = ref<string>("");
const activeSubUpdateTime = ref<string>("");

  function setSubData(content: string) {
    try {
      const doc = yaml.load(content) as any;
      if (!doc) {
        subProxyGroups.value = [];
        subRules.value = [];
        return;
      }

      const proxiesMap = new Map<string, string>();
      if (Array.isArray(doc.proxies)) {
        for (const p of doc.proxies) {
          if (p.name) proxiesMap.set(p.name, p.type || "");
        }
      }

      if (Array.isArray(doc["proxy-groups"])) {
        subProxyGroups.value = doc["proxy-groups"].map((g: any) => ({
          name: g.name || "",
          type: g.type || "Selector",
          now: g.now || g.proxies?.[0] || "",
          all: (g.proxies || []).map((p: string) => ({
            name: p,
            type: proxiesMap.get(p) || "",
            delay: undefined,
          })),
        }));
      } else if (Array.isArray(doc.proxies)) {
        subProxyGroups.value = [{
          name: "Proxy",
          type: "Selector",
          now: doc.proxies[0]?.name || "",
          all: doc.proxies.map((p: any) => ({
            name: p.name || "",
            type: p.type || "",
            delay: undefined,
          })),
        }];
      } else {
        subProxyGroups.value = [];
      }

      if (Array.isArray(doc.rules)) {
        subRules.value = doc.rules.map((r: string) => {
          const parts = r.split(",").map((s: string) => s.trim());
          const type = parts[0] || "";
          const proxy = parts[parts.length - 1] || "";
          const payload = parts.slice(1, -1).join(",") || "";
          const behavior = type.startsWith("DOMAIN") ? "Domain" : type.startsWith("IP") || type === "GEOIP" ? "IPCIDR" : "Other";
          return { type, payload, proxy, behavior };
        });
      } else {
        subRules.value = [];
      }
    } catch {
      subProxyGroups.value = [];
      subRules.value = [];
    }
  }

  const isDark = computed(() => {
    if (theme.value === "auto") return window.matchMedia("(prefers-color-scheme: dark)").matches;
    return theme.value === "dark";
  });

  const currentProxyGroup = computed(() => proxyGroups.value.find(g => g.type === "Selector"));
  const currentNode = computed(() => currentProxyGroup.value?.now ?? "");

  // ---- Backend sync ----
  async function syncFromBackend() {
    try {
      const cfg = await getConfig();
      mixedPort.value = cfg.mixed_port;
      apiPort.value = cfg.api_port;
      allowLan.value = cfg.allow_lan;
      bindAddress.value = cfg.bind_address;
      proxyMode.value = cfg.mode;
      logLevel.value = cfg.log_level;
      ipv6.value = cfg.ipv6;
      systemProxy.value = cfg.system_proxy;
      tunMode.value = cfg.tun_mode;
      corePath.value = cfg.core_path;
      language.value = cfg.language;
      theme.value = cfg.theme as "dark" | "light" | "auto";
      startAtBoot.value = cfg.start_at_boot;
      silentStart.value = cfg.silent_start;
      unifiedDelay.value = cfg.unified_delay;
      trayClickAction.value = cfg.tray_click_action;
      copyEnvType.value = cfg.copy_env_type;
      startupPage.value = cfg.startup_page;
      liteMode.value = cfg.lite_mode;
      proxyRunning.value = cfg.proxy_running;
      dnsEnable.value = cfg.dns.enable;
      dnsListen.value = cfg.dns.listen;
      dnsEnhancedMode.value = cfg.dns.enhanced_mode;
      fakeIpRange.value = cfg.dns.fake_ip_range;
      nameservers.value = cfg.dns.nameserver;
      fallbackNameservers.value = cfg.dns.fallback;
    } catch { /* backend not available, use local */ }
  }

  async function pushToBackend() {
    try {
      const cfg: AppConfig = {
        mixed_port: mixedPort.value,
        socks_port: 7891,
        port: 7892,
        redir_port: 7893,
        tproxy_port: 7894,
        api_port: apiPort.value,
        allow_lan: allowLan.value,
        bind_address: bindAddress.value,
        mode: proxyMode.value,
        log_level: logLevel.value,
        ipv6: ipv6.value,
        external_controller: `127.0.0.1:${apiPort.value}`,
        tun: { enable: tunMode.value, stack: "mixed", dns_hijack: ["any:53"], auto_route: true, strict_route: false },
        dns: { enable: dnsEnable.value, listen: dnsListen.value, enhanced_mode: dnsEnhancedMode.value, fake_ip_range: fakeIpRange.value, nameserver: nameservers.value, fallback: fallbackNameservers.value, fallback_filter: true },
        language: language.value,
        theme: theme.value,
        system_proxy: systemProxy.value,
        tun_mode: tunMode.value,
        start_at_boot: startAtBoot.value,
        silent_start: silentStart.value,
        unified_delay: unifiedDelay.value,
        tray_click_action: trayClickAction.value,
        copy_env_type: copyEnvType.value,
        startup_page: startupPage.value,
        lite_mode: liteMode.value,
        core_path: corePath.value,
        proxy_running: proxyRunning.value,
      };
      await updateConfig(cfg);
    } catch { /* backend not available */ }
  }

  // ---- Polling ----
  let pollInterval: ReturnType<typeof setInterval> | null = null;

  async function pollTraffic() {
    try {
      const data = await getTraffic();
      traffic.value = data;
    } catch {}
  }

  async function pollProxies() {
    try {
      proxyGroups.value = await getProxies();
    } catch {}
  }

  async function pollConnections() {
    try {
      const data = await getConnections();
      connections.value = data.connections;
      connectionUploadTotal.value = data.upload_total;
      connectionDownloadTotal.value = data.download_total;
    } catch {}
  }

  async function pollRules() {
    try {
      rules.value = await getRules();
      rulesCount.value = rules.value.length;
    } catch {}
  }

  async function pollLogs() {
    try {
      const entries = await getLogs();
      if (entries.length > 0) {
        logs.value = entries;
      }
    } catch {}
  }

  async function pollCoreStatus() {
    try {
      const status = await getCoreStatus();
      proxyRunning.value = status.running;
    } catch {}
  }

  function startPolling() {
    if (pollInterval) return;
    pollTraffic();
    pollProxies();
    pollCoreStatus();
    pollInterval = setInterval(() => {
      pollTraffic();
      pollConnections();
      pollRules();
      pollLogs();
      pollCoreStatus();
    }, 2000);
    // Proxies less frequently
    setInterval(pollProxies, 10000);
  }

  function stopPolling() {
    if (pollInterval) {
      clearInterval(pollInterval);
      pollInterval = null;
    }
  }

  // ---- Actions ----
  function setTheme(t: "dark" | "light" | "auto") { theme.value = t; }
  function setProxyRunning(running: boolean) { proxyRunning.value = running; }
  function toggleSidebar() { sidebarCollapsed.value = !sidebarCollapsed.value; }

  async function setSystemProxyMode(enable: boolean) {
    try {
      await setSystemProxy(enable);
      systemProxy.value = enable;
    } catch {}
  }

  async function setTunModeEnabled(enable: boolean) {
    try {
      await setTunMode(enable);
      tunMode.value = enable;
    } catch {}
  }

  async function changeProxyMode(mode: string) {
    proxyMode.value = mode;
    await pushToBackend();
    if (proxyRunning.value) {
      try {
        await tauriChangeMode(mode);
      } catch {}
    }
  }

  async function startCoreCmd() {
    try {
      await startCore();
      proxyRunning.value = true;
    } catch {}
  }

  async function stopCoreCmd() {
    try {
      await stopCore();
      proxyRunning.value = false;
    } catch {}
  }

  async function selectProxyNode(group: string, name: string) {
    try {
      await tauriSelectProxy(group, name);
      await pollProxies();
    } catch {}
  }

  async function testNodeDelay(name: string, url?: string) {
    try {
      return await tauriTestDelay(name, url);
    } catch { return 0; }
  }

  async function closeConn(id: string) {
    try { await tauriCloseConnection(id); await pollConnections(); } catch {}
  }

  async function closeAllConns() {
    try { await tauriCloseAllConnections(); await pollConnections(); } catch {}
  }

  // Persist local settings
  watch([theme, language, sidebarCollapsed, logFontSize, windowScale, accentColor, bgColor], () => {
    saveLocalSettings({
      theme: theme.value, language: language.value,
      logFontSize: logFontSize.value, windowScale: windowScale.value,
      accentColor: accentColor.value, bgColor: bgColor.value,
    });
  }, { deep: true });

  // Sync language with i18n
  watch(language, (val) => { i18n.global.locale.value = val; }, { immediate: true });

  return {
    // Local
    theme, language, sidebarCollapsed, logFontSize, windowScale, accentColor, bgColor, isDark,
    // Backend-synced
    mixedPort, apiPort, allowLan, bindAddress, proxyMode, logLevel, ipv6,
    systemProxy, tunMode, corePath, startAtBoot, silentStart, unifiedDelay,
    trayClickAction, copyEnvType, startupPage, liteMode, autoUpdate,
    checkUpdateFreq, adminMode, disableQuic, dnsEnable, dnsListen,
    dnsEnhancedMode, fakeIpRange, nameservers, fallbackNameservers,
    overrideMerge, overrideMergeContent, overrideScript, overrideScriptContent,
    tcpConcurrent, globalClientFingerprint, findProcess, snifferEnabled,
    snifferOverrideDestination, secret, serviceMode, configPath,
    // Runtime
    proxyRunning, traffic, proxyGroups, connections, rules, logs,
    coreVersion, rulesCount, connectionUploadTotal, connectionDownloadTotal,
    currentProxyGroup, currentNode,
    // Subscription data
    subProxyGroups, subRules, setSubData, activeSubId, activeSubName, activeSubUrl, activeSubUpdateTime,
    // Actions
    setTheme, setProxyRunning, toggleSidebar, setSystemProxyMode,
    setTunModeEnabled, changeProxyMode, startCoreCmd, stopCoreCmd,
    selectProxyNode, testNodeDelay, closeConn, closeAllConns,
    syncFromBackend, pushToBackend, startPolling, stopPolling,
    pollTraffic, pollProxies, pollConnections, pollRules, pollLogs, pollCoreStatus,
  };
});
