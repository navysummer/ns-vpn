<template>
  <n-modal
    :show="open"
    @update:show="onCloseWrapper"
    :mask-closable="false"
    preset="card"
    style="max-width: 90vw; width: 100%;"
    :title="resolvedTitle"
    :segmented="{ content: true }"
  >
    <div
      style="
        width: auto;
        height: calc(100vh - 185px);
        display: flex;
        flex-direction: column;
        overflow: hidden;
        position: relative;
      "
    >
      <div style="position: relative; flex: 1 1 auto; min-height: 0;">
        <BaseLoadingOverlay :is-loading="loading" />
        <div v-if="!loading" style="height: 100%;">
          <MonacoEditor
            height="100%"
            :path="path"
            :value="value"
            :language="language"
            :theme="themeMode === 'light' ? 'light' : 'vs-dark'"
            :loading="null"
            :save-view-state="true"
            :keep-current-model="false"
            @mount="onEditorMount"
            @change="onChange"
            @validate="onValidate"
            :options="editorOptions"
          />
        </div>
      </div>
      <div style="position: absolute; left: 14px; bottom: 8px;" class="MuiButtonGroup-root">
        <button
          v-if="!readOnly"
          class="MuiIconButton-root MuiIconButton-sizeMedium"
          :disabled="loading"
          :title="t('profiles.page.importForm.actions.paste')"
          style="display: inline-flex; align-items: center; justify-content: center;"
          @click="handlePaste"
        >
          <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M19 2h-4.18C14.4.84 13.3 0 12 0S9.6.84 9.18 2H5c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm7 18H5V4h2v3h10V4h2v16z"/></svg>
        </button>
        <button
          v-if="!readOnly"
          class="MuiIconButton-root MuiIconButton-sizeMedium"
          :disabled="loading"
          :title="t('profiles.modals.editor.actions.format')"
          style="display: inline-flex; align-items: center; justify-content: center;"
          @click="handleFormat"
        >
          <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M18 4V3c0-.55-.45-1-1-1H5c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h12c.55 0 1-.45 1-1V6h1v4H9v11c0 .55.45 1 1 1h2c.55 0 1-.45 1-1v-9h8V4h-3z"/></svg>
        </button>
        <button
          class="MuiIconButton-root MuiIconButton-sizeMedium"
          :title="t(isMaximized ? 'shared.window.minimize' : 'shared.window.maximize')"
          style="display: inline-flex; align-items: center; justify-content: center;"
          @click="handleToggleMaximize"
        >
          <svg v-if="isMaximized" viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M5 5h14v2H5z"/></svg>
          <svg v-else viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M19 19H5V5h14v14zm-16 2h18V3H3v18z"/></svg>
        </button>
      </div>
    </div>
    <template #footer>
      <div style="display: flex; justify-content: flex-end; gap: 8px; width: 100%;">
        <button
          v-if="!readOnly && onResetToDefault"
          class="MuiButton-root MuiButton-outlined MuiButton-colorWarning"
          :disabled="loading"
          style="margin-right: auto;"
          @click="onResetToDefault?.()"
        >
          <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor" style="margin-right: 4px;"><path d="M12 5V1L7 6l5 5V7c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6H4c0 4.42 3.58 8 8 8s8-3.58 8-8-3.58-8-8-8z"/></svg>
          {{ t('shared.actions.resetToDefault') }}
        </button>
        <button class="MuiButton-root MuiButton-outlined" @click="handleClose">
          {{ t(readOnly ? 'shared.actions.close' : 'shared.actions.cancel') }}
        </button>
        <button
          v-if="!readOnly"
          class="MuiButton-root MuiButton-contained"
          :disabled="disableSave"
          @click="handleSave"
        >
          {{ t('shared.actions.save') }}
        </button>
      </div>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { BaseLoadingOverlay, MonacoEditor } from '@/components/base'
import { showNotice } from '@/services/notice-service'
import { useThemeMode } from '@/services/states'
import type { MonacoEditorInstance, MonacoMarker } from '@/types/monaco'
import debounce from '@/utils/debounce'
import getSystem from '@/utils/get-system'

const appWindow = getCurrentWebviewWindow()

const props = defineProps<{
  open: boolean
  title?: string
  value: string
  language: 'yaml' | 'javascript' | 'css'
  path: string
  readOnly?: boolean
  loading?: boolean
  dirty?: boolean
  saveDisabled?: boolean
  onChange?: (value: string) => void
  onSave?: () => void | Promise<void>
  onResetToDefault?: () => void
  onClose: () => void
  onValidate?: (markers: MonacoMarker[]) => void
}>()

const emit = defineEmits<{
  change: [value: string]
  save: []
  close: []
}>()

const { t } = useI18n()
const themeMode = useThemeMode()
const isMaximized = ref(false)
const editorRef = ref<MonacoEditorInstance | null>(null)

const resolvedTitle = computed(() => props.title ?? t('profiles.components.menu.editFile'))
const disableSave = computed(() => props.loading || !!props.saveDisabled || props.dirty === false)

const syncEditorValue = () => {
  const model = editorRef.value?.getModel()
  if (model && model.getValue() !== props.value) {
    model.setValue(props.value)
  }
}

const syncMaximizedState = async () => {
  try { isMaximized.value = await appWindow.isMaximized() }
  catch { isMaximized.value = false }
}

let saveLock = false
const handleSave = async () => {
  if (saveLock) return
  saveLock = true
  try {
    if (!props.readOnly) await props.onSave?.()
    props.onClose()
  } catch (error) { showNotice.error(error) }
  finally { saveLock = false }
}

const handleClose = () => { try { props.onClose() } catch (error) { showNotice.error(error) } }
const onCloseWrapper = (val: boolean) => { if (!val) handleClose() }

let pasteLock = false
const handlePaste = async () => {
  if (pasteLock || props.readOnly || props.loading || !editorRef.value) return
  pasteLock = true
  try {
    const text = await navigator.clipboard.readText()
    if (!text) return
    const editorInstance = editorRef.value
    const model = editorInstance.getModel()
    const selections = editorInstance.getSelections()
    if (!model || !selections || selections.length === 0) return
    editorInstance.pushUndoStop()
    editorInstance.executeEdits('explicit-paste',
      selections.map((selection: any) => ({ range: selection, text, forceMoveMarkers: true })),
    )
    editorInstance.pushUndoStop()
    editorInstance.focus()
  } catch (error) { showNotice.error(error) }
  finally { pasteLock = false }
}

let formatLock = false
const handleFormat = async () => {
  if (formatLock || props.loading) return
  formatLock = true
  try { await editorRef.value?.getAction('editor.action.formatDocument')?.run() }
  catch (error) { showNotice.error(error) }
  finally { formatLock = false }
}

let maximizeLock = false
const handleToggleMaximize = async () => {
  if (maximizeLock) return
  maximizeLock = true
  try {
    await appWindow.toggleMaximize()
    await syncMaximizedState()
    editorRef.value?.layout()
  } catch (error) { showNotice.error(error) }
  finally { maximizeLock = false }
}

watch(() => props.open, (val) => { if (val) syncMaximizedState() })
watch([() => props.open, () => props.loading], ([open, loading]) => {
  if (open && !loading) syncEditorValue()
})

onMounted(() => {
  if (props.open) {
    syncMaximizedState()
    const onResized = debounce(() => {
      syncMaximizedState()
      try { editorRef.value?.layout() } catch { /* ignore */ }
    }, 100)
    const unlistenPromise = appWindow.onResized(onResized)
    onUnmounted(() => { unlistenPromise.then((unlisten) => unlisten()) })
  }
})

const onEditorMount = (editorInstance: MonacoEditorInstance) => {
  editorRef.value = editorInstance
  syncEditorValue()
}

const editorOptions = computed(() => ({
  automaticLayout: true,
  tabSize: 2,
  minimap: { enabled: typeof document !== 'undefined' && document.documentElement.clientWidth >= 1500 },
  mouseWheelZoom: true,
  readOnly: props.readOnly,
  readOnlyMessage: { value: t('profiles.modals.editor.messages.readOnly') },
  renderValidationDecorations: 'on',
  quickSuggestions: { strings: true, comments: true, other: true },
  padding: { top: 33 },
  fontFamily: `Fira Code, JetBrains Mono, Roboto Mono, "Source Code Pro", Consolas, Menlo, Monaco, monospace, "Courier New", "Apple Color Emoji"${getSystem() === 'windows' ? ', twemoji mozilla' : ''}`,
  fontLigatures: false,
  smoothScrolling: true,
}))
</script>
