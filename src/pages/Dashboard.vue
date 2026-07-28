<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { ArrowUp, ArrowDown, Activity, Cpu } from "lucide-vue-next";
import { formatBytes, formatSpeed } from "@/utils/format";
import BasePage from "@/components/BasePage.vue";
import EnhancedCard from "@/components/EnhancedCard.vue";
import ClashInfoCard from "@/components/home/ClashInfoCard.vue";
import SystemInfoCard from "@/components/home/SystemInfoCard.vue";
import IpInfoCard from "@/components/home/IpInfoCard.vue";
import CurrentProxyCard from "@/components/home/CurrentProxyCard.vue";

const uploadSpeed = ref(0);
const downloadSpeed = ref(0);
const uploadTotal = ref(0);
const downloadTotal = ref(0);
const activeConnections = ref(0);
const memoryUsage = ref(0);
const coreRunning = ref(false);
const loading = ref(true);

const speedHistory = ref<{ up: number; down: number }[]>([]);
const maxHistory = 60;

let interval: ReturnType<typeof setInterval> | null = null;

function updateTraffic() {
  const up = Math.random() * 50000;
  const down = Math.random() * 500000;
  uploadSpeed.value = up;
  downloadSpeed.value = down;
  uploadTotal.value += up * 0.1;
  downloadTotal.value += down * 0.1;
  activeConnections.value = Math.floor(Math.random() * 50) + 5;
  memoryUsage.value = Math.floor(Math.random() * 80) + 20;
  coreRunning.value = true;

  speedHistory.value.push({ up, down });
  if (speedHistory.value.length > maxHistory) {
    speedHistory.value.shift();
  }
}

const chartPath = (type: "up" | "down") => {
  if (speedHistory.value.length < 2) return "";
  const w = 200;
  const h = 50;
  const maxVal = Math.max(...speedHistory.value.map((s) => Math.max(s.up, s.down)), 1);
  const points = speedHistory.value.map((s, i) => {
    const x = (i / (speedHistory.value.length - 1)) * w;
    const y = h - ((type === "up" ? s.up : s.down) / maxVal) * h * 0.9;
    return `${x.toFixed(1)},${y.toFixed(1)}`;
  });
  return points.join(" ");
};

onMounted(() => {
  setTimeout(() => { loading.value = false; }, 300);
  updateTraffic();
  interval = setInterval(updateTraffic, 2000);
});

onUnmounted(() => {
  if (interval) clearInterval(interval);
});
</script>

<template>
  <BasePage title="仪表盘">
    <template #actions>
      <div class="flex items-center gap-2 text-sm" :style="{ color: coreRunning ? 'var(--green)' : 'var(--text-secondary)' }">
        <span class="w-2 h-2 rounded-full pulse-dot" :style="{ backgroundColor: 'currentColor' }"></span>
        {{ coreRunning ? "核心运行中" : "核心未启动" }}
      </div>
    </template>

    <div v-if="loading" class="flex items-center justify-center py-20" :style="{ color: 'var(--text-secondary)' }">
      <div class="spin"><Activity :size="24" /></div>
    </div>

    <template v-else>
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
        <div class="stat-card">
          <div class="stat-icon" :style="{ backgroundColor: 'rgba(79,142,247,0.12)' }">
            <ArrowDown :size="18" :style="{ color: 'var(--accent)' }" />
          </div>
          <div class="stat-body">
            <div class="stat-label">下载速度</div>
            <div class="stat-value mono" :style="{ color: 'var(--accent)' }">{{ formatSpeed(downloadSpeed) }}</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon" :style="{ backgroundColor: 'rgba(255,159,10,0.12)' }">
            <ArrowUp :size="18" :style="{ color: 'var(--orange)' }" />
          </div>
          <div class="stat-body">
            <div class="stat-label">上传速度</div>
            <div class="stat-value mono" :style="{ color: 'var(--orange)' }">{{ formatSpeed(uploadSpeed) }}</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon" :style="{ backgroundColor: 'rgba(52,199,89,0.12)' }">
            <Activity :size="18" :style="{ color: 'var(--green)' }" />
          </div>
          <div class="stat-body">
            <div class="stat-label">活跃连接</div>
            <div class="stat-value mono" :style="{ color: 'var(--green)' }">{{ activeConnections }}</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon" :style="{ backgroundColor: 'rgba(191,90,242,0.12)' }">
            <Cpu :size="18" :style="{ color: '#bf5af2' }" />
          </div>
          <div class="stat-body">
            <div class="stat-label">内存占用</div>
            <div class="stat-value mono">{{ memoryUsage }} MB</div>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-3">
        <div class="lg:col-span-2">
          <EnhancedCard title="实时流量" :icon="Activity" icon-color="var(--accent)">
            <template #action>
              <div class="flex items-center gap-3 text-xs">
                <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full" :style="{ backgroundColor: 'var(--accent)' }"></span>下载</span>
                <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full" :style="{ backgroundColor: 'var(--orange)' }"></span>上传</span>
              </div>
            </template>
            <div class="h-36">
              <svg v-if="speedHistory.length > 1" viewBox="0 0 200 50" class="w-full h-full" preserveAspectRatio="none">
                <polyline :points="chartPath('down')" fill="none" stroke="var(--accent)" stroke-width="1" stroke-linecap="round" opacity="0.8" />
                <polyline :points="chartPath('up')" fill="none" stroke="var(--orange)" stroke-width="1" stroke-linecap="round" opacity="0.6" />
              </svg>
              <div v-else class="h-full flex items-center justify-center" :style="{ color: 'var(--text-secondary)' }">
                <span class="text-sm">等待流量数据...</span>
              </div>
            </div>
            <div class="grid grid-cols-2 gap-4 mt-3 pt-3" :style="{ borderTop: '1px solid var(--border)' }">
              <div>
                <div class="text-xs" :style="{ color: 'var(--text-secondary)' }">总上传</div>
                <div class="text-sm font-bold mono" :style="{ color: 'var(--orange)' }">{{ formatBytes(uploadTotal) }}</div>
              </div>
              <div>
                <div class="text-xs" :style="{ color: 'var(--text-secondary)' }">总下载</div>
                <div class="text-sm font-bold mono" :style="{ color: 'var(--accent)' }">{{ formatBytes(downloadTotal) }}</div>
              </div>
            </div>
          </EnhancedCard>
        </div>

        <div class="space-y-3">
          <CurrentProxyCard />
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
        <ClashInfoCard />
        <SystemInfoCard />
        <IpInfoCard />
      </div>
    </template>
  </BasePage>
</template>

<style scoped>
.stat-card {
  border-radius: 12px;
  padding: 16px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
  display: flex;
  gap: 12px;
  align-items: center;
}

.stat-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.stat-body {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
}

.stat-value {
  font-size: 18px;
  font-weight: 700;
  line-height: 1.2;
}
</style>