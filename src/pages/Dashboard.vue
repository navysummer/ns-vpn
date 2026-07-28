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

let interval: ReturnType<typeof setInterval> | null = null;

function simulateTraffic() {
  // In a real app, this would come from Tauri commands
  uploadSpeed.value = Math.random() * 50000;
  downloadSpeed.value = Math.random() * 500000;
  uploadTotal.value += uploadSpeed.value * 0.1;
  downloadTotal.value += downloadSpeed.value * 0.1;
  activeConnections.value = Math.floor(Math.random() * 50) + 5;
  memoryUsage.value = Math.floor(Math.random() * 80) + 20;
  coreRunning.value = true;
}

onMounted(() => {
  simulateTraffic();
  interval = setInterval(simulateTraffic, 2000);
});

onUnmounted(() => {
  if (interval) clearInterval(interval);
});
</script>

<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-semibold">仪表盘</h1>
      <div
        class="flex items-center gap-2 text-sm"
        :style="{ color: coreRunning ? 'var(--green)' : 'var(--text-secondary)' }"
      >
        <span class="w-2 h-2 rounded-full" :style="{ backgroundColor: 'currentColor' }"></span>
        {{ coreRunning ? "核心运行中" : "核心未启动" }}
      </div>
    </div>

    <!-- Stats Grid -->
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

    <!-- Traffic totals -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-3">
      <div class="card">
        <div class="text-xs font-medium mb-3" :style="{ color: 'var(--text-secondary)' }">
          总上传
        </div>
        <div class="flex items-baseline gap-2">
          <span class="text-xl font-bold mono" :style="{ color: 'var(--orange)' }">
            {{ formatBytes(uploadTotal) }}
          </span>
        </div>
        <div class="mt-3 h-1 rounded-full" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
          <div
            class="h-full rounded-full transition-all duration-500"
            :style="{
              width: '45%',
              backgroundColor: 'var(--orange)',
            }"
          ></div>
        </div>
      </div>

      <div class="card">
        <div class="text-xs font-medium mb-3" :style="{ color: 'var(--text-secondary)' }">
          总下载
        </div>
        <div class="flex items-baseline gap-2">
          <span class="text-xl font-bold mono" :style="{ color: 'var(--accent)' }">
            {{ formatBytes(downloadTotal) }}
          </span>
        </div>
        <div class="mt-3 h-1 rounded-full" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
          <div
            class="h-full rounded-full transition-all duration-500"
            :style="{
              width: '68%',
              backgroundColor: 'var(--accent)',
            }"
          ></div>
        </div>
      </div>
    </div>

    <!-- Speed chart placeholder -->
    <div class="card">
      <div class="text-xs font-medium mb-3" :style="{ color: 'var(--text-secondary)' }">
        实时流量
      </div>
      <div class="h-48 flex items-center justify-center" :style="{ color: 'var(--text-secondary)' }">
        <div class="text-center">
          <Activity :size="32" class="mx-auto mb-2 opacity-30" />
          <p class="text-sm">流量图表将在此显示</p>
          <p class="text-xs mt-1">连接 mihomo API 后实时更新</p>
        </div>
      </div>
    </div>
  </div>
</template>