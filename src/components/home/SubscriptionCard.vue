<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { FileText, Link, List, Power, Layers, Server, Clock } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import EnhancedCard from "@/components/EnhancedCard.vue";
import { useToast } from "@/utils/toast";

const app = useAppStore();
const { show } = useToast();
const router = useRouter();
const { t } = useI18n();

const groups = computed(() => {
  const gs = app.proxyRunning ? app.proxyGroups : app.subProxyGroups;
  return gs.filter(g => g.name !== "GLOBAL");
});
const groupCount = computed(() => groups.value.length);
const proxyCount = computed(() => {
  let total = 0;
  for (const g of groups.value) {
    total += g.all.length;
  }
  return total;
});

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
      <span class="sub-title">{{ app.activeSubName || t('home.subscription.noSub') }}</span>
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
        <Link :size="14" :style="{ color: 'var(--text-secondary)' }" />
        <span class="sub-label">{{ t('home.subscription.source') }}:</span>
        <span class="sub-value mono sub-url">{{ app.activeSubUrl || '—' }}</span>
      </div>
      <div class="sub-stats">
        <div class="stat-item">
          <Layers :size="14" :style="{ color: 'var(--accent)' }" />
          <span class="stat-value">{{ groupCount }}</span>
          <span class="stat-label">{{ t('home.subscription.groups') }}</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat-item">
          <Server :size="14" :style="{ color: 'var(--green)' }" />
          <span class="stat-value">{{ proxyCount }}</span>
          <span class="stat-label">{{ t('home.subscription.proxies') }}</span>
        </div>
      </div>
      <div class="sub-row">
        <Clock :size="14" :style="{ color: 'var(--text-secondary)' }" />
        <span class="sub-label">{{ t('home.subscription.updateTime') }}:</span>
        <span class="sub-value">{{ app.activeSubUpdateTime || '—' }}</span>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.sub-title { font-size: 16px; font-weight: 700; }
.sub-body { padding: 12px 16px; display: flex; flex-direction: column; gap: 10px; }
.sub-row { display: flex; align-items: center; gap: 8px; font-size: 13px; }
.sub-label { color: var(--text-secondary); flex-shrink: 0; }
.sub-value { color: var(--text-primary); }
.sub-url { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; max-width: 240px; font-size: 11px; }
.sub-stats { display: flex; align-items: center; gap: 12px; padding: 8px 12px; border-radius: 8px; background-color: var(--bg-tertiary); }
.stat-item { display: flex; align-items: center; gap: 6px; flex: 1; justify-content: center; }
.stat-value { font-size: 16px; font-weight: 700; color: var(--text-primary); }
.stat-label { font-size: 11px; color: var(--text-secondary); }
.stat-divider { width: 1px; height: 24px; background-color: var(--border); }
.action-btn { display: inline-flex; align-items: center; gap: 4px; padding: 4px 10px; border-radius: 6px; font-size: 12px; font-weight: 500; border: 1px solid var(--border); background: transparent; color: var(--text-primary); cursor: pointer; transition: all 150ms ease; }
.action-btn:hover { border-color: var(--accent); color: var(--accent); }
.start-btn { display: inline-flex; align-items: center; gap: 4px; padding: 4px 10px; border-radius: 6px; font-size: 12px; font-weight: 600; border: 1px solid var(--green); background: rgba(34, 197, 94, 0.1); color: var(--green); cursor: pointer; transition: all 150ms ease; }
.start-btn:hover { background: rgba(34, 197, 94, 0.2); }
.start-btn-running { border-color: var(--red); background: rgba(239, 68, 68, 0.1); color: var(--red); }
.start-btn-running:hover { background: rgba(239, 68, 68, 0.2); }
</style>