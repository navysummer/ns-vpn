<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { useI18n } from "vue-i18n";
import { BarChart3, ArrowUp, ArrowDown, Activity, Upload, Download, Cpu } from "lucide-vue-next";
import { formatBytes, formatSpeed } from "@/utils/format";
import { useAppStore } from "@/stores/app";
import EnhancedCard from "@/components/EnhancedCard.vue";

const { t } = useI18n();
const app = useAppStore();

const uploadSpeed = ref(0);
const downloadSpeed = ref(0);
const uploadTotal = ref(0);
const downloadTotal = ref(0);
const activeConnections = ref(0);
const memoryUsage = ref(0);
const speedHistory = ref<{ up: number; down: number }[]>([]);
const maxHistory = 60;

let interval: ReturnType<typeof setInterval> | null = null;

async function updateTraffic() {
  await app.fetchTraffic();
  uploadSpeed.value = app.traffic.upload_speed;
  downloadSpeed.value = app.traffic.download_speed;
  uploadTotal.value = app.traffic.upload_total;
  downloadTotal.value = app.traffic.download_total;
  activeConnections.value = app.traffic.active_connections;
  speedHistory.value.push({ up: uploadSpeed.value, down: downloadSpeed.value });
  if (speedHistory.value.length > maxHistory) speedHistory.value.shift();
}

const chartPath = (type: "up" | "down") => {
  if (speedHistory.value.length < 2) return "";
  const w = 800, h = 120;
  const maxVal = Math.max(...speedHistory.value.map((s) => Math.max(s.up, s.down)), 1);
  const points = speedHistory.value.map((s, i) => {
    const x = (i / (speedHistory.value.length - 1)) * w;
    const y = h - ((type === "up" ? s.up : s.down) / maxVal) * h * 0.9;
    return `${x.toFixed(1)},${y.toFixed(1)}`;
  });
  return points.join(" ");
};

const gridLines = [0, 0.25, 0.5, 0.75, 1];

onMounted(() => {
  updateTraffic();
  interval = setInterval(updateTraffic, 2000);
});

onUnmounted(() => {
  if (interval) clearInterval(interval);
});
</script>

<template>
  <EnhancedCard :title="t('home.traffic.title')" :icon="BarChart3" icon-color="var(--green)">
    <template #action>
      <span class="time-badge">10 {{ t('home.traffic.minutes') }}</span>
    </template>
    <div class="chart-wrapper">
      <div class="chart-y-axis">
        <span class="y-label">1KB</span>
        <span class="y-label">512B</span>
        <span class="y-label">0</span>
      </div>
      <div class="chart-main">
        <div class="chart-area">
          <svg v-if="speedHistory.length > 1" viewBox="0 0 800 120" class="chart-svg" preserveAspectRatio="none">
            <line v-for="(g, i) in gridLines" :key="i" x1="0" :y1="g * 120" x2="800" :y2="g * 120" stroke="var(--border)" stroke-width="0.5" stroke-dasharray="4,4" />
            <polyline :points="chartPath('down')" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" opacity="0.8" />
            <polyline :points="chartPath('up')" fill="none" stroke="var(--orange)" stroke-width="1.5" stroke-linecap="round" opacity="0.6" />
          </svg>
          <div v-else class="chart-empty">{{ t('home.traffic.waitingData') }}</div>
        </div>
        <div class="chart-legend-right">
          <div class="legend-item">
            <span class="legend-dot" :style="{ backgroundColor: 'var(--orange)' }"></span>
            <span class="legend-text">{{ t('home.traffic.upload') }}</span>
          </div>
          <div class="legend-item">
            <span class="legend-dot" :style="{ backgroundColor: 'var(--accent)' }"></span>
            <span class="legend-text">{{ t('home.traffic.download') }}</span>
          </div>
        </div>
      </div>
    </div>
    <div class="stat-grid">
      <div class="stat-item">
        <div class="stat-icon" :style="{ backgroundColor: 'rgba(255,159,10,0.12)' }">
          <ArrowUp :size="16" :style="{ color: 'var(--orange)' }" />
        </div>
        <div class="stat-content">
          <div class="stat-label">{{ t('home.traffic.uploadSpeed') }}</div>
          <div class="stat-value mono" :style="{ color: 'var(--orange)' }">{{ formatSpeed(uploadSpeed) }}</div>
        </div>
      </div>
      <div class="stat-item">
        <div class="stat-icon" :style="{ backgroundColor: 'rgba(79,142,247,0.12)' }">
          <ArrowDown :size="16" :style="{ color: 'var(--accent)' }" />
        </div>
        <div class="stat-content">
          <div class="stat-label">{{ t('home.traffic.downloadSpeed') }}</div>
          <div class="stat-value mono" :style="{ color: 'var(--accent)' }">{{ formatSpeed(downloadSpeed) }}</div>
        </div>
      </div>
      <div class="stat-item">
        <div class="stat-icon" :style="{ backgroundColor: 'rgba(52,199,89,0.12)' }">
          <Activity :size="16" :style="{ color: 'var(--green)' }" />
        </div>
        <div class="stat-content">
          <div class="stat-label">{{ t('home.traffic.activeConnections') }}</div>
          <div class="stat-value mono" :style="{ color: 'var(--green)' }">{{ activeConnections }}</div>
        </div>
      </div>
      <div class="stat-item">
        <div class="stat-icon" :style="{ backgroundColor: 'rgba(255,69,58,0.12)' }">
          <Upload :size="16" :style="{ color: 'var(--red)' }" />
        </div>
        <div class="stat-content">
          <div class="stat-label">{{ t('home.traffic.uploadTotal') }}</div>
          <div class="stat-value mono">{{ formatBytes(uploadTotal) }}</div>
        </div>
      </div>
      <div class="stat-item">
        <div class="stat-icon" :style="{ backgroundColor: 'rgba(79,142,247,0.12)' }">
          <Download :size="16" :style="{ color: 'var(--accent)' }" />
        </div>
        <div class="stat-content">
          <div class="stat-label">{{ t('home.traffic.downloadTotal') }}</div>
          <div class="stat-value mono">{{ formatBytes(downloadTotal) }}</div>
        </div>
      </div>
      <div class="stat-item">
        <div class="stat-icon" :style="{ backgroundColor: 'rgba(255,69,58,0.12)' }">
          <Cpu :size="16" :style="{ color: 'var(--red)' }" />
        </div>
        <div class="stat-content">
          <div class="stat-label">{{ t('home.traffic.coreMemory') }}</div>
          <div class="stat-value mono">{{ memoryUsage }} MB</div>
        </div>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.time-badge { font-size: 11px; padding: 2px 8px; border-radius: 4px; background-color: var(--bg-tertiary); color: var(--text-secondary); }
.chart-wrapper { display: flex; gap: 8px; }
.chart-y-axis { display: flex; flex-direction: column; justify-content: space-between; font-size: 10px; color: var(--text-secondary); padding: 8px 0; }
.y-label { font-family: "SF Mono", "Fira Code", monospace; }
.chart-main { flex: 1; display: flex; gap: 12px; }
.chart-area { flex: 1; border: 1px solid var(--border); border-radius: 6px; overflow: hidden; position: relative; min-height: 140px; }
.chart-svg { width: 100%; height: 100%; }
.chart-empty { display: flex; align-items: center; justify-content: center; height: 100%; color: var(--text-secondary); font-size: 12px; }
.chart-legend-right { display: flex; flex-direction: column; justify-content: flex-start; gap: 8px; padding-top: 8px; }
.legend-item { display: flex; align-items: center; gap: 6px; font-size: 12px; }
.legend-dot { width: 8px; height: 8px; border-radius: 50%; }
.legend-text { color: var(--text-secondary); }
.stat-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; margin-top: 12px; }
.stat-item { display: flex; align-items: center; gap: 10px; padding: 12px 14px; border-radius: 10px; border: 1px solid var(--border); background-color: var(--bg-tertiary); }
.stat-icon { width: 36px; height: 36px; border-radius: 8px; display: flex; align-items: center; justify-content: center; flex-shrink: 0; }
.stat-content { display: flex; flex-direction: column; gap: 2px; }
.stat-label { font-size: 11px; color: var(--text-secondary); }
.stat-value { font-size: 15px; font-weight: 700; }
@media (max-width: 768px) { .stat-grid { grid-template-columns: repeat(2, 1fr); } }
</style>
