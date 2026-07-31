<script setup lang="ts">
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { FileText, RefreshCw, List, Power } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import EnhancedCard from "@/components/EnhancedCard.vue";
import { useToast } from "@/utils/toast";

const app = useAppStore();
const { show } = useToast();
const router = useRouter();
const { t } = useI18n();

const STORAGE_KEY = "ns-vpn-subscriptions";
const ACTIVE_KEY = "ns-vpn-active-sub";

function getActiveSubName(): string {
  try {
    const activeId = localStorage.getItem(ACTIVE_KEY);
    if (!activeId) return "";
    const saved = localStorage.getItem(STORAGE_KEY);
    if (!saved) return "";
    const subs = JSON.parse(saved);
    const sub = subs.find((s: any) => s.id === activeId);
    return sub?.name || "";
  } catch { return ""; }
}

function goToSubscriptions() {
  router.push("/subscriptions");
}

async function toggleCore() {
  if (app.proxyRunning) {
    await app.stopCoreCmd();
    show(t("dashboard.coreStopped"), "info");
  } else {
    await app.startCoreCmd();
    show(t("dashboard.coreStarted"), "info");
  }
}
</script>

<template>
  <EnhancedCard title="" :no-padding="true">
    <template #icon>
      <FileText :size="18" style="color: var(--accent)" />
    </template>
    <template #title>
      <span class="sub-title">{{ app.proxyGroups.length }} {{ t('nav.proxies') }}</span>
    </template>
    <template #action>
      <div class="flex items-center gap-2">
        <button class="start-btn" :class="{ 'start-btn-running': app.proxyRunning }" @click="toggleCore">
          <Power :size="12" />
          {{ app.proxyRunning ? t('dashboard.stopCore') : t('dashboard.startCore') }}
        </button>
        <button class="action-btn" @click="goToSubscriptions">
          <List :size="12" />
        </button>
      </div>
    </template>
    <div class="sub-body">
      <div class="sub-row">
        <FileText :size="14" :style="{ color: 'var(--text-secondary)' }" />
        <span class="sub-label">{{ t('home.subscription.source') }}:</span>
        <span class="sub-value mono">{{ getActiveSubName() || '—' }}</span>
      </div>
      <div class="sub-row">
        <RefreshCw :size="14" :style="{ color: 'var(--text-secondary)' }" />
        <span class="sub-label">{{ t('home.subscription.updateTime') }}:</span>
        <span class="sub-value">{{ new Date().toLocaleString("zh-CN", { hour12: false }) }}</span>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.sub-title { font-size: 16px; font-weight: 700; }
.sub-body { padding: 12px 16px; display: flex; flex-direction: column; gap: 10px; }
.sub-row { display: flex; align-items: center; gap: 8px; font-size: 13px; }
.sub-label { color: var(--text-secondary); }
.sub-value { color: var(--text-primary); }
.action-btn { display: inline-flex; align-items: center; gap: 4px; padding: 4px 10px; border-radius: 6px; font-size: 12px; font-weight: 500; border: 1px solid var(--border); background: transparent; color: var(--text-primary); cursor: pointer; transition: all 150ms ease; }
.action-btn:hover { border-color: var(--accent); color: var(--accent); }
.start-btn { display: inline-flex; align-items: center; gap: 4px; padding: 4px 10px; border-radius: 6px; font-size: 12px; font-weight: 600; border: 1px solid var(--green); background: rgba(34, 197, 94, 0.1); color: var(--green); cursor: pointer; transition: all 150ms ease; }
.start-btn:hover { background: rgba(34, 197, 94, 0.2); }
.start-btn-running { border-color: var(--red); background: rgba(239, 68, 68, 0.1); color: var(--red); }
.start-btn-running:hover { background: rgba(239, 68, 68, 0.2); }
</style>
