<script setup lang="ts">
import { ref, computed } from "vue";
import { X, ArrowUp, ArrowDown, Search } from "lucide-vue-next";
import { formatBytes, formatTime } from "@/utils/format";

interface Connection {
  id: string;
  host: string;
  port: number;
  network: "tcp" | "udp";
  type: string;
  rule: string;
  chains: string[];
  upload: number;
  download: number;
  start: number;
  alive: number;
}

const connections = ref<Connection[]>(
  Array.from({ length: 30 }, (_, i) => ({
    id: `conn-${i}`,
    host: `192.168.1.${Math.floor(Math.random() * 255)}`,
    port: Math.floor(Math.random() * 65535) + 1024,
    network: Math.random() > 0.2 ? "tcp" : "udp",
    type: ["HTTP", "HTTPS", "QUIC", "WebSocket"][Math.floor(Math.random() * 4)],
    rule: ["Proxy", "Direct", "Reject", "Media"][Math.floor(Math.random() * 4)],
    chains: ["Proxy", "Auto"],
    upload: Math.floor(Math.random() * 10000),
    download: Math.floor(Math.random() * 1000000),
    start: Date.now() - Math.floor(Math.random() * 600000),
    alive: Math.floor(Math.random() * 300),
  }))
);

const searchQuery = ref("");
const filterNetwork = ref<"all" | "tcp" | "udp">("all");

const filteredConnections = computed(() => {
  return connections.value.filter((c) => {
    if (searchQuery.value && !c.host.includes(searchQuery.value)) return false;
    if (filterNetwork.value !== "all" && c.network !== filterNetwork.value) return false;
    return true;
  });
});

function closeConnection(id: string) {
  connections.value = connections.value.filter((c) => c.id !== id);
}

function closeAll() {
  connections.value = [];
}
</script>

<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-semibold">连接</h1>
      <div class="flex items-center gap-2">
        <span class="text-sm" :style="{ color: 'var(--text-secondary)' }">
          {{ connections.length }} 个活跃连接
        </span>
        <button class="btn-ghost text-xs" @click="closeAll">关闭全部</button>
      </div>
    </div>

    <!-- Filters -->
    <div class="flex items-center gap-3">
      <div
        class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm flex-1 max-w-xs"
        :style="{ backgroundColor: 'var(--bg-tertiary)' }"
      >
        <Search :size="14" :style="{ color: 'var(--text-secondary)' }" />
        <input
          v-model="searchQuery"
          placeholder="搜索主机..."
          class="bg-transparent outline-none flex-1 text-sm"
          :style="{ color: 'var(--text-primary)' }"
        />
      </div>
      <div class="flex gap-1 p-0.5 rounded-lg" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
        <button
          v-for="opt in ([{ label: '全部', value: 'all' }, { label: 'TCP', value: 'tcp' }, { label: 'UDP', value: 'udp' }] as const)"
          :key="opt.value"
          class="px-3 py-1 rounded-md text-xs font-medium transition-colors"
          :style="{
            backgroundColor: filterNetwork === opt.value ? 'var(--accent)' : 'transparent',
            color: filterNetwork === opt.value ? '#fff' : 'var(--text-secondary)',
          }"
          @click="filterNetwork = opt.value"
        >
          {{ opt.label }}
        </button>
      </div>
    </div>

    <!-- Table -->
    <div class="rounded-xl overflow-hidden border" :style="{ borderColor: 'var(--border)' }">
      <!-- Table header -->
      <div
        class="grid grid-cols-7 gap-2 px-4 py-2.5 text-xs font-medium"
        :style="{ backgroundColor: 'var(--bg-secondary)', color: 'var(--text-secondary)', borderBottom: '1px solid var(--border)' }"
      >
        <div class="col-span-2">主机</div>
        <div>网络</div>
        <div>类型</div>
        <div>规则</div>
        <div class="text-right">上传</div>
        <div class="text-right">下载</div>
      </div>

      <!-- Table body -->
      <div
        class="divide-y max-h-[calc(100vh-320px)] overflow-y-auto"
        :style="{ borderColor: 'var(--border)' }"
      >
        <div
          v-for="conn in filteredConnections"
          :key="conn.id"
          class="grid grid-cols-7 gap-2 px-4 py-2.5 text-sm items-center transition-colors duration-150 group"
          :style="{
            borderColor: 'var(--border)',
          }"
          @mouseenter="(e) => (e.currentTarget as HTMLElement).style.backgroundColor = 'var(--bg-hover)'"
          @mouseleave="(e) => (e.currentTarget as HTMLElement).style.backgroundColor = 'transparent'"
        >
          <div class="col-span-2 flex items-center gap-2">
            <span class="truncate">{{ conn.host }}:{{ conn.port }}</span>
            <button
              class="opacity-0 group-hover:opacity-100 transition-opacity"
              :style="{ color: 'var(--red)' }"
              @click="closeConnection(conn.id)"
            >
              <X :size="12" />
            </button>
          </div>
          <div>
            <span
              class="text-xs px-1.5 py-0.5 rounded"
              :style="{
                backgroundColor: conn.network === 'tcp' ? 'rgba(79,142,247,0.15)' : 'rgba(52,199,89,0.15)',
                color: conn.network === 'tcp' ? 'var(--accent)' : 'var(--green)',
              }"
            >
              {{ conn.network.toUpperCase() }}
            </span>
          </div>
          <div :style="{ color: 'var(--text-secondary)' }">{{ conn.type }}</div>
          <div>
            <span
              class="text-xs"
              :style="{
                color: conn.rule === 'Direct' ? 'var(--green)' : conn.rule === 'Reject' ? 'var(--red)' : 'var(--accent)',
              }"
            >
              {{ conn.rule }}
            </span>
          </div>
          <div class="text-right font-mono text-xs" :style="{ color: 'var(--orange)' }">
            {{ formatBytes(conn.upload) }}
          </div>
          <div class="text-right font-mono text-xs" :style="{ color: 'var(--accent)' }">
            {{ formatBytes(conn.download) }}
          </div>
        </div>

        <!-- Empty state -->
        <div
          v-if="filteredConnections.length === 0"
          class="px-4 py-12 text-center text-sm"
          :style="{ color: 'var(--text-secondary)' }"
        >
          暂无连接
        </div>
      </div>
    </div>
  </div>
</template>