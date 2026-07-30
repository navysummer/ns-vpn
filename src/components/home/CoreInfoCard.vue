<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { Cpu } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import { useI18n } from "vue-i18n";
import EnhancedCard from "@/components/EnhancedCard.vue";

const app = useAppStore();
const { t } = useI18n();

const uptime = ref(0);
let interval: ReturnType<typeof setInterval> | null = null;

function formatUptime(ms: number): string {
  const h = Math.floor(ms / 3600000);
  const m = Math.floor((ms % 3600000) / 60000);
  const s = Math.floor((ms % 60000) / 1000);
  return `${h}:${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
}

onMounted(() => {
  interval = setInterval(() => { if (app.proxyRunning) uptime.value += 1000; }, 1000);
});

onUnmounted(() => {
  if (interval) clearInterval(interval);
});
</script>

<template>
  <EnhancedCard :title="t('home.coreInfo.title')" :icon="Cpu" icon-color="var(--orange)">
    <div class="info-list">
      <div class="info-row">
        <span class="info-label">{{ t('home.coreInfo.coreVersion') }}</span>
        <span class="info-value mono">{{ app.coreVersion }}</span>
      </div>
      <div class="info-row">
        <span class="info-label">{{ t('home.coreInfo.systemProxyAddr') }}</span>
        <span class="info-value mono">127.0.0.1:{{ app.mixedPort }}</span>
      </div>
      <div class="info-row">
        <span class="info-label">{{ t('home.coreInfo.mixedPort') }}</span>
        <span class="info-value mono">{{ app.mixedPort }}</span>
      </div>
      <div class="info-row">
        <span class="info-label">{{ t('home.coreInfo.uptime') }}</span>
        <span class="info-value mono">{{ formatUptime(uptime) }}</span>
      </div>
      <div class="info-row">
        <span class="info-label">{{ t('home.coreInfo.rulesCount') }}</span>
        <span class="info-value mono">{{ app.rulesCount }}</span>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.info-list { display: flex; flex-direction: column; }
.info-row { display: flex; align-items: center; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid var(--border); }
.info-row:last-child { border-bottom: none; }
.info-label { font-size: 12px; color: var(--text-secondary); }
.info-value { font-size: 12px; font-weight: 500; color: var(--text-primary); }
</style>
