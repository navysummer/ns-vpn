<script setup lang="ts">
import { ref } from "vue";
import { FolderOpen, RotateCw, ExternalLink } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import { useToast } from "@/utils/toast";
import BasePage from "@/components/BasePage.vue";

const app = useAppStore();
const { show } = useToast();

const mixedPort = ref(7890);
const apiPort = ref(9090);
const allowLan = ref(false);
const logLevel = ref("info");
const systemProxy = ref(false);
const tunMode = ref(false);
const startAtBoot = ref(false);
const autoUpdate = ref(false);
const themeMode = ref(app.theme);
const language = ref("zh-CN");
const saving = ref(false);
const corePath = ref("");
const configPath = ref("");

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

function selectCorePath() {
  show("文件选择器将在 Tauri 环境中生效", "info");
}

function selectConfigPath() {
  show("目录选择器将在 Tauri 环境中生效", "info");
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

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
      <div class="card space-y-4">
        <h2 class="setting-section-title">通用设置</h2>

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

      <div class="card space-y-4">
        <h2 class="setting-section-title">代理设置</h2>

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

      <div class="card space-y-4">
        <h2 class="setting-section-title">系统代理</h2>

        <div class="setting-row">
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-lg flex items-center justify-center" :style="{ backgroundColor: 'rgba(79,142,247,0.15)' }">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 19a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h4l2 2h6a2 2 0 0 1 2 2"/><path d="M5 19h14a2 2 0 0 0 2-2v-5a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2v5a2 2 0 0 1-2 2Z"/></svg>
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
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--orange)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="4" y="4" width="16" height="16" rx="2"/><path d="M9 8v8"/><path d="M15 8v8"/><path d="M9 12h6"/></svg>
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

      <div class="card space-y-4">
        <h2 class="setting-section-title">核心与配置</h2>

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
      </div>
    </div>
  </BasePage>
</template>

<style scoped>
.setting-section-title {
  font-size: 14px;
  font-weight: 600;
  margin: 0;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--border);
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 0;
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
  width: 100px;
  border-radius: 8px;
  padding: 6px 12px;
  font-size: 13px;
  text-align: right;
  outline: none;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}
</style>