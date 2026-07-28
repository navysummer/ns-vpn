<script setup lang="ts">
import { ref } from "vue";
import { RefreshCw, Check, Zap } from "lucide-vue-next";
import { delayQuality, formatDelay } from "@/utils/format";

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
    all: [
      { name: "HK-01", type: "Shadowsocks", delay: 32, now: false },
      { name: "HK-02", type: "Shadowsocks", delay: 0, now: false },
      { name: "JP-01", type: "VMess", delay: 78, now: false },
      { name: "US-01", type: "Trojan", delay: 210, now: false },
      { name: "SG-01", type: "Shadowsocks", delay: 55, now: false },
    ],
  },
]);

const selectedGroup = ref(groups.value[0].name);
const testingAll = ref(false);
const testingGroup = ref<string | null>(null);
const expandedInfo = ref<string | null>(null);

function selectGroup(name: string) {
  selectedGroup.value = name;
}

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
    group.all.forEach((n) => {
      n.delay = 0;
    });
    group.all.forEach((n) => {
      setTimeout(() => {
        n.delay = Math.floor(Math.random() * 300) + 20;
      }, Math.random() * 1500);
    });
  }
  setTimeout(() => { testingGroup.value = null; }, 2000);
}

function testAllDelay() {
  testingAll.value = true;
  groups.value.forEach((g) => testGroupDelay(g.name));
  setTimeout(() => { testingAll.value = false; }, 2500);
}

function toggleGroupInfo(name: string) {
  expandedInfo.value = expandedInfo.value === name ? null : name;
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

function nodeTypeColor(type: string): string {
  if (type === "Direct") return "var(--green)";
  if (type === "Reject") return "var(--red)";
  if (type === "URLTest") return "var(--text-secondary)";
  return "var(--text-secondary)";
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-semibold">代理</h1>
      <button
        class="btn-ghost text-sm"
        :disabled="testingAll"
        @click="testAllDelay"
      >
        <Zap :size="14" :class="{ spin: testingAll }" />
        {{ testingAll ? "测试中..." : "延迟测试" }}
      </button>
    </div>

    <div class="flex gap-1 p-1 rounded-lg overflow-x-auto" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
      <button
        v-for="group in groups"
        :key="group.name"
        class="tab-btn"
        :class="selectedGroup === group.name ? 'tab-btn-active' : 'tab-btn-inactive'"
        @click="selectGroup(group.name)"
      >
        {{ group.name }}
        <span
          class="type-badge"
          :style="{ backgroundColor: typeBadgeBg(group.type), color: typeBadgeColor(group.type) }"
        >
          {{ group.type }}
        </span>
        <span class="ml-1 opacity-60">({{ group.all.length }})</span>
      </button>
    </div>

    <div
      v-for="group in groups"
      v-show="group.name === selectedGroup"
      :key="group.name"
      class="space-y-2"
    >
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <span class="text-sm font-medium">{{ group.name }}</span>
          <span
            class="type-badge"
            :style="{ backgroundColor: typeBadgeBg(group.type), color: typeBadgeColor(group.type) }"
          >
            {{ group.type }}
          </span>
          <span class="text-xs" :style="{ color: 'var(--text-secondary)' }">
            当前: {{ group.now }}
          </span>
        </div>
        <button
          class="btn-ghost text-xs"
          :disabled="testingGroup === group.name"
          @click="testGroupDelay(group.name)"
        >
          <Zap :size="12" :class="{ spin: testingGroup === group.name }" />
          {{ testingGroup === group.name ? "测试中..." : "测试" }}
        </button>
      </div>

      <div class="space-y-0.5">
        <div
          v-for="node in group.all"
          :key="node.name"
          class="proxy-node"
          :class="{ 'proxy-node-active': node.now }"
          @click="selectNode(group.name, node.name)"
        >
          <div class="w-5 shrink-0">
            <Check v-if="node.now" :size="14" />
          </div>

          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
              <span class="text-sm font-medium truncate">{{ node.name }}</span>
              <span
                class="node-type-tag"
                :style="{ color: nodeTypeColor(node.type) }"
              >
                {{ node.type }}
              </span>
            </div>
          </div>

          <div
            class="delay-badge"
            :class="{
              'delay-good': delayQuality(node.delay) === 'good',
              'delay-medium': delayQuality(node.delay) === 'medium',
              'delay-bad': delayQuality(node.delay) === 'bad',
              'delay-none': delayQuality(node.delay) === 'none',
              'delay-active': node.now,
            }"
          >
            <Zap v-if="node.delay > 0" :size="10" />
            {{ formatDelay(node.delay) }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.proxy-node {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 16px;
  border-radius: 8px;
  cursor: pointer;
  transition: background-color 150ms ease, color 150ms ease;
  color: var(--text-primary);
  background: transparent;
}
.proxy-node:hover {
  background-color: var(--bg-hover);
}
.proxy-node-active {
  background-color: var(--accent) !important;
  color: #fff !important;
}

.type-badge {
  display: inline-flex;
  align-items: center;
  padding: 1px 6px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
  line-height: 1.4;
}

.node-type-tag {
  font-size: 11px;
  opacity: 0.7;
}
.proxy-node-active .node-type-tag {
  color: rgba(255,255,255,0.6) !important;
}

.delay-badge {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-family: "SF Mono", "Fira Code", "Cascadia Code", monospace;
  background-color: var(--bg-tertiary);
  transition: background-color 150ms ease;
  min-width: 50px;
  justify-content: center;
}
.delay-good { color: var(--green); }
.delay-medium { color: var(--orange); }
.delay-bad { color: var(--red); }
.delay-none { color: var(--text-secondary); }
.delay-active { background-color: rgba(255,255,255,0.15); }
</style>