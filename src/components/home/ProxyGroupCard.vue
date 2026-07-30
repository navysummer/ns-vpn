<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { ChevronRight, Route, Globe, Shield } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import EnhancedCard from "@/components/EnhancedCard.vue";

const app = useAppStore();
const { t } = useI18n();

const currentGroup = computed(() => app.currentProxyGroup);
const currentNodeName = computed(() => app.currentNode);
const proxyMode = computed(() => app.proxyMode);

function modeIcon(mode: string) {
  switch (mode) { case "global": return Globe; case "direct": return Shield; default: return Route; }
}
function modeLabel(mode: string): string {
  switch (mode) { case "global": return t("dashboard.globalMode"); case "direct": return t("dashboard.directMode"); default: return t("dashboard.ruleMode"); }
}
function modeColor(mode: string): string {
  switch (mode) { case "global": return "var(--accent)"; case "direct": return "var(--green)"; default: return "var(--orange)"; }
}
</script>

<template>
  <EnhancedCard :title="t('home.proxyGroup.title')" :icon="Route" icon-color="var(--accent)">
    <div class="current-proxy">
      <div class="proxy-main">
        <div class="proxy-mode-badge" :style="{ color: modeColor(proxyMode) }">
          <component :is="modeIcon(proxyMode)" :size="14" />
          <span class="text-xs font-medium">{{ modeLabel(proxyMode) }}</span>
        </div>
        <div class="flex items-center gap-1.5" v-if="currentGroup">
          <span class="text-sm font-semibold">{{ currentGroup.name }}</span>
          <ChevronRight :size="12" style="color: var(--text-secondary)" />
          <span class="text-sm" :style="{ color: 'var(--accent)' }">{{ currentNodeName || '-' }}</span>
        </div>
        <div v-else class="text-sm" style="color: var(--text-secondary)">-</div>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.current-proxy { display: flex; flex-direction: column; gap: 12px; }
.proxy-main { display: flex; flex-direction: column; gap: 8px; }
.proxy-mode-badge { display: inline-flex; align-items: center; gap: 4px; padding: 3px 8px; border-radius: 6px; background-color: var(--bg-tertiary); width: fit-content; }
</style>
