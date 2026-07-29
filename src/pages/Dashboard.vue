<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useAppStore } from "@/stores/app";
import { useI18n } from "vue-i18n";
import SubscriptionCard from "@/components/home/SubscriptionCard.vue";
import CurrentNodeCard from "@/components/home/CurrentNodeCard.vue";
import NetworkSettingsCard from "@/components/home/NetworkSettingsCard.vue";
import TrafficStatsCard from "@/components/home/TrafficStatsCard.vue";
import WebsiteTestCard from "@/components/home/WebsiteTestCard.vue";
import IpInfoCard from "@/components/home/IpInfoCard.vue";
import CoreInfoCard from "@/components/home/CoreInfoCard.vue";
import SystemInfoCard from "@/components/home/SystemInfoCard.vue";

const app = useAppStore();
const { t } = useI18n();
const loading = ref(true);

onMounted(() => {
  setTimeout(() => { loading.value = false; }, 300);
});
</script>

<template>
  <div class="dashboard-page">
    <div class="dashboard-header">
      <h1 class="dashboard-title">{{ t('dashboard.title') }}</h1>
    </div>

    <div v-if="loading" class="flex items-center justify-center py-20" :style="{ color: 'var(--text-secondary)' }">
      <div class="spin">{{ t('common.loading') }}</div>
    </div>

    <template v-else>
      <div class="dashboard-grid">
        <div class="grid-top">
          <SubscriptionCard />
          <CurrentNodeCard />
        </div>

        <div class="grid-middle">
          <NetworkSettingsCard />
          <div class="proxy-mode-card">
            <div class="pmc-header">
              <div class="pmc-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m16 12-4-4-4 4"/><path d="M12 16V8"/></svg>
              </div>
              <span class="pmc-title">{{ t('dashboard.proxyMode') }}</span>
            </div>
            <div class="pmc-body">
              <div class="pmc-modes">
                <button
                  class="pmc-mode-btn"
                  :class="{ 'pmc-mode-active': app.proxyMode === 'rule' }"
                  @click="app.changeProxyMode('rule')"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/></svg>
                  <div>
                    <div class="pmc-mode-label">{{ t('dashboard.ruleMode') }}</div>
                    <div class="pmc-mode-desc">{{ t('dashboard.ruleModeDesc') }}</div>
                  </div>
                </button>
                <button
                  class="pmc-mode-btn"
                  :class="{ 'pmc-mode-active': app.proxyMode === 'global' }"
                  @click="app.changeProxyMode('global')"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m2 12 10 10 10-10"/><path d="m2 12 10-10 10 10"/></svg>
                  <div>
                    <div class="pmc-mode-label">{{ t('dashboard.globalMode') }}</div>
                    <div class="pmc-mode-desc">{{ t('dashboard.globalModeDesc') }}</div>
                  </div>
                </button>
                <button
                  class="pmc-mode-btn"
                  :class="{ 'pmc-mode-active': app.proxyMode === 'direct' }"
                  @click="app.changeProxyMode('direct')"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/></svg>
                  <div>
                    <div class="pmc-mode-label">{{ t('dashboard.directMode') }}</div>
                    <div class="pmc-mode-desc">{{ t('dashboard.directModeDesc') }}</div>
                  </div>
                </button>
              </div>
              <div class="pmc-hint">
                {{ t(`dashboard.${app.proxyMode}ModeHint`) }}
              </div>
            </div>
          </div>
        </div>

        <div class="grid-traffic">
          <TrafficStatsCard />
        </div>

        <div class="grid-bottom">
          <WebsiteTestCard />
          <IpInfoCard />
        </div>

        <div class="grid-info">
          <CoreInfoCard />
          <SystemInfoCard />
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.dashboard-page {
  max-width: 100%;
}

.dashboard-header {
  margin-bottom: 20px;
}

.dashboard-title {
  font-size: 22px;
  font-weight: 700;
  margin: 0;
}

.dashboard-grid {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.grid-top {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.grid-middle {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.grid-traffic {
  display: grid;
  grid-template-columns: 1fr;
  gap: 16px;
}

.grid-bottom {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.grid-info {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.proxy-mode-card {
  border-radius: 12px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
  overflow: hidden;
}

.pmc-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
}

.pmc-icon {
  width: 34px;
  height: 34px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: rgba(79, 142, 247, 0.12);
}

.pmc-title {
  font-size: 14px;
  font-weight: 600;
}

.pmc-body {
  padding: 12px 16px;
}

.pmc-modes {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.pmc-mode-btn {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: transparent;
  cursor: pointer;
  transition: all 150ms ease;
  text-align: left;
  color: var(--text-primary);
}
.pmc-mode-btn:hover {
  border-color: var(--accent);
}
.pmc-mode-active {
  border-color: var(--accent) !important;
  background-color: rgba(79, 142, 247, 0.08) !important;
}

.pmc-mode-label {
  font-size: 13px;
  font-weight: 600;
}

.pmc-mode-desc {
  font-size: 11px;
  color: var(--text-secondary);
  margin-top: 1px;
}

.pmc-hint {
  font-size: 12px;
  color: var(--text-secondary);
  text-align: center;
  padding: 8px 12px;
  border-radius: 6px;
  border: 1px dashed var(--border);
}

@media (max-width: 768px) {
  .grid-top,
  .grid-middle,
  .grid-bottom,
  .grid-info {
    grid-template-columns: 1fr;
  }
}
</style>
