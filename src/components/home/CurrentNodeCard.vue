<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { Radio, ChevronDown, Globe, Zap } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import EnhancedCard from "@/components/EnhancedCard.vue";

const app = useAppStore();
const { t } = useI18n();

const proxyGroups = computed(() => app.proxyGroups.filter(g => g.type === "Selector"));
const selectedGroup = ref(proxyGroups.value[0]?.name ?? "");
const selectedNode = computed(() => app.currentNode);

const nodes = computed(() => {
  const group = proxyGroups.value.find(g => g.name === selectedGroup.value);
  return group?.all.map(n => n.name) ?? [];
});

function modeLabel(mode: string): string {
  switch (mode) { case "global": return t("home.currentNode.globalMode"); case "direct": return t("home.currentNode.directMode"); default: return t("home.currentNode.ruleMode"); }
}

function modeColor(mode: string): string {
  switch (mode) { case "global": return "var(--orange)"; case "direct": return "var(--green)"; default: return "var(--accent)"; }
}

import { ref } from "vue";
</script>

<template>
  <EnhancedCard title="" :no-padding="true">
    <template #icon>
      <Radio :size="18" style="color: var(--accent)" />
    </template>
    <template #title>
      <span class="node-title">{{ t('home.currentNode.title') }}</span>
    </template>
    <template #action>
      <div class="flex items-center gap-2">
        <Zap :size="14" style="color: var(--text-secondary)" />
        <button class="mode-badge" :style="{ backgroundColor: modeColor(app.proxyMode), color: '#fff' }">
          <Globe :size="10" />
          {{ modeLabel(app.proxyMode) }}
        </button>
      </div>
    </template>
    <div class="node-body">
      <div class="current-node">
        <span class="node-name">{{ selectedNode || '-' }}</span>
      </div>
      <div class="select-group">
        <label class="select-label">{{ t('home.currentNode.proxyGroup') }}</label>
        <div class="select-wrap">
          <select v-model="selectedGroup" class="select-input">
            <option v-for="g in proxyGroups" :key="g.name" :value="g.name">{{ g.name }}</option>
          </select>
          <ChevronDown :size="14" class="select-icon" />
        </div>
      </div>
      <div class="select-group">
        <label class="select-label">{{ t('home.currentNode.node') }}</label>
        <div class="select-wrap">
          <select :value="selectedNode" class="select-input" @change="app.selectProxyNode(selectedGroup, ($event.target as HTMLSelectElement).value)">
            <option v-for="n in nodes" :key="n" :value="n">{{ n }}</option>
          </select>
          <ChevronDown :size="14" class="select-icon" />
        </div>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.node-title { font-size: 16px; font-weight: 700; }
.node-body { padding: 12px 16px; display: flex; flex-direction: column; gap: 12px; }
.current-node { display: flex; align-items: center; gap: 8px; padding: 10px 12px; border-radius: 8px; border: 1px solid var(--border); background-color: var(--bg-tertiary); }
.node-name { font-size: 14px; font-weight: 600; flex: 1; }
.mode-badge { display: inline-flex; align-items: center; gap: 4px; padding: 3px 8px; border-radius: 4px; font-size: 11px; font-weight: 600; }
.select-group { display: flex; flex-direction: column; gap: 4px; }
.select-label { font-size: 11px; color: var(--text-secondary); font-weight: 500; }
.select-wrap { position: relative; }
.select-input { width: 100%; padding: 6px 10px; padding-right: 28px; border-radius: 6px; border: 1px solid var(--border); background-color: var(--bg-tertiary); color: var(--text-primary); font-size: 13px; outline: none; cursor: pointer; appearance: none; }
.select-input:focus { border-color: var(--accent); }
.select-icon { position: absolute; right: 8px; top: 50%; transform: translateY(-50%); pointer-events: none; color: var(--text-secondary); }
</style>
