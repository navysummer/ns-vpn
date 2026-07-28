<script setup lang="ts">
import { ref } from "vue";
import { Plus, Download, Check, FileJson, RefreshCw, Globe, GitMerge, ScrollText, Trash2, Upload, FileUp } from "lucide-vue-next";
import { useToast } from "@/utils/toast";
import ConfirmDialog from "@/components/ConfirmDialog.vue";
import BasePage from "@/components/BasePage.vue";

const { show } = useToast();

interface Profile {
  name: string;
  type: "local" | "remote" | "merge" | "script";
  url?: string;
  active: boolean;
  updated: string;
  nodes: number;
}

const profiles = ref<Profile[]>([
  { name: "config.yaml", type: "local", active: true, updated: "2026-01-15 14:30", nodes: 45 },
  { name: "机场订阅", type: "remote", url: "https://example.com/sub", active: false, updated: "2026-01-14 10:00", nodes: 32 },
  { name: "规则合并", type: "merge", active: false, updated: "2026-01-13 09:15", nodes: 0 },
  { name: "自定义脚本", type: "script", active: false, updated: "2026-01-12 16:45", nodes: 0 },
]);

const showAddDialog = ref(false);
const showDeleteDialog = ref(false);
const deleteTarget = ref<string | null>(null);
const newProfileName = ref("");
const newProfileUrl = ref("");
const newProfileType = ref<"local" | "remote">("local");
const selectedProfiles = ref<Set<string>>(new Set());
const isDragOver = ref(false);
const dropFiles = ref<File[]>([]);

function activateProfile(name: string) {
  profiles.value.forEach((p) => (p.active = p.name === name));
  show(`已切换到配置: ${name}`, "success");
}

function updateProfile(name: string) {
  show(`正在更新配置: ${name}`, "info");
}

function importProfile() {
  show("文件选择器将在 Tauri 环境中生效", "info");
}

function confirmDelete(name: string) {
  deleteTarget.value = name;
  showDeleteDialog.value = true;
}

function doDelete() {
  if (deleteTarget.value) {
    profiles.value = profiles.value.filter(p => p.name !== deleteTarget.value);
    show(`已删除配置: ${deleteTarget.value}`, "success");
  }
  showDeleteDialog.value = false;
  deleteTarget.value = null;
}

function toggleSelect(name: string) {
  if (selectedProfiles.value.has(name)) {
    selectedProfiles.value.delete(name);
  } else {
    selectedProfiles.value.add(name);
  }
}

function deleteSelected() {
  profiles.value = profiles.value.filter(p => !selectedProfiles.value.has(p.name));
  show(`已删除 ${selectedProfiles.value.size} 个配置`, "success");
  selectedProfiles.value.clear();
}

function addProfile() {
  if (!newProfileName.value.trim()) {
    show("请输入配置名称", "error");
    return;
  }
  profiles.value.push({
    name: newProfileName.value.trim(),
    type: newProfileType.value,
    url: newProfileUrl.value || undefined,
    active: false,
    updated: new Date().toLocaleString("zh-CN", { hour12: false }),
    nodes: 0,
  });
  show(`已添加配置: ${newProfileName.value}`, "success");
  showAddDialog.value = false;
  newProfileName.value = "";
  newProfileUrl.value = "";
}

function onDragEnter(e: DragEvent) {
  e.preventDefault();
  isDragOver.value = true;
}

function onDragOver(e: DragEvent) {
  e.preventDefault();
}

function onDragLeave() {
  isDragOver.value = false;
}

function onDrop(e: DragEvent) {
  e.preventDefault();
  isDragOver.value = false;

  const files = e.dataTransfer?.files;
  if (!files || files.length === 0) return;

  const validExts = [".yaml", ".yml", ".json", ".txt"];
  const dropped = Array.from(files).filter(f =>
    validExts.some(ext => f.name.endsWith(ext))
  );

  if (dropped.length === 0) {
    show("不支持的文件格式，请使用 .yaml/.yml/.json/.txt", "error");
    return;
  }

  dropFiles.value = dropped;
  processDroppedFiles(dropped);
}

function processDroppedFiles(files: File[]) {
  for (const file of files) {
    profiles.value.push({
      name: file.name,
      type: "local",
      active: false,
      updated: new Date().toLocaleString("zh-CN", { hour12: false }),
      nodes: 0,
    });
  }
  show(`已导入 ${files.length} 个配置文件`, "success");
  dropFiles.value = [];
}

function typeIcon(type: string) {
  switch (type) {
    case "local": return FileJson;
    case "remote": return Globe;
    case "merge": return GitMerge;
    case "script": return ScrollText;
    default: return FileJson;
  }
}

function typeLabel(type: string) {
  switch (type) {
    case "local": return "本地";
    case "remote": return "远程";
    case "merge": return "合并";
    case "script": return "脚本";
    default: return type;
  }
}

function typeColor(type: string): string {
  switch (type) {
    case "local": return "var(--accent)";
    case "remote": return "var(--green)";
    case "merge": return "var(--orange)";
    case "script": return "#bf5af2";
    default: return "var(--text-secondary)";
  }
}
</script>

<template>
  <BasePage
    title="配置"
    :class="{ 'drop-active': isDragOver }"
    @dragenter="onDragEnter"
    @dragover="onDragOver"
    @dragleave="onDragLeave"
    @drop="onDrop"
  >
    <template #actions>
      <div class="flex items-center gap-2">
        <button v-if="selectedProfiles.size > 0" class="btn-ghost text-xs" :style="{ color: 'var(--red)' }" @click="deleteSelected">
          <Trash2 :size="12" />
          删除 ({{ selectedProfiles.size }})
        </button>
        <button class="btn-ghost text-xs" @click="importProfile">
          <Upload :size="14" />
          导入
        </button>
        <button class="btn-primary text-xs" @click="showAddDialog = true">
          <Plus :size="14" />
          添加
        </button>
      </div>
    </template>

    <!-- Drop zone overlay -->
    <Transition name="page">
      <div v-if="isDragOver" class="drop-overlay">
        <div class="drop-zone">
          <FileUp :size="48" style="color: var(--accent)" />
          <div class="text-lg font-medium mt-4">拖放配置文件到此处</div>
          <div class="text-sm mt-2" style="color: var(--text-secondary)">支持 .yaml .yml .json .txt 格式</div>
        </div>
      </div>
    </Transition>

    <div class="space-y-2 flex-1">
      <div
        v-for="profile in profiles"
        :key="profile.name"
        class="profile-card"
        :class="{ 'profile-card-active': profile.active }"
        @click="activateProfile(profile.name)"
      >
        <div class="w-5 shrink-0" @click.stop>
          <input
            type="checkbox"
            :checked="selectedProfiles.has(profile.name)"
            class="profile-checkbox"
            @change="toggleSelect(profile.name)"
          />
        </div>

        <div
          class="w-10 h-10 rounded-lg flex items-center justify-center shrink-0"
          :style="{ backgroundColor: profile.active ? 'var(--accent)' : 'var(--bg-tertiary)' }"
        >
          <component :is="typeIcon(profile.type)" :size="18" :style="{ color: profile.active ? '#fff' : typeColor(profile.type) }" />
        </div>

        <div class="flex-1 min-w-0">
          <div class="text-sm font-medium truncate">{{ profile.name }}</div>
          <div class="flex items-center gap-2 text-xs mt-0.5" :style="{ color: 'var(--text-secondary)' }">
            <span class="type-tag" :style="{ color: typeColor(profile.type) }">{{ typeLabel(profile.type) }}</span>
            <span>·</span>
            <span>{{ profile.updated }}</span>
            <span v-if="profile.nodes > 0">·</span>
            <span v-if="profile.nodes > 0">{{ profile.nodes }} 节点</span>
          </div>
        </div>

        <div class="flex items-center gap-1 shrink-0">
          <Check v-if="profile.active" :size="16" style="color: var(--green)" />
          <button v-if="profile.type === 'remote'" class="btn-ghost text-xs" @click.stop="updateProfile(profile.name)">
            <RefreshCw :size="12" />
            更新
          </button>
          <button class="btn-ghost p-1" @click.stop="confirmDelete(profile.name)" :style="{ color: 'var(--red)' }">
            <Trash2 :size="14" />
          </button>
        </div>
      </div>
    </div>

    <ConfirmDialog
      :show="showDeleteDialog"
      title="删除配置"
      :message="`确定要删除配置「${deleteTarget}」吗？此操作不可撤销。`"
      confirm-text="删除"
      type="danger"
      @confirm="doDelete"
      @cancel="showDeleteDialog = false"
    />

    <Teleport to="body">
      <Transition name="page">
        <div v-if="showAddDialog" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showAddDialog = false">
          <div class="card w-full max-w-md mx-4 space-y-4" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
            <h3 class="text-base font-medium">添加配置</h3>
            <div class="space-y-3">
              <div>
                <label class="text-xs font-medium mb-1 block" :style="{ color: 'var(--text-secondary)' }">类型</label>
                <div class="flex gap-2">
                  <button class="tab-btn flex-1" :class="newProfileType === 'local' ? 'tab-btn-active' : 'tab-btn-inactive'" @click="newProfileType = 'local'">
                    <FileJson :size="14" />
                    本地
                  </button>
                  <button class="tab-btn flex-1" :class="newProfileType === 'remote' ? 'tab-btn-active' : 'tab-btn-inactive'" @click="newProfileType = 'remote'">
                    <Globe :size="14" />
                    远程
                  </button>
                </div>
              </div>
              <div>
                <label class="text-xs font-medium mb-1 block" :style="{ color: 'var(--text-secondary)' }">名称</label>
                <input v-model="newProfileName" placeholder="配置名称" class="w-full rounded-lg px-3 py-2 text-sm outline-none border" :style="{ backgroundColor: 'var(--bg-tertiary)', color: 'var(--text-primary)', borderColor: 'var(--border)' }" />
              </div>
              <div v-if="newProfileType === 'remote'">
                <label class="text-xs font-medium mb-1 block" :style="{ color: 'var(--text-secondary)' }">订阅 URL</label>
                <input v-model="newProfileUrl" placeholder="https://example.com/sub" class="w-full rounded-lg px-3 py-2 text-sm outline-none border" :style="{ backgroundColor: 'var(--bg-tertiary)', color: 'var(--text-primary)', borderColor: 'var(--border)' }" />
              </div>
            </div>
            <div class="flex justify-end gap-2">
              <button class="btn-ghost text-xs" @click="showAddDialog = false">取消</button>
              <button class="btn-primary text-xs" @click="addProfile">添加</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </BasePage>
</template>

<style scoped>
.profile-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 16px;
  border-radius: 12px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
  cursor: pointer;
  transition: border-color 150ms ease;
}
.profile-card:hover { border-color: var(--accent); }
.profile-card-active { border-color: var(--accent); }

.profile-checkbox {
  width: 14px;
  height: 14px;
  accent-color: var(--accent);
  cursor: pointer;
}

.type-tag {
  font-size: 10px;
  padding: 1px 6px;
  border-radius: 4px;
  font-weight: 500;
}

.drop-active {
  position: relative;
}

.drop-overlay {
  position: absolute;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  border-radius: 12px;
}

.drop-zone {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 48px;
  border: 2px dashed var(--accent);
  border-radius: 16px;
  background-color: rgba(79, 142, 247, 0.05);
}
</style>
