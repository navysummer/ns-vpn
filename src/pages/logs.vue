<script setup lang="ts">
import { ref, computed, nextTick, watch } from "vue";
import { Terminal } from "lucide-vue-next";

interface LogEntry {
  time: string;
  type: string;
  level: string;
  payload: string;
}

const logLevel = ref<"all" | "info" | "warning" | "error">("all");
const autoScroll = ref(true);

const logs = ref<LogEntry[]>(
  Array.from({ length: 100 }, (_, i) => {
    const levels = ["info", "warning", "error"] as const;
    const types = ["INIT", "PROXY", "DNS", "TUN", "HTTP"];
    const messages = [
      "Connected to mihomo core v1.18.0",
      "Loading configuration from /etc/mihomo/config.yaml",
      "Proxy group Auto: health check passed",
      "DNS query: example.com -> 1.1.1.1",
      "New TCP connection: 192.168.1.100:54321 -> 10.0.0.1:443",
      "TUN device opened: utun4",
      "Rule match: DOMAIN-SUFFIX,google.com,Proxy",
      "Memory usage: 45.2MB",
      "Subscription updated: 45 nodes available",
      "Proxy HK-01: delay 32ms",
    ];
    const time = new Date(Date.now() - (100 - i) * 5000);
    return {
      time: time.toLocaleTimeString("zh-CN", { hour12: false }),
      type: types[Math.floor(Math.random() * types.length)],
      level: levels[Math.floor(Math.random() * 3)],
      payload: messages[Math.floor(Math.random() * messages.length)],
    };
  })
);

const filteredLogs = computed(() => {
  if (logLevel.value === "all") return logs.value;
  return logs.value.filter((l) => l.level === logLevel.value);
});

const logContainer = ref<HTMLDivElement | null>(null);

watch(
  () => logs.value.length,
  async () => {
    if (autoScroll.value) {
      await nextTick();
      if (logContainer.value) {
        logContainer.value.scrollTop = logContainer.value.scrollHeight;
      }
    }
  }
);

function clearLogs() {
  logs.value = [];
}
</script>

<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-semibold">日志</h1>
      <button class="btn-ghost text-xs" @click="clearLogs">清空</button>
    </div>

    <!-- Controls -->
    <div class="flex items-center gap-3">
      <div class="flex gap-1 p-0.5 rounded-lg" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
        <button
          v-for="opt in ([{ label: '全部', value: 'all' }, { label: '信息', value: 'info' }, { label: '警告', value: 'warning' }, { label: '错误', value: 'error' }] as const)"
          :key="opt.value"
          class="px-3 py-1 rounded-md text-xs font-medium transition-colors"
          :style="{
            backgroundColor: logLevel === opt.value ? 'var(--accent)' : 'transparent',
            color: logLevel === opt.value ? '#fff' : 'var(--text-secondary)',
          }"
          @click="logLevel = opt.value"
        >
          {{ opt.label }}
        </button>
      </div>

      <label class="flex items-center gap-2 text-sm cursor-pointer" :style="{ color: 'var(--text-secondary)' }">
        <div
          class="toggle"
          :class="{ 'toggle-bg': true, active: autoScroll }"
          @click="autoScroll = !autoScroll"
        >
          <div class="toggle-knob"></div>
        </div>
        自动滚动
      </label>
    </div>

    <!-- Log output -->
    <div
      ref="logContainer"
      class="rounded-xl border overflow-y-auto font-mono text-xs leading-relaxed"
      :style="{
        backgroundColor: 'var(--bg-secondary)',
        borderColor: 'var(--border)',
        height: 'calc(100vh - 260px)',
        color: 'var(--text-primary)',
      }"
    >
      <div class="p-4 space-y-0.5">
        <div
          v-for="(log, i) in filteredLogs"
          :key="i"
          class="flex gap-3 py-0.5 hover:opacity-80"
        >
          <span class="shrink-0" :style="{ color: 'var(--text-secondary)' }">[{{ log.time }}]</span>
          <span
            class="shrink-0"
            :style="{
              color: log.level === 'error'
                ? 'var(--red)'
                : log.level === 'warning'
                ? 'var(--orange)'
                : 'var(--text-secondary)',
            }"
          >
            {{ log.level.toUpperCase() }}
          </span>
          <span
            class="shrink-0"
            :style="{ color: 'var(--accent)' }"
          >
            [{{ log.type }}]
          </span>
          <span>{{ log.payload }}</span>
        </div>
        <div v-if="filteredLogs.length === 0" class="py-8 text-center" :style="{ color: 'var(--text-secondary)' }">
          <Terminal :size="24" class="mx-auto mb-2 opacity-30" />
          暂无日志
        </div>
      </div>
    </div>
  </div>
</template>