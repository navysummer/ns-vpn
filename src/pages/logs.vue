<script setup lang="ts">
import { ref, computed, nextTick, watch } from "vue";
import { Terminal, Download, Pause, Play, ArrowUpDown } from "lucide-vue-next";
import BasePage from "@/components/BasePage.vue";

interface LogEntry {
  time: string;
  type: string;
  level: string;
  payload: string;
}

const logLevel = ref<"all" | "info" | "warning" | "error">("all");
const autoScroll = ref(true);
const paused = ref(false);
const sortOrder = ref<"asc" | "desc">("asc");
const searchQuery = ref("");

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
  let result = logs.value;
  if (logLevel.value !== "all") {
    result = result.filter((l) => l.level === logLevel.value);
  }
  if (searchQuery.value) {
    const q = searchQuery.value.toLowerCase();
    result = result.filter(l => l.payload.toLowerCase().includes(q) || l.type.toLowerCase().includes(q));
  }
  if (sortOrder.value === "desc") {
    result = [...result].reverse();
  }
  return result;
});

const logContainer = ref<HTMLDivElement | null>(null);

watch(
  () => logs.value.length,
  async () => {
    if (autoScroll.value && !paused.value) {
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

function togglePause() {
  paused.value = !paused.value;
}

function levelColor(level: string): string {
  switch (level) {
    case "error": return "var(--red)";
    case "warning": return "var(--orange)";
    default: return "var(--text-secondary)";
  }
}

function levelBadge(level: string): string {
  switch (level) {
    case "error": return "ERROR";
    case "warning": return "WARN";
    default: return "INFO";
  }
}

function typeColor(type: string): string {
  switch (type) {
    case "INIT": return "#34c759";
    case "PROXY": return "#4f8ef7";
    case "DNS": return "#bf5af2";
    case "TUN": return "#ff9f0a";
    case "HTTP": return "#ff453a";
    default: return "var(--text-secondary)";
  }
}

function exportLogs() {
  const text = filteredLogs.value
    .map(l => `[${l.time}] ${levelBadge(l.level)} [${l.type}] ${l.payload}`)
    .join("\n");
  const blob = new Blob([text], { type: "text/plain" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `ns-vpn-logs-${new Date().toISOString().slice(0, 10)}.txt`;
  a.click();
  URL.revokeObjectURL(url);
}
</script>

<template>
  <BasePage title="日志">
    <template #actions>
      <div class="flex items-center gap-2">
        <span class="text-xs" :style="{ color: 'var(--text-secondary)' }">{{ filteredLogs.length }} 条</span>
        <button class="btn-ghost text-xs" @click="togglePause">
          <Pause v-if="!paused" :size="12" />
          <Play v-else :size="12" />
          {{ paused ? "继续" : "暂停" }}
        </button>
        <button class="btn-ghost text-xs" @click="exportLogs">
          <Download :size="12" />
          导出
        </button>
        <button class="btn-ghost text-xs" @click="clearLogs">清空</button>
      </div>
    </template>

    <div class="flex items-center gap-3 mb-3">
      <div class="flex gap-1 p-0.5 rounded-lg" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
        <button v-for="opt in ([{ label: '全部', value: 'all' }, { label: '信息', value: 'info' }, { label: '警告', value: 'warning' }, { label: '错误', value: 'error' }] as const)" :key="opt.value" class="tab-btn" :class="logLevel === opt.value ? 'tab-btn-active' : 'tab-btn-inactive'" @click="logLevel = opt.value">
          {{ opt.label }}
        </button>
      </div>
      <div class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm flex-1 max-w-xs" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
        <input v-model="searchQuery" placeholder="搜索日志..." class="bg-transparent outline-none flex-1 text-sm" :style="{ color: 'var(--text-primary)' }" />
      </div>
      <label class="flex items-center gap-2 text-sm cursor-pointer" :style="{ color: 'var(--text-secondary)' }">
        <div class="toggle" :class="{ active: autoScroll }" @click="autoScroll = !autoScroll">
          <div class="toggle-knob"></div>
        </div>
        自动滚动
      </label>
      <button class="btn-ghost text-xs" @click="sortOrder = sortOrder === 'asc' ? 'desc' : 'asc'">
        <ArrowUpDown :size="12" />
        {{ sortOrder === 'asc' ? '正序' : '倒序' }}
      </button>
    </div>

    <div ref="logContainer" class="rounded-xl border overflow-y-auto font-mono text-xs leading-relaxed flex-1" :style="{ backgroundColor: 'var(--bg-secondary)', borderColor: 'var(--border)', color: 'var(--text-primary)' }">
      <div class="p-4 space-y-0.5">
        <div v-for="(log, i) in filteredLogs" :key="i" class="log-line hover:opacity-80">
          <span class="log-time">[{{ log.time }}]</span>
          <span class="log-level" :style="{ color: levelColor(log.level) }">{{ levelBadge(log.level).padEnd(5) }}</span>
          <span class="log-type" :style="{ color: typeColor(log.type) }">[{{ log.type }}]</span>
          <span class="log-payload">{{ log.payload }}</span>
        </div>
        <div v-if="filteredLogs.length === 0" class="py-8 text-center" :style="{ color: 'var(--text-secondary)' }">
          <Terminal :size="24" class="mx-auto mb-2 opacity-30" />
          暂无日志
        </div>
      </div>
    </div>
  </BasePage>
</template>

<style scoped>
.log-line {
  display: flex;
  gap: 8px;
  padding: 1px 0;
}
.log-time { color: var(--text-secondary); flex-shrink: 0; }
.log-level { flex-shrink: 0; font-weight: 600; }
.log-type { flex-shrink: 0; }
.log-payload { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; }
</style>