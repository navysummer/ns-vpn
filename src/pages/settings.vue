<script setup lang="ts">
import { ref, computed } from "vue";
import { FolderOpen, RotateCw, ExternalLink, Save, Eye, EyeOff, Settings as SettingsIcon, Sliders, Link2, FileCode, Info, Monitor, Server, Globe, Zap, Shield, ChevronDown, ChevronRight } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import { useToast } from "@/utils/toast";

const app = useAppStore();
const { show } = useToast();

const activeTab = ref("general");

const tabs = [
  { key: "general", label: "常规设置", icon: SettingsIcon },
  { key: "subscription", label: "订阅配置", icon: Link2 },
  { key: "proxy", label: "代理", icon: Globe },
  { key: "rules", label: "规则", icon: Sliders },
  { key: "override", label: "覆写", icon: FileCode },
  { key: "about", label: "关于", icon: Info },
];

// General settings
const language = ref("zh-CN");
const themeMode = ref(app.theme);
const windowScale = ref(false);
const startAtBoot = ref(false);
const autoUpdate = ref(true);
const checkUpdateFreq = ref("daily");
const adminMode = ref(false);
const disableQuic = ref(false);
const logFontSize = ref(14);

// Subscription settings
const subscriptionUrl = ref("https://example.com/sub");
const autoRefreshSub = ref(true);
const subRefreshInterval = ref(120);

// Proxy settings
const mixedPort = ref(7890);
const apiPort = ref(9090);
const allowLan = ref(false);
let bindAddress = ref("*");
const mode = ref("rule");
const logLevel = ref("info");
const ipv6 = ref(false);
const tcpConcurrent = ref(true);
let globalClientFingerprint = ref("chrome");
let findProcess = ref("strict");
let snifferEnabled = ref(true);
let snifferOverrideDestination = ref(true);
let dnsEnhancedMode = ref("fake-ip");

// Core settings
const corePath = ref("");
const configPath = ref("");
const secret = ref("");
const showSecret = ref(false);
const serviceMode = ref("service");
let vergeVersion = ref("2.4.3");

// DNS settings
const dnsEnable = ref(true);
const dnsListen = ref("0.0.0.0:1053");
const fakeIpRange = ref("198.18.0.1/16");
const nameservers = ref(["223.5.5.5", "119.29.29.29"]);
const fallbackNameservers = ref(["8.8.8.8", "1.1.1.1"]);

// Clash config editor
const showClashConfig = ref(false);
const clashConfigText = ref(`mixed-port: 7890
allow-lan: false
bind-address: "*"
mode: rule
log-level: info
external-controller: 127.0.0.1:9090
ipv6: false
tcp-concurrent: true
find-process: strict
global-client-fingerprint: chrome
sniffer:
  enable: true
  override-destination: true

dns:
  enable: true
  listen: 0.0.0.0:1053
  enhanced-mode: fake-ip
  fake-ip-range: 198.18.0.1/16
  nameserver:
    - 223.5.5.5
    - 119.29.29.29
  fallback:
    - 8.8.8.8
    - 1.1.1.1`);

// Override settings
const overrideMerge = ref(true);
const overrideScript = ref(false);
const overrideMergeContent = ref(`# 全局扩展覆写配置
# 在此添加需要覆写的配置项`);

const overrideScriptContent = ref(`// 全局扩展脚本
// 在此添加需要执行的脚本`);

// About
const lastCheckUpdate = ref(new Date().toLocaleString("zh-CN", { hour12: false }));

const saving = ref(false);

function saveConfig() {
  saving.value = true;
  app.setTheme(themeMode.value as "dark" | "light" | "auto");
  setTimeout(() => {
    saving.value = false;
    show("设置已保存", "success");
  }, 400);
}

function saveClashConfig() {
  show("Clash 配置已保存", "success");
  showClashConfig.value = false;
}

function saveOverride() {
  show("覆写配置已保存", "success");
}

function selectCorePath() {
  show("文件选择器将在 Tauri 环境中生效", "info");
}

function selectConfigPath() {
  show("目录选择器将在 Tauri 环境中生效", "info");
}

function openClashConfig() {
  showClashConfig.value = true;
}

function checkUpdate() {
  show("正在检查更新...", "info");
  setTimeout(() => { show("当前已是最新版本", "success"); }, 1500);
}

function openInTerminal() {
  show("终端功能将在 Tauri 环境中生效", "info");
}

function openExternalController() {
  show("外部控制功能将在 Tauri 环境中生效", "info");
}

function openSubscriptionConfig() {
  show("订阅配置将在 Tauri 环境中生效", "info");
}

function openClashConfigFile() {
  showClashConfig.value = true;
}

function startService() {
  show("服务模式启动中...", "info");
}

function restartCore() {
  show("核心已重启", "success");
}

function stopCore() {
  show("核心已停止", "success");
}
</script>

<template>
  <div class="settings-page">
    <div class="settings-sidebar">
      <div class="sidebar-title">设置</div>
      <nav class="sidebar-nav">
        <button
          v-for="tab in tabs"
          :key="tab.key"
          class="sidebar-item"
          :class="{ 'sidebar-item-active': activeTab === tab.key }"
          @click="activeTab = tab.key"
        >
          <component :is="tab.icon" :size="16" />
          <span>{{ tab.label }}</span>
        </button>
      </nav>
    </div>

    <div class="settings-content">
      <!-- General Settings -->
      <template v-if="activeTab === 'general'">
        <div class="settings-section">
          <h3 class="section-title">外观</h3>
          <div class="section-body">
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">语言</div>
              </div>
              <select v-model="language" class="setting-select">
                <option value="zh-CN">简体中文</option>
                <option value="en">English</option>
              </select>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">主题设置</div>
              </div>
              <select v-model="themeMode" class="setting-select">
                <option value="dark">深色</option>
                <option value="light">浅色</option>
                <option value="auto">跟随系统</option>
              </select>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">窗口放大</div>
              </div>
              <div class="toggle" :class="{ active: windowScale }" @click="windowScale = !windowScale">
                <div class="toggle-knob"></div>
              </div>
            </div>
          </div>
        </div>

        <div class="settings-section">
          <h3 class="section-title">启动</h3>
          <div class="section-body">
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">开机自启</div>
              </div>
              <div class="toggle" :class="{ active: startAtBoot }" @click="startAtBoot = !startAtBoot">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">检查更新</div>
              </div>
              <div class="toggle" :class="{ active: autoUpdate }" @click="autoUpdate = !autoUpdate">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">检查更新频率</div>
              </div>
              <select v-model="checkUpdateFreq" class="setting-select">
                <option value="daily">每天</option>
                <option value="weekly">每周</option>
                <option value="monthly">每月</option>
              </select>
            </div>
          </div>
        </div>

        <div class="settings-section">
          <h3 class="section-title">核心</h3>
          <div class="section-body">
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">外部控制</div>
                <div class="setting-desc">外部控制地址，可通过 API 控制核心</div>
              </div>
              <button class="btn-ghost text-xs" @click="openExternalController">
                <ExternalLink :size="14" />
                打开
              </button>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">Clash 核心路径</div>
              </div>
              <button class="btn-ghost text-xs" @click="selectCorePath">
                <FolderOpen :size="14" />
                选择文件
              </button>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">服务模式</div>
                <div class="setting-desc">Tauri 服务模式启动</div>
              </div>
              <button class="btn-ghost text-xs" @click="startService">
                <Zap :size="14" />
                启动
              </button>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">服务模式启动</div>
              </div>
              <div class="toggle active">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">订阅配置</div>
                <div class="setting-desc">配置文件目录</div>
              </div>
              <button class="btn-ghost text-xs" @click="openSubscriptionConfig">
                <FolderOpen :size="14" />
                选择文件
              </button>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">Clash 配置</div>
                <div class="setting-desc">编辑 clash.yaml 配置文件</div>
              </div>
              <button class="btn-ghost text-xs" @click="openClashConfigFile">
                编辑
              </button>
            </div>
          </div>
        </div>

        <div class="settings-section">
          <h3 class="section-title">其他设置</h3>
          <div class="section-body">
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">以管理员身份运行</div>
              </div>
              <div class="toggle" :class="{ active: adminMode }" @click="adminMode = !adminMode">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">禁用 QUIC</div>
              </div>
              <div class="toggle" :class="{ active: disableQuic }" @click="disableQuic = !disableQuic">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">Log Font Size</div>
              </div>
              <input v-model.number="logFontSize" type="number" class="setting-input font-mono" style="width: 80px;" />
            </div>
          </div>
        </div>
      </template>

      <!-- Subscription Config -->
      <template v-if="activeTab === 'subscription'">
        <div class="settings-section">
          <h3 class="section-title">订阅配置</h3>
          <div class="section-body">
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">订阅文件链接</div>
                <div class="setting-desc">配置文件 URL</div>
              </div>
              <div class="flex gap-2">
                <input v-model="subscriptionUrl" class="setting-input font-mono" style="width: 300px;" placeholder="https://example.com/sub" />
                <button class="btn-ghost text-xs">导入</button>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">自动刷新订阅</div>
              </div>
              <div class="toggle" :class="{ active: autoRefreshSub }" @click="autoRefreshSub = !autoRefreshSub">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">刷新间隔 (分钟)</div>
              </div>
              <input v-model.number="subRefreshInterval" type="number" class="setting-input font-mono" style="width: 100px;" />
            </div>
          </div>
        </div>
      </template>

      <!-- Proxy Settings -->
      <template v-if="activeTab === 'proxy'">
        <div class="settings-section">
          <h3 class="section-title">代理</h3>
          <div class="section-body">
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">混合端口</div>
                <div class="setting-desc">HTTP/SOCKS5 混合代理端口</div>
              </div>
              <input v-model.number="mixedPort" type="number" class="setting-input font-mono" />
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">允许局域网连接</div>
              </div>
              <div class="toggle" :class="{ active: allowLan }" @click="allowLan = !allowLan">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">绑定地址</div>
              </div>
              <input v-model="bindAddress" class="setting-input font-mono" style="width: 120px;" placeholder="*" />
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">运行模式</div>
              </div>
              <select v-model="mode" class="setting-select">
                <option value="rule">规则</option>
                <option value="global">全局</option>
                <option value="direct">直连</option>
              </select>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">日志等级</div>
              </div>
              <select v-model="logLevel" class="setting-select">
                <option value="debug">Debug</option>
                <option value="info">Info</option>
                <option value="warning">Warning</option>
                <option value="error">Error</option>
              </select>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">IPv6</div>
              </div>
              <div class="toggle" :class="{ active: ipv6 }" @click="ipv6 = !ipv6">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">TCP 并发</div>
                <div class="setting-desc">同时建立多个 TCP 连接</div>
              </div>
              <div class="toggle" :class="{ active: tcpConcurrent }" @click="tcpConcurrent = !tcpConcurrent">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">嗅探</div>
                <div class="setting-desc">从流量中提取域名和协议</div>
              </div>
              <div class="toggle" :class="{ active: snifferEnabled }" @click="snifferEnabled = !snifferEnabled">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">嗅探覆盖目标地址</div>
              </div>
              <div class="toggle" :class="{ active: snifferOverrideDestination }" @click="snifferOverrideDestination = !snifferOverrideDestination">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">客户端指纹</div>
              </div>
              <select v-model="globalClientFingerprint" class="setting-select">
                <option value="chrome">Chrome</option>
                <option value="firefox">Firefox</option>
                <option value="safari">Safari</option>
                <option value="edge">Edge</option>
                <option value="ios">iOS</option>
                <option value="android">Android</option>
              </select>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">进程匹配模式</div>
              </div>
              <select v-model="findProcess" class="setting-select">
                <option value="strict">Strict</option>
                <option value="moderate">Moderate</option>
                <option value="off">Off</option>
              </select>
            </div>
          </div>
        </div>

        <div class="settings-section">
          <h3 class="section-title">DNS</h3>
          <div class="section-body">
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">启用 DNS</div>
              </div>
              <div class="toggle" :class="{ active: dnsEnable }" @click="dnsEnable = !dnsEnable">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">DNS 增强模式</div>
              </div>
              <select v-model="dnsEnhancedMode" class="setting-select">
                <option value="fake-ip">fake-ip</option>
                <option value="redir-host">redir-host</option>
              </select>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">DNS 监听地址</div>
              </div>
              <input v-model="dnsListen" class="setting-input font-mono" placeholder="0.0.0.0:1053" />
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">fake-ip-range</div>
              </div>
              <input v-model="fakeIpRange" class="setting-input font-mono" placeholder="198.18.0.1/16" />
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">Nameservers</div>
              </div>
              <textarea v-model="nameservers" class="setting-textarea font-mono" rows="3" placeholder="223.5.5.5&#10;119.29.29.29"></textarea>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">Fallback Nameservers</div>
              </div>
              <textarea v-model="fallbackNameservers" class="setting-textarea font-mono" rows="3" placeholder="8.8.8.8&#10;1.1.1.1"></textarea>
            </div>
          </div>
        </div>
      </template>

      <!-- Rules Settings -->
      <template v-if="activeTab === 'rules'">
        <div class="settings-section">
          <h3 class="section-title">规则</h3>
          <div class="section-body">
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">规则模式</div>
                <div class="setting-desc">规则匹配模式</div>
              </div>
              <select class="setting-select">
                <option value="domain">域名匹配</option>
                <option value="ip">IP 匹配</option>
                <option value="all">全部匹配</option>
              </select>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">GeoIP 数据库</div>
              </div>
              <button class="btn-ghost text-xs">
                <FolderOpen :size="14" />
                选择文件
              </button>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">GeoSite 数据库</div>
              </div>
              <button class="btn-ghost text-xs">
                <FolderOpen :size="14" />
                选择文件
              </button>
            </div>
          </div>
        </div>
      </template>

      <!-- Override Settings -->
      <template v-if="activeTab === 'override'">
        <div class="settings-section">
          <h3 class="section-title">全局扩展覆写配置</h3>
          <div class="section-body">
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">启用覆写</div>
              </div>
              <div class="toggle" :class="{ active: overrideMerge }" @click="overrideMerge = !overrideMerge">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div v-if="overrideMerge" class="setting-row" style="flex-direction: column; align-items: stretch;">
              <textarea v-model="overrideMergeContent" class="setting-textarea font-mono" rows="12" placeholder="# 在此添加覆写配置"></textarea>
            </div>
          </div>
        </div>

        <div class="settings-section">
          <h3 class="section-title">全局扩展脚本</h3>
          <div class="section-body">
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">启用脚本</div>
              </div>
              <div class="toggle" :class="{ active: overrideScript }" @click="overrideScript = !overrideScript">
                <div class="toggle-knob"></div>
              </div>
            </div>
            <div v-if="overrideScript" class="setting-row" style="flex-direction: column; align-items: stretch;">
              <textarea v-model="overrideScriptContent" class="setting-textarea font-mono" rows="12" placeholder="// 在此添加脚本"></textarea>
            </div>
          </div>
        </div>
      </template>

      <!-- About -->
      <template v-if="activeTab === 'about'">
        <div class="settings-section">
          <h3 class="section-title">关于</h3>
          <div class="section-body">
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">应用名称</div>
              </div>
              <span class="setting-value">NS VPN</span>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">版本</div>
              </div>
              <span class="setting-value mono">v{{ vergeVersion }}</span>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">操作系统信息</div>
              </div>
              <span class="setting-value">macOS arm64</span>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">最后检查更新</div>
              </div>
              <span class="setting-value text-xs">{{ lastCheckUpdate }}</span>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">检查更新</div>
              </div>
              <button class="btn-ghost text-xs" @click="checkUpdate">
                <RotateCw :size="14" />
                检查更新
              </button>
            </div>
            <div class="setting-row">
              <div class="setting-left">
                <div class="setting-label">GitHub</div>
              </div>
              <a href="https://github.com" target="_blank" class="btn-ghost text-xs">
                <ExternalLink :size="14" />
                打开
              </a>
            </div>
          </div>
        </div>
      </template>

      <!-- Bottom save bar -->
      <div class="settings-save-bar">
        <button class="save-btn" :disabled="saving" @click="saveConfig">
          <RotateCw v-if="saving" :size="14" class="spin" />
          <RotateCw v-else :size="14" />
          {{ saving ? "保存中..." : "保存设置" }}
        </button>
      </div>
    </div>

    <!-- Clash Config Modal -->
    <Teleport to="body">
      <Transition name="page">
        <div v-if="showClashConfig" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showClashConfig = false">
          <div class="clash-editor-modal" @click.stop>
            <div class="clash-editor-header">
              <span class="text-sm font-medium">Clash 配置编辑器</span>
              <div class="flex items-center gap-2">
                <button class="btn-ghost text-xs" @click="showClashConfig = false">取消</button>
                <button class="btn-primary text-xs" @click="saveClashConfig">
                  <Save :size="12" />
                  保存
                </button>
              </div>
            </div>
            <textarea
              v-model="clashConfigText"
              class="clash-editor-textarea"
              spellcheck="false"
            ></textarea>
          </div>
        </div>
      </Transition>
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

.settings-sidebar {
  width: 180px;
  min-width: 180px;
  background-color: var(--card-bg);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 12px;
  height: fit-content;
  position: sticky;
  top: 20px;
}

.sidebar-title {
  font-size: 16px;
  font-weight: 700;
  padding: 8px 12px 16px;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.sidebar-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 8px;
  border: none;
  background: transparent;
  cursor: pointer;
  transition: all 150ms ease;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  text-align: left;
  width: 100%;
}
.sidebar-item:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}
.sidebar-item-active {
  background-color: var(--accent);
  color: #fff !important;
}

.settings-content {
  flex: 1;
  min-width: 0;
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
}
.setting-row:last-child {
  border-bottom: none;
}

.setting-left {
  display: flex;
  flex-direction: column;
  gap: 2px;
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

.setting-select {
  border-radius: 8px;
  padding: 6px 12px;
  font-size: 13px;
  outline: none;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  cursor: pointer;
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

.setting-textarea {
  width: 100%;
  border-radius: 8px;
  padding: 10px 12px;
  font-size: 12px;
  line-height: 1.5;
  resize: vertical;
  outline: none;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.settings-save-bar {
  display: flex;
  justify-content: flex-end;
  padding: 12px 0;
  margin-top: auto;
}

.save-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 20px;
  border-radius: 8px;
  border: none;
  background-color: var(--accent);
  color: #fff;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: opacity 150ms ease;
}
.save-btn:hover {
  opacity: 0.9;
}
.save-btn:disabled {
  opacity: 0.6;
}

.clash-editor-modal {
  width: 90%;
  max-width: 800px;
  height: 80vh;
  display: flex;
  flex-direction: column;
  border-radius: 12px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border);
  overflow: hidden;
}

.clash-editor-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
}

.clash-editor-textarea {
  flex: 1;
  width: 100%;
  padding: 16px;
  font-family: "SF Mono", "Fira Code", "JetBrains Mono", monospace;
  font-size: 13px;
  line-height: 1.6;
  resize: none;
  outline: none;
  border: none;
  background-color: transparent;
  color: var(--text-primary);
  tab-size: 2;
}

.spin {
  animation: spin 1s linear infinite;
}
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
