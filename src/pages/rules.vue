<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from "vue";
import { useI18n } from "vue-i18n";
import { Search } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import BasePage from "@/components/BasePage.vue";
import EmptyState from "@/components/EmptyState.vue";

const app = useAppStore();
const { t } = useI18n();

const loading = ref(false);

onMounted(async () => {
  if (app.proxyRunning) {
    loading.value = true;
    await nextTick();
    await app.fetchRules();
    loading.value = false;
  }
});

const rules = computed(() => {
  return app.rules.map(r => ({
    type: r.type,
    payload: r.payload,
    proxy: r.proxy,
    behavior: r.type.startsWith("DOMAIN") ? "Domain" : r.type.startsWith("IP") ? "IPCIDR" : r.type,
  }));
});

const searchQuery = ref("");
const filterType = ref<"all" | "DOMAIN" | "IP-CIDR" | "GEOIP" | "MATCH">("all");

const filteredRules = computed(() => {
  return rules.value.filter((r) => {
    if (filterType.value !== "all") {
      if (filterType.value === "DOMAIN" && !r.type.startsWith("DOMAIN")) return false;
      if (filterType.value === "IP-CIDR" && !r.type.startsWith("IP-CIDR")) return false;
      if (filterType.value === "GEOIP" && r.type !== "GEOIP") return false;
      if (filterType.value === "MATCH" && r.type !== "MATCH") return false;
    }
    if (searchQuery.value) {
      const q = searchQuery.value.toLowerCase();
      return r.payload.toLowerCase().includes(q) || r.type.toLowerCase().includes(q) || r.proxy.toLowerCase().includes(q);
    }
    return true;
  });
});

function proxyColor(proxy: string): string {
  switch (proxy) {
    case "Proxy": return "var(--accent)";
    case "Direct": return "var(--green)";
    case "Reject": return "var(--red)";
    case "Media": return "var(--orange)";
    case "Ai": return "#bf5af2";
    default: return "var(--text-secondary)";
  }
}

function proxyBg(proxy: string): string {
  switch (proxy) {
    case "Proxy": return "rgba(79,142,247,0.12)";
    case "Direct": return "rgba(52,199,89,0.12)";
    case "Reject": return "rgba(255,69,58,0.12)";
    case "Media": return "rgba(255,159,10,0.12)";
    case "Ai": return "rgba(191,90,242,0.12)";
    default: return "rgba(152,152,158,0.12)";
  }
}

function typeColor(type: string): string {
  if (type.startsWith("DOMAIN")) return "#4f8ef7";
  if (type.startsWith("IP-CIDR") || type === "GEOIP") return "#34c759";
  if (type === "MATCH") return "#ff9f0a";
  return "var(--text-secondary)";
}

function typeBg(type: string): string {
  if (type.startsWith("DOMAIN")) return "rgba(79,142,247,0.1)";
  if (type.startsWith("IP-CIDR") || type === "GEOIP") return "rgba(52,199,89,0.1)";
  if (type === "MATCH") return "rgba(255,159,10,0.1)";
  return "rgba(152,152,158,0.1)";
}
</script>

<template>
  <BasePage :title="t('rules.title')">
    <template #actions>
      <span class="text-sm" :style="{ color: 'var(--text-secondary)' }">
        {{ t('rules.countText', { filtered: filteredRules.length, total: rules.length }) }}
      </span>
    </template>

    <div class="flex gap-4 flex-1 min-h-0">
      <div class="flex-1 flex flex-col min-w-0">
        <div class="flex items-center gap-3 mb-3">
          <div class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm flex-1 max-w-xs" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
            <Search :size="14" :style="{ color: 'var(--text-secondary)' }" />
            <input v-model="searchQuery" :placeholder="t('rules.searchPlaceholder')" class="bg-transparent outline-none flex-1 text-sm" :style="{ color: 'var(--text-primary)' }" />
          </div>
          <div class="flex gap-1 p-0.5 rounded-lg" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
            <button v-for="opt in ([{ label: t('common.all'), value: 'all' }, { label: t('rules.domain'), value: 'DOMAIN' }, { label: 'IP', value: 'IP-CIDR' }, { label: 'GEOIP', value: 'GEOIP' }, { label: 'MATCH', value: 'MATCH' }] as const)" :key="opt.value" class="tab-btn" :class="filterType === opt.value ? 'tab-btn-active' : 'tab-btn-inactive'" @click="filterType = opt.value">
              {{ opt.label }}
            </button>
          </div>
        </div>

        <div class="rounded-xl overflow-hidden border flex-1 flex flex-col" :style="{ borderColor: 'var(--border)' }">
          <div class="grid grid-cols-4 gap-2 px-4 py-2.5 text-xs font-medium shrink-0" :style="{ backgroundColor: 'var(--bg-secondary)', color: 'var(--text-secondary)', borderBottom: '1px solid var(--border)' }">
            <div>{{ t('rules.type') }}</div>
            <div>{{ t('rules.content') }}</div>
            <div>{{ t('rules.behavior') }}</div>
            <div>{{ t('rules.proxy') }}</div>
          </div>
          <div class="rules-scroll">
            <template v-if="loading">
              <div v-for="i in 10" :key="'skeleton-' + i" class="grid grid-cols-4 gap-2 px-4 py-2.5 text-sm items-center" :style="{ borderColor: 'var(--border)' }">
                <div><span class="skeleton skeleton-tag">&nbsp;</span></div>
                <div><span class="skeleton skeleton-text w-48">&nbsp;</span></div>
                <div><span class="skeleton skeleton-text w-20">&nbsp;</span></div>
                <div><span class="skeleton skeleton-tag-proxy">&nbsp;</span></div>
              </div>
            </template>
            <TransitionGroup v-else name="rule-list" tag="div">
              <div v-for="(rule, i) in filteredRules" :key="i" class="grid grid-cols-4 gap-2 px-4 py-2.5 text-sm items-center row-hover" :style="{ borderColor: 'var(--border)' }">
                <div><span class="tag" :style="{ backgroundColor: typeBg(rule.type), color: typeColor(rule.type) }">{{ rule.type }}</span></div>
                <div class="font-mono text-xs truncate" :style="{ color: 'var(--text-primary)' }">{{ rule.payload }}</div>
                <div class="text-xs" :style="{ color: 'var(--text-secondary)' }">{{ rule.behavior }}</div>
                <div><span class="tag" :style="{ backgroundColor: proxyBg(rule.proxy), color: proxyColor(rule.proxy) }">{{ rule.proxy }}</span></div>
              </div>
            </TransitionGroup>
          </div>
          <EmptyState v-if="!loading && filteredRules.length === 0" :title="t('rules.noMatchingRules')" />
        </div>
      </div>
    </div>
  </BasePage>
</template>

<style scoped>
.rules-scroll {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
}

/* ── Skeleton loader ── */
@keyframes pulse {
  0%, 100% { opacity: 0.25; }
  50% { opacity: 0.55; }
}
.skeleton {
  display: inline-block;
  border-radius: 6px;
  background: var(--bg-tertiary);
  animation: pulse 1.2s ease-in-out infinite;
  pointer-events: none;
  user-select: none;
}
.skeleton-text { height: 14px; vertical-align: middle; }
.skeleton-tag { width: 48px; height: 18px; border-radius: 4px; }
.skeleton-tag-proxy { width: 52px; height: 18px; border-radius: 4px; }
.w-20 { width: 80px; }
.w-48 { width: 192px; }

/* ── TransitionGroup animations ── */
.rule-list-enter-active {
  transition: all 0.15s ease;
}
.rule-list-enter-from {
  opacity: 0;
  transform: translateY(4px);
}
.rule-list-move {
  transition: transform 0.15s ease;
}

.providers-panel {
  width: 240px;
  min-width: 240px;
  padding: 16px;
  border-radius: 12px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
  height: fit-content;
}
</style>
