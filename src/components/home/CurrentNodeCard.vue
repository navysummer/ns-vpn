<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useI18n } from "vue-i18n";
import { Radio, Zap, Globe, Shield, Route } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import EnhancedCard from "@/components/EnhancedCard.vue";

const app = useAppStore();
const { t } = useI18n();

const proxyGroups = computed(() => {
  const groups = app.proxyRunning ? app.proxyGroups : app.subProxyGroups;
  return groups.filter(g => g.name !== "GLOBAL" && g.type === "Selector");
});

const selectedGroup = ref("");
const selectedNode = ref("");

watch(proxyGroups, (groups) => {
  if (groups.length > 0 && !groups.find(g => g.name === selectedGroup.value)) {
    selectedGroup.value = groups[0].name;
  }
}, { immediate: true });

watch(selectedGroup, (groupName) => {
  const group = proxyGroups.value.find(g => g.name === groupName);
  if (group && group.now) {
    selectedNode.value = group.now;
  }
}, { immediate: true });

const currentNodes = computed(() => {
  const group = proxyGroups.value.find(g => g.name === selectedGroup.value);
  return group?.all ?? [];
});

const selectedNodeType = computed(() => {
  const node = currentNodes.value.find(n => n.name === selectedNode.value);
  return node?.type || '';
});

const currentMode = computed(() => app.proxyMode);

function modeLabel(mode: string): string {
  switch (mode) { case "global": return t("home.currentNode.globalMode"); case "direct": return t("home.currentNode.directMode"); default: return t("home.currentNode.ruleMode"); }
}

function modeColor(mode: string): string {
  switch (mode) { case "global": return "var(--orange)"; case "direct": return "var(--green)"; default: return "var(--accent)"; }
}

async function selectNode(nodeName: string) {
  if (!app.proxyRunning) return;
  selectedNode.value = nodeName;
  await app.selectProxyNode(selectedGroup.value, nodeName);
}
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
        <button class="mode-badge" :style="{ backgroundColor: modeColor(currentMode), color: '#fff' }">
          <Globe :size="10" />
          {{ modeLabel(currentMode) }}
        </button>
      </div>
    </template>
    <div class="node-body">
      <div class="current-node">
        <span class="node-label">{{ t('home.currentNode.current') }}</span>
        <span class="node-name">{{ selectedNode || '-' }}</span>
        <span v-if="selectedNodeType" class="node-type-tag">{{ selectedNodeType }}</span>
      </div>
      <div class="select-group">
        <label class="select-label">{{ t('home.currentNode.proxyGroup') }}</label>
        <div class="select-wrap">
          <select v-model="selectedGroup" class="select-input">
            <option v-for="g in proxyGroups" :key="g.name" :value="g.name">{{ g.name }}</option>
          </select>
        </div>
      </div>
      <div class="select-group" v-if="currentNodes.length > 0">
        <label class="select-label">{{ t('home.currentNode.node') }}</label>
        <div class="select-wrap">
          <select v-model="selectedNode" class="select-input" @change="selectNode(selectedNode)">
            <option v-for="n in currentNodes" :key="n.name" :value="n.name">{{ n.type }} - {{ n.name }}</option>
          </select>
        </div>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.node-title { font-size: 16px; font-weight: 700; }
.node-body { padding: 12px 16px; display: flex; flex-direction: column; gap: 12px; }
.current-node { display: flex; align-items: center; gap: 8px; padding: 10px 12px; border-radius: 8px; border: 1px solid var(--border); background-color: var(--bg-tertiary); }
.node-label { font-size: 11px; color: var(--text-secondary); flex-shrink: 0; }
.node-name { font-size: 14px; font-weight: 600; flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.node-type-tag { font-size: 10px; padding: 1px 6px; border-radius: 4px; background: var(--bg-secondary); color: var(--text-secondary); font-weight: 500; flex-shrink: 0; }
.mode-badge { display: inline-flex; align-items: center; gap: 4px; padding: 3px 8px; border-radius: 4px; font-size: 11px; font-weight: 600; border: none; }
.select-group { display: flex; flex-direction: column; gap: 4px; }
.select-label { font-size: 11px; color: var(--text-secondary); font-weight: 500; }
.select-wrap { position: relative; }
.select-input { width: 100%; padding: 6px 10px; padding-right: 28px; border-radius: 6px; border: 1px solid var(--border); background-color: var(--bg-tertiary); color: var(--text-primary); font-size: 13px; outline: none; cursor: pointer; appearance: none; background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='14' height='14' viewBox='0 0 24 24' fill='none' stroke='%23888' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E"); background-repeat: no-repeat; background-position: right 6px center; }
.select-input:focus { border-color: var(--accent); }
</style>