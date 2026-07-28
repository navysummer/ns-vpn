<script setup lang="ts">
import { ref, computed } from "vue";
import { X, Search, ChevronRight, List, LayoutGrid } from "lucide-vue-next";
import { formatBytes } from "@/utils/format";
import BasePage from "@/components/BasePage.vue";

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
  sourceIP: string;
  destinationIP: string;
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
    sourceIP: `192.168.1.${Math.floor(Math.random() * 255)}`,
    destinationIP: `${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}`,
  }))
);

const searchQuery = ref("");
const filterNetwork = ref<"all" | "tcp" | "udp">("all");
const selectedConnection = ref<Connection | null>(null);
const viewMode = ref<"table" | "list">("table");

const filteredConnections = computed(() => {
  return connections.value.filter((c) => {
    if (searchQuery.value && !c.host.includes(searchQuery.value)) return false;
    if (filterNetwork.value !== "all" && c.network !== filterNetwork.value) return false;
    return true;
  });
});

const totalUpload = computed(() => connections.value.reduce((sum, c) => sum + c.upload, 0));
const totalDownload = computed(() => connections.value.reduce((sum, c) => sum + c.download, 0));

function closeConnection(id: string) {
  connections.value = connections.value.filter((c) => c.id !== id);
  if (selectedConnection.value?.id === id) {
    selectedConnection.value = null;
  }
}

function closeAll() {
  connections.value = [];
  selectedConnection.value = null;
}

function selectConnection(conn: Connection) {
  selectedConnection.value = selectedConnection.value?.id === conn.id ? null : conn;
}

function ruleColor(rule: string): string {
  switch (rule) {
    case "Direct": return "var(--green)";
    case "Reject": return "var(--red)";
    case "Media": return "var(--orange)";
    default: return "var(--accent)";
  }
}

function formatAlive(seconds: number): string {
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m`;
  return `${Math.floor(seconds / 3600)}h`;
}
</script>

<template>
  <BasePage title="连接">
    <template #actions>
      <div class="flex items-center gap-3">
        <span class="text-xs" :style="{ color: 'var(--text-secondary)' }">
          {{ connections.length }} 个连接 · ↓{{ formatBytes(totalDownload) }} ↑{{ formatBytes(totalUpload) }}
        </span>
        <div class="flex gap-1 p-0.5 rounded-lg" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
          <button
            class="mode-btn"
            :class="{ 'mode-btn-active': viewMode === 'table' }"
            @click="viewMode = 'table'"
          >
            <List :size="12" />
          </button>
          <button
            class="mode-btn"
            :class="{ 'mode-btn-active': viewMode === 'list' }"
            @click="viewMode = 'list'"
          >
            <LayoutGrid :size="12" />
          </button>
        </div>
        <button class="btn-ghost text-xs" @click="closeAll">关闭全部</button>
      </div>
    </template>

    <div class="flex gap-4 flex-1 min-h-0">
      <div class="flex-1 flex flex-col min-w-0">
        <div class="flex items-center gap-3 mb-3">
          <div class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm flex-1 max-w-xs" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
            <Search :size="14" :style="{ color: 'var(--text-secondary)' }" />
            <input v-model="searchQuery" placeholder="搜索主机..." class="bg-transparent outline-none flex-1 text-sm" :style="{ color: 'var(--text-primary)' }" />
          </div>
          <div class="flex gap-1 p-0.5 rounded-lg" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
            <button v-for="opt in ([{ label: '全部', value: 'all' }, { label: 'TCP', value: 'tcp' }, { label: 'UDP', value: 'udp' }] as const)" :key="opt.value" class="tab-btn" :class="filterNetwork === opt.value ? 'tab-btn-active' : 'tab-btn-inactive'" @click="filterNetwork = opt.value">
              {{ opt.label }}
            </button>
          </div>
        </div>

        <div class="rounded-xl border flex-1 overflow-hidden flex flex-col" :style="{ borderColor: 'var(--border)' }">
          <div v-if="viewMode === 'table'" class="flex flex-col flex-1">
            <div class="grid grid-cols-8 gap-2 px-4 py-2.5 text-xs font-medium shrink-0" :style="{ backgroundColor: 'var(--bg-secondary)', color: 'var(--text-secondary)', borderBottom: '1px solid var(--border)' }">
              <div class="col-span-2">主机</div>
              <div>网络</div>
              <div>类型</div>
              <div>规则</div>
              <div>存活</div>
              <div class="text-right">上传</div>
              <div class="text-right">下载</div>
            </div>
            <div class="divide-y flex-1 overflow-y-auto" :style="{ borderColor: 'var(--border)' }">
              <div v-for="conn in filteredConnections" :key="conn.id" class="conn-row" :class="{ 'conn-row-active': selectedConnection?.id === conn.id }" @click="selectConnection(conn)">
                <div class="col-span-2 flex items-center gap-2">
                  <span class="truncate text-xs font-mono">{{ conn.host }}:{{ conn.port }}</span>
                  <button class="opacity-0 group-hover:opacity-100 transition-opacity shrink-0" :style="{ color: 'var(--red)' }" @click.stop="closeConnection(conn.id)"><X :size="10" /></button>
                </div>
                <div><span class="tag" :style="{ backgroundColor: conn.network === 'tcp' ? 'rgba(79,142,247,0.12)' : 'rgba(52,199,89,0.12)', color: conn.network === 'tcp' ? 'var(--accent)' : 'var(--green)' }">{{ conn.network.toUpperCase() }}</span></div>
                <div class="text-xs" :style="{ color: 'var(--text-secondary)' }">{{ conn.type }}</div>
                <div><span class="text-xs font-medium" :style="{ color: ruleColor(conn.rule) }">{{ conn.rule }}</span></div>
                <div class="text-xs mono" :style="{ color: 'var(--text-secondary)' }">{{ formatAlive(conn.alive) }}</div>
                <div class="text-right font-mono text-xs" :style="{ color: 'var(--orange)' }">{{ formatBytes(conn.upload) }}</div>
                <div class="text-right font-mono text-xs" :style="{ color: 'var(--accent)' }">{{ formatBytes(conn.download) }}</div>
              </div>
              <div v-if="filteredConnections.length === 0" class="px-4 py-12 text-center text-sm" :style="{ color: 'var(--text-secondary)' }">暂无连接</div>
            </div>
          </div>

          <div v-else class="flex-1 overflow-y-auto p-2 space-y-1">
            <div v-for="conn in filteredConnections" :key="conn.id" class="conn-list-item" :class="{ 'conn-list-item-active': selectedConnection?.id === conn.id }" @click="selectConnection(conn)">
              <div class="flex items-center justify-between">
                <span class="text-xs font-mono truncate">{{ conn.host }}:{{ conn.port }}</span>
                <div class="flex items-center gap-2">
                  <span class="tag" :style="{ backgroundColor: conn.network === 'tcp' ? 'rgba(79,142,247,0.12)' : 'rgba(52,199,89,0.12)', color: conn.network === 'tcp' ? 'var(--accent)' : 'var(--green)' }">{{ conn.network.toUpperCase() }}</span>
                  <span class="text-xs" :style="{ color: ruleColor(conn.rule) }">{{ conn.rule }}</span>
                </div>
              </div>
              <div class="flex items-center justify-between mt-1">
                <span class="text-xs" :style="{ color: 'var(--text-secondary)' }">{{ conn.type }} · {{ formatAlive(conn.alive) }}</span>
                <div class="flex items-center gap-2 text-xs mono">
                  <span :style="{ color: 'var(--orange)' }">↑{{ formatBytes(conn.upload) }}</span>
                  <span :style="{ color: 'var(--accent)' }">↓{{ formatBytes(conn.download) }}</span>
                </div>
              </div>
            </div>
            <div v-if="filteredConnections.length === 0" class="py-12 text-center text-sm" :style="{ color: 'var(--text-secondary)' }">暂无连接</div>
          </div>
        </div>
      </div>

      <Transition name="page">
        <div v-if="selectedConnection" class="detail-panel">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-sm font-medium">连接详情</h3>
            <button class="p-1 rounded hover:opacity-80" :style="{ color: 'var(--text-secondary)' }" @click="selectedConnection = null"><X :size="14" /></button>
          </div>
          <div class="space-y-2">
            <div class="detail-row"><span class="detail-label">主机</span><span class="detail-value mono">{{ selectedConnection.host }}:{{ selectedConnection.port }}</span></div>
            <div class="detail-row"><span class="detail-label">网络</span><span class="tag" :style="{ backgroundColor: selectedConnection.network === 'tcp' ? 'rgba(79,142,247,0.12)' : 'rgba(52,199,89,0.12)', color: selectedConnection.network === 'tcp' ? 'var(--accent)' : 'var(--green)' }">{{ selectedConnection.network.toUpperCase() }}</span></div>
            <div class="detail-row"><span class="detail-label">类型</span><span class="detail-value">{{ selectedConnection.type }}</span></div>
            <div class="detail-row"><span class="detail-label">规则</span><span class="detail-value" :style="{ color: ruleColor(selectedConnection.rule) }">{{ selectedConnection.rule }}</span></div>
            <div class="detail-row"><span class="detail-label">链路</span><div class="flex items-center gap-1"><span v-for="(chain, i) in selectedConnection.chains" :key="i" class="text-xs"><span :style="{ color: 'var(--accent)' }">{{ chain }}</span><ChevronRight v-if="i < selectedConnection.chains.length - 1" :size="10" class="inline" :style="{ color: 'var(--text-secondary)' }" /></span></div></div>
            <div class="detail-row"><span class="detail-label">来源</span><span class="detail-value mono text-xs">{{ selectedConnection.sourceIP }}</span></div>
            <div class="detail-row"><span class="detail-label">目标</span><span class="detail-value mono text-xs">{{ selectedConnection.destinationIP }}</span></div>
            <div class="detail-row"><span class="detail-label">上传</span><span class="detail-value mono" :style="{ color: 'var(--orange)' }">{{ formatBytes(selectedConnection.upload) }}</span></div>
            <div class="detail-row"><span class="detail-label">下载</span><span class="detail-value mono" :style="{ color: 'var(--accent)' }">{{ formatBytes(selectedConnection.download) }}</span></div>
            <div class="detail-row"><span class="detail-label">存活</span><span class="detail-value mono">{{ formatAlive(selectedConnection.alive) }}</span></div>
          </div>
          <button class="btn-ghost text-xs w-full mt-4 justify-center" :style="{ color: 'var(--red)' }" @click="closeConnection(selectedConnection.id)"><X :size="12" />关闭此连接</button>
        </div>
      </Transition>
    </div>
  </BasePage>
</template>

<style scoped>
.mode-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 24px;
  border-radius: 4px;
  border: none;
  cursor: pointer;
  transition: all 150ms ease;
  background: transparent;
  color: var(--text-secondary);
}
.mode-btn:hover { color: var(--text-primary); }
.mode-btn-active { background-color: var(--accent); color: #fff; }

.conn-row {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr 1fr 0.5fr 0.5fr 0.5fr;
  gap: 8px;
  padding: 8px 16px;
  font-size: 13px;
  align-items: center;
  transition: background-color 100ms ease;
  cursor: pointer;
}
.conn-row:hover { background-color: var(--bg-hover); }
.conn-row-active { background-color: rgba(79,142,247,0.08) !important; }

.conn-list-item {
  padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: background-color 100ms ease;
}
.conn-list-item:hover { background-color: var(--bg-hover); }
.conn-list-item-active { background-color: rgba(79,142,247,0.08) !important; }

.detail-panel {
  width: 280px;
  min-width: 280px;
  padding: 16px;
  border-radius: 12px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
  height: fit-content;
  max-height: calc(100vh - 100px);
  overflow-y: auto;
}

.detail-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 0;
  border-bottom: 1px solid var(--border);
}
.detail-row:last-child { border-bottom: none; }

.detail-label {
  font-size: 11px;
  color: var(--text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.detail-value {
  font-size: 12px;
  color: var(--text-primary);
}
</style>