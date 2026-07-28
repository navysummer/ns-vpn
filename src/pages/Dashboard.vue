<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { ArrowUp, ArrowDown, Activity, Globe } from "lucide-vue-next";
import { formatBytes, formatSpeed } from "@/utils/format";

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

const chartPath = () => {
  if (speedHistory.value.length < 2) return "";
  const w = 100;
  const h = 40;
  const maxVal = Math.max(
    ...speedHistory.value.map((s) => Math.max(s.up, s.down)),
    1
  );
  const points = speedHistory.value.map((s, i) => {
    const x = (i / (speedHistory.value.length - 1)) * w;
    const y = h - (Math.max(s.up, s.down) / maxVal) * h;
    return `${x.toFixed(1)},${y.toFixed(1)}`;
  });
  return points.join(" ");
};

onMounted(() => {
  setTimeout(() => { loading.value = false; }, 400);
  updateTraffic();
  interval = setInterval(updateTraffic, 2000);
});

onUnmounted(() => {
  if (interval) clearInterval(interval);
});
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-semibold">仪表盘</h1>
      <div
        class="flex items-center gap-2 text-sm"
        :style="{ color: coreRunning ? 'var(--green)' : 'var(--text-secondary)' }"
      >
        <span class="w-2 h-2 rounded-full pulse-dot" :style="{ backgroundColor: 'currentColor' }"></span>
        {{ coreRunning ? "核心运行中" : "核心未启动" }}
      </div>
    </div>

    <div v-if="loading" class="flex items-center justify-center py-20" :style="{ color: 'var(--text-secondary)' }">
      <div class="spin">
        <Activity :size="24" />
      </div>
    </div>

    <template v-else>
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
        <div class="card">
          <div class="flex items-center gap-2 mb-2 text-xs" :style="{ color: 'var(--text-secondary)' }">
            <ArrowUp :size="14" style="color: var(--orange)" />
            上传速度
          </div>
          <div class="text-2xl font-bold mono" :style="{ color: 'var(--orange)' }">
            {{ formatSpeed(uploadSpeed) }}
          </div>
        </div>

        <div class="card">
          <div class="flex items-center gap-2 mb-2 text-xs" :style="{ color: 'var(--text-secondary)' }">
            <ArrowDown :size="14" style="color: var(--accent)" />
            下载速度
          </div>
          <div class="text-2xl font-bold mono" :style="{ color: 'var(--accent)' }">
            {{ formatSpeed(downloadSpeed) }}
          </div>
        </div>

        <div class="card">
          <div class="flex items-center gap-2 mb-2 text-xs" :style="{ color: 'var(--text-secondary)' }">
            <Activity :size="14" style="color: var(--green)" />
            活跃连接
          </div>
          <div class="text-2xl font-bold mono" :style="{ color: 'var(--green)' }">
            {{ activeConnections }}
          </div>
        </div>

        <div class="card">
          <div class="flex items-center gap-2 mb-2 text-xs" :style="{ color: 'var(--text-secondary)' }">
            <Globe :size="14" :style="{ color: 'var(--text-secondary)' }" />
            内存占用
          </div>
          <div class="text-2xl font-bold mono">{{ memoryUsage }} MB</div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-3">
        <div class="card">
          <div class="text-xs font-medium mb-3" :style="{ color: 'var(--text-secondary)' }">总上传</div>
          <div class="flex items-baseline gap-2">
            <span class="text-xl font-bold mono" :style="{ color: 'var(--orange)' }">
              {{ formatBytes(uploadTotal) }}
            </span>
          </div>
          <div class="mt-3 h-1 rounded-full" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
            <div
              class="h-full rounded-full transition-all duration-500"
              :style="{
                width: Math.min((uploadTotal / 1073741824) * 100, 100) + '%',
                backgroundColor: 'var(--orange)',
              }"
            ></div>
          </div>
        </div>

        <div class="card">
          <div class="text-xs font-medium mb-3" :style="{ color: 'var(--text-secondary)' }">总下载</div>
          <div class="flex items-baseline gap-2">
            <span class="text-xl font-bold mono" :style="{ color: 'var(--accent)' }">
              {{ formatBytes(downloadTotal) }}
            </span>
          </div>
          <div class="mt-3 h-1 rounded-full" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
            <div
              class="h-full rounded-full transition-all duration-500"
              :style="{
                width: Math.min((downloadTotal / 1073741824) * 100, 100) + '%',
                backgroundColor: 'var(--accent)',
              }"
            ></div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="text-xs font-medium mb-3" :style="{ color: 'var(--text-secondary)' }">实时流量</div>
        <div class="h-48">
          <svg
            v-if="speedHistory.length > 1"
            viewBox="0 0 100 40"
            class="w-full h-full"
            preserveAspectRatio="none"
          >
            <polyline
              :points="chartPath()"
              fill="none"
              stroke="var(--accent)"
              stroke-width="0.5"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <polyline
              :points="chartPath()"
              fill="none"
              stroke="var(--accent)"
              stroke-width="0.3"
              stroke-dasharray="2 1"
              opacity="0.5"
            />
          </svg>
          <div
            v-else
            class="h-full flex items-center justify-center"
            :style="{ color: 'var(--text-secondary)' }"
          >
            <div class="text-center">
              <Activity :size="32" class="mx-auto mb-2 opacity-30" />
              <p class="text-sm">等待流量数据...</p>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>