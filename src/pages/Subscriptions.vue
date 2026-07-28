<script setup lang="ts">
import { ref } from "vue";
import { RefreshCw, Trash2, Clipboard, ArrowDownToLine, Plus, GripVertical, Settings, FileCode } from "lucide-vue-next";
import ConfirmDialog from "@/components/ConfirmDialog.vue";
import { useToast } from "@/utils/toast";

const { show } = useToast();

interface Subscription {
  name: string;
  url: string;
  lastUpdate: string;
  timeAgo: string;
}

const subscriptions = ref<Subscription[]>([
  { name: "awesome-vpn", url: "raw.githubusercontent.com", lastUpdate: "2026-07-27", timeAgo: "1 天前" },
  { name: "cn-news", url: "raw.githubusercontent.com", lastUpdate: "2026-07-27", timeAgo: "1 天前" },
  { name: "daily_free_vpn", url: "sub.466688.xyz", lastUpdate: "2026-06-28", timeAgo: "1 个月前" },
  { name: "BestClash", url: "cdn.jsdelivr.net", lastUpdate: "2026-07-27", timeAgo: "1 天前" },
  { name: "chromeego-sub", url: "chromeego-sub.netlify.app", lastUpdate: "2026-07-27", timeAgo: "1 天前" },
  { name: "proxypool", url: "raw.githubusercontent.com", lastUpdate: "2026-07-14", timeAgo: "15 天前" },
  { name: "Proxypool2", url: "raw.githubusercontent.com", lastUpdate: "2026-07-29", timeAgo: "5 分钟前" },
  { name: "ermao", url: "www.ermao.net", lastUpdate: "2026-07-18", timeAgo: "10 天前" },
  { name: "FREEE-VPN", url: "raw.githubusercontent.com", lastUpdate: "2026-07-27", timeAgo: "1 天前" },
  { name: "free18", url: "ghfast.top", lastUpdate: "2026-07-27", timeAgo: "2 天前" },
]);

const subUrl = ref("");
const showDeleteDialog = ref(false);
const deleteTarget = ref<string | null>(null);
const updating = ref<string | null>(null);
const selectedSub = ref<string | null>("Proxypool2");

function refreshSubscription(name: string) {
  updating.value = name;
  setTimeout(() => {
    const sub = subscriptions.value.find(s => s.name === name);
    if (sub) {
      sub.lastUpdate = new Date().toISOString().split("T")[0];
      sub.timeAgo = "刚刚";
    }
    updating.value = null;
    show(`已更新订阅: ${name}`, "success");
  }, 1500);
}

function refreshAll() {
  subscriptions.value.forEach(sub => refreshSubscription(sub.name));
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

function importSub() {
  if (!subUrl.value.trim()) {
    show("请输入订阅链接", "error");
    return;
  }
  show("导入成功", "success");
  subUrl.value = "";
}

function createNew() {
  show("新建订阅", "info");
}
</script>

<template>
  <div class="sub-page">
    <div class="sub-header">
      <h1 class="sub-title">订阅</h1>
      <div class="sub-header-actions">
        <button class="header-icon-btn" title="刷新全部" @click="refreshAll">
          <RefreshCw :size="18" />
        </button>
        <button class="header-icon-btn" title="列表视图">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/></svg>
        </button>
        <button class="header-icon-btn" title="剪贴板">
          <Clipboard :size="18" />
        </button>
      </div>
    </div>

    <div class="sub-input-bar">
      <input v-model="subUrl" placeholder="订阅文件链接" class="sub-url-input" />
      <button class="sub-import-btn" @click="importSub">导入</button>
      <button class="sub-create-btn" @click="createNew">新建</button>
    </div>

    <div class="sub-grid">
      <div
        v-for="sub in subscriptions"
        :key="sub.name"
        class="sub-card"
        :class="{ 'sub-card-active': selectedSub === sub.name }"
        @click="selectedSub = sub.name"
      >
        <div class="sub-drag">
          <GripVertical :size="14" />
        </div>
        <div class="sub-content">
          <div class="sub-top-row">
            <span class="sub-name">{{ sub.name }}</span>
            <button class="sub-refresh-btn" :disabled="updating === sub.name" @click.stop="refreshSubscription(sub.name)">
              <RefreshCw :size="12" :class="{ spin: updating === sub.name }" />
            </button>
          </div>
          <div class="sub-bottom-row">
            <span class="sub-url">{{ sub.url }}</span>
            <span class="sub-time">{{ sub.timeAgo }}</span>
          </div>
          <div class="sub-date">{{ sub.lastUpdate }}</div>
        </div>
      </div>
    </div>

    <div class="sub-footer">
      <div class="sub-footer-card">
        <span class="footer-label">全局扩展覆写配置</span>
        <span class="footer-badge footer-badge-merge">Merge</span>
      </div>
      <div class="sub-footer-card">
        <span class="footer-label">全局扩展脚本</span>
        <span class="footer-badge footer-badge-script">Script</span>
        <FileCode :size="14" class="footer-icon" />
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
  </div>
</template>

<style scoped>
.sub-page {
  max-width: 100%;
}

.sub-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}

.sub-title {
  font-size: 22px;
  font-weight: 700;
  margin: 0;
}

.sub-header-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.header-icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 8px;
  border: none;
  cursor: pointer;
  transition: all 150ms ease;
  background: transparent;
  color: var(--text-secondary);
}
.header-icon-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.sub-input-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 20px;
  padding: 4px 4px 4px 12px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
}

.sub-url-input {
  flex: 1;
  border: none;
  background: transparent;
  font-size: 13px;
  color: var(--text-primary);
  outline: none;
}
.sub-url-input::placeholder {
  color: var(--text-secondary);
}

.sub-import-btn {
  padding: 6px 16px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: transparent;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}
.sub-import-btn:hover {
  border-color: var(--accent);
  color: var(--text-primary);
}

.sub-create-btn {
  padding: 6px 16px;
  border-radius: 6px;
  border: none;
  background-color: var(--accent);
  font-size: 13px;
  font-weight: 500;
  color: #fff;
  cursor: pointer;
  transition: all 150ms ease;
}
.sub-create-btn:hover {
  opacity: 0.9;
}

.sub-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
  margin-bottom: 20px;
}

.sub-card {
  display: flex;
  align-items: stretch;
  padding: 12px;
  border-radius: 10px;
  border: 1px solid var(--border);
  border-left: 3px solid transparent;
  background-color: var(--card-bg);
  cursor: pointer;
  transition: all 150ms ease;
}
.sub-card:hover {
  border-color: var(--accent);
  border-left-color: var(--accent);
}

.sub-card-active {
  border-color: var(--accent) !important;
  border-left-color: var(--accent) !important;
  background-color: rgba(79,142,247,0.05);
}

.sub-drag {
  display: flex;
  align-items: flex-start;
  padding-top: 2px;
  color: var(--text-secondary);
  opacity: 0.5;
  cursor: grab;
}

.sub-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.sub-top-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.sub-name {
  font-size: 13px;
  font-weight: 600;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sub-refresh-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: 4px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}
.sub-refresh-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}
.sub-refresh-btn:disabled {
  opacity: 0.5;
}

.sub-bottom-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.sub-url {
  font-size: 11px;
  color: var(--text-secondary);
  font-family: "SF Mono", "Fira Code", monospace;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sub-time {
  font-size: 11px;
  color: var(--text-secondary);
  white-space: nowrap;
  margin-left: 8px;
}

.sub-date {
  font-size: 11px;
  color: var(--text-secondary);
  text-align: right;
}

.sub-footer {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.sub-footer-card {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 16px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
}

.footer-label {
  font-size: 13px;
  font-weight: 500;
}

.footer-badge {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
}

.footer-badge-merge {
  background-color: rgba(52,199,89,0.12);
  color: var(--green);
}

.footer-badge-script {
  background-color: rgba(79,142,247,0.12);
  color: var(--accent);
}

.footer-icon {
  color: var(--text-secondary);
  margin-left: auto;
}

.spin {
  animation: spin 1s linear infinite;
}
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@media (max-width: 1200px) {
  .sub-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}

@media (max-width: 900px) {
  .sub-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 600px) {
  .sub-grid {
    grid-template-columns: 1fr;
  }
  .sub-footer {
    grid-template-columns: 1fr;
  }
}
</style>
