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
        <svg xmlns="http://www.w3.org/2000/svg" width="26" height="26" viewBox="0 0 512 512" class="shrink-0">
          <defs>
            <linearGradient id="sideIcon" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stop-color="#4a90d9"/>
              <stop offset="100%" stop-color="#2a6cb8"/>
            </linearGradient>
            <linearGradient id="sideAccent" x1="0" y1="0" x2="1" y2="1">
              <stop offset="0%" stop-color="#00d4ff"/>
              <stop offset="100%" stop-color="#0099cc"/>
            </linearGradient>
          </defs>
          <path d="M256 80 L408 142 L408 262 C408 344 344 406 256 436 C168 406 104 344 104 262 L104 142 Z" fill="url(#sideIcon)" stroke="url(#sideAccent)" stroke-width="14"/>
          <circle cx="256" cy="240" r="44" fill="url(#sideAccent)"/>
          <circle cx="256" cy="148" r="20" fill="#00d4ff" opacity="0.85"/>
          <circle cx="184" cy="300" r="20" fill="#00d4ff" opacity="0.85"/>
          <circle cx="328" cy="300" r="20" fill="#00d4ff" opacity="0.85"/>
          <line x1="236" y1="220" x2="210" y2="170" stroke="#00d4ff" stroke-width="6" opacity="0.6" stroke-linecap="round"/>
          <line x1="276" y1="220" x2="302" y2="170" stroke="#00d4ff" stroke-width="6" opacity="0.6" stroke-linecap="round"/>
          <line x1="236" y1="260" x2="196" y2="292" stroke="#00d4ff" stroke-width="6" opacity="0.6" stroke-linecap="round"/>
          <line x1="276" y1="260" x2="316" y2="292" stroke="#00d4ff" stroke-width="6" opacity="0.6" stroke-linecap="round"/>
        </svg>
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
        <span class="status-text">{{ app.proxyRunning ? t('sidebar.running') : t('sidebar.stopped') }}</span>
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
