<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { ArrowUp, ArrowDown, Activity, Globe, Cpu, Zap, TrendingUp, Shield } from "lucide-vue-next";
import { formatBytes, formatSpeed } from "@/utils/format";
import BasePage from "@/components/BasePage.vue";

const uploadSpeed = ref(0);
const downloadSpeed = ref(0);
const uploadTotal = ref(0);
const downloadTotal = ref(0);
const activeConnections = ref(0);
const memoryUsage = ref(0);
const cpuUsage = ref(0);
const coreRunning = ref(false);
const loading = ref(true);
const systemProxy = ref(false);
const tunMode = ref(false);
const coreVersion = ref("v1.18.0");

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
  cpuUsage.value = Math.floor(Math.random() * 30) + 5;
  coreRunning.value = true;
  systemProxy.value = true;

  speedHistory.value.push({ up, down });
  if (speedHistory.value.length > maxHistory) {
    speedHistory.value.shift();
  }
}

const chartPath = () => {
  if (speedHistory.value.length < 2) return "";
  const w = 200;
  const h = 60;
  const maxVal = Math.max(
    ...speedHistory.value.map((s) => Math.max(s.up, s.down)),
    1
  );
  const points = speedHistory.value.map((s, i) => {
    const x = (i / (speedHistory.value.length - 1)) * w;
    const y = h - (Math.max(s.up, s.down) / maxVal) * h * 0.9;
    return `${x.toFixed(1)},${y.toFixed(1)}`;
  });
  return points.join(" ");
};

const chartPathDown = () => {
  if (speedHistory.value.length < 2) return "";
  const w = 200;
  const h = 60;
  const maxVal = Math.max(
    ...speedHistory.value.map((s) => Math.max(s.up, s.down)),
    1
  );
  const points = speedHistory.value.map((s, i) => {
    const x = (i / (speedHistory.value.length - 1)) * w;
    const y = h - (s.down / maxVal) * h * 0.9;
    return `${x.toFixed(1)},${y.toFixed(1)}`;
  });
  return points.join(" ");
};

const chartPathUp = () => {
  if (speedHistory.value.length < 2) return "";
  const w = 200;
  const h = 60;
  const maxVal = Math.max(
    ...speedHistory.value.map((s) => Math.max(s.up, s.down)),
    1
  );
  const points = speedHistory.value.map((s, i) => {
    const x = (i / (speedHistory.value.length - 1)) * w;
    const y = h - (s.up / maxVal) * h * 0.9;
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
        <div class="dash-card">
          <div class="dash-card-icon" :style="{ backgroundColor: 'rgba(79,142,247,0.12)' }">
            <ArrowDown :size="18" :style="{ color: 'var(--accent)' }" />
          </div>
          <div class="dash-card-body">
            <div class="dash-card-label">下载速度</div>
            <div class="dash-card-value mono" :style="{ color: 'var(--accent)' }">{{ formatSpeed(downloadSpeed) }}</div>
          </div>
        </div>

        <div class="dash-card">
          <div class="dash-card-icon" :style="{ backgroundColor: 'rgba(255,159,10,0.12)' }">
            <ArrowUp :size="18" :style="{ color: 'var(--orange)' }" />
          </div>
          <div class="dash-card-body">
            <div class="dash-card-label">上传速度</div>
            <div class="dash-card-value mono" :style="{ color: 'var(--orange)' }">{{ formatSpeed(uploadSpeed) }}</div>
          </div>
        </div>

        <div class="dash-card">
          <div class="dash-card-icon" :style="{ backgroundColor: 'rgba(52,199,89,0.12)' }">
            <Activity :size="18" :style="{ color: 'var(--green)' }" />
          </div>
          <div class="dash-card-body">
            <div class="dash-card-label">活跃连接</div>
            <div class="dash-card-value mono" :style="{ color: 'var(--green)' }">{{ activeConnections }}</div>
          </div>
        </div>

        <div class="dash-card">
          <div class="dash-card-icon" :style="{ backgroundColor: 'rgba(191,90,242,0.12)' }">
            <Cpu :size="18" :style="{ color: '#bf5af2' }" />
          </div>
          <div class="dash-card-body">
            <div class="dash-card-label">内存占用</div>
            <div class="dash-card-value mono">{{ memoryUsage }} MB</div>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-3">
        <div class="dash-card lg:col-span-2">
          <div class="dash-card-header">
            <span class="dash-card-title">实时流量</span>
            <div class="flex items-center gap-3 text-xs">
              <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full" :style="{ backgroundColor: 'var(--accent)' }"></span>下载</span>
              <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full" :style="{ backgroundColor: 'var(--orange)' }"></span>上传</span>
            </div>
          </div>
          <div class="h-40">
            <svg
              v-if="speedHistory.length > 1"
              viewBox="0 0 200 60"
              class="w-full h-full"
              preserveAspectRatio="none"
            >
              <polyline
                :points="chartPathDown()"
                fill="none"
                stroke="var(--accent)"
                stroke-width="1"
                stroke-linecap="round"
                stroke-linejoin="round"
                opacity="0.8"
              />
              <polyline
                :points="chartPathUp()"
                fill="none"
                stroke="var(--orange)"
                stroke-width="1"
                stroke-linecap="round"
                stroke-linejoin="round"
                opacity="0.6"
              />
            </svg>
            <div v-else class="h-full flex items-center justify-center" :style="{ color: 'var(--text-secondary)' }">
              <span class="text-sm">等待流量数据...</span>
            </div>
          </div>
        </div>

        <div class="dash-card space-y-3">
          <div class="dash-card-header">
            <span class="dash-card-title">系统状态</span>
          </div>
          <div class="space-y-2">
            <div class="status-row">
              <span class="status-label">核心</span>
              <span class="status-value" :style="{ color: coreRunning ? 'var(--green)' : 'var(--red)' }">
                {{ coreRunning ? "运行中" : "未启动" }}
              </span>
            </div>
            <div class="status-row">
              <span class="status-label">版本</span>
              <span class="status-value mono">{{ coreVersion }}</span>
            </div>
            <div class="status-row">
              <span class="status-label">系统代理</span>
              <span class="status-value" :style="{ color: systemProxy ? 'var(--green)' : 'var(--text-secondary)' }">
                {{ systemProxy ? "已开启" : "未开启" }}
              </span>
            </div>
            <div class="status-row">
              <span class="status-label">TUN 模式</span>
              <span class="status-value" :style="{ color: tunMode ? 'var(--green)' : 'var(--text-secondary)' }">
                {{ tunMode ? "已开启" : "未开启" }}
              </span>
            </div>
            <div class="status-row">
              <span class="status-label">CPU</span>
              <span class="status-value mono">{{ cpuUsage }}%</span>
            </div>
            <div class="status-row">
              <span class="status-label">内存</span>
              <span class="status-value mono">{{ memoryUsage }} MB</span>
            </div>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-3">
        <div class="dash-card">
          <div class="dash-card-header">
            <span class="dash-card-title">总上传</span>
            <ArrowUp :size="14" :style="{ color: 'var(--orange)' }" />
          </div>
          <div class="dash-card-value mono text-xl" :style="{ color: 'var(--orange)' }">
            {{ formatBytes(uploadTotal) }}
          </div>
          <div class="progress-bar mt-3">
            <div class="progress-fill" :style="{ width: Math.min((uploadTotal / 1073741824) * 100, 100) + '%', backgroundColor: 'var(--orange)' }"></div>
          </div>
        </div>

        <div class="dash-card">
          <div class="dash-card-header">
            <span class="dash-card-title">总下载</span>
            <ArrowDown :size="14" :style="{ color: 'var(--accent)' }" />
          </div>
          <div class="dash-card-value mono text-xl" :style="{ color: 'var(--accent)' }">
            {{ formatBytes(downloadTotal) }}
          </div>
          <div class="progress-bar mt-3">
            <div class="progress-fill" :style="{ width: Math.min((downloadTotal / 1073741824) * 100, 100) + '%', backgroundColor: 'var(--accent)' }"></div>
          </div>
        </div>
      </div>
    </template>
  </BasePage>
</template>

<style scoped>
.dash-card {
  border-radius: 12px;
  padding: 16px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
}

.dash-card-icon {
  width: 36px;
  height: 36px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 12px;
}

.dash-card-body {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.dash-card-label {
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
}

.dash-card-value {
  font-size: 20px;
  font-weight: 700;
  line-height: 1.2;
}

.dash-card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.dash-card-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.status-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 0;
  border-bottom: 1px solid var(--border);
}
.status-row:last-child {
  border-bottom: none;
}

.status-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.status-value {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
}

.progress-bar {
  height: 4px;
  border-radius: 2px;
  background-color: var(--bg-tertiary);
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  border-radius: 2px;
  transition: width 500ms ease;
}
</style>