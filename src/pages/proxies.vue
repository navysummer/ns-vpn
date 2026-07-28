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
  type: string;
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

function testDelay(groupName: string) {
  const group = groups.value.find((g) => g.name === groupName);
  if (group) {
    group.all.forEach((n) => {
      if (n.delay === 0) n.delay = Math.floor(Math.random() * 300) + 20;
    });
  }
}

function testAllDelay() {
  testingAll.value = true;
  groups.value.forEach((g) => testDelay(g.name));
  setTimeout(() => {
    testingAll.value = false;
  }, 2000);
}

const currentGroup = ref(groups.value[0]);
</script>

<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-semibold">代理</h1>
      <button
        class="btn-ghost text-sm"
        :disabled="testingAll"
        @click="testAllDelay"
      >
        <Zap :size="14" />
        {{ testingAll ? "测试中..." : "延迟测试" }}
      </button>
    </div>

    <!-- Group tabs -->
    <div
      class="flex gap-1 p-1 rounded-lg overflow-x-auto"
      :style="{ backgroundColor: 'var(--bg-tertiary)' }"
    >
      <button
        v-for="group in groups"
        :key="group.name"
        class="px-3 py-1.5 rounded-md text-sm font-medium whitespace-nowrap transition-colors duration-150"
        :style="{
          backgroundColor: selectedGroup === group.name ? 'var(--accent)' : 'transparent',
          color: selectedGroup === group.name ? '#fff' : 'var(--text-secondary)',
        }"
        @click="selectGroup(group.name)"
      >
        {{ group.name }}
        <span class="ml-1 opacity-60">({{ group.all.length }})</span>
      </button>
    </div>

    <!-- Nodes -->
    <div class="space-y-1">
      <div
        v-for="node in groups.find((g) => g.name === selectedGroup)?.all || []"
        :key="node.name"
        class="flex items-center gap-3 px-4 py-2.5 rounded-lg cursor-pointer transition-colors duration-150"
        :style="{
          backgroundColor: node.now ? 'var(--accent)' : 'transparent',
          color: node.now ? '#fff' : 'var(--text-primary)',
        }"
        @mouseenter="
          (e) => {
            if (!node.now)
              (e.currentTarget as HTMLElement).style.backgroundColor = 'var(--bg-hover)';
          }
        "
        @mouseleave="
          (e) => {
            if (!node.now)
              (e.currentTarget as HTMLElement).style.backgroundColor = 'transparent';
          }
        "
        @click="selectNode(selectedGroup, node.name)"
      >
        <!-- Check icon -->
        <div class="w-4 shrink-0">
          <Check v-if="node.now" :size="14" />
        </div>

        <!-- Name & type -->
        <div class="flex-1 min-w-0">
          <div class="text-sm font-medium truncate">{{ node.name }}</div>
          <div class="text-xs" :style="{ color: node.now ? 'rgba(255,255,255,0.6)' : 'var(--text-secondary)' }">
            {{ node.type }}
          </div>
        </div>

        <!-- Delay -->
        <div
          class="flex items-center gap-1 text-xs font-mono px-2 py-0.5 rounded"
          :style="{
            backgroundColor: node.now ? 'rgba(255,255,255,0.15)' : 'var(--bg-tertiary)',
            color: delayQuality(node.delay) === 'good'
              ? 'var(--green)'
              : delayQuality(node.delay) === 'medium'
              ? 'var(--orange)'
              : delayQuality(node.delay) === 'bad'
              ? 'var(--red)'
              : 'var(--text-secondary)',
          }"
        >
          <Zap v-if="node.delay > 0" :size="10" />
          {{ formatDelay(node.delay) }}
        </div>
      </div>
    </div>
  </div>
</template>