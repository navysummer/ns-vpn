<script setup lang="ts">
import { ref } from "vue";
import { RefreshCw, Check, Zap, Globe, Shield, Route, Heart, AlertTriangle } from "lucide-vue-next";
import { delayQuality, formatDelay } from "@/utils/format";
import BasePage from "@/components/BasePage.vue";
import ProviderButton from "@/components/ProviderButton.vue";
import EmptyState from "@/components/EmptyState.vue";
import { useToast } from "@/utils/toast";

const { show } = useToast();

interface ProxyNode {
  name: string;
  type: string;
  delay: number;
  now: boolean;
}

interface ProxyGroup {
  name: string;
  type: "Selector" | "URLTest" | "Fallback" | "LoadBalance";
  now: string;
  all: ProxyNode[];
  lastHealthCheck?: number;
  healthy?: boolean;
}

const groups = ref<ProxyGroup[]>([
  {
    name: "Proxy",
    type: "Selector",
    now: "Auto",
    all: [
      { name: "Auto", type: "URLTest", delay: 45, now: false },
      { name: "HK-01", type: "Shadowsocks", delay: 32, now: false },
      { name: "HK-02", type: "Shadowsocks", delay: 0, now: false },
      { name: "JP-01", type: "VMess", delay: 78, now: false },
      { name: "JP-02", type: "VMess", delay: 120, now: false },
      { name: "US-01", type: "Trojan", delay: 210, now: false },
      { name: "US-02", type: "Trojan", delay: 0, now: false },
      { name: "SG-01", type: "Shadowsocks", delay: 55, now: false },
      { name: "Direct", type: "Direct", delay: 0, now: true },
      { name: "Reject", type: "Reject", delay: 0, now: false },
    ],
  },
  {
    name: "Ai",
    type: "Selector",
    now: "Auto",
    all: [
      { name: "Auto", type: "URLTest", delay: 45, now: true },
      { name: "HK-01", type: "Shadowsocks", delay: 32, now: false },
      { name: "JP-01", type: "VMess", delay: 78, now: false },
      { name: "US-01", type: "Trojan", delay: 210, now: false },
    ],
  },
  {
    name: "Media",
    type: "Selector",
    now: "Auto",
    all: [
      { name: "Auto", type: "URLTest", delay: 45, now: true },
      { name: "HK-01", type: "Shadowsocks", delay: 32, now: false },
      { name: "JP-01", type: "VMess", delay: 78, now: false },
      { name: "SG-01", type: "Shadowsocks", delay: 55, now: false },
    ],
  },
  {
    name: "Auto",
    type: "URLTest",
    now: "HK-01",
    lastHealthCheck: Date.now() - 30000,
    healthy: true,
    all: [
      { name: "HK-01", type: "Shadowsocks", delay: 32, now: false },
      { name: "HK-02", type: "Shadowsocks", delay: 0, now: false },
      { name: "JP-01", type: "VMess", delay: 78, now: false },
      { name: "US-01", type: "Trojan", delay: 210, now: false },
      { name: "SG-01", type: "Shadowsocks", delay: 55, now: false },
    ],
  },
]);

const proxyProviders = ref([
  { name: "airport-nodes", count: 45, loading: false },
  { name: "free-nodes", count: 12, loading: false },
]);

const selectedGroup = ref(groups.value[0].name);
const testingAll = ref(false);
const testingGroup = ref<string | null>(null);
const proxyMode = ref<"rule" | "global" | "direct">("rule");
const searchQuery = ref("");

function selectGroup(name: string) { selectedGroup.value = name; }

function selectNode(groupName: string, nodeName: string) {
  const group = groups.value.find((g) => g.name === groupName);
  if (group) {
    group.now = nodeName;
    group.all.forEach((n) => (n.now = n.name === nodeName));
  }
}

function testGroupDelay(groupName: string) {
  testingGroup.value = groupName;
  const group = groups.value.find((g) => g.name === groupName);
  if (group) {
    group.all.forEach((n) => { n.delay = 0; });
    group.all.forEach((n) => {
      setTimeout(() => { n.delay = Math.floor(Math.random() * 300) + 20; }, Math.random() * 1500);
    });
    if (group.type === "URLTest") {
      group.lastHealthCheck = Date.now();
      group.healthy = true;
    }
  }
  setTimeout(() => { testingGroup.value = null; }, 2000);
}

function testAllDelay() {
  testingAll.value = true;
  groups.value.forEach((g) => testGroupDelay(g.name));
  setTimeout(() => { testingAll.value = false; }, 2500);
}

function setProxyMode(mode: "rule" | "global" | "direct") {
  proxyMode.value = mode;
  show(`已切换到${mode === "rule" ? "规则" : mode === "global" ? "全局" : "直连"}模式`, "info");
}

function filteredNodes(nodes: ProxyNode[]): ProxyNode[] {
  if (!searchQuery.value) return nodes;
  const q = searchQuery.value.toLowerCase();
  return nodes.filter(n => n.name.toLowerCase().includes(q) || n.type.toLowerCase().includes(q));
}

function typeBadgeColor(type: ProxyGroup["type"]): string {
  switch (type) {
    case "Selector": return "#4f8ef7";
    case "URLTest": return "#34c759";
    case "Fallback": return "#ff9f0a";
    case "LoadBalance": return "#bf5af2";
    default: return "#98989e";
  }
}

function typeBadgeBg(type: ProxyGroup["type"]): string {
  switch (type) {
    case "Selector": return "rgba(79,142,247,0.12)";
    case "URLTest": return "rgba(52,199,89,0.12)";
    case "Fallback": return "rgba(255,159,10,0.12)";
    case "LoadBalance": return "rgba(191,90,242,0.12)";
    default: return "rgba(152,152,158,0.12)";
  }
}

function refreshProvider(name: string) {
  const p = proxyProviders.value.find(x => x.name === name);
  if (p) {
    p.loading = true;
    setTimeout(() => { p.loading = false; }, 1500);
  }
}

function healthyNodes(group: ProxyGroup): number {
  return group.all.filter(n => n.delay > 0).length;
}

function lastCheckAgo(ts?: number): string {
  if (!ts) return "未测试";
  const diff = Math.floor((Date.now() - ts) / 1000);
  if (diff < 60) return `${diff}秒前`;
  if (diff < 3600) return `${Math.floor(diff / 60)}分钟前`;
  return `${Math.floor(diff / 3600)}小时前`;
}
</script>

<template>
  <BasePage title="代理">
    <template #actions>
      <div class="flex items-center gap-2">
        <div class="mode-switcher">
          <button class="mode-btn" :class="{ 'mode-btn-active': proxyMode === 'rule' }" @click="setProxyMode('rule')"><Route :size="12" />规则</button>
          <button class="mode-btn" :class="{ 'mode-btn-active': proxyMode === 'global' }" @click="setProxyMode('global')"><Globe :size="12" />全局</button>
          <button class="mode-btn" :class="{ 'mode-btn-active': proxyMode === 'direct' }" @click="setProxyMode('direct')"><Shield :size="12" />直连</button>
        </div>
        <button class="btn-ghost text-xs" :disabled="testingAll" @click="testAllDelay">
          <Zap :size="14" :class="{ spin: testingAll }" />
          {{ testingAll ? "测试中..." : "延迟测试" }}
        </button>
      </div>
    </template>

    <div class="flex gap-4 flex-1 min-h-0">
      <div class="proxy-sidebar">
        <div class="proxy-search">
          <input v-model="searchQuery" placeholder="搜索节点..." class="proxy-search-input" :style="{ color: 'var(--text-primary)' }" />
        </div>
        <div class="proxy-groups-list">
          <button
            v-for="group in groups"
            :key="group.name"
            class="proxy-group-btn"
            :class="{ 'proxy-group-btn-active': selectedGroup === group.name }"
            @click="selectGroup(group.name)"
          >
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium truncate">{{ group.name }}</div>
              <div class="flex items-center gap-1 mt-0.5">
                <span class="type-badge" :style="{ backgroundColor: typeBadgeBg(group.type), color: typeBadgeColor(group.type) }">{{ group.type }}</span>
                <span class="text-xs" :style="{ color: 'var(--text-secondary)' }">{{ group.all.length }}</span>
              </div>
            </div>
            <div class="flex flex-col items-end gap-1">
              <span class="text-xs mono" :style="{ color: 'var(--accent)' }">{{ group.now }}</span>
              <div v-if="group.type === 'URLTest'" class="flex items-center gap-1">
                <span v-if="group.healthy" class="health-dot health-ok"></span>
                <span v-else class="health-dot health-bad"></span>
                <span class="text-xs" :style="{ color: 'var(--text-secondary)' }">{{ lastCheckAgo(group.lastHealthCheck) }}</span>
              </div>
            </div>
          </button>
        </div>
      </div>

      <div class="proxy-nodes-panel flex-1">
        <div class="flex items-center justify-between mb-3">
          <div class="flex items-center gap-2">
            <span class="text-sm font-medium">{{ selectedGroup }}</span>
            <span class="type-badge" :style="{ backgroundColor: typeBadgeBg(groups.find(g => g.name === selectedGroup)!.type), color: typeBadgeColor(groups.find(g => g.name === selectedGroup)!.type) }">
              {{ groups.find(g => g.name === selectedGroup)!.type }}
            </span>
            <span v-if="groups.find(g => g.name === selectedGroup)?.type === 'URLTest'" class="text-xs" :style="{ color: 'var(--text-secondary)' }">
              {{ healthyNodes(groups.find(g => g.name === selectedGroup)!) }}/{{ groups.find(g => g.name === selectedGroup)!.all.length }} 可用
            </span>
          </div>
          <button class="btn-ghost text-xs" :disabled="testingGroup === selectedGroup" @click="testGroupDelay(selectedGroup)">
            <Zap :size="12" :class="{ spin: testingGroup === selectedGroup }" />
            {{ testingGroup === selectedGroup ? "测试中..." : "测试" }}
          </button>
        </div>

        <div class="proxy-nodes-list">
          <div
            v-for="node in filteredNodes(groups.find(g => g.name === selectedGroup)?.all || [])"
            :key="node.name"
            class="proxy-node"
            :class="{ 'proxy-node-active': node.now }"
            @click="selectNode(selectedGroup, node.name)"
          >
            <div class="w-5 shrink-0"><Check v-if="node.now" :size="14" /></div>
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium truncate">{{ node.name }}</div>
              <div class="text-xs" :style="{ color: node.now ? 'rgba(255,255,255,0.6)' : 'var(--text-secondary)' }">{{ node.type }}</div>
            </div>
            <div class="delay-badge" :class="{ 'delay-good': delayQuality(node.delay) === 'good', 'delay-medium': delayQuality(node.delay) === 'medium', 'delay-bad': delayQuality(node.delay) === 'bad', 'delay-none': delayQuality(node.delay) === 'none', 'delay-active': node.now }">
              <Zap v-if="node.delay > 0" :size="10" />
              {{ formatDelay(node.delay) }}
            </div>
          </div>
          <EmptyState v-if="filteredNodes(groups.find(g => g.name === selectedGroup)?.all || []).length === 0" title="暂无节点" />
        </div>
      </div>

      <div class="providers-panel">
        <div class="text-xs font-medium mb-3" :style="{ color: 'var(--text-secondary)' }">代理提供者</div>
        <div class="space-y-2">
          <ProviderButton
            v-for="provider in proxyProviders"
            :key="provider.name"
            :name="provider.name"
            type="proxy"
            :count="provider.count"
            :loading="provider.loading"
            @refresh="refreshProvider(provider.name)"
          />
        </div>
      </div>
    </div>
  </BasePage>
</template>

<style scoped>
.mode-switcher {
  display: flex;
  gap: 2px;
  padding: 2px;
  border-radius: 8px;
  background-color: var(--bg-tertiary);
}

.mode-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  border: none;
  cursor: pointer;
  transition: all 150ms ease;
  background: transparent;
  color: var(--text-secondary);
}
.mode-btn:hover { color: var(--text-primary); }
.mode-btn-active { background-color: var(--accent); color: #fff; }

.proxy-sidebar {
  width: 200px;
  min-width: 200px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.proxy-search-input {
  width: 100%;
  padding: 6px 10px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  font-size: 12px;
  outline: none;
}
.proxy-search-input:focus { border-color: var(--accent); }

.proxy-groups-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
  overflow-y: auto;
  flex: 1;
}

.proxy-group-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  border-radius: 8px;
  border: none;
  cursor: pointer;
  transition: all 150ms ease;
  background: transparent;
  text-align: left;
  width: 100%;
}
.proxy-group-btn:hover { background-color: var(--bg-hover); }
.proxy-group-btn-active { background-color: rgba(79,142,247,0.1); border: 1px solid rgba(79,142,247,0.3); }

.proxy-nodes-panel {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.proxy-nodes-list {
  display: flex;
  flex-direction: column;
  gap: 1px;
  overflow-y: auto;
  flex: 1;
  max-height: calc(100vh - 240px);
}

.proxy-node {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: background-color 150ms ease;
  color: var(--text-primary);
}
.proxy-node:hover { background-color: var(--bg-hover); }
.proxy-node-active { background-color: var(--accent) !important; color: #fff !important; }

.type-badge {
  display: inline-flex;
  padding: 1px 6px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
}

.delay-badge {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-family: "SF Mono", "Fira Code", monospace;
  background-color: var(--bg-tertiary);
  min-width: 50px;
  justify-content: center;
}
.delay-good { color: var(--green); }
.delay-medium { color: var(--orange); }
.delay-bad { color: var(--red); }
.delay-none { color: var(--text-secondary); }
.delay-active { background-color: rgba(255,255,255,0.15); }

.health-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}
.health-ok { background-color: var(--green); }
.health-bad { background-color: var(--red); }

.providers-panel {
  width: 200px;
  min-width: 200px;
  padding: 16px;
  border-radius: 12px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
  height: fit-content;
}
</style>
