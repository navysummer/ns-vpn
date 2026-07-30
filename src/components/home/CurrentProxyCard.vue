<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { Globe, Route, Shield } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import { useToast } from "@/utils/toast";
import EnhancedCard from "@/components/EnhancedCard.vue";

const app = useAppStore();
const { show } = useToast();
const { t } = useI18n();

const proxyMode = computed(() => app.proxyMode);

async function setMode(mode: "rule" | "global" | "direct") {
  await app.changeProxyMode(mode);
  show(t("dashboard.switchedToMode", { mode: t(`dashboard.${mode}Mode`) }), "info");
}

function modeColor(mode: string): string {
  switch (mode) { case "rule": return "var(--accent)"; case "global": return "var(--orange)"; case "direct": return "var(--green)"; default: return "var(--text-secondary)"; }
}
</script>

<template>
  <EnhancedCard :title="t('home.currentProxy.title')" :icon="Globe" icon-color="var(--accent)">
    <div class="proxy-modes">
      <button v-for="mode in (['rule', 'global', 'direct'] as const)" :key="mode"
        class="proxy-mode-btn" :class="{ active: proxyMode === mode }"
        :style="proxyMode === mode ? { borderColor: modeColor(mode), backgroundColor: `color-mix(in srgb, ${modeColor(mode)} 8%, transparent)` } : {}"
        @click="setMode(mode)">
        <component :is="mode === 'rule' ? Route : mode === 'global' ? Globe : Shield" :size="20" :style="{ color: modeColor(mode) }" />
        <div>
          <div class="pm-label">{{ t(`home.currentProxy.${mode}Mode`) }}</div>
          <div class="pm-desc">{{ t(`home.currentProxy.${mode}ModeDesc`) }}</div>
        </div>
      </button>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.proxy-modes { display: flex; flex-direction: column; gap: 8px; }
.proxy-mode-btn { display: flex; align-items: center; gap: 12px; padding: 12px 14px; border-radius: 10px; border: 1px solid var(--border); background: transparent; cursor: pointer; transition: all 150ms ease; text-align: left; }
.proxy-mode-btn:hover { border-color: var(--accent); }
.proxy-mode-btn.active { border-width: 2px; }
.pm-label { font-size: 13px; font-weight: 600; color: var(--text-primary); }
.pm-desc { font-size: 11px; color: var(--text-secondary); margin-top: 1px; }
</style>
