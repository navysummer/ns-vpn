<script setup lang="ts">
import { ref, computed } from "vue";
import { X, Search, ChevronRight, List, LayoutGrid, Activity, Settings2, PieChart } from "lucide-vue-next";
import { formatBytes, formatRelativeTime, formatAlive } from "@/utils/format";
import BasePage from "@/components/BasePage.vue";
import EmptyState from "@/components/EmptyState.vue";

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

interface ColumnDef {
  key: string;
  label: string;
  visible: boolean;
  width: string;
}

const connections = ref<Connection[]>(
  Array.from({ length: 50 }, (_, i) => ({
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
const showColumnManager = ref(false);
const showProtocolChart = ref(false);

const columns = ref<ColumnDef[]>([
  { key: "host", label: "主机", visible: true, width: "2fr" },
  { key: "network", label: "网络", visible: true, width: "1fr" },
  { key: "type", label: "类型", visible: true, width: "1fr" },
  { key: "rule", label: "规则", visible: true, width: "1fr" },
  { key: "alive", label: "存活", visible: true, width: "0.5fr" },
  { key: "upload", label: "上传", visible: true, width: "0.5fr" },
  { key: "download", label: "下载", visible: true, width: "0.5fr" },
]);

const visibleColumns = computed(() => columns.value.filter(c => c.visible));
const gridTemplate = computed(() => visibleColumns.value.map(c => c.width).join(" "));

const filteredConnections = computed(() => {
  return connections.value.filter((c) => {
    if (searchQuery.value && !c.host.includes(searchQuery.value)) return false;
    if (filterNetwork.value !== "all" && c.network !== filterNetwork.value) return false;
    return true;
  });
});

const totalUpload = computed(() => connections.value.reduce((sum, c) => sum + c.upload, 0));
const totalDownload = computed(() => connections.value.reduce((sum, c) => sum + c.download, 0));

// Protocol breakdown
const protocolStats = computed(() => {
  const stats: Record<string, { count: number; upload: number; download: number }> = {};
  connections.value.forEach(c => {
    if (!stats[c.type]) stats[c.type] = { count: 0, upload: 0, download: 0 };
    stats[c.type].count++;
    stats[c.type].upload += c.upload;
    stats[c.type].download += c.download;
  });
  return Object.entries(stats)
    .map(([type, data]) => ({ type, ...data }))
    .sort((a, b) => b.count - a.count);
});

const protocolColors: Record<string, string> = {
  HTTP: "#4f8ef7",
  HTTPS: "#34c759",
  QUIC: "#bf5af2",
  WebSocket: "#ff9f0a",
};

function closeConnection(id: string) {
  connections.value = connections.value.filter((c) => c.id !== id);
  if (selectedConnection.value?.id === id) selectedConnection.value = null;
}

function closeAll() {
  connections.value = [];
  selectedConnection.value = null;
}

function selectConnection(conn: Connection) {
  selectedConnection.value = selectedConnection.value?.id === conn.id ? null : conn;
}

function toggleColumn(key: string) {
  const col = columns.value.find(c => c.key === key);
  if (col) col.visible = !col.visible;
}

function ruleColor(rule: string): string {
  switch (rule) {
    case "Direct": return "var(--green)";
    case "Reject": return "var(--red)";
    case "Media": return "var(--orange)";
    default: return "var(--accent)";
  }
}
</script>

<template>
  <BasePage title="连接">
    <template #actions>
      <div class="flex items-center gap-3">
        <span class="text-xs" :style="{ color: 'var(--text-secondary)' }">
          {{ connections.length }} 连接 · ↓{{ formatBytes(totalDownload) }} ↑{{ formatBytes(totalUpload) }}
        </span>
        <div class="flex gap-1 p-0.5 rounded-lg" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
          <button class="view-btn" :class="{ active: viewMode === 'table' }" @click="viewMode = 'table'"><List :size="12" /></button>
          <button class="view-btn" :class="{ active: viewMode === 'list' }" @click="viewMode = 'list'"><LayoutGrid :size="12" /></button>
        </div>
        <button class="btn-ghost text-xs" @click="showProtocolChart = !showProtocolChart">
          <PieChart :size="12" />
          协议
        </button>
        <button class="btn-ghost text-xs" @click="showColumnManager = !showColumnManager">
          <Settings2 :size="12" />
          列管理
        </button>
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

        <!-- Protocol Breakdown -->
        <Transition name="page">
          <div v-if="showProtocolChart" class="protocol-chart mb-3">
            <div class="text-xs font-medium mb-2" :style="{ color: 'var(--text-secondary)' }">协议分布</div>
            <div class="flex gap-3 flex-wrap">
              <div v-for="stat in protocolStats" :key="stat.type" class="protocol-item">
                <div class="flex items-center gap-2">
                  <span class="w-2 h-2 rounded-full" :style="{ backgroundColor: protocolColors[stat.type] || 'var(--text-secondary)' }"></span>
                  <span class="text-xs font-medium">{{ stat.type }}</span>
                  <span class="text-xs" :style="{ color: 'var(--text-secondary)' }">{{ stat.count }}</span>
                </div>
                <div class="flex items-center gap-2 text-xs mono">
                  <span :style="{ color: 'var(--orange)' }">↑{{ formatBytes(stat.upload) }}</span>
                  <span :style="{ color: 'var(--accent)' }">↓{{ formatBytes(stat.download) }}</span>
                </div>
              </div>
            </div>
          </div>
        </Transition>

        <!-- Column Manager -->
        <Transition name="page">
          <div v-if="showColumnManager" class="column-manager mb-3">
            <div class="text-xs font-medium mb-2" :style="{ color: 'var(--text-secondary)' }">显示列</div>
            <div class="flex flex-wrap gap-2">
              <label v-for="col in columns" :key="col.key" class="column-toggle">
                <input
                  type="checkbox"
                  :checked="col.visible"
                  class="column-checkbox"
                  @change="toggleColumn(col.key)"
                />
                <span class="text-xs">{{ col.label }}</span>
              </label>
            </div>
          </div>
        </Transition>

        <div class="rounded-xl border flex-1 overflow-hidden flex flex-col" :style="{ borderColor: 'var(--border)' }">
          <template v-if="viewMode === 'table'">
            <div
              class="conn-header"
              :style="{ gridTemplateColumns: gridTemplate }"
            >
              <div v-for="col in visibleColumns" :key="col.key" :class="{ 'text-right': col.key === 'upload' || col.key === 'download' }">
                {{ col.label }}
              </div>
            </div>
            <div class="conn-scroll">
              <div
                v-for="conn in filteredConnections"
                :key="conn.id"
                class="conn-row"
                :class="{ 'conn-row-active': selectedConnection?.id === conn.id }"
                :style="{ gridTemplateColumns: gridTemplate }"
                @click="selectConnection(conn)"
              >
                <template v-if="visibleColumns.some(c => c.key === 'host')">
                  <div class="flex items-center gap-2 min-w-0">
                    <span class="truncate text-xs font-mono">{{ conn.host }}:{{ conn.port }}</span>
                    <button class="conn-close opacity-0 group-hover:opacity-100" :style="{ color: 'var(--red)' }" @click.stop="closeConnection(conn.id)"><X :size="10" /></button>
                  </div>
                </template>
                <template v-if="visibleColumns.some(c => c.key === 'network')">
                  <div><span class="tag" :style="{ backgroundColor: conn.network === 'tcp' ? 'rgba(79,142,247,0.12)' : 'rgba(52,199,89,0.12)', color: conn.network === 'tcp' ? 'var(--accent)' : 'var(--green)' }">{{ conn.network.toUpperCase() }}</span></div>
                </template>
                <template v-if="visibleColumns.some(c => c.key === 'type')">
                  <div class="text-xs" :style="{ color: 'var(--text-secondary)' }">{{ conn.type }}</div>
                </template>
                <template v-if="visibleColumns.some(c => c.key === 'rule')">
                  <div><span class="text-xs font-medium" :style="{ color: ruleColor(conn.rule) }">{{ conn.rule }}</span></div>
                </template>
                <template v-if="visibleColumns.some(c => c.key === 'alive')">
                  <div class="text-xs mono" :style="{ color: 'var(--text-secondary)' }">{{ formatAlive(conn.alive) }}</div>
                </template>
                <template v-if="visibleColumns.some(c => c.key === 'upload')">
                  <div class="text-right font-mono text-xs" :style="{ color: 'var(--orange)' }">{{ formatBytes(conn.upload) }}</div>
                </template>
                <template v-if="visibleColumns.some(c => c.key === 'download')">
                  <div class="text-right font-mono text-xs" :style="{ color: 'var(--accent)' }">{{ formatBytes(conn.download) }}</div>
                </template>
              </div>
            </div>
          </template>

          <template v-else>
            <div class="conn-scroll p-2 space-y-1">
              <div v-for="conn in filteredConnections" :key="conn.id" class="conn-list-item" :class="{ 'conn-list-item-active': selectedConnection?.id === conn.id }" @click="selectConnection(conn)">
                <div class="flex items-center justify-between">
                  <span class="text-xs font-mono truncate">{{ conn.host }}:{{ conn.port }}</span>
                  <div class="flex items-center gap-2">
                    <span class="tag" :style="{ backgroundColor: conn.network === 'tcp' ? 'rgba(79,142,247,0.12)' : 'rgba(52,199,89,0.12)', color: conn.network === 'tcp' ? 'var(--accent)' : 'var(--green)' }">{{ conn.network.toUpperCase() }}</span>
                    <span class="text-xs" :style="{ color: ruleColor(conn.rule) }">{{ conn.rule }}</span>
                  </div>
                </div>
                <div class="flex items-center justify-between mt-1">
                  <span class="text-xs" :style="{ color: 'var(--text-secondary)' }">{{ conn.type }} · {{ formatRelativeTime(conn.start) }}</span>
                  <div class="flex items-center gap-2 text-xs mono">
                    <span :style="{ color: 'var(--orange)' }">↑{{ formatBytes(conn.upload) }}</span>
                    <span :style="{ color: 'var(--accent)' }">↓{{ formatBytes(conn.download) }}</span>
                  </div>
                </div>
              </div>
            </div>
          </template>

          <EmptyState v-if="filteredConnections.length === 0" :icon="Activity" title="暂无连接" description="当前没有活跃的网络连接" />
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
            <div class="detail-row"><span class="detail-label">创建时间</span><span class="detail-value text-xs">{{ formatRelativeTime(selectedConnection.start) }}</span></div>
          </div>
          <button class="btn-ghost text-xs w-full mt-4 justify-center" :style="{ color: 'var(--red)' }" @click="closeConnection(selectedConnection.id)"><X :size="12" />关闭此连接</button>
        </div>
      </Transition>
    </div>
  </BasePage>
</template>

<style scoped>
.view-btn {
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
.view-btn:hover { color: var(--text-primary); }
.view-btn.active { background-color: var(--accent); color: #fff; }

.conn-scroll {
  flex: 1;
  overflow-y: auto;
  max-height: calc(100vh - 280px);
}

.conn-header {
  display: grid;
  gap: 8px;
  padding: 8px 16px;
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.conn-row {
  display: grid;
  gap: 8px;
  padding: 6px 16px;
  font-size: 13px;
  align-items: center;
  transition: background-color 100ms ease;
  cursor: pointer;
}
.conn-row:hover { background-color: var(--bg-hover); }
.conn-row-active { background-color: rgba(79,142,247,0.08) !important; }

.conn-close {
  transition: opacity 150ms ease;
}

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

.column-manager {
  padding: 10px 12px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
}

.column-toggle {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 100ms ease;
}

.column-toggle:hover {
  background-color: var(--bg-hover);
}

.column-checkbox {
  width: 12px;
  height: 12px;
  accent-color: var(--accent);
}

.protocol-chart {
  padding: 10px 12px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
}

.protocol-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 6px 10px;
  border-radius: 6px;
  background-color: var(--bg-tertiary);
}
</style>
