<script setup lang="ts">
import { ref, computed } from "vue";
import { RefreshCw, Check, Globe, Shield, Route, ArrowUpDown, Eye, Filter, Link, RotateCcw } from "lucide-vue-next";
import { useToast } from "@/utils/toast";
import { useAppStore } from "@/stores/app";
import { useI18n } from "vue-i18n";

const app = useAppStore();
const { show } = useToast();
const { t } = useI18n();

const selectedGroup = ref("");
const testingAll = ref(false);
const showChain = ref(false);
const sortBy = ref<"none" | "delay" | "name">("none");
const sortAsc = ref(true);

const groups = computed(() => app.proxyGroups);

const currentGroup = computed(() => {
  return groups.value.find(g => g.name === selectedGroup.value) ?? groups.value[0];
});

const sortedNodes = computed(() => {
  if (!currentGroup.value) return [];
  let nodes = [...currentGroup.value.all];
  if (sortBy.value === "delay") {
    nodes.sort((a, b) => sortAsc.value ? (a.delay ?? 0) - (b.delay ?? 0) : (b.delay ?? 0) - (a.delay ?? 0));
  } else if (sortBy.value === "name") {
    nodes.sort((a, b) => sortAsc.value ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name));
  }
  return nodes;
});

async function selectNode(nodeName: string) {
  if (!currentGroup.value) return;
  await app.selectProxyNode(currentGroup.value.name, nodeName);
}

async function testAllDelay() {
  testingAll.value = true;
  if (currentGroup.value) {
    for (const node of currentGroup.value.all) {
      app.testNodeDelay(node.name).then(delay => {
        node.delay = delay;
      });
    }
  }
  setTimeout(() => { testingAll.value = false; }, 3000);
}

async function setProxyMode(mode: "rule" | "global" | "direct") {
  await app.changeProxyMode(mode);
  show(t("dashboard.switchedToMode", { mode: t(`dashboard.${mode}Mode`) }), "info");
}

function toggleSort(field: "delay" | "name") {
  if (sortBy.value === field) { sortAsc.value = !sortAsc.value; } else { sortBy.value = field; sortAsc.value = true; }
}

function delayColor(delay: number): string {
  if (delay === 0) return "var(--text-secondary)";
  if (delay < 200) return "var(--green)";
  if (delay < 400) return "var(--blue)";
  if (delay < 800) return "var(--orange)";
  return "var(--red)";
}
</script>

<template>
  <div class="proxy-page">
    <div class="proxy-header">
      <h1 class="proxy-title">{{ t('proxies.title') }}</h1>
      <div class="proxy-header-actions">
        <div class="mode-switcher">
          <button class="mode-btn" :class="{ 'mode-btn-active': app.proxyMode === 'rule' }" @click="setProxyMode('rule')">{{ t('dashboard.ruleMode') }}</button>
          <button class="mode-btn" :class="{ 'mode-btn-active': app.proxyMode === 'global' }" @click="setProxyMode('global')">{{ t('dashboard.globalMode') }}</button>
          <button class="mode-btn" :class="{ 'mode-btn-active': app.proxyMode === 'direct' }" @click="setProxyMode('direct')">{{ t('dashboard.directMode') }}</button>
        </div>
      </div>
    </div>

    <div class="proxy-toolbar">
      <div class="group-tabs">
        <button v-for="g in groups" :key="g.name" class="group-tab" :class="{ active: selectedGroup === g.name || (!selectedGroup && g === groups[0]) }" @click="selectedGroup = g.name">
          {{ g.name }} ({{ g.all.length }})
        </button>
      </div>
      <div class="toolbar-actions">
        <button class="tool-btn" :title="t('common.refresh')" @click="testAllDelay" :disabled="testingAll">
          <RefreshCw :size="16" :class="{ spin: testingAll }" />
        </button>
        <button class="tool-btn" :class="{ 'tool-btn-active': sortBy === 'delay' }" :title="t('proxies.sortByDelay')" @click="toggleSort('delay')">
          <ArrowUpDown :size="16" />
        </button>
      </div>
    </div>

    <div class="proxy-nodes-grid">
      <div v-for="node in sortedNodes" :key="node.name" class="proxy-node-card" :class="{ 'proxy-node-active': node.name === currentGroup?.now }" @click="selectNode(node.name)">
        <div class="node-top">
          <span class="node-name">{{ node.name }}</span>
          <span v-if="(node.delay ?? 0) > 0" class="node-delay" :style="{ color: delayColor(node.delay ?? 0) }">{{ node.delay }}</span>
          <span v-else class="node-type-label">{{ node.type }}</span>
        </div>
        <div class="node-tags">
          <span class="node-tag">{{ node.type }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.proxy-page { max-width: 100%; }
.proxy-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px; }
.proxy-title { font-size: 22px; font-weight: 700; margin: 0; }
.proxy-header-actions { display: flex; align-items: center; gap: 12px; }
.mode-switcher { display: flex; gap: 2px; padding: 2px; border-radius: 8px; background-color: var(--bg-tertiary); }
.mode-btn { padding: 6px 14px; border-radius: 6px; font-size: 12px; font-weight: 500; border: none; cursor: pointer; transition: all 150ms ease; background: transparent; color: var(--text-secondary); }
.mode-btn:hover { color: var(--text-primary); }
.mode-btn.active, .mode-btn-active { background-color: var(--accent); color: #fff; }
.chain-btn { display: flex; align-items: center; gap: 6px; padding: 6px 14px; border-radius: 8px; font-size: 12px; font-weight: 500; border: 1px solid var(--border); background: transparent; cursor: pointer; color: var(--text-secondary); transition: all 150ms ease; }
.chain-btn-active { background-color: var(--accent); color: #fff; border-color: var(--accent); }
.proxy-toolbar { display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px; gap: 12px; }
.group-tabs { display: flex; gap: 4px; flex-wrap: wrap; }
.group-tab { padding: 6px 12px; border-radius: 6px; font-size: 12px; font-weight: 500; border: 1px solid var(--border); background: transparent; cursor: pointer; color: var(--text-secondary); transition: all 150ms ease; }
.group-tab.active { background-color: var(--accent); color: #fff; border-color: var(--accent); }
.toolbar-actions { display: flex; gap: 4px; }
.tool-btn { display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; border-radius: 8px; border: 1px solid var(--border); background: transparent; cursor: pointer; color: var(--text-secondary); transition: all 150ms ease; }
.tool-btn:hover { background-color: var(--bg-hover); color: var(--text-primary); }
.tool-btn-active { background-color: var(--accent); color: #fff; border-color: var(--accent); }
.spin { animation: spin 1s linear infinite; }
@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }
.proxy-nodes-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 10px; }
.proxy-node-card { padding: 12px 14px; border-radius: 10px; border: 1px solid var(--border); cursor: pointer; transition: all 150ms ease; background-color: var(--bg-tertiary); }
.proxy-node-card:hover { border-color: var(--accent); }
.proxy-node-active { border-color: var(--accent); border-width: 2px; background-color: color-mix(in srgb, var(--accent) 8%, transparent); }
.node-top { display: flex; align-items: center; justify-content: space-between; margin-bottom: 6px; }
.node-name { font-size: 13px; font-weight: 500; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1; }
.node-delay { font-size: 12px; font-weight: 600; font-family: "SF Mono", "Fira Code", monospace; }
.node-type-label { font-size: 11px; color: var(--text-secondary); }
.node-tags { display: flex; gap: 4px; flex-wrap: wrap; }
.node-tag { font-size: 10px; padding: 1px 6px; border-radius: 4px; background-color: var(--bg-secondary); color: var(--text-secondary); }
</style>
