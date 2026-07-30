<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { ArrowUp, ArrowDown, Activity, Globe, Zap } from "lucide-vue-next";
import { formatSpeed } from "@/utils/format";

const uploadSpeed = ref(0);
const downloadSpeed = ref(0);
const proxyMode = ref("rule");
const coreRunning = ref(true);

let interval: ReturnType<typeof setInterval> | null = null;

function update() {
  uploadSpeed.value = Math.random() * 50000;
  downloadSpeed.value = Math.random() * 500000;
}

onMounted(() => {
  update();
  interval = setInterval(update, 2000);
});

onUnmounted(() => {
  if (interval) clearInterval(interval);
});
</script>

<template>
  <div class="status-bar">
    <div class="sb-left">
      <div class="sb-item" :style="{ color: coreRunning ? 'var(--green)' : 'var(--text-secondary)' }">
        <span class="w-1.5 h-1.5 rounded-full pulse-dot" :style="{ backgroundColor: 'currentColor' }"></span>
        {{ coreRunning ? "运行中" : "未启动" }}
      </div>
      <div class="sb-divider"></div>
      <div class="sb-item">
        <ArrowDown :size="11" :style="{ color: 'var(--accent)' }" />
        <span class="mono text-xs">{{ formatSpeed(downloadSpeed) }}</span>
      </div>
      <div class="sb-item">
        <ArrowUp :size="11" :style="{ color: 'var(--orange)' }" />
        <span class="mono text-xs">{{ formatSpeed(uploadSpeed) }}</span>
      </div>
    </div>
    <div class="sb-right">
      <div class="sb-item">
        <Globe :size="11" />
        <span class="text-xs">{{ proxyMode === 'rule' ? '规则' : proxyMode === 'global' ? '全局' : '直连' }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 28px;
  padding: 0 12px;
  border-top: 1px solid var(--border);
  background-color: var(--bg-secondary);
  font-size: 11px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.sb-left, .sb-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.sb-item {
  display: flex;
  align-items: center;
  gap: 4px;
}

.sb-divider {
  width: 1px;
  height: 12px;
  background-color: var(--border);
}
</style>