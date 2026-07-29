<script setup lang="ts">
import { ref } from "vue";
import { RefreshCw, Trash2, Clipboard, GripVertical, FileCode, Edit, FolderOpen, Upload, Link, FileText, FileJson } from "lucide-vue-next";
import { useToast } from "@/utils/toast";
import { useI18n } from "vue-i18n";
import ConfirmDialog from "@/components/ConfirmDialog.vue";

const { show } = useToast();
const { t } = useI18n();

type ConfigFormat = "clash" | "v2rayn" | "singbox" | "openvpn";
type ProfileType = "remote" | "local" | "script";

interface Subscription {
  name: string;
  description: string;
  type: ProfileType;
  url: string;
  format: ConfigFormat;
  userAgent: string;
  httpTimeout: number;
  updateInterval: number;
  useSystemProxy: boolean;
  useCoreProxy: boolean;
  allowInsecure: boolean;
  allowAutoUpdate: boolean;
  lastUpdate: string;
  timeAgo: string;
  fileName?: string;
  fileContent?: string;
  pasteContent?: string;
}

const subscriptions = ref<Subscription[]>([]);

const showDeleteDialog = ref(false);
const deleteTarget = ref<string | null>(null);
const updating = ref<string | null>(null);
const selectedSub = ref<string | null>(null);

const showCreateDialog = ref(false);
const newName = ref("");
const newDescription = ref("");
const newType = ref<ProfileType>("remote");
const newUrl = ref("");
const newFormat = ref<ConfigFormat>("clash");
const newUserAgent = ref("");
const newHttpTimeout = ref(10);
const newUpdateInterval = ref(120);
const newUseSystemProxy = ref(false);
const newUseCoreProxy = ref(false);
const newAllowInsecure = ref(false);
const newAllowAutoUpdate = ref(false);
const newPasteContent = ref("");
const newFileName = ref("");
const newFileContent = ref("");
const isDragOver = ref(false);
const fileInput = ref<HTMLInputElement | null>(null);

function resetCreateForm() {
  newName.value = "";
  newDescription.value = "";
  newType.value = "remote";
  newUrl.value = "";
  newFormat.value = "clash";
  newUserAgent.value = "";
  newHttpTimeout.value = 10;
  newUpdateInterval.value = 120;
  newUseSystemProxy.value = false;
  newUseCoreProxy.value = false;
  newAllowInsecure.value = false;
  newAllowAutoUpdate.value = false;
  newPasteContent.value = "";
  newFileName.value = "";
  newFileContent.value = "";
}

function createNew() {
  resetCreateForm();
  showCreateDialog.value = true;
}

function handleFileSelect(e: Event) {
  const input = e.target as HTMLInputElement;
  if (input.files && input.files[0]) {
    readFile(input.files[0]);
  }
}

function readFile(file: File) {
  newFileName.value = file.name;
  const reader = new FileReader();
  reader.onload = (e) => {
    newFileContent.value = (e.target?.result as string) || "";
  };
  reader.readAsText(file);
}

function onDragOver(e: DragEvent) {
  e.preventDefault();
  isDragOver.value = true;
}

function onDragLeave() {
  isDragOver.value = false;
}

function onDrop(e: DragEvent) {
  e.preventDefault();
  isDragOver.value = false;
  const file = e.dataTransfer?.files[0];
  if (file) readFile(file);
}

function clearFile() {
  newFileName.value = "";
  newFileContent.value = "";
  if (fileInput.value) fileInput.value.value = "";
}

function doCreate() {
  if (!newName.value.trim()) {
    show(t("profiles.enterName"), "error");
    return;
  }
  if (newType.value === "remote" && !newUrl.value.trim()) {
    show(t("subscriptions.enterSubUrl"), "error");
    return;
  }
  if (newType.value === "local" && !newFileContent.value) {
    show(t("subscriptions.importFile"), "error");
    return;
  }
  if (newType.value === "script" && !newPasteContent.value.trim()) {
    show(t("subscriptions.pasteContent"), "error");
    return;
  }
  subscriptions.value.push({
    name: newName.value.trim(),
    description: newDescription.value.trim(),
    type: newType.value,
    url: newUrl.value.trim(),
    format: newFormat.value,
    userAgent: newUserAgent.value.trim(),
    httpTimeout: newHttpTimeout.value,
    updateInterval: newUpdateInterval.value,
    useSystemProxy: newUseSystemProxy.value,
    useCoreProxy: newUseCoreProxy.value,
    allowInsecure: newAllowInsecure.value,
    allowAutoUpdate: newAllowAutoUpdate.value,
    lastUpdate: new Date().toISOString().split("T")[0],
    timeAgo: t("subscriptions.justNow"),
    fileName: newFileName.value || undefined,
    fileContent: newFileContent.value || undefined,
    pasteContent: newPasteContent.value || undefined,
  });
  show(t("subscriptions.added", { name: newName.value }), "success");
  showCreateDialog.value = false;
}

function refreshSubscription(name: string) {
  updating.value = name;
  setTimeout(() => {
    const sub = subscriptions.value.find(s => s.name === name);
    if (sub) {
      sub.lastUpdate = new Date().toISOString().split("T")[0];
      sub.timeAgo = t("subscriptions.justNow");
    }
    updating.value = null;
    show(`${t('subscriptions.refresh')}: ${name}`, "success");
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
    show(`${t('subscriptions.delete')}: ${deleteTarget.value}`, "success");
  }
  showDeleteDialog.value = false;
  deleteTarget.value = null;
}

function formatLabel(f: ConfigFormat): string {
  switch (f) {
    case "clash": return t("subscriptions.formatClash");
    case "v2rayn": return t("subscriptions.formatV2rayN");
    case "singbox": return t("subscriptions.formatSingbox");
    case "openvpn": return t("subscriptions.formatOpenvpn");
  }
}

function typeIcon(type: ProfileType) {
  switch (type) {
    case "remote": return Link;
    case "local": return FolderOpen;
    case "script": return FileCode;
  }
}
</script>

<template>
  <div class="sub-page">
    <div class="sub-header">
      <h1 class="sub-title">{{ t('subscriptions.title') }}</h1>
      <div class="sub-header-actions">
        <button class="header-icon-btn" :title="t('common.refresh')" @click="refreshAll">
          <RefreshCw :size="18" />
        </button>
        <button class="header-icon-btn" :title="t('subscriptions.clipboard')">
          <Clipboard :size="18" />
        </button>
      </div>
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
          <div class="sub-desc" v-if="sub.description">{{ sub.description }}</div>
          <div class="sub-bottom-row">
            <span class="sub-url">{{ sub.type === 'remote' ? sub.url : sub.fileName || sub.type }}</span>
            <span class="sub-time">{{ sub.timeAgo }}</span>
          </div>
          <div class="sub-meta">
            <span class="sub-format-tag">{{ formatLabel(sub.format) }}</span>
            <span class="sub-date">{{ sub.lastUpdate }}</span>
          </div>
        </div>
        <button class="sub-delete-btn" @click.stop="confirmDelete(sub.name)">
          <Trash2 :size="12" />
        </button>
      </div>

      <div v-if="subscriptions.length === 0" class="sub-empty">
        <FileText :size="40" :style="{ color: 'var(--text-secondary)', opacity: 0.3 }" />
        <div class="text-sm mt-2" :style="{ color: 'var(--text-secondary)' }">{{ t('subscriptions.create') }}...</div>
      </div>
    </div>

    <div class="sub-footer">
      <button class="sub-create-btn" @click="createNew">+ {{ t('subscriptions.create') }}</button>
    </div>

    <ConfirmDialog
      :show="showDeleteDialog"
      :title="t('subscriptions.delete')"
      :message="t('subscriptions.confirmDeleteMsg', { name: deleteTarget })"
      :confirm-text="t('common.delete')"
      type="danger"
      @confirm="doDelete"
      @cancel="showDeleteDialog = false"
    />

    <Teleport to="body">
      <Transition name="page">
        <div v-if="showCreateDialog" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showCreateDialog = false">
          <div class="create-dialog" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
            <h3 class="dialog-title">{{ t('subscriptions.createTitle') }}</h3>

            <div class="dialog-body">
              <div class="field">
                <label class="field-label">{{ t('subscriptions.type') }}</label>
                <div class="type-tabs">
                  <button class="type-tab" :class="{ active: newType === 'remote' }" @click="newType = 'remote'">
                    <Link :size="14" /> {{ t('subscriptions.typeRemote') }}
                  </button>
                  <button class="type-tab" :class="{ active: newType === 'local' }" @click="newType = 'local'">
                    <FolderOpen :size="14" /> {{ t('subscriptions.typeLocal') }}
                  </button>
                  <button class="type-tab" :class="{ active: newType === 'script' }" @click="newType = 'script'">
                    <FileCode :size="14" /> {{ t('subscriptions.typeScript') }}
                  </button>
                </div>
              </div>

              <div class="field">
                <label class="field-label">{{ t('subscriptions.name') }}</label>
                <input v-model="newName" :placeholder="t('subscriptions.namePlaceholder')" class="field-input" />
              </div>

              <div class="field">
                <label class="field-label">{{ t('subscriptions.description') }}</label>
                <input v-model="newDescription" :placeholder="t('subscriptions.descriptionPlaceholder')" class="field-input" />
              </div>

              <div class="field">
                <label class="field-label">{{ t('subscriptions.configFormat') }}</label>
                <select v-model="newFormat" class="field-input field-select">
                  <option value="clash">{{ t('subscriptions.formatClash') }}</option>
                  <option value="v2rayn">{{ t('subscriptions.formatV2rayN') }}</option>
                  <option value="singbox">{{ t('subscriptions.formatSingbox') }}</option>
                  <option value="openvpn">{{ t('subscriptions.formatOpenvpn') }}</option>
                </select>
              </div>

              <template v-if="newType === 'remote'">
                <div class="field">
                  <label class="field-label">{{ t('subscriptions.url') }}</label>
                  <input v-model="newUrl" :placeholder="t('subscriptions.urlPlaceholder')" class="field-input" />
                </div>
                <div class="field">
                  <label class="field-label">{{ t('subscriptions.userAgent') }}</label>
                  <input v-model="newUserAgent" :placeholder="t('subscriptions.userAgentPlaceholder')" class="field-input" />
                </div>
              </template>

              <template v-if="newType === 'local'">
                <div class="field">
                  <label class="field-label">{{ t('subscriptions.importFile') }}</label>
                  <div
                    class="file-drop-zone"
                    :class="{ dragover: isDragOver, 'has-file': newFileName }"
                    @dragover="onDragOver"
                    @dragleave="onDragLeave"
                    @drop="onDrop"
                    @click="fileInput?.click()"
                  >
                    <input ref="fileInput" type="file" accept=".yaml,.yml,.json,.txt,.ovpn,.conf" class="hidden" @change="handleFileSelect" />
                    <template v-if="newFileName">
                      <FileText :size="18" :style="{ color: 'var(--green)' }" />
                      <span class="file-name">{{ newFileName }}</span>
                      <button class="file-clear" @click.stop="clearFile"><Trash2 :size="12" /></button>
                    </template>
                    <template v-else>
                      <Upload :size="18" :style="{ color: 'var(--text-secondary)' }" />
                      <span :style="{ color: 'var(--text-secondary)' }">{{ t('subscriptions.dragOrClick') }}</span>
                    </template>
                  </div>
                </div>
              </template>

              <template v-if="newType === 'script'">
                <div class="field">
                  <label class="field-label">{{ t('subscriptions.pasteContent') }}</label>
                  <textarea
                    v-model="newPasteContent"
                    :placeholder="t('subscriptions.pastePlaceholder')"
                    class="field-input field-textarea"
                    rows="8"
                  />
                </div>
              </template>

              <div class="field" v-if="newType === 'remote'">
                <label class="field-label">{{ t('subscriptions.httpTimeout') }}</label>
                <div class="field-with-unit">
                  <input v-model.number="newHttpTimeout" type="number" min="1" max="60" class="field-input field-number" />
                  <span class="field-unit">{{ t('subscriptions.seconds') }}</span>
                </div>
              </div>

              <div class="field" v-if="newType === 'remote'">
                <label class="field-label">{{ t('subscriptions.updateInterval') }}</label>
                <div class="field-with-unit">
                  <input v-model.number="newUpdateInterval" type="number" min="0" max="10080" class="field-input field-number" />
                  <span class="field-unit">{{ t('subscriptions.minutes') }}</span>
                </div>
              </div>

              <div class="toggles" v-if="newType === 'remote'">
                <label class="toggle-row">
                  <span>{{ t('subscriptions.useSystemProxy') }}</span>
                  <div class="toggle" :class="{ active: newUseSystemProxy }" @click="newUseSystemProxy = !newUseSystemProxy">
                    <div class="toggle-knob"></div>
                  </div>
                </label>
                <label class="toggle-row">
                  <span>{{ t('subscriptions.useCoreProxy') }}</span>
                  <div class="toggle" :class="{ active: newUseCoreProxy }" @click="newUseCoreProxy = !newUseCoreProxy">
                    <div class="toggle-knob"></div>
                  </div>
                </label>
                <label class="toggle-row">
                  <span>{{ t('subscriptions.allowInsecure') }}</span>
                  <div class="toggle" :class="{ active: newAllowInsecure }" @click="newAllowInsecure = !newAllowInsecure">
                    <div class="toggle-knob"></div>
                  </div>
                </label>
                <label class="toggle-row">
                  <span>{{ t('subscriptions.allowAutoUpdate') }}</span>
                  <div class="toggle" :class="{ active: newAllowAutoUpdate }" @click="newAllowAutoUpdate = !newAllowAutoUpdate">
                    <div class="toggle-knob"></div>
                  </div>
                </label>
              </div>
            </div>

            <div class="dialog-footer">
              <button class="btn-ghost text-xs" @click="showCreateDialog = false">{{ t('subscriptions.cancel') }}</button>
              <button class="btn-primary text-xs" @click="doCreate">{{ t('subscriptions.save') }}</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
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
  position: relative;
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

.sub-desc {
  font-size: 11px;
  color: var(--text-secondary);
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

.sub-meta {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.sub-format-tag {
  font-size: 10px;
  padding: 1px 6px;
  border-radius: 4px;
  background-color: rgba(79,142,247,0.12);
  color: var(--accent);
  font-weight: 500;
}

.sub-date {
  font-size: 11px;
  color: var(--text-secondary);
  text-align: right;
}

.sub-delete-btn {
  position: absolute;
  top: 8px;
  right: 8px;
  display: none;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: 4px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
}
.sub-card:hover .sub-delete-btn {
  display: flex;
}
.sub-delete-btn:hover {
  background-color: rgba(255,69,58,0.12);
  color: var(--red);
}

.sub-empty {
  grid-column: 1 / -1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 0;
}

.sub-footer {
  display: flex;
}

.sub-create-btn {
  padding: 8px 24px;
  border-radius: 8px;
  border: 1px dashed var(--border);
  background: transparent;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}
.sub-create-btn:hover {
  border-color: var(--accent);
  color: var(--accent);
}

/* Dialog */
.create-dialog {
  width: 560px;
  max-width: 90vw;
  max-height: 85vh;
  border-radius: 14px;
  border: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.dialog-title {
  font-size: 16px;
  font-weight: 600;
  padding: 20px 24px 0;
  margin: 0;
}

.dialog-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px 24px;
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.field-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
}

.field-input {
  width: 100%;
  padding: 8px 12px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
  transition: border-color 150ms;
}
.field-input:focus {
  border-color: var(--accent);
}
.field-input::placeholder {
  color: var(--text-secondary);
  opacity: 0.6;
}

.field-select {
  appearance: none;
  cursor: pointer;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23999' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 10px center;
  padding-right: 32px;
}

.field-textarea {
  resize: vertical;
  min-height: 120px;
  font-family: "SF Mono", "Fira Code", monospace;
  font-size: 12px;
  line-height: 1.5;
}

.field-number {
  width: 100px;
}

.field-with-unit {
  display: flex;
  align-items: center;
  gap: 8px;
}

.field-unit {
  font-size: 12px;
  color: var(--text-secondary);
}

.type-tabs {
  display: flex;
  gap: 6px;
}

.type-tab {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 12px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: transparent;
  font-size: 12px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}
.type-tab:hover {
  border-color: var(--accent);
  color: var(--text-primary);
}
.type-tab.active {
  border-color: var(--accent);
  background-color: rgba(79,142,247,0.08);
  color: var(--accent);
}

.file-drop-zone {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 24px;
  border-radius: 8px;
  border: 2px dashed var(--border);
  background: transparent;
  cursor: pointer;
  transition: all 150ms ease;
  min-height: 80px;
}
.file-drop-zone:hover,
.file-drop-zone.dragover {
  border-color: var(--accent);
  background-color: rgba(79,142,247,0.04);
}
.file-drop-zone.has-file {
  border-style: solid;
  border-color: var(--green);
  background-color: rgba(52,199,89,0.04);
}

.file-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.file-clear {
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
  margin-left: auto;
}
.file-clear:hover {
  color: var(--red);
}

.hidden {
  display: none;
}

.toggles {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.toggle-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 0;
  font-size: 13px;
  cursor: pointer;
}

.toggle {
  position: relative;
  width: 36px;
  height: 20px;
  border-radius: 10px;
  background-color: var(--bg-tertiary);
  border: 1px solid var(--border);
  cursor: pointer;
  transition: all 200ms ease;
  flex-shrink: 0;
}
.toggle.active {
  background-color: var(--accent);
  border-color: var(--accent);
}
.toggle-knob {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: #fff;
  transition: transform 200ms ease;
  box-shadow: 0 1px 3px rgba(0,0,0,0.15);
}
.toggle.active .toggle-knob {
  transform: translateX(16px);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 12px 24px 16px;
  border-top: 1px solid var(--border);
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
  .type-tabs {
    flex-direction: column;
  }
}
</style>
