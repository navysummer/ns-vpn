<script setup lang="ts">
import { ref, computed } from "vue";
import { RefreshCw, Check, Zap, Globe, Shield, Route, ArrowUpDown, Eye, EyeOff, Filter, Link, RotateCcw } from "lucide-vue-next";
import { delayQuality } from "@/utils/format";
import { useToast } from "@/utils/toast";
import EnhancedCard from "@/components/EnhancedCard.vue";

const { show } = useToast();

interface ProxyNode {
  name: string;
  type: string;
  protocol: string[];
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
    now: "HK-01",
    all: [
      { name: "DIRECT", type: "Direct", protocol: ["Direct", "UDP"], delay: 0, now: false },
      { name: "REJECT", type: "Reject", protocol: ["Reject", "UDP"], delay: 0, now: false },
      { name: "🇨🇦 加拿大 | CAN", type: "Hysteria2", protocol: ["Hysteria2", "UDP"], delay: 271, now: false },
      { name: "🇨🇭 瑞士 | CHE", type: "Shadowsocks", protocol: ["Shadowsocks"], delay: 0, now: false },
      { name: "🇨🇭 瑞士 | CHE 2", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 0, now: false },
      { name: "🇨🇴 哥伦比亚 | COL", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 0, now: false },
      { name: "🇨🇴 哥伦比亚 | COL 2", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 0, now: false },
      { name: "🇩🇪 德国 | DEU", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 0, now: false },
      { name: "🇩🇪 德国 | DEU 2", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 0, now: false },
      { name: "🇩🇪 德国 | DEU 3", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 325, now: false },
      { name: "🇩🇪 德国 | DEU 4", type: "Vmess", protocol: ["Vmess"], delay: 0, now: false },
      { name: "🇩🇪 德国 | DEU 5", type: "Vmess", protocol: ["Vmess"], delay: 0, now: false },
      { name: "🇩🇪 德国 | DEU 6", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 378, now: false },
      { name: "🇩🇪 德国 | DEU 7", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 0, now: false },
      { name: "🇩🇪 德国 | DEU 8", type: "Hysteria", protocol: ["Hysteria", "UDP"], delay: 268, now: true },
      { name: "🇩🇪 德国 | DEU 9", type: "Hysteria2", protocol: ["Hysteria2", "UDP"], delay: 363, now: false },
      { name: "🇩🇪 德国 | DEU 10", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 0, now: false },
      { name: "🇩🇪 德国 | DEU 11", type: "Vless", protocol: ["Vless", "XUDP"], delay: 1006, now: false },
      { name: "🇩🇪 德国 | DEU 12", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 0, now: false },
      { name: "🇩🇪 德国 | DEU 13", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 334, now: false },
      { name: "🇩🇪 德国 | DEU 14", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 0, now: false },
      { name: "🇩🇪 德国 | DEU 15", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 340, now: false },
      { name: "🇩🇪 德国 | DEU 16", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 240, now: false },
      { name: "🇩🇪 德国 | DEU 17", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 0, now: false },
      { name: "🇩🇪 德国 | DEU 18", type: "Hysteria", protocol: ["Hysteria", "UDP"], delay: 327, now: false },
      { name: "🇩🇪 德国 | DEU 19", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 317, now: false },
      { name: "🇩🇪 德国 | DEU 20", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 259, now: false },
      { name: "🇩🇪 德国 | DEU 21", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 358, now: false },
      { name: "🇩🇪 德国 | DEU 22", type: "Vless", protocol: ["Vless", "UDP", "XUDP"], delay: 0, now: false },
    ],
  },
]);

const selectedGroup = ref(groups.value[0].name);
const testingAll = ref(false);
const proxyMode = ref<"rule" | "global" | "direct">("rule");
const showChain = ref(false);
const sortBy = ref<"none" | "delay" | "name">("none");
const sortAsc = ref(true);

const currentGroup = computed(() => groups.value.find(g => g.name === selectedGroup.value)!);

const sortedNodes = computed(() => {
  let nodes = [...currentGroup.value.all];
  if (sortBy.value === "delay") {
    nodes.sort((a, b) => sortAsc.value ? a.delay - b.delay : b.delay - a.delay);
  } else if (sortBy.value === "name") {
    nodes.sort((a, b) => sortAsc.value ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name));
  }
  return nodes;
});

function selectNode(nodeName: string) {
  currentGroup.value.now = nodeName;
  currentGroup.value.all.forEach(n => (n.now = n.name === nodeName));
}

function testAllDelay() {
  testingAll.value = true;
  currentGroup.value.all.forEach(n => { n.delay = 0; });
  currentGroup.value.all.forEach(n => {
    setTimeout(() => { n.delay = Math.floor(Math.random() * 500) + 20; }, Math.random() * 2000);
  });
  setTimeout(() => { testingAll.value = false; }, 2500);
}

function setProxyMode(mode: "rule" | "global" | "direct") {
  proxyMode.value = mode;
  show(`已切换到${mode === "rule" ? "规则" : mode === "global" ? "全局" : "直连"}模式`, "info");
}

function toggleSort(field: "delay" | "name") {
  if (sortBy.value === field) {
    sortAsc.value = !sortAsc.value;
  } else {
    sortBy.value = field;
    sortAsc.value = true;
  }
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
      <h1 class="proxy-title">代理组</h1>
      <div class="proxy-header-actions">
        <div class="mode-switcher">
          <button class="mode-btn" :class="{ 'mode-btn-active': proxyMode === 'rule' }" @click="setProxyMode('rule')">规则</button>
          <button class="mode-btn" :class="{ 'mode-btn-active': proxyMode === 'global' }" @click="setProxyMode('global')">全局</button>
          <button class="mode-btn" :class="{ 'mode-btn-active': proxyMode === 'direct' }" @click="setProxyMode('direct')">直连</button>
        </div>
        <button class="chain-btn" :class="{ 'chain-btn-active': showChain }" @click="showChain = !showChain">
          <Link :size="14" />
          链式代理
        </button>
      </div>
    </div>

    <div class="proxy-toolbar">
      <button class="tool-btn" title="刷新" @click="testAllDelay" :disabled="testingAll">
        <RefreshCw :size="16" :class="{ spin: testingAll }" />
      </button>
      <button class="tool-btn" :class="{ 'tool-btn-active': sortBy === 'delay' }" title="按延迟排序" @click="toggleSort('delay')">
        <ArrowUpDown :size="16" />
      </button>
      <button class="tool-btn" title="筛选">
        <Filter :size="16" />
      </button>
      <button class="tool-btn" title="显示隐藏">
        <Eye :size="16" />
      </button>
      <button class="tool-btn" title="取消选择">
        <RotateCcw :size="16" />
      </button>
    </div>

    <div class="proxy-nodes-grid">
      <div
        v-for="node in sortedNodes"
        :key="node.name"
        class="proxy-node-card"
        :class="{ 'proxy-node-active': node.now }"
        @click="selectNode(node.name)"
      >
        <div class="node-top">
          <span class="node-name">{{ node.name }}</span>
          <span v-if="node.delay > 0" class="node-delay" :style="{ color: delayColor(node.delay) }">
            {{ node.delay }}
          </span>
          <span v-else-if="node.type === 'Direct' || node.type === 'Reject'" class="node-type-label">{{ node.type }}</span>
          <span v-else class="node-error">Error</span>
        </div>
        <div class="node-tags">
          <span v-for="tag in node.protocol" :key="tag" class="node-tag">{{ tag }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.proxy-page {
  max-width: 100%;
}

.proxy-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.proxy-title {
  font-size: 22px;
  font-weight: 700;
  margin: 0;
}

.proxy-header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

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
  padding: 5px 12px;
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

.chain-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 5px 12px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  border: 1px solid var(--border);
  cursor: pointer;
  transition: all 150ms ease;
  background: transparent;
  color: var(--text-secondary);
}
.chain-btn:hover { border-color: var(--accent); color: var(--text-primary); }
.chain-btn-active { background-color: rgba(79,142,247,0.1); border-color: var(--accent); color: var(--accent); }

.proxy-toolbar {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-bottom: 16px;
}

.tool-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  border: none;
  cursor: pointer;
  transition: all 150ms ease;
  background: transparent;
  color: var(--text-secondary);
}
.tool-btn:hover { background-color: var(--bg-hover); color: var(--text-primary); }
.tool-btn-active { background-color: rgba(79,142,247,0.1); color: var(--accent); }

.proxy-nodes-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.proxy-node-card {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 12px 14px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
  cursor: pointer;
  transition: all 150ms ease;
}
.proxy-node-card:hover {
  border-color: var(--accent);
  background-color: var(--bg-hover);
}

.proxy-node-active {
  border-color: var(--accent) !important;
  background-color: rgba(79,142,247,0.1) !important;
}

.node-top {
  display: flex;
  align-items: center;
  gap: 8px;
}

.node-name {
  font-size: 13px;
  font-weight: 500;
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.node-delay {
  font-size: 12px;
  font-weight: 600;
  font-family: "SF Mono", "Fira Code", monospace;
}

.node-error {
  font-size: 12px;
  font-weight: 600;
  color: var(--red);
}

.node-type-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.node-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.node-tag {
  display: inline-flex;
  padding: 1px 6px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 500;
  background-color: var(--bg-tertiary);
  color: var(--text-secondary);
}

.spin {
  animation: spin 1s linear infinite;
}
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@media (max-width: 1024px) {
  .proxy-nodes-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 640px) {
  .proxy-nodes-grid {
    grid-template-columns: 1fr;
  }
}
</style>
