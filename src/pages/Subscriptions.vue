<script setup lang="ts">
import { ref } from "vue";
import { Plus, RefreshCw, Trash2, ExternalLink, FileText, Clock, Check } from "lucide-vue-next";
import BasePage from "@/components/BasePage.vue";
import ConfirmDialog from "@/components/ConfirmDialog.vue";
import { useToast } from "@/utils/toast";

const { show } = useToast();

interface Subscription {
  name: string;
  url: string;
  nodeCount: number;
  lastUpdate: string;
  active: boolean;
}

const subscriptions = ref<Subscription[]>([
  { name: "proxypool", url: "raw.githubusercontent.com", nodeCount: 45, lastUpdate: "2026-07-13 14:25", active: true },
  { name: "airport-sub", url: "example.com/sub", nodeCount: 32, lastUpdate: "2026-07-12 10:00", active: false },
]);

const showAddDialog = ref(false);
const showDeleteDialog = ref(false);
const deleteTarget = ref<string | null>(null);
const newName = ref("");
const newUrl = ref("");
const updating = ref<string | null>(null);

function addSubscription() {
  if (!newName.value.trim() || !newUrl.value.trim()) {
    show("请填写名称和 URL", "error");
    return;
  }
  subscriptions.value.push({
    name: newName.value.trim(),
    url: newUrl.value.trim(),
    nodeCount: 0,
    lastUpdate: new Date().toLocaleString("zh-CN", { hour12: false }),
    active: false,
  });
  show(`已添加订阅: ${newName.value}`, "success");
  showAddDialog.value = false;
  newName.value = "";
  newUrl.value = "";
}

function updateSubscription(name: string) {
  updating.value = name;
  setTimeout(() => {
    const sub = subscriptions.value.find(s => s.name === name);
    if (sub) {
      sub.lastUpdate = new Date().toLocaleString("zh-CN", { hour12: false });
      sub.nodeCount = Math.floor(Math.random() * 50) + 10;
    }
    updating.value = null;
    show(`已更新订阅: ${name}`, "success");
  }, 1500);
}

function confirmDelete(name: string) {
  deleteTarget.value = name;
  showDeleteDialog.value = true;
}

function doDelete() {
  if (deleteTarget.value) {
    subscriptions.value = subscriptions.value.filter(s => s.name !== deleteTarget.value);
    show(`已删除订阅: ${deleteTarget.value}`, "success");
  }
  showDeleteDialog.value = false;
  deleteTarget.value = null;
}
</script>

<template>
  <BasePage title="订阅">
    <template #actions>
      <button class="btn-primary text-xs" @click="showAddDialog = true">
        <Plus :size="14" />
        添加订阅
      </button>
    </template>

    <div class="space-y-3">
      <div
        v-for="sub in subscriptions"
        :key="sub.name"
        class="sub-card"
        :class="{ 'sub-card-active': sub.active }"
      >
        <div class="sub-icon">
          <FileText :size="20" :style="{ color: sub.active ? 'var(--accent)' : 'var(--text-secondary)' }" />
        </div>
        <div class="sub-info">
          <div class="sub-name">{{ sub.name }}</div>
          <div class="sub-meta">
            <span class="sub-url">来自: {{ sub.url }}</span>
          </div>
          <div class="sub-meta">
            <Clock :size="11" />
            <span>更新时间: {{ sub.lastUpdate }}</span>
          </div>
        </div>
        <div class="sub-actions">
          <span class="node-count">{{ sub.nodeCount }} 节点</span>
          <button class="btn-ghost text-xs" :disabled="updating === sub.name" @click="updateSubscription(sub.name)">
            <RefreshCw :size="12" :class="{ spin: updating === sub.name }" />
            {{ updating === sub.name ? '更新中...' : '订阅' }}
          </button>
          <button class="btn-ghost p-1" :style="{ color: 'var(--red)' }" @click="confirmDelete(sub.name)">
            <Trash2 :size="14" />
          </button>
        </div>
      </div>
    </div>

    <ConfirmDialog
      :show="showDeleteDialog"
      title="删除订阅"
      :message="`确定要删除订阅「${deleteTarget}」吗？`"
      confirm-text="删除"
      type="danger"
      @confirm="doDelete"
      @cancel="showDeleteDialog = false"
    />

    <Teleport to="body">
      <Transition name="page">
        <div v-if="showAddDialog" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showAddDialog = false">
          <div class="card w-full max-w-md mx-4 space-y-4" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
            <h3 class="text-base font-medium">添加订阅</h3>
            <div class="space-y-3">
              <div>
                <label class="text-xs font-medium mb-1 block" :style="{ color: 'var(--text-secondary)' }">名称</label>
                <input v-model="newName" placeholder="订阅名称" class="w-full rounded-lg px-3 py-2 text-sm outline-none border" :style="{ backgroundColor: 'var(--bg-tertiary)', color: 'var(--text-primary)', borderColor: 'var(--border)' }" />
              </div>
              <div>
                <label class="text-xs font-medium mb-1 block" :style="{ color: 'var(--text-secondary)' }">订阅 URL</label>
                <input v-model="newUrl" placeholder="https://example.com/sub" class="w-full rounded-lg px-3 py-2 text-sm outline-none border" :style="{ backgroundColor: 'var(--bg-tertiary)', color: 'var(--text-primary)', borderColor: 'var(--border)' }" />
              </div>
            </div>
            <div class="flex justify-end gap-2">
              <button class="btn-ghost text-xs" @click="showAddDialog = false">取消</button>
              <button class="btn-primary text-xs" @click="addSubscription">添加</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </BasePage>
</template>

<style scoped>
.sub-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  border-radius: 12px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
  transition: border-color 150ms ease;
}
.sub-card:hover { border-color: var(--accent); }
.sub-card-active { border-color: var(--accent); }

.sub-icon {
  width: 44px;
  height: 44px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-tertiary);
  flex-shrink: 0;
}

.sub-info {
  flex: 1;
  min-width: 0;
}

.sub-name {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 4px;
}

.sub-meta {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 2px;
}

.sub-url {
  font-family: "SF Mono", "Fira Code", monospace;
}

.sub-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.node-count {
  font-size: 12px;
  color: var(--text-secondary);
  padding: 2px 8px;
  border-radius: 4px;
  background-color: var(--bg-tertiary);
}
</style>
