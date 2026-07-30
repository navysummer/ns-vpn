<script setup lang="ts">
import { useRoute, useRouter } from "vue-router";
import { useAppStore } from "@/stores/app";
import { useI18n } from "vue-i18n";
import { Home, Wifi, FileText, Globe, ArrowLeftRight, ScrollText, Lock, Settings, ArrowUp, ArrowDown } from "lucide-vue-next";
import { computed } from "vue";
import { formatSpeed } from "@/utils/format";

const route = useRoute();
const router = useRouter();
const app = useAppStore();
const { t } = useI18n();

const uploadSpeed = computed(() => app.traffic.upload_speed);
const downloadSpeed = computed(() => app.traffic.download_speed);

const navItems = [
  { key: "home", path: "/dashboard", icon: Home },
  { key: "proxies", path: "/proxies", icon: Wifi },
  { key: "subscriptions", path: "/subscriptions", icon: FileText },
  { key: "connections", path: "/connections", icon: Globe },
  { key: "rules", path: "/rules", icon: ArrowLeftRight },
  { key: "logs", path: "/logs", icon: ScrollText },
  { key: "test", path: "/test", icon: Lock },
  { key: "settings", path: "/settings", icon: Settings },
];

const isActive = (path: string) => route.path === path;
</script>

<template>
  <aside class="sidebar-container">
    <div class="sidebar-header">
      <div class="flex items-center gap-3 overflow-hidden">
        <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="shrink-0" style="color: var(--text-primary)"><path d="M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z"/></svg>
        <span class="text-base font-semibold whitespace-nowrap">NS VPN</span>
      </div>
    </div>

    <nav class="sidebar-nav">
      <button v-for="item in navItems" :key="item.path" class="nav-item" :class="{ 'nav-item-active': isActive(item.path) }" @click="router.push(item.path)">
        <component :is="item.icon" :size="18" class="shrink-0" />
        <span class="nav-label">{{ t(`nav.${item.key}`) }}</span>
      </button>
    </nav>

    <div class="sidebar-footer">
      <div class="speed-row">
        <div class="speed-item">
          <ArrowUp :size="12" :style="{ color: 'var(--accent)' }" />
          <span class="speed-value mono" :style="{ color: 'var(--accent)' }">{{ formatSpeed(uploadSpeed) }}</span>
        </div>
        <div class="speed-item">
          <ArrowDown :size="12" :style="{ color: 'var(--green)' }" />
          <span class="speed-value mono" :style="{ color: 'var(--green)' }">{{ formatSpeed(downloadSpeed) }}</span>
        </div>
      </div>
      <div class="status-row">
        <span class="status-dot" :class="{ online: app.proxyRunning }"></span>
        <span class="status-text">{{ app.proxyRunning ? 'Running' : 'Stopped' }}</span>
      </div>
    </div>
  </aside>
</template>

<style scoped>
.sidebar-container { display: flex; flex-direction: column; border-right: 1px solid var(--border); background-color: var(--bg-secondary); width: 200px; min-width: 200px; flex-shrink: 0; }
.sidebar-header { display: flex; align-items: center; height: 56px; padding: 0 20px; border-bottom: 1px solid var(--border); flex-shrink: 0; }
.sidebar-nav { flex: 1; padding: 12px 8px; overflow-y: auto; display: flex; flex-direction: column; gap: 2px; }
.nav-item { display: flex; align-items: center; gap: 12px; height: 40px; padding: 0 16px; border-radius: 8px; font-size: 14px; font-weight: 500; transition: background-color 150ms ease, color 150ms ease; border: none; cursor: pointer; color: var(--text-secondary); background: transparent; width: 100%; overflow: hidden; white-space: nowrap; }
.nav-label { overflow: hidden; text-overflow: ellipsis; }
.nav-item:hover { background-color: var(--bg-hover); color: var(--text-primary); }
.nav-item-active { background-color: var(--accent) !important; color: #fff !important; }
.sidebar-footer { padding: 12px 20px 16px; border-top: 1px solid var(--border); flex-shrink: 0; }
.speed-row { display: flex; flex-direction: column; gap: 4px; margin-bottom: 8px; }
.speed-item { display: flex; align-items: center; gap: 6px; }
.speed-value { font-size: 13px; font-weight: 600; }
.status-row { display: flex; align-items: center; gap: 6px; font-size: 12px; color: var(--text-secondary); }
.status-dot { width: 6px; height: 6px; border-radius: 50%; background-color: var(--text-secondary); }
.status-dot.online { background-color: var(--green); }
.status-text { font-weight: 500; }
</style>
