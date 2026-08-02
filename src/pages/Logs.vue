<script setup lang="ts">
import { ref, computed, nextTick, watch } from "vue";
import { useI18n } from "vue-i18n";
import { Terminal, Download, Pause, Play, ArrowUpDown, ChevronDown, AlertCircle, AlertTriangle, Info } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import BasePage from "@/components/BasePage.vue";
import EmptyState from "@/components/EmptyState.vue";

const app = useAppStore();
const { t } = useI18n();

const logLevel = ref<"all" | "info" | "warning" | "error">("all");
const autoScroll = ref(true);
const paused = ref(false);
const sortOrder = ref<"asc" | "desc">("asc");
const searchQuery = ref("");
const isScrolledUp = ref(false);
const clearLocal = ref(false);

const logs = computed(() => app.logs);

const filteredLogs = computed(() => {
  if (clearLocal.value) return [];
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

const logStats = computed(() => {
  const all = logs.value;
  return {
    total: all.length,
    info: all.filter(l => l.level === "info").length,
    warning: all.filter(l => l.level === "warning").length,
    error: all.filter(l => l.level === "error").length,
  };
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

function onScroll() {
  if (!logContainer.value) return;
  const { scrollTop, scrollHeight, clientHeight } = logContainer.value;
  isScrolledUp.value = scrollHeight - scrollTop - clientHeight > 100;
}

function scrollToBottom() {
  if (logContainer.value) {
    logContainer.value.scrollTop = logContainer.value.scrollHeight;
    isScrolledUp.value = false;
  }
}

function clearLogs() { clearLocal.value = true; }
function togglePause() { paused.value = !paused.value; }
function levelColor(level: string): string {
  switch (level) {
    case "error": return "var(--red)";
    case "warning": return "var(--orange)";
    default: return "var(--text-secondary)";
  }
}

function levelBg(level: string): string {
  switch (level) {
    case "error": return "rgba(255,69,58,0.08)";
    case "warning": return "rgba(255,159,10,0.06)";
    default: return "transparent";
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
  <BasePage :title="t('logs.title')">
    <template #actions>
      <div class="flex items-center gap-2">
        <span class="text-xs" :style="{ color: 'var(--text-secondary)' }">{{ t('logs.countText', { count: filteredLogs.length }) }}</span>
        <button class="btn-ghost text-xs" @click="togglePause">
          <Pause v-if="!paused" :size="12" /><Play v-else :size="12" />
          {{ paused ? t('logs.resume') : t('logs.pause') }}
        </button>
        <button class="btn-ghost text-xs" @click="exportLogs"><Download :size="12" />{{ t('logs.export') }}</button>
        <button class="btn-ghost text-xs" @click="clearLogs">{{ t('logs.clear') }}</button>
      </div>
    </template>

    <div class="log-stats-bar">
      <div class="stat-item">
        <Info :size="12" style="color: var(--text-secondary)" />
        <span class="stat-count">{{ logStats.info }}</span>
        <span class="stat-label">{{ t('logs.info') }}</span>
      </div>
      <div class="stat-item">
        <AlertTriangle :size="12" style="color: var(--orange)" />
        <span class="stat-count" style="color: var(--orange)">{{ logStats.warning }}</span>
        <span class="stat-label">{{ t('logs.warning') }}</span>
      </div>
      <div class="stat-item">
        <AlertCircle :size="12" style="color: var(--red)" />
        <span class="stat-count" style="color: var(--red)">{{ logStats.error }}</span>
        <span class="stat-label">{{ t('logs.error') }}</span>
      </div>
    </div>

    <div class="flex items-center gap-3 mb-3">
      <div class="flex gap-1 p-0.5 rounded-lg" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
        <button v-for="opt in ([{ label: t('common.all'), value: 'all' }, { label: t('logs.info'), value: 'info' }, { label: t('logs.warning'), value: 'warning' }, { label: t('logs.error'), value: 'error' }] as const)" :key="opt.value" class="tab-btn" :class="logLevel === opt.value ? 'tab-btn-active' : 'tab-btn-inactive'" @click="logLevel = opt.value">
          {{ opt.label }}
        </button>
      </div>
      <div class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm flex-1 max-w-xs" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
        <input v-model="searchQuery" :placeholder="t('logs.searchPlaceholder')" class="bg-transparent outline-none flex-1 text-sm" :style="{ color: 'var(--text-primary)' }" />
      </div>
      <label class="flex items-center gap-2 text-sm cursor-pointer" :style="{ color: 'var(--text-secondary)' }">
        <div class="toggle" :class="{ active: autoScroll }" @click="autoScroll = !autoScroll"><div class="toggle-knob"></div></div>
        {{ t('logs.autoScroll') }}
      </label>
      <button class="btn-ghost text-xs" @click="sortOrder = sortOrder === 'asc' ? 'desc' : 'asc'">
        <ArrowUpDown :size="12" />
        {{ sortOrder === 'asc' ? t('logs.asc') : t('logs.desc') }}
      </button>
    </div>

    <div class="log-wrapper">
      <div ref="logContainer" class="log-container rounded-xl border overflow-y-auto font-mono text-xs leading-relaxed flex-1" :style="{ backgroundColor: 'var(--bg-secondary)', borderColor: 'var(--border)', color: 'var(--text-primary)' }" @scroll="onScroll">
        <div class="p-4 space-y-0.5">
          <div v-for="(log, i) in filteredLogs" :key="i" class="log-line" :style="{ backgroundColor: levelBg(log.level) }">
            <span class="log-time">[{{ log.time }}]</span>
            <span class="log-level" :style="{ color: levelColor(log.level) }">{{ levelBadge(log.level).padEnd(5) }}</span>
            <span class="log-type" :style="{ color: typeColor(log.type) }">[{{ log.type }}]</span>
            <span class="log-payload">{{ log.payload }}</span>
          </div>
        </div>
        <EmptyState v-if="filteredLogs.length === 0" :icon="Terminal" :title="t('logs.noLogs')" />
      </div>

      <Transition name="page">
        <button v-if="isScrolledUp" class="scroll-bottom-btn" @click="scrollToBottom">
          <ChevronDown :size="16" />
        </button>
      </Transition>
    </div>
  </BasePage>
</template>

<style scoped>
.log-stats-bar {
  display: flex;
  gap: 16px;
  padding: 8px 16px;
  border-radius: 8px;
  background-color: var(--card-bg);
  border: 1px solid var(--border);
  margin-bottom: 12px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.stat-count {
  font-size: 13px;
  font-weight: 600;
  font-family: "SF Mono", "Fira Code", monospace;
}

.stat-label {
  font-size: 11px;
  color: var(--text-secondary);
}

.log-wrapper {
  position: relative;
  flex: 1;
  min-height: 0;
}

.log-container {
  min-height: 0;
}

.log-line {
  display: flex;
  gap: 8px;
  padding: 2px 4px;
  border-radius: 3px;
}
.log-time { color: var(--text-secondary); flex-shrink: 0; }
.log-level { flex-shrink: 0; font-weight: 600; }
.log-type { flex-shrink: 0; }
.log-payload { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; }

.scroll-bottom-btn {
  position: absolute;
  bottom: 16px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background-color: var(--accent);
  color: #fff;
  border: none;
  cursor: pointer;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  transition: transform 150ms ease, box-shadow 150ms ease;
  z-index: 10;
}
.scroll-bottom-btn:hover {
  transform: translateX(-50%) scale(1.1);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.4);
}
</style>
