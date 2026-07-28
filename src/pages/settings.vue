<script setup lang="ts">
import { ref, computed } from "vue";
import { FolderOpen, RotateCw, ExternalLink, Save, ChevronDown, ChevronRight, Bug, Server, Shield, Zap, Globe, Eye, EyeOff } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import { useToast } from "@/utils/toast";
import BasePage from "@/components/BasePage.vue";

const app = useAppStore();
const { show } = useToast();

// General settings
const themeMode = ref(app.theme);
const language = ref("zh-CN");
const startAtBoot = ref(false);
const autoUpdate = ref(false);

// Proxy settings
const mixedPort = ref(7890);
const apiPort = ref(9090);
const allowLan = ref(false);
const logLevel = ref("info");
const systemProxy = ref(false);
const tunMode = ref(false);

// Core settings
const corePath = ref("");
const configPath = ref("");
const secret = ref("");
const showSecret = ref(false);

// Advanced settings
const ipv6 = ref(false);
const tcpConcurrent = ref(true);
const globalClientFingerprint = ref("chrome");
const findProcess = ref("strict");
const sniffer = ref(true);
const enhancedMode = ref("tun");
const dnsEnable = ref(true);
const dnsListen = ref("0.0.0.0:1053");

// Clash config editor
const showClashConfig = ref(false);
const clashConfigText = ref(`# Clash 配置文件
# 编辑此配置后点击保存

mixed-port: 7890
allow-lan: false
bind-address: "*"
mode: rule
log-level: info
external-controller: 127.0.0.1:9090

dns:
  enable: true
  listen: 0.0.0.0:1053
  enhanced-mode: fake-ip
  fake-ip-range: 198.18.0.1/16
  nameserver:
    - 223.5.5.5
    - 119.29.29.29

proxies: []

proxy-groups: []

rules:
  - GEOIP,CN,DIRECT
  - MATCH,SYSTEM`);

const saving = ref(false);

// Expandable sections
const expandedSections = ref<Set<string>>(new Set(["general", "proxy", "system", "advanced"]));

function toggleSection(key: string) {
  if (expandedSections.value.has(key)) {
    expandedSections.value.delete(key);
  } else {
    expandedSections.value.add(key);
  }
}

function saveConfig() {
  saving.value = true;
  app.setTheme(themeMode.value as "dark" | "light" | "auto");
  app.setSystemProxy(systemProxy.value);
  app.setTunMode(tunMode.value);
  setTimeout(() => {
    saving.value = false;
    show("设置已保存", "success");
  }, 400);
}

function saveClashConfig() {
  show("Clash 配置已保存", "success");
  showClashConfig.value = false;
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

function openInTerminal() {
  show("终端功能将在 Tauri 环境中生效", "info");
}
</script>

<template>
  <BasePage title="设置">
    <template #actions>
      <div class="flex items-center gap-2">
        <a href="https://github.com" target="_blank" class="btn-ghost text-xs">
          <ExternalLink :size="12" />
          GitHub
        </a>
        <button class="btn-primary" :disabled="saving" @click="saveConfig">
          <RotateCw v-if="saving" :size="14" class="spin" />
          <RotateCw v-else :size="14" />
          {{ saving ? "保存中..." : "保存设置" }}
        </button>
      </div>
    </template>

    <!-- General Settings -->
    <div class="setting-group">
      <div class="setting-group-header" @click="toggleSection('general')">
        <div class="flex items-center gap-2">
          <component :is="expandedSections.has('general') ? ChevronDown : ChevronRight" :size="14" />
          <span class="setting-group-title">通用设置</span>
        </div>
      </div>
      <Transition name="page">
        <div v-if="expandedSections.has('general')" class="setting-group-content">
          <div class="setting-row">
            <div>
              <div class="setting-label">主题模式</div>
              <div class="setting-desc">深色 / 浅色 / 跟随系统</div>
            </div>
            <select v-model="themeMode" class="setting-select">
              <option value="dark">深色</option>
              <option value="light">浅色</option>
              <option value="auto">跟随系统</option>
            </select>
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">语言</div>
              <div class="setting-desc">界面语言</div>
            </div>
            <select v-model="language" class="setting-select">
              <option value="zh-CN">简体中文</option>
              <option value="en">English</option>
            </select>
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">开机自启</div>
              <div class="setting-desc">系统启动时自动运行</div>
            </div>
            <div class="toggle" :class="{ active: startAtBoot }" @click="startAtBoot = !startAtBoot">
              <div class="toggle-knob"></div>
            </div>
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">自动更新</div>
              <div class="setting-desc">检查并下载新版本</div>
            </div>
            <div class="toggle" :class="{ active: autoUpdate }" @click="autoUpdate = !autoUpdate">
              <div class="toggle-knob"></div>
            </div>
          </div>
        </div>
      </Transition>
    </div>

    <!-- Proxy Settings -->
    <div class="setting-group">
      <div class="setting-group-header" @click="toggleSection('proxy')">
        <div class="flex items-center gap-2">
          <component :is="expandedSections.has('proxy') ? ChevronDown : ChevronRight" :size="14" />
          <span class="setting-group-title">代理设置</span>
        </div>
      </div>
      <Transition name="page">
        <div v-if="expandedSections.has('proxy')" class="setting-group-content">
          <div class="setting-row">
            <div>
              <div class="setting-label">混合端口</div>
              <div class="setting-desc">HTTP/SOCKS5 混合代理端口</div>
            </div>
            <input v-model.number="mixedPort" type="number" class="setting-input font-mono" />
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">API 端口</div>
              <div class="setting-desc">mihomo RESTful API 端口</div>
            </div>
            <input v-model.number="apiPort" type="number" class="setting-input font-mono" />
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">允许局域网连接</div>
              <div class="setting-desc">允许局域网设备使用代理</div>
            </div>
            <div class="toggle" :class="{ active: allowLan }" @click="allowLan = !allowLan">
              <div class="toggle-knob"></div>
            </div>
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">日志等级</div>
              <div class="setting-desc">核心日志输出级别</div>
            </div>
            <select v-model="logLevel" class="setting-select">
              <option value="debug">Debug</option>
              <option value="info">Info</option>
              <option value="warning">Warning</option>
              <option value="error">Error</option>
            </select>
          </div>
        </div>
      </Transition>
    </div>

    <!-- System Proxy -->
    <div class="setting-group">
      <div class="setting-group-header" @click="toggleSection('system')">
        <div class="flex items-center gap-2">
          <component :is="expandedSections.has('system') ? ChevronDown : ChevronRight" :size="14" />
          <span class="setting-group-title">系统代理</span>
        </div>
      </div>
      <Transition name="page">
        <div v-if="expandedSections.has('system')" class="setting-group-content">
          <div class="setting-row">
            <div class="flex items-center gap-3">
              <div class="w-8 h-8 rounded-lg flex items-center justify-center" :style="{ backgroundColor: 'rgba(79,142,247,0.15)' }">
                <Globe :size="16" style="color: var(--accent)" />
              </div>
              <div>
                <div class="setting-label">系统代理</div>
                <div class="setting-desc">劫持系统 HTTP/HTTPS 流量</div>
              </div>
            </div>
            <div class="toggle" :class="{ active: systemProxy }" @click="systemProxy = !systemProxy">
              <div class="toggle-knob"></div>
            </div>
          </div>

          <div class="setting-row">
            <div class="flex items-center gap-3">
              <div class="w-8 h-8 rounded-lg flex items-center justify-center" :style="{ backgroundColor: 'rgba(255,159,10,0.15)' }">
                <Zap :size="16" style="color: var(--orange)" />
              </div>
              <div>
                <div class="setting-label">TUN 模式</div>
                <div class="setting-desc">虚拟网卡模式，接管所有流量</div>
              </div>
            </div>
            <div class="toggle" :class="{ active: tunMode }" @click="tunMode = !tunMode">
              <div class="toggle-knob"></div>
            </div>
          </div>
        </div>
      </Transition>
    </div>

    <!-- Core & Config -->
    <div class="setting-group">
      <div class="setting-group-header" @click="toggleSection('core')">
        <div class="flex items-center gap-2">
          <component :is="expandedSections.has('core') ? ChevronDown : ChevronRight" :size="14" />
          <span class="setting-group-title">核心与配置</span>
        </div>
      </div>
      <Transition name="page">
        <div v-if="expandedSections.has('core')" class="setting-group-content">
          <div class="setting-row">
            <div>
              <div class="setting-label">核心路径</div>
              <div class="setting-desc">mihomo 核心可执行文件路径</div>
            </div>
            <button class="btn-ghost text-xs" @click="selectCorePath">
              <FolderOpen :size="14" />
              选择文件
            </button>
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">配置路径</div>
              <div class="setting-desc">配置文件目录</div>
            </div>
            <button class="btn-ghost text-xs" @click="selectConfigPath">
              <FolderOpen :size="14" />
              选择目录
            </button>
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">API 密钥</div>
              <div class="setting-desc">mihomo API 访问密钥</div>
            </div>
            <div class="relative">
              <input
                v-model="secret"
                :type="showSecret ? 'text' : 'password'"
                class="setting-input font-mono pr-8"
                placeholder="留空则不验证"
              />
              <button
                class="absolute right-2 top-1/2 -translate-y-1/2"
                @click="showSecret = !showSecret"
              >
                <Eye v-if="!showSecret" :size="12" style="color: var(--text-secondary)" />
                <EyeOff v-else :size="12" style="color: var(--text-secondary)" />
              </button>
            </div>
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">编辑 Clash 配置</div>
              <div class="setting-desc">直接编辑 clash.yaml 配置文件</div>
            </div>
            <button class="btn-ghost text-xs" @click="openClashConfig">
              <Server :size="14" />
              编辑配置
            </button>
          </div>
        </div>
      </Transition>
    </div>

    <!-- Advanced Settings -->
    <div class="setting-group">
      <div class="setting-group-header" @click="toggleSection('advanced')">
        <div class="flex items-center gap-2">
          <component :is="expandedSections.has('advanced') ? ChevronDown : ChevronRight" :size="14" />
          <span class="setting-group-title">高级设置</span>
        </div>
      </div>
      <Transition name="page">
        <div v-if="expandedSections.has('advanced')" class="setting-group-content">
          <div class="setting-row">
            <div>
              <div class="setting-label">IPv6</div>
              <div class="setting-desc">启用 IPv6 网络支持</div>
            </div>
            <div class="toggle" :class="{ active: ipv6 }" @click="ipv6 = !ipv6">
              <div class="toggle-knob"></div>
            </div>
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">TCP 并发</div>
              <div class="setting-desc">同时建立多个 TCP 连接</div>
            </div>
            <div class="toggle" :class="{ active: tcpConcurrent }" @click="tcpConcurrent = !tcpConcurrent">
              <div class="toggle-knob"></div>
            </div>
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">嗅探</div>
              <div class="setting-desc">从流量中提取域名和协议</div>
            </div>
            <div class="toggle" :class="{ active: sniffer }" @click="sniffer = !sniffer">
              <div class="toggle-knob"></div>
            </div>
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">进程匹配模式</div>
              <div class="setting-desc">查找匹配进程的方式</div>
            </div>
            <select v-model="findProcess" class="setting-select">
              <option value="strict">Strict</option>
              <option value="moderate">Moderate</option>
              <option value="off">Off</option>
            </select>
          </div>

          <div class="setting-row">
            <div>
              <div class="setting-label">客户端指纹</div>
              <div class="setting-desc">TLS 指纹伪装</div>
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
            <div>
              <div class="setting-label">DNS 设置</div>
              <div class="setting-desc">内置 DNS 服务配置</div>
            </div>
            <div class="toggle" :class="{ active: dnsEnable }" @click="dnsEnable = !dnsEnable">
              <div class="toggle-knob"></div>
            </div>
          </div>

          <div v-if="dnsEnable" class="setting-row">
            <div>
              <div class="setting-label">DNS 监听地址</div>
              <div class="setting-desc">内置 DNS 服务监听地址</div>
            </div>
            <input v-model="dnsListen" class="setting-input font-mono" placeholder="0.0.0.0:1053" />
          </div>
        </div>
      </Transition>
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
  </BasePage>
</template>

<style scoped>
.setting-group {
  border: 1px solid var(--border);
  border-radius: 12px;
  background-color: var(--card-bg);
  margin-bottom: 12px;
  overflow: hidden;
}

.setting-group-header {
  padding: 12px 16px;
  cursor: pointer;
  user-select: none;
  transition: background-color 150ms ease;
}

.setting-group-header:hover {
  background-color: var(--bg-tertiary);
}

.setting-group-title {
  font-size: 14px;
  font-weight: 600;
}

.setting-group-content {
  padding: 0 16px 12px;
  border-top: 1px solid var(--border);
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

.setting-label {
  font-size: 13px;
  font-weight: 500;
}

.setting-desc {
  font-size: 11px;
  color: var(--text-secondary);
  margin-top: 2px;
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
</style>
