<template>
  <div>
    <ProfileBox
      @dblclick="onEditFile"
      @contextmenu="onContextMenu"
    >
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;">
        <h2
          style="margin: 0; width: calc(100% - 52px); font-size: 1.25rem; font-weight: 500; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;"
          :title="t(globalTitles[id])"
        >
          {{ t(globalTitles[id]) }}
        </h2>
        <span
          class="MuiChip-root MuiChip-sizeSmall MuiChip-outlined MuiChip-colorPrimary"
          style="height: 20px; font-size: 0.75rem; padding: 0 8px; border: 1px solid; border-radius: 12px; text-transform: capitalize; display: inline-flex; align-items: center;"
        >
          {{ t(chipLabels[id]) }}
        </span>
      </div>
      <div style="height: 26px; display: flex; align-items: center; justify-content: space-between; line-height: 1;">
        <template v-if="id === 'Script'">
          <span
            v-if="hasError"
            style="position: relative; display: inline-flex;"
          >
            <span style="position: absolute; top: -4px; right: -4px; width: 8px; height: 8px; background: #f44336; border-radius: 50%;" />
            <button
              class="MuiIconButton-root MuiIconButton-sizeSmall"
              style="color: #f44336;"
              :title="t('profiles.modals.logViewer.title')"
              @click="logOpen = true"
            >
              <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H3V5h18v14zM5 10h14v2H5zm0 4h8v2H5z"/></svg>
            </button>
          </span>
          <button
            v-else
            class="MuiIconButton-root MuiIconButton-sizeSmall"
            :title="t('profiles.modals.logViewer.title')"
            @click="logOpen = true"
          >
            <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H3V5h18v14zM5 10h14v2H5zm0 4h8v2H5z"/></svg>
          </button>
        </template>
      </div>
    </ProfileBox>

    <div
      v-if="anchorEl"
      class="MuiMenu-root MuiMenu-paper"
      :style="{ position: 'fixed', left: `${position.left}px`, top: `${position.top}px`, zIndex: 1300, minWidth: 120, backgroundColor: 'var(--bg-paper)', border: '1px solid var(--divider-color)', borderRadius: '4px', boxShadow: '0 5px 15px rgba(0,0,0,0.3)', padding: '4px 0' }"
      @click.stop
      @contextmenu.prevent="anchorEl = null"
    >
      <div
        v-for="item in itemMenu"
        :key="item.label"
        class="MuiMenuItem-root MuiMenuItem-dense"
        style="padding: 4px 16px; min-width: 120; font-size: 14px; cursor: pointer;"
        @click="item.handler"
      >
        {{ t(item.label) }}
      </div>
    </div>

    <EditorViewer
      v-if="fileOpen"
      :open="true"
      :title="t(globalTitles[id])"
      :value="document.value"
      :language="id === 'Merge' ? 'yaml' : 'javascript'"
      :path="`profile-more:${id}.${id === 'Merge' ? 'yaml' : 'js'}`"
      :loading="document.loading"
      :dirty="document.dirty"
      @change="document.setValue"
      @save="handleSave"
      :on-reset-to-default="id === 'Script' ? handleResetToDefault : undefined"
      @close="fileOpen = false"
    />
    <LogViewer
      v-if="logOpen"
      :open="logOpen"
      :log-info="entries"
      @close="logOpen = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import EditorViewer from '@/components/profile/editor-viewer.vue'
import { useEditorDocument } from '@/hooks/use-editor-document'
import { viewProfile, readProfileFile, saveProfileFile } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import LogViewer from './log-viewer.vue'
import ProfileBox from './profile-box.vue'

const props = defineProps<{
  logInfo?: [string, string][]
  id: 'Merge' | 'Script'
  onSave?: (prev?: string, curr?: string) => void
}>()

const { t } = useI18n()
const entries = computed(() => props.logInfo ?? [])
const anchorEl = ref<HTMLElement | null>(null)
const position = ref({ left: 0, top: 0 })
const fileOpen = ref(false)
const logOpen = ref(false)

const loadDocument = () => readProfileFile(props.id)
const document = useEditorDocument({ open: fileOpen, load: loadDocument })

const onEditFile = () => { anchorEl.value = null; fileOpen.value = true }

let openFileLock = false
const onOpenFile = async () => {
  if (openFileLock) return
  openFileLock = true
  anchorEl.value = null
  try { await viewProfile(props.id) }
  catch (err) { showNotice.error(err) }
  finally { openFileLock = false }
}

const hasError = computed(() => entries.value.some(([level]) => level === 'exception'))

const globalTitles: Record<string, string> = {
  Merge: 'profiles.components.more.global.merge',
  Script: 'profiles.components.more.global.script',
}

const chipLabels: Record<string, string> = {
  Merge: 'profiles.components.more.chips.merge',
  Script: 'profiles.components.more.chips.script',
}

const itemMenu = [
  { label: 'profiles.components.menu.editFile', handler: onEditFile },
  { label: 'profiles.components.menu.openFile', handler: onOpenFile },
]

const onContextMenu = (event: MouseEvent) => {
  position.value = { top: event.clientY, left: event.clientX }
  anchorEl.value = event.currentTarget as HTMLElement
  event.preventDefault()
}

let saveLock = false
const handleSave = async () => {
  if (saveLock) return
  saveLock = true
  try {
    const currentValue = document.value
    if (!(await saveProfileFile(props.id, currentValue))) { await document.reload(); return }
    props.onSave?.(document.savedValue, currentValue)
    document.markSaved(currentValue)
  } finally { saveLock = false }
}

const handleResetToDefault = () => {
  document.setValue(`// Define main function (script entry)

function main(config, profileName) {
  return config;
}
`)
}
</script>
