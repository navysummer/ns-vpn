<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { RefreshCw, Zap, Globe, Shield, Route, Gauge } from "lucide-vue-next";
import { useToast } from "@/utils/toast";
import { useAppStore } from "@/stores/app";
import { useI18n } from "vue-i18n";

const app = useAppStore();
const { show } = useToast();
const { t } = useI18n();

const selectedGroup = ref("");
const testingNodes = ref<Set<string>>(new Set());
const testingAll = ref(false);
const nodeDelays = ref<Map<string, number>>(new Map());

const proxyGroups = computed(() => {
  if (app.proxyRunning && app.proxyGroups.length > 0) {
    return app.proxyGroups;
  }
  return app.subProxyGroups;
});

const filteredGroups = computed(() => {
  const groups = proxyGroups.value;
  if (app.proxyMode === "global") {
    return groups.filter(g => g.name === "GLOBAL");
  }
  return groups.filter(g => g.name !== "GLOBAL");
});

const currentGroup = computed(() => {
  return filteredGroups.value.find(g => g.name === selectedGroup.value);
});

const currentNodes = computed(() => {
  return currentGroup.value?.all ?? [];
});

const isDirectMode = computed(() => app.proxyMode === "direct");

watch(() => filteredGroups.value, (groups) => {
  if (groups.length > 0) {
    if (!selectedGroup.value || !groups.find(g => g.name === selectedGroup.value)) {
      selectedGroup.value = groups[0].name;
    }
  }
}, { immediate: true });

async function selectNode(nodeName: string) {
  if (!app.proxyRunning) {
    show(t("proxies.coreNotRunning"), "error");
    return;
  }
  if (!currentGroup.value) return;
  await app.selectProxyNode(currentGroup.value.name, nodeName);
}

async function testNodeDelay(nodeName: string) {
  if (!app.proxyRunning) return;
  testingNodes.value = new Set(testingNodes.value).add(nodeName);
  const delay = await app.testNodeDelay(nodeName);
  const next = new Map(nodeDelays.value);
  next.set(nodeName, delay);
  nodeDelays.value = next;
  const nextSet = new Set(testingNodes.value);
  nextSet.delete(nodeName);
  testingNodes.value = nextSet;
}

async function testAllDelay() {
  if (!app.proxyRunning || !currentGroup.value) return;
  testingAll.value = true;
  const group = currentGroup.value;
  await Promise.all(group.all.map(node => testNodeDelay(node.name)));
  setTimeout(() => { testingAll.value = false; }, 1000);
}

function delayColor(delay: number): string {
  if (delay === 0) return "var(--text-secondary)";
  if (delay < 200) return "var(--green)";
  if (delay < 400) return "var(--blue)";
  if (delay < 800) return "var(--orange)";
  return "var(--red)";
}

function groupTypeLabel(type: string): string {
  switch (type) {
    case "Selector": return "手动选择";
    case "URLTest": return "自动测速";
    case "Fallback": return "故障转移";
    case "LoadBalance": return "负载均衡";
    default: return type;
  }
}

function nodeTypeIcon(type: string): string {
  switch (type) {
    case "Vless": case "Vmess": return "V";
    case "Trojan": return "T";
    case "Shadowsocks": case "ss": return "S";
    case "Hysteria": case "Hysteria2": return "H";
    case "Direct": return "D";
    case "Reject": return "R";
    case "URLTest": return "U";
    case "Fallback": return "F";
    case "LoadBalance": return "L";
    default: return "?";
  }
}
</script>

<template>
  <div class="proxy-page">
    <div class="proxy-header">
      <h1 class="proxy-title">{{ t('proxies.title') }}</h1>
      <div class="proxy-header-actions">
        <div class="mode-switcher">
          <button class="mode-btn" :class="{ 'mode-btn-active': app.proxyMode === 'rule' }" @click="app.changeProxyMode('rule')">
            <Route :size="14" /> {{ t('dashboard.ruleMode') }}
          </button>
          <button class="mode-btn" :class="{ 'mode-btn-active': app.proxyMode === 'global' }" @click="app.changeProxyMode('global')">
            <Globe :size="14" /> {{ t('dashboard.globalMode') }}
          </button>
          <button class="mode-btn" :class="{ 'mode-btn-active': app.proxyMode === 'direct' }" @click="app.changeProxyMode('direct')">
            <Shield :size="14" /> {{ t('dashboard.directMode') }}
          </button>
        </div>
      </div>
    </div>

    <div v-if="isDirectMode" class="proxy-empty-state">
      <Shield :size="48" :style="{ color: 'var(--green)', opacity: 0.5 }" />
      <h3>{{ t('proxies.directModeTitle') }}</h3>
      <p>{{ t('proxies.directModeDesc') }}</p>
    </div>

    <template v-else>
      <div class="group-selector-bar">
        <div class="group-selector">
          <template v-if="app.proxyMode === 'global'">
            <div class="global-group-label">
              <Globe :size="18" class="global-icon" />
              <span class="global-group-name">GLOBAL</span>
            </div>
          </template>
          <template v-else>
            <select v-model="selectedGroup" class="group-select">
              <option v-for="g in filteredGroups" :key="g.name" :value="g.name">{{ g.name }}</option>
            </select>
          </template>
          <div class="group-info-row">
            <span class="group-type-badge">{{ currentGroup ? groupTypeLabel(currentGroup.type) : '' }}</span>
            <span class="group-now">{{ currentGroup?.now ?? '' }}</span>
            <span class="group-count">{{ currentNodes.length }} {{ t('proxies.nodes') }}</span>
          </div>
        </div>
        <div class="group-actions">
          <button class="action-btn" :class="{ spinning: testingAll }" :disabled="testingAll || !app.proxyRunning" @click="testAllDelay" :title="t('proxies.testAll')">
            <Gauge :size="16" :class="{ spin: testingAll }" />
            <span>{{ t('proxies.testAll') }}</span>
          </button>
        </div>
      </div>

      <div v-if="filteredGroups.length === 0" class="proxy-empty-state">
        <Shield :size="48" :style="{ color: 'var(--text-secondary)', opacity: 0.3 }" />
        <h3>{{ t('proxies.noGroups') }}</h3>
        <p>{{ app.proxyRunning ? t('proxies.noGroupsDesc') : t('proxies.coreNotRunning') }}</p>
      </div>

      <div v-else-if="filteredGroups.length > 0" class="nodes-container">
        <div class="nodes-header">
          <span class="nodes-title">{{ currentGroup?.name }}</span>
          <span v-if="!app.proxyRunning" class="offline-badge">{{ t('proxies.offline') }}</span>
        </div>
        <div class="nodes-grid">
          <div v-for="node in currentNodes" :key="node.name"
            class="node-card">
            <div class="node-top">
              <span class="node-type-badge" :class="'type-' + node.type.toLowerCase()">{{ nodeTypeIcon(node.type) }}</span>
              <span class="node-name">{{ node.name }}</span>
              <span class="node-type-label">{{ node.type }}</span>
            </div>
            <div class="node-bottom">
              <div class="node-delay" :style="{ color: delayColor(nodeDelays.get(node.name) ?? 0) }">
                <span v-if="testingNodes.has(node.name)" class="delay-testing">测试中</span>
                <template v-else-if="!nodeDelays.has(node.name)">
                  <span class="delay-na">待测</span>
                </template>
                <template v-else-if="nodeDelays.get(node.name) === 0">
                  <span class="delay-fail">超时</span>
                </template>
                <template v-else>
                  <Zap :size="10" />
                  <span>{{ nodeDelays.get(node.name) }} ms</span>
                </template>
              </div>
              <button class="node-test-btn" :class="{ spinning: testingNodes.has(node.name) }"
                @click.stop="testNodeDelay(node.name)"
                :title="t('proxies.testDelay')"
              >
                <RefreshCw :size="12" :class="{ spin: testingNodes.has(node.name) }" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.proxy-page { max-width: 100%; }

.proxy-header {
  display: flex; align-items: center; justify-content: space-between;
  margin-bottom: 16px;
}
.proxy-title { font-size: 22px; font-weight: 700; margin: 0; }
.proxy-header-actions { display: flex; align-items: center; gap: 12px; }

.mode-switcher {
  display: flex; gap: 2px; padding: 2px;
  border-radius: 8px; background-color: var(--bg-tertiary);
}
.mode-btn {
  display: inline-flex; align-items: center; gap: 5px;
  padding: 6px 14px; border-radius: 6px; font-size: 12px; font-weight: 500;
  border: none; cursor: pointer; transition: all 150ms ease;
  background: transparent; color: var(--text-secondary);
}
.mode-btn:hover { color: var(--text-primary); }
.mode-btn-active { background-color: var(--accent); color: #fff; }

.proxy-empty-state {
  display: flex; flex-direction: column; align-items: center; justify-content: center;
  padding: 60px 20px; gap: 12px; text-align: center;
}
.proxy-empty-state h3 { font-size: 16px; font-weight: 600; margin: 0; color: var(--text-primary); }
.proxy-empty-state p { font-size: 13px; margin: 0; color: var(--text-secondary); }

.group-selector-bar {
  display: flex; align-items: center; justify-content: space-between;
  gap: 12px; margin-bottom: 16px;
  padding: 12px 16px; border-radius: 10px;
  border: 1px solid var(--border); background-color: var(--card-bg);
}
.group-selector { flex: 1; display: flex; flex-direction: column; gap: 8px; }
.global-group-label {
  display: flex; align-items: center; gap: 8px;
  padding: 8px 12px; border-radius: 8px;
  background-color: var(--bg-tertiary);
}
.global-icon { color: var(--accent); flex-shrink: 0; }
.global-group-name { font-size: 14px; font-weight: 600; color: var(--text-primary); }
.group-select {
  width: 100%; padding: 8px 12px; padding-right: 32px;
  border-radius: 8px; border: 1px solid var(--border);
  background-color: var(--bg-tertiary); color: var(--text-primary);
  font-size: 14px; font-weight: 600; outline: none;
  cursor: pointer; appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%23888' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E");
  background-repeat: no-repeat; background-position: right 8px center;
}
.group-select:focus { border-color: var(--accent); }
.group-info-row { display: flex; align-items: center; gap: 8px; }
.group-type-badge {
  font-size: 11px; padding: 2px 8px; border-radius: 4px;
  background: rgba(79, 142, 247, 0.1); color: var(--accent); font-weight: 500;
}
.group-now {
  font-size: 12px; color: var(--text-secondary);
  overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
}
.group-count { font-size: 12px; color: var(--text-secondary); }

.group-actions { flex-shrink: 0; }
.action-btn {
  display: inline-flex; align-items: center; gap: 6px;
  padding: 8px 14px; border-radius: 8px; font-size: 13px; font-weight: 500;
  border: 1px solid var(--border); background: transparent;
  color: var(--text-primary); cursor: pointer; transition: all 150ms ease;
}
.action-btn:hover { border-color: var(--accent); color: var(--accent); }
.action-btn:disabled { opacity: 0.5; cursor: not-allowed; }

.nodes-container {
  display: flex; flex-direction: column; gap: 12px;
}
.nodes-header {
  display: flex; align-items: center; gap: 8px; padding: 0 2px;
}
.nodes-title { font-size: 15px; font-weight: 600; color: var(--text-primary); }
.offline-badge {
  font-size: 11px; padding: 2px 8px; border-radius: 4px;
  background: rgba(255, 159, 10, 0.15); color: var(--orange); font-weight: 500;
}

.nodes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 8px;
}

.node-card {
  display: flex; flex-direction: column; gap: 8px;
  padding: 12px 14px; border-radius: 10px;
  border: 1px solid var(--border); background-color: var(--card-bg);
  transition: all 150ms ease;
}
.node-card:hover { border-color: var(--accent); }

.node-top { display: flex; align-items: center; gap: 8px; min-width: 0; }
.node-type-badge {
  width: 20px; height: 20px; border-radius: 4px;
  display: flex; align-items: center; justify-content: center;
  font-size: 10px; font-weight: 700; flex-shrink: 0;
  background: var(--bg-tertiary); color: var(--text-secondary);
}
.type-vless, .type-vmess { background: rgba(79, 142, 247, 0.12); color: var(--accent); }
.type-trojan { background: rgba(255, 69, 58, 0.12); color: var(--red); }
.type-shadowsocks, .type-ss { background: rgba(52, 199, 89, 0.12); color: var(--green); }
.type-hysteria, .type-hysteria2 { background: rgba(255, 159, 10, 0.12); color: var(--orange); }
.type-direct { background: rgba(52, 199, 89, 0.12); color: var(--green); }
.type-reject { background: rgba(255, 69, 58, 0.12); color: var(--red); }

.node-name {
  font-size: 13px; font-weight: 500; color: var(--text-primary);
  overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
}
.node-type-label {
  font-size: 10px; color: var(--text-secondary); flex-shrink: 0;
  margin-left: auto; padding: 1px 6px; border-radius: 4px;
  background: var(--bg-tertiary); font-weight: 500;
}

.node-bottom { display: flex; align-items: center; justify-content: space-between; }
.node-delay {
  display: flex; align-items: center; gap: 4px;
  font-size: 12px; font-weight: 600;
  font-family: "SF Mono", "Fira Code", monospace;
}
.delay-na { color: var(--text-secondary); font-weight: 400; }
.delay-testing { color: var(--accent); font-size: 11px; font-weight: 500; }
.delay-fail { color: var(--red); font-size: 11px; font-weight: 500; }

.node-test-btn {
  display: flex; align-items: center; justify-content: center;
  width: 26px; height: 26px; border-radius: 6px;
  border: 1px solid var(--border); background: transparent;
  cursor: pointer; color: var(--text-secondary); transition: all 150ms ease;
}
.node-test-btn:hover { border-color: var(--accent); color: var(--accent); }
.node-test-btn.spinning { border-color: var(--accent); color: var(--accent); background: rgba(79,142,247,0.08); }

.spin { animation: spin 1s linear infinite; }
@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }

@media (max-width: 768px) {
  .nodes-grid { grid-template-columns: 1fr; }
  .group-selector-bar { flex-direction: column; align-items: stretch; }
}
</style>