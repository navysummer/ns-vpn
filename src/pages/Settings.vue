<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { ChevronRight, X, RotateCw, ExternalLink, FolderOpen, Copy, Trash2, Download, Upload } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import { useToast } from "@/utils/toast";
import { useI18n } from "vue-i18n";
import { enable as autostartEnable, disable as autostartDisable } from "@tauri-apps/plugin-autostart";
import { getCoreVersion, openAppDir, getConfigFileInfo, getLogDir, openLogDir as tauriOpenLogDir, getVersion, exportDiagnostics as tauriExportDiagnostics, type ConfigFileInfo } from "@/utils/tauri";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-shell";
import { save, open as dialogOpen } from "@tauri-apps/plugin-dialog";
import { writeTextFile, readTextFile } from "@tauri-apps/plugin-fs";

const app = useAppStore();
const { show } = useToast();
const { t } = useI18n();

const saving = ref(false);
const lastCheckUpdate = ref(new Date().toLocaleString("zh-CN", { hour12: false }));
const activePanel = ref<string | null>(null);
const coreVersion = ref("");
const configFileInfo = ref<ConfigFileInfo | null>(null);
const systemInfo = ref<{ version: string; os: string; arch: string; core_version: string | null } | null>(null);
const isCheckingUpdate = ref(false);
const isExporting = ref(false);
const startupScriptName = ref("");
const appVersion = ref("");

onMounted(() => {
  getVersion().then(v => appVersion.value = v).catch(() => appVersion.value = "0.0.1");
});

watch(() => app.startAtBoot, async (val) => {
  try {
    if (val) await autostartEnable();
    else await autostartDisable();
  } catch {
    show(t("common.error"), "error");
  }
});

function openPanel(name: string) {
  activePanel.value = name;
}
function closePanel() {
  activePanel.value = null;
}

// ---- Left column actions ----
function openExternalController() {
  openPanel("externalControl");
}
function openWebInterface() {
  openPanel("webInterface");
}
function openCoreInfo() {
  openPanel("coreInfo");
  coreVersion.value = "";
  configFileInfo.value = null;
  getCoreVersion().then(v => coreVersion.value = v).catch(() => {});
  getConfigFileInfo().then(i => configFileInfo.value = i).catch(() => {});
}
function updateGeoData() {
  show(t("settings.checkingUpdate"), "info");
  setTimeout(() => show(t("settings.alreadyLatest"), "success"), 2000);
}
function openTrafficTunnel() {
  openPanel("trafficTunnel");
}

// ---- Right column - Basic ----
function openStartupScript() {
  dialogOpen({
    filters: [{ name: "Scripts", extensions: ["sh", "bat", "ps1", "js"] }],
    multiple: false,
    directory: false,
  }).then(path => {
    if (path) {
      app.overrideScript = true;
      app.overrideScriptContent = path;
      startupScriptName.value = path.split("/").pop()?.split("\\").pop() || path;
      show(t("common.success"), "success");
    }
  }).catch(() => show(t("common.error"), "error"));
}
function openThemeSettings() {
  openPanel("themeSettings");
}
function openInterfaceSettings() {
  openPanel("interfaceSettings");
}
function openMiscSettings() {
  openPanel("miscSettings");
}
function openHotkeySettings() {
  openPanel("hotkeySettings");
}

// ---- Right column - Advanced ----
function openBackupSettings() {
  openPanel("backupSettings");
}
function openCurrentConfig() {
  openPanel("currentConfig");
}
function openConfigDir() {
  openAppDir().catch(() => show(t("common.error"), "error"));
}
function openLogDir() {
  tauriOpenLogDir().catch(() => show(t("common.error"), "error"));
}
function checkUpdate() {
  show(t("settings.checkingUpdate"), "info");
  setTimeout(() => {
    lastCheckUpdate.value = new Date().toLocaleString("zh-CN", { hour12: false });
    show(t("settings.alreadyLatest"), "success");
  }, 1500);
}
function openDevTools() {
  show(t("settings.fileSelectorHint"), "info");
}
function openLiteMode() {
  openPanel("liteMode");
}
function exitApp() {
  try { getCurrentWindow().close(); } catch { show(t("common.error"), "error"); }
}
function exportDiagnostics() {
  isExporting.value = true;
  save({
    filters: [{ name: "Diagnostics", extensions: ["json"] }],
    defaultPath: `ns-vpn-diagnostics-${new Date().toISOString().split("T")[0]}.json`,
  }).then(async path => {
    if (!path) { isExporting.value = false; return; }
    try {
      const diag = await tauriExportDiagnostics();
      await writeTextFile(path, JSON.stringify(diag, null, 2));
      show(t("common.success"), "success");
    } catch { show(t("common.error"), "error"); }
    isExporting.value = false;
  }).catch(() => { isExporting.value = false; show(t("common.error"), "error"); });
}

// ---- Backup actions ----
function backupConfig() {
  save({
    filters: [{ name: "YAML", extensions: ["yaml"] }],
    defaultPath: "ns-vpn-config-backup.yaml",
  }).then(async path => {
    if (!path) return;
    try {
      const info = await getConfigFileInfo();
      const content = await readTextFile(info.path);
      await writeTextFile(path, content);
      show(t("common.success"), "success");
    } catch { show(t("common.error"), "error"); }
  }).catch(() => show(t("common.error"), "error"));
}
function restoreConfig() {
  dialogOpen({
    filters: [{ name: "YAML", extensions: ["yaml", "yml"] }],
    multiple: false,
  }).then(async path => {
    if (!path) return;
    try {
      const content = await readTextFile(path);
      const info = await getConfigFileInfo();
      await writeTextFile(info.path, content);
      show(t("common.success"), "success");
    } catch { show(t("common.error"), "error"); }
  }).catch(() => show(t("common.error"), "error"));
}

// ---- Current config actions ----
function openConfigFile() {
  if (configFileInfo.value) {
    open(configFileInfo.value.path).catch(() => show(t("common.error"), "error"));
  }
}
function copyConfigPath() {
  if (configFileInfo.value) {
    navigator.clipboard.writeText(configFileInfo.value.path).then(
      () => show(t("common.success"), "success"),
      () => show(t("common.error"), "error"),
    );
  }
}

const accentColors = ["#4f8ef7", "#6366f1", "#8b5cf6", "#ec4899", "#f43f5e", "#f97316", "#22c55e", "#06b6d4", "#3b82f6"];
const bgThemeNames: Record<string, string> = { default: "默认深色", navy: "深邃海军", midnight: "午夜蓝", forest: "森林绿", warm: "暖棕", dracula: "紫罗兰" };
const bgThemeColors: Record<string, string> = { default: "#0f0f11", navy: "#0d1117", midnight: "#0a0a1a", forest: "#0a140a", warm: "#14100c", dracula: "#1e1e2e" };

const panelTitle = computed(() => {
  const key = activePanel.value;
  if (!key) return "";
  const map: Record<string, string> = {
    externalControl: "settings.externalControl",
    webInterface: "settings.webInterface",
    coreInfo: "settings.nsvpnCore",
    trafficTunnel: "settings.trafficTunnel",
    themeSettings: "settings.themeSettings",
    interfaceSettings: "settings.interfaceSettings",
    miscSettings: "settings.miscSettings",
    hotkeySettings: "settings.hotkeySettings",
    backupSettings: "settings.backupSettings",
    currentConfig: "settings.currentConfig",
    liteMode: "settings.liteMode",
  };
  return map[key] ?? key;
});
</script>

<template>
  <div class="settings-page">
    <!-- Left Column -->
    <div class="settings-left">
      <!-- System Settings -->
      <div class="settings-section">
        <h3 class="section-title">{{ t('settings.systemSettings') }}</h3>
        <div class="section-body">
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-icon">⚡</span>
              <span class="setting-label">{{ t('home.networkSettings.tunMode') }}</span>
              <span class="setting-icon-btn">⚙</span>
            </div>
            <div class="toggle" :class="{ active: app.tunMode }" @click="app.tunMode = !app.tunMode">
              <div class="toggle-knob"></div>
            </div>
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-icon">⏸</span>
              <span class="setting-label">{{ t('home.networkSettings.systemProxy') }}</span>
              <span class="setting-icon-btn">⚙</span>
            </div>
            <div class="toggle" :class="{ active: app.systemProxy }" @click="app.systemProxy = !app.systemProxy">
              <div class="toggle-knob"></div>
            </div>
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-icon">🚀</span>
              <span class="setting-label">{{ t('settings.autoStart') }}</span>
            </div>
            <div class="toggle" :class="{ active: app.startAtBoot }" @click="app.startAtBoot = !app.startAtBoot">
              <div class="toggle-knob"></div>
            </div>
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-icon">🔇</span>
              <span class="setting-label">{{ t('settings.silentStart') }}</span>
            </div>
            <div class="toggle" :class="{ active: app.silentStart }" @click="app.silentStart = !app.silentStart">
              <div class="toggle-knob"></div>
            </div>
          </div>
        </div>
      </div>

      <!-- NS-VPN Settings -->
      <div class="settings-section">
        <h3 class="section-title">{{ t('settings.nsvpnSettings') }}</h3>
        <div class="section-body">
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-icon">🌐</span>
              <span class="setting-label">{{ t('settings.allowLan') }}</span>
              <span class="setting-icon">👥</span>
            </div>
            <div class="toggle" :class="{ active: app.allowLan }" @click="app.allowLan = !app.allowLan">
              <div class="toggle-knob"></div>
            </div>
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-icon">🔧</span>
              <span class="setting-label">{{ t('settings.dnsOverride') }}</span>
              <span class="setting-icon-btn">⚙</span>
            </div>
            <div class="toggle" :class="{ active: app.dnsEnable }" @click="app.dnsEnable = !app.dnsEnable">
              <div class="toggle-knob"></div>
            </div>
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.ipv6') }}</span>
            </div>
            <div class="toggle" :class="{ active: app.ipv6 }" @click="app.ipv6 = !app.ipv6">
              <div class="toggle-knob"></div>
            </div>
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.unifiedDelay') }}</span>
              <span class="setting-icon-btn">ℹ</span>
            </div>
            <div class="toggle" :class="{ active: app.unifiedDelay }" @click="app.unifiedDelay = !app.unifiedDelay">
              <div class="toggle-knob"></div>
            </div>
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.logLevel') }}</span>
              <span class="setting-icon-btn">ℹ</span>
            </div>
            <select v-model="app.logLevel" class="setting-select">
              <option value="debug">Debug</option>
              <option value="info">Info</option>
              <option value="warning">Warning</option>
              <option value="error">Error</option>
            </select>
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.portSettings') }}</span>
            </div>
            <input v-model.number="app.mixedPort" type="number" class="setting-input font-mono" />
          </div>
          <div class="setting-row setting-row-link" @click="openExternalController">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.externalControl') }}</span>
              <span class="setting-icon-btn">⚙</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="openWebInterface">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.webInterface') }}</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="openCoreInfo">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.nsvpnCore') }}</span>
              <span class="setting-icon-btn">⚙</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="updateGeoData">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.updateGeoData') }}</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="openTrafficTunnel">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.trafficTunnel') }}</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
        </div>
      </div>
    </div>

    <!-- Right Column -->
    <div class="settings-right">
      <!-- NS-VPN Basic Settings -->
      <div class="settings-section">
        <h3 class="section-title">{{ t('settings.nsvpnBasic') }}</h3>
        <div class="section-body">
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.language') }}</span>
            </div>
            <select v-model="app.language" class="setting-select">
              <option value="zh-CN">{{ t('settings.simplifiedChinese') }}</option>
              <option value="en">English</option>
            </select>
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.themeMode') }}</span>
            </div>
            <div class="theme-btns">
              <button class="theme-btn" :class="{ active: app.theme === 'light' }" @click="app.theme = 'light'">{{ t('settings.light') }}</button>
              <button class="theme-btn" :class="{ active: app.theme === 'dark' }" @click="app.theme = 'dark'">{{ t('settings.dark') }}</button>
              <button class="theme-btn" :class="{ active: app.theme === 'auto' }" @click="app.theme = 'auto'">{{ t('settings.auto') }}</button>
            </div>
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.trayClick') }}</span>
            </div>
            <select v-model="app.trayClickAction" class="setting-select">
              <option value="show">{{ t('settings.showMainWindow') }}</option>
              <option value="toggle">{{ t('settings.toggleWindow') }}</option>
            </select>
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.copyEnvType') }}</span>
              <span class="setting-icon">📋</span>
            </div>
            <select v-model="app.copyEnvType" class="setting-select">
              <option value="bash">Bash</option>
              <option value="powershell">PowerShell</option>
              <option value="cmd">CMD</option>
            </select>
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.startupPage') }}</span>
            </div>
            <select v-model="app.startupPage" class="setting-select">
              <option value="dashboard">{{ t('nav.home') }}</option>
              <option value="proxies">{{ t('nav.proxies') }}</option>
              <option value="settings">{{ t('nav.settings') }}</option>
            </select>
          </div>
          <div class="setting-row setting-row-link" @click="openStartupScript">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.startupScript') }}</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="openThemeSettings">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.themeSettings') }}</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="openInterfaceSettings">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.interfaceSettings') }}</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="openMiscSettings">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.miscSettings') }}</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="openHotkeySettings">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.hotkeySettings') }}</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
        </div>
      </div>

      <!-- NS-VPN Advanced Settings -->
      <div class="settings-section">
        <h3 class="section-title">{{ t('settings.nsvpnAdvanced') }}</h3>
        <div class="section-body">
          <div class="setting-row setting-row-link" @click="openBackupSettings">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.backupSettings') }}</span>
              <span class="setting-icon-btn">ℹ</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="openCurrentConfig">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.currentConfig') }}</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="openConfigDir">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.configDir') }}</span>
              <span class="setting-icon-btn">ℹ</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="openLogDir">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.logDir') }}</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="checkUpdate">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.checkUpdate') }}</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="openLiteMode">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.liteMode') }}</span>
              <span class="setting-icon-btn">ℹ</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row setting-row-link" @click="exportDiagnostics">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.exportDiagnostics') }}</span>
              <span class="setting-icon">📋</span>
            </div>
            <ChevronRight :size="16" class="setting-arrow" />
          </div>
          <div class="setting-row">
            <div class="setting-left">
              <span class="setting-label">{{ t('settings.nsvpnVersion') }}</span>
              <span class="setting-icon">📋</span>
            </div>
            <span class="setting-value mono">v{{ appVersion || '0.0.1' }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Modal Panel -->
    <Teleport to="body">
      <div v-if="activePanel" class="modal-overlay" @click.self="closePanel">
        <div class="modal-panel">
          <div class="modal-header">
            <h3 class="modal-title">{{ t(panelTitle) }}</h3>
            <button class="modal-close" @click="closePanel">
              <X :size="18" />
            </button>
          </div>
          <div class="modal-body">
            <!-- External Control -->
            <template v-if="activePanel === 'externalControl'">
              <div class="modal-desc">{{ t('settings.externalControlDesc') }}</div>
              <div class="modal-field">
                <label class="modal-label">API URL</label>
                <div class="modal-row">
                  <input class="modal-input" value="http://127.0.0.1:9090" readonly />
                  <button class="modal-btn-icon" @click="show(t('common.success'), 'success')"><Copy :size="14" /></button>
                </div>
              </div>
            </template>

            <!-- Web Interface -->
            <template v-if="activePanel === 'webInterface'">
              <div class="modal-desc">{{ t('settings.webInterfaceDesc') }}</div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.webInterface') }}</label>
                <div class="modal-row">
                  <input class="modal-input" value="http://127.0.0.1:9090/ui" readonly />
                  <button class="modal-btn-icon" @click="show(t('common.success'), 'success')"><Copy :size="14" /></button>
                  <button class="modal-btn-icon" @click="show(t('settings.fileSelectorHint'), 'info')"><ExternalLink :size="14" /></button>
                </div>
              </div>
            </template>

            <!-- Core Info -->
            <template v-if="activePanel === 'coreInfo'">
              <div class="modal-field">
                <label class="modal-label">{{ t('home.coreInfo.coreVersion') }}</label>
                <div class="modal-row">
                  <input class="modal-input" :value="coreVersion || t('common.loading')" readonly />
                </div>
              </div>
            </template>

            <!-- Traffic Tunnel -->
            <template v-if="activePanel === 'trafficTunnel'">
              <div class="modal-desc">{{ t('settings.trafficTunnelDesc') }}</div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.trafficTunnel') }}</label>
                <select class="modal-select">
                  <option value="system">{{ t('home.networkSettings.systemProxy') }}</option>
                  <option value="tun">{{ t('home.networkSettings.tunMode') }}</option>
                </select>
              </div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.allowLan') }}</label>
                <div class="toggle" :class="{ active: app.allowLan }" @click="app.allowLan = !app.allowLan">
                  <div class="toggle-knob"></div>
                </div>
              </div>
            </template>

            <!-- Theme Settings -->
            <template v-if="activePanel === 'themeSettings'">
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.themeMode') }}</label>
                <div class="theme-btns modal-btns">
                  <button class="theme-btn" :class="{ active: app.theme === 'light' }" @click="app.theme = 'light'">{{ t('settings.light') }}</button>
                  <button class="theme-btn" :class="{ active: app.theme === 'dark' }" @click="app.theme = 'dark'">{{ t('settings.dark') }}</button>
                  <button class="theme-btn" :class="{ active: app.theme === 'auto' }" @click="app.theme = 'auto'">{{ t('settings.auto') }}</button>
                </div>
              </div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.accentColor') }}</label>
                <div class="color-grid">
                  <button v-for="c in accentColors" :key="c" class="color-swatch" :class="{ active: app.accentColor === c }" :style="{ background: c }" @click="app.accentColor = c">
                    <svg v-if="app.accentColor === c" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#fff" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
                  </button>
                </div>
              </div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.bgTheme') }}</label>
                <div class="bg-grid">
                  <button v-for="(label, key) in bgThemeNames" :key="key" class="bg-option" :class="{ active: app.bgColor === key }" @click="app.bgColor = key">
                    <div class="bg-preview" :style="{ background: bgThemeColors[key], border: '1px solid var(--border)' }"></div>
                    <span class="bg-name">{{ label }}</span>
                  </button>
                </div>
              </div>
            </template>

            <!-- Interface Settings -->
            <template v-if="activePanel === 'interfaceSettings'">
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.windowScale') }}</label>
                <div class="toggle" :class="{ active: app.windowScale }" @click="app.windowScale = !app.windowScale">
                  <div class="toggle-knob"></div>
                </div>
              </div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.logFontSize') }}</label>
                <div class="modal-row">
                  <input v-model.number="app.logFontSize" type="range" min="10" max="24" class="modal-range" />
                  <span class="modal-text">{{ app.logFontSize }}px</span>
                </div>
              </div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.sidebarCompact') }}</label>
                <div class="toggle" :class="{ active: app.sidebarCollapsed }" @click="app.sidebarCollapsed = !app.sidebarCollapsed">
                  <div class="toggle-knob"></div>
                </div>
              </div>
            </template>

            <!-- Misc Settings -->
            <template v-if="activePanel === 'miscSettings'">
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.adminMode') }}</label>
                <div class="toggle" :class="{ active: app.adminMode }" @click="app.adminMode = !app.adminMode">
                  <div class="toggle-knob"></div>
                </div>
              </div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.disableQuic') }}</label>
                <div class="toggle" :class="{ active: app.disableQuic }" @click="app.disableQuic = !app.disableQuic">
                  <div class="toggle-knob"></div>
                </div>
              </div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.autoUpdate') }}</label>
                <div class="toggle" :class="{ active: app.autoUpdate }" @click="app.autoUpdate = !app.autoUpdate">
                  <div class="toggle-knob"></div>
                </div>
              </div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.checkUpdateFreq') }}</label>
                <select v-model="app.checkUpdateFreq" class="modal-select">
                  <option value="daily">{{ t('settings.freqDaily') }}</option>
                  <option value="weekly">{{ t('settings.freqWeekly') }}</option>
                  <option value="monthly">{{ t('settings.freqMonthly') }}</option>
                  <option value="never">{{ t('settings.freqNever') }}</option>
                </select>
              </div>
            </template>

            <!-- Hotkey Settings -->
            <template v-if="activePanel === 'hotkeySettings'">
              <div class="modal-desc">{{ t('settings.hotkeyDesc') }}</div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.hotkeyShowHide') }}</label>
                <input class="modal-input" value="Ctrl+Shift+Space" readonly />
              </div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.hotkeyQuickSwitch') }}</label>
                <input class="modal-input" value="Ctrl+Shift+Q" readonly />
              </div>
            </template>

            <!-- Backup Settings -->
            <template v-if="activePanel === 'backupSettings'">
              <div class="modal-desc">{{ t('settings.backupDesc') }}</div>
              <div class="modal-actions">
                <button class="modal-btn primary" @click="backupConfig">
                  <Download :size="14" />
                  {{ t('settings.exportConfig') }}
                </button>
                <button class="modal-btn" @click="restoreConfig">
                  <Upload :size="14" />
                  {{ t('settings.importConfig') }}
                </button>
              </div>
              <div class="modal-field" style="margin-top:16px">
                <label class="modal-label">{{ t('settings.autoBackup') }}</label>
                <div class="toggle active">
                  <div class="toggle-knob"></div>
                </div>
              </div>
            </template>

            <!-- Current Config -->
            <template v-if="activePanel === 'currentConfig'">
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.configName') }}</label>
                <span class="modal-text">{{ configFileInfo?.path?.split('/')?.pop()?.split('\\')?.pop() || 'config.yaml' }}</span>
              </div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.configSize') }}</label>
                <span class="modal-text">{{ configFileInfo ? (configFileInfo.size / 1024).toFixed(1) + ' KB' : t('common.loading') }}</span>
              </div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.configModified') }}</label>
                <span class="modal-text">{{ configFileInfo?.modified || t('common.loading') }}</span>
              </div>
              <div class="modal-actions">
                <button class="modal-btn primary" @click="openConfigFile">
                  <FolderOpen :size="14" />
                  {{ t('settings.openConfigFile') }}
                </button>
                <button class="modal-btn" @click="copyConfigPath">
                  <Copy :size="14" />
                  {{ t('settings.copyConfigPath') }}
                </button>
              </div>
            </template>

            <!-- Lite Mode -->
            <template v-if="activePanel === 'liteMode'">
              <div class="modal-desc">{{ t('settings.liteModeDesc') }}</div>
              <div class="modal-field">
                <label class="modal-label">{{ t('settings.liteMode') }}</label>
                <div class="toggle" :class="{ active: app.liteMode }" @click="app.liteMode = !app.liteMode">
                  <div class="toggle-knob"></div>
                </div>
              </div>
            </template>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.settings-page {
  display: flex;
  gap: 16px;
  max-width: 100%;
  min-height: calc(100vh - 120px);
}

.settings-left {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.settings-right {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.settings-section {
  background-color: var(--card-bg);
  border: 1px solid var(--border);
  border-radius: 12px;
  overflow: hidden;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  padding: 12px 16px;
  margin: 0;
  border-bottom: 1px solid var(--border);
}

.section-body {
  padding: 0 16px;
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 0;
  border-bottom: 1px solid var(--border);
  min-height: 44px;
}
.setting-row:last-child {
  border-bottom: none;
}

.setting-row-link {
  cursor: pointer;
  transition: background-color 100ms ease;
}
.setting-row-link:hover {
  background-color: var(--bg-hover);
  margin: 0 -16px;
  padding: 10px 16px;
}

.setting-left {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
}

.setting-icon {
  font-size: 14px;
}

.setting-icon-btn {
  font-size: 12px;
  color: var(--text-secondary);
  cursor: help;
}

.setting-label {
  font-size: 13px;
  font-weight: 500;
}

.setting-desc {
  font-size: 11px;
  color: var(--text-secondary);
}

.setting-value {
  font-size: 13px;
  color: var(--text-secondary);
}

.setting-link {
  font-size: 13px;
  color: var(--accent);
  cursor: pointer;
}

.setting-arrow {
  color: var(--text-secondary);
}

.setting-select {
  border-radius: 8px;
  padding: 6px 12px;
  font-size: 13px;
  outline: none;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  cursor: pointer;
  min-width: 120px;
}

.setting-input {
  width: 120px;
  border-radius: 8px;
  padding: 6px 12px;
  font-size: 13px;
  text-align: right;
  outline: none;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.theme-btns {
  display: flex;
  gap: 2px;
  padding: 2px;
  border-radius: 8px;
  background-color: var(--bg-tertiary);
}

.theme-btn {
  padding: 5px 12px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  border: none;
  cursor: pointer;
  transition: all 150ms ease;
  background: transparent;
  color: var(--text-secondary);
}
.theme-btn:hover {
  color: var(--text-primary);
}
.theme-btn.active {
  background-color: var(--accent);
  color: #fff;
}

/* Modal */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.modal-panel {
  background: var(--card-bg);
  border: 1px solid var(--border);
  border-radius: 14px;
  width: 440px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0,0,0,0.3);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border);
}

.modal-title {
  font-size: 15px;
  font-weight: 600;
  margin: 0;
}

.modal-close {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 4px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
}
.modal-close:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.modal-body {
  padding: 20px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.modal-desc {
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.5;
}

.modal-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.modal-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
}

.modal-text {
  font-size: 13px;
  color: var(--text-primary);
}

.modal-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.modal-input {
  flex: 1;
  border-radius: 8px;
  padding: 8px 12px;
  font-size: 13px;
  outline: none;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.modal-select {
  border-radius: 8px;
  padding: 8px 12px;
  font-size: 13px;
  outline: none;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  cursor: pointer;
}

.modal-range {
  flex: 1;
  accent-color: var(--accent);
}

.modal-btn-icon {
  background: none;
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 6px;
  cursor: pointer;
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
}
.modal-btn-icon:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.modal-actions {
  display: flex;
  gap: 8px;
}

.modal-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px 16px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  cursor: pointer;
  transition: background-color 100ms ease;
}
.modal-btn:hover {
  background-color: var(--bg-hover);
}
.modal-btn.primary {
  background-color: var(--accent);
  color: #fff;
  border-color: var(--accent);
}
.modal-btn.primary:hover {
  opacity: 0.9;
}

.modal-btns {
  align-self: flex-start;
}

.color-grid {
  display: flex;
  gap: 8px;
}

.color-swatch {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: 2px solid transparent;
  cursor: pointer;
  transition: all 150ms ease;
  display: flex;
  align-items: center;
  justify-content: center;
}
.color-swatch:hover {
  transform: scale(1.15);
}
.color-swatch.active {
  border-color: var(--text-primary);
  box-shadow: 0 0 0 2px var(--card-bg);
}

.bg-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 8px;
}

.bg-option {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 10px 8px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: transparent;
  cursor: pointer;
  transition: all 150ms ease;
}
.bg-option:hover {
  border-color: var(--accent);
}
.bg-option.active {
  border-color: var(--accent);
  background-color: color-mix(in srgb, var(--accent) 8%, transparent);
}

.bg-preview {
  width: 100%;
  height: 40px;
  border-radius: 6px;
}

.bg-name {
  font-size: 11px;
  color: var(--text-primary);
  font-weight: 500;
}

/* Version Management */
.modal-btn-primary {
  padding: 6px 14px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  border: none;
  background-color: var(--accent);
  color: #fff;
  cursor: pointer;
  white-space: nowrap;
  transition: opacity 100ms ease;
}
.modal-btn-primary:hover:not(:disabled) { opacity: 0.9; }
.modal-btn-primary:disabled { opacity: 0.5; cursor: not-allowed; }

.version-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-top: 6px;
}

.version-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border-radius: 8px;
  background-color: var(--bg-tertiary);
  border: 1px solid var(--border);
}

.version-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.version-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.version-badge {
  font-size: 11px;
  padding: 1px 6px;
  border-radius: 4px;
  background-color: var(--accent);
  color: #fff;
}

.version-actions {
  display: flex;
  gap: 6px;
}

.modal-btn-sm {
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  border: 1px solid var(--border);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  transition: background-color 100ms ease;
}
.modal-btn-sm:hover {
  background-color: var(--bg-hover);
}
.modal-btn-sm.danger {
  border-color: #ef4444;
  color: #ef4444;
}
.modal-btn-sm.danger:hover {
  background-color: rgba(239,68,68,0.1);
}
</style>
