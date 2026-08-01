<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useI18n } from "vue-i18n";
import { Globe, RefreshCw, ChevronDown } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import EnhancedCard from "@/components/EnhancedCard.vue";

const app = useAppStore();
const { t } = useI18n();

const proxyGroups = computed(() => {
  const groups = app.proxyRunning ? app.proxyGroups : app.subProxyGroups;
  return groups.filter(g => g.name !== "GLOBAL");
});

const selectedGroup = ref("");
const selectedNode = ref("");
const nodeDelays = ref<Map<string, number>>(new Map());
const testingAll = ref(false);
const showNodeDropdown = ref(false);

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

const currentMode = computed(() => app.proxyMode);

function delayColor(delay: number): string {
  if (delay === 0) return "var(--text-secondary)";
  if (delay < 200) return "var(--green)";
  if (delay < 400) return "var(--blue)";
  if (delay < 800) return "var(--orange)";
  return "var(--red)";
}

function delayText(name: string): string {
  if (testingAll.value) return "测试中";
  const d = nodeDelays.value.get(name);
  if (d === undefined) return "待测";
  if (d === 0) return "超时";
  return `${d}ms`;
}

function delayClass(name: string): string {
  if (testingAll.value) return "testing";
  const d = nodeDelays.value.get(name);
  if (d === undefined) return "";
  if (d === 0) return "fail";
  return "";
}

function delayStyle(name: string): string {
  if (testingAll.value) return "var(--accent)";
  const d = nodeDelays.value.get(name);
  if (d === undefined || d === 0) return "var(--text-secondary)";
  return delayColor(d);
}

function modeLabel(mode: string): string {
  switch (mode) { case "global": return t("home.currentNode.globalMode"); case "direct": return t("home.currentNode.directMode"); default: return t("home.currentNode.ruleMode"); }
}

function modeColor(mode: string): string {
  switch (mode) { case "global": return "var(--orange)"; case "direct": return "var(--green)"; default: return "var(--accent)"; }
}

async function selectNode(nodeName: string) {
  if (!app.proxyRunning) return;
  selectedNode.value = nodeName;
  showNodeDropdown.value = false;
  await app.selectProxyNode(selectedGroup.value, nodeName);
}

async function testAllDelay() {
  if (!app.proxyRunning || !currentNodes.value.length) return;
  testingAll.value = true;
  const nodes = currentNodes.value;
  await Promise.all(nodes.map(async (n) => {
    const delay = await app.testNodeDelay(n.name);
    const next = new Map(nodeDelays.value);
    next.set(n.name, delay);
    nodeDelays.value = next;
  }));
  testingAll.value = false;
}

function toggleNodeDropdown() {
  showNodeDropdown.value = !showNodeDropdown.value;
}

function onClickOutside(e: MouseEvent) {
  const target = e.target as HTMLElement;
  if (!target.closest('.node-dropdown-wrap')) {
    showNodeDropdown.value = false;
  }
}

if (typeof window !== 'undefined') {
  watch(showNodeDropdown, (val) => {
    if (val) {
      document.addEventListener('click', onClickOutside);
    } else {
      document.removeEventListener('click', onClickOutside);
    }
  });
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
        <button class="test-btn" :disabled="!app.proxyRunning || currentNodes.length === 0 || testingAll" @click="testAllDelay" :title="t('proxies.testAll')">
          <RefreshCw :size="12" :class="{ spin: testingAll }" />
        </button>
        <button class="mode-badge" :style="{ backgroundColor: modeColor(currentMode), color: '#fff' }">
          <Globe :size="10" />
          {{ modeLabel(currentMode) }}
        </button>
      </div>
    </template>
    <div class="node-body">
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
        <div class="node-dropdown-wrap">
          <div class="node-dropdown-trigger" @click="toggleNodeDropdown">
            <span class="node-dropdown-selected">{{ selectedNode || '-' }}</span>
            <span class="node-dropdown-delay" :style="{ color: delayStyle(selectedNode) }" :class="delayClass(selectedNode)">{{ delayText(selectedNode) }}</span>
            <ChevronDown :size="14" class="node-dropdown-chevron" :class="{ open: showNodeDropdown }" />
          </div>
          <div v-if="showNodeDropdown" class="node-dropdown-panel">
            <div v-for="n in currentNodes" :key="n.name"
              class="node-dropdown-item"
              :class="{ 'node-dropdown-item-active': n.name === selectedNode }"
              @click="selectNode(n.name)"
            >
              <span class="node-dropdown-item-name">{{ n.type }} - {{ n.name }}</span>
              <span class="node-dropdown-item-delay" :style="{ color: delayStyle(n.name) }" :class="delayClass(n.name)">{{ delayText(n.name) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.node-title { font-size: 16px; font-weight: 700; }
.node-body { padding: 12px 16px; display: flex; flex-direction: column; gap: 12px; }
.mode-badge { display: inline-flex; align-items: center; gap: 4px; padding: 3px 8px; border-radius: 4px; font-size: 11px; font-weight: 600; border: none; }
.test-btn { display: flex; align-items: center; justify-content: center; width: 26px; height: 26px; border-radius: 6px; border: 1px solid var(--border); background: transparent; cursor: pointer; color: var(--text-secondary); transition: all 150ms ease; }
.test-btn:hover { border-color: var(--accent); color: var(--accent); }
.test-btn:disabled { opacity: 0.5; cursor: not-allowed; }
.select-group { display: flex; flex-direction: column; gap: 4px; }
.select-label { font-size: 11px; color: var(--text-secondary); font-weight: 500; }
.select-wrap { position: relative; }
.select-input { width: 100%; padding: 6px 10px; padding-right: 28px; border-radius: 6px; border: 1px solid var(--border); background-color: var(--bg-tertiary); color: var(--text-primary); font-size: 13px; outline: none; cursor: pointer; appearance: none; background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='14' height='14' viewBox='0 0 24 24' fill='none' stroke='%23888' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E"); background-repeat: no-repeat; background-position: right 6px center; }
.select-input:focus { border-color: var(--accent); }

.node-dropdown-wrap { position: relative; }
.node-dropdown-trigger { display: flex; align-items: center; gap: 8px; padding: 6px 10px; border-radius: 6px; border: 1px solid var(--border); background-color: var(--bg-tertiary); cursor: pointer; transition: all 150ms ease; }
.node-dropdown-trigger:hover { border-color: var(--accent); }
.node-dropdown-selected { flex: 1; font-size: 13px; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.node-dropdown-delay { font-size: 11px; font-weight: 600; font-family: "SF Mono", "Fira Code", monospace; flex-shrink: 0; }
.node-dropdown-delay.testing { color: var(--accent); font-weight: 500; }
.node-dropdown-delay.fail { color: var(--red); font-weight: 500; }
.node-dropdown-chevron { color: var(--text-secondary); flex-shrink: 0; transition: transform 200ms ease; }
.node-dropdown-chevron.open { transform: rotate(180deg); }
.node-dropdown-panel { position: absolute; top: 100%; left: 0; right: 0; z-index: 50; margin-top: 4px; border-radius: 8px; border: 1px solid var(--border); background-color: var(--bg-secondary); box-shadow: 0 8px 24px rgba(0,0,0,0.3); max-height: 240px; overflow-y: auto; }
.node-dropdown-item { display: flex; align-items: center; justify-content: space-between; padding: 8px 12px; cursor: pointer; transition: background-color 100ms ease; }
.node-dropdown-item:hover { background-color: var(--bg-hover); }
.node-dropdown-item-active { background-color: color-mix(in srgb, var(--accent) 8%, transparent); }
.node-dropdown-item-name { font-size: 13px; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1; }
.node-dropdown-item-delay { font-size: 11px; font-weight: 600; font-family: "SF Mono", "Fira Code", monospace; flex-shrink: 0; margin-left: 8px; }
.node-dropdown-item-delay.testing { color: var(--accent); font-weight: 500; }
.node-dropdown-item-delay.fail { color: var(--red); font-weight: 500; }
.spin { animation: spin 1s linear infinite; }
@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }
</style>