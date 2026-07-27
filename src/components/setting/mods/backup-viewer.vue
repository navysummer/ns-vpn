<script setup lang="ts">
import { computed, ref } from 'vue'
import i18n from 'i18next'
import { open as openDialog } from '@tauri-apps/plugin-dialog'

import { BaseDialog, DialogRef } from '@/components/base'
import { useVerge } from '@/hooks/use-verge'
import {
  createLocalBackup,
  createWebdavBackup,
  importLocalBackup,
} from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { buildWebdavSignature, setWebdavStatus } from '@/services/webdav-status'

import AutoBackupSettings from './auto-backup-settings.vue'
import BackupHistoryViewer from './backup-history-viewer.vue'
import BackupWebdavDialog from './backup-webdav-dialog.vue'

type BackupSource = 'local' | 'webdav'

const { verge } = useVerge()
const open = ref(false)
const busyAction = ref<BackupSource | null>(null)
const localImporting = ref(false)
const historyOpen = ref(false)
const historySource = ref<BackupSource>('local')
const historyPage = ref(0)
const webdavDialogOpen = ref(false)
const webdavSignature = computed(() => buildWebdavSignature(verge))

defineExpose<DialogRef>({
  open: () => { open.value = true },
  close: () => { open.value = false },
})

const openHistory = (target: BackupSource) => {
  historySource.value = target
  historyPage.value = 0
  historyOpen.value = true
}

const handleBackup = async (target: BackupSource) => {
  try {
    busyAction.value = target
    if (target === 'local') {
      await createLocalBackup()
      showNotice.success('settings.modals.backup.messages.localBackupCreated')
    } else {
      await createWebdavBackup()
      showNotice.success('settings.modals.backup.messages.backupCreated')
      setWebdavStatus(webdavSignature.value, 'ready')
    }
  } catch (error) {
    console.error(error)
    showNotice.error(
      target === 'local'
        ? 'settings.modals.backup.messages.localBackupFailed'
        : 'settings.modals.backup.messages.backupFailed',
      target === 'local' ? undefined : { error },
    )
    if (target === 'webdav') {
      setWebdavStatus(webdavSignature.value, 'failed')
    }
  } finally {
    busyAction.value = null
  }
}

const handleImport = async () => {
  const selected = await openDialog({
    multiple: false,
    filters: [{ name: 'Backup File', extensions: ['zip'] }],
  })
  if (!selected || Array.isArray(selected)) return
  try {
    localImporting.value = true
    await importLocalBackup(selected)
    showNotice.success('settings.modals.backup.messages.localBackupImported')
    openHistory('local')
  } catch (error) {
    console.error(error)
    showNotice.error('settings.modals.backup.messages.localBackupImportFailed', { error })
  } finally {
    localImporting.value = false
  }
}

const setWebdavBusy = (loading: boolean) => {
  busyAction.value = loading ? 'webdav' : null
}

const isLocalBusy = computed(() => busyAction.value === 'local' || localImporting.value)

const manualItems = computed(() => [
  {
    key: 'local' as BackupSource,
    title: i18n.t('settings.modals.backup.tabs.local'),
    description: i18n.t('settings.modals.backup.manual.local'),
  },
  {
    key: 'webdav' as BackupSource,
    title: i18n.t('settings.modals.backup.tabs.webdav'),
    description: i18n.t('settings.modals.backup.manual.webdav'),
  },
])
</script>

<template>
  <BaseDialog
    :open="open"
    :title="i18n.t('settings.modals.backup.title')"
    :contentSx="{ width: '520px' }"
    :disableOk="true"
    :cancelBtn="i18n.t('shared.actions.close')"
    @onCancel="open = false"
    @onClose="open = false"
  >
    <div style="display: flex; flex-direction: column; gap: 16px;">
      <div style="border: 1px solid var(--divider-color); border-radius: 16px; padding: 16px;">
        <span class="MuiTypography-root MuiTypography-subtitle1">{{ i18n.t('settings.modals.backup.auto.title') }}</span>
        <ul class="MuiList-root" style="padding: 0; list-style: none;">
          <AutoBackupSettings />
        </ul>
      </div>

      <div style="border: 1px solid var(--divider-color); border-radius: 16px; padding: 16px;">
        <span class="MuiTypography-root MuiTypography-subtitle1">{{ i18n.t('settings.modals.backup.manual.title') }}</span>
        <ul class="MuiList-root" style="padding: 0; list-style: none;">
          <li v-for="(item, idx) in manualItems" :key="item.key" class="MuiListItem-root" :class="{ 'MuiListItem-divider': idx === 0 }" style="padding: 0;">
            <div style="width: 100%; padding: 8px 0;">
              <div class="MuiListItemText-root">
                <span class="MuiListItemText-primary">{{ item.title }}</span>
                <span class="MuiListItemText-secondary">{{ item.description }}</span>
              </div>
              <div style="display: flex; flex-wrap: wrap; gap: 8px; align-items: center;">
                <template v-if="item.key === 'local'">
                  <button class="MuiButton-root MuiButton-contained MuiButton-sizeSmall" :disabled="localImporting" @click="handleBackup('local')">
                    {{ i18n.t('settings.modals.backup.actions.backup') }}
                  </button>
                  <button class="MuiButton-root MuiButton-outlined MuiButton-sizeSmall" :disabled="isLocalBusy" @click="openHistory('local')">
                    {{ i18n.t('settings.modals.backup.actions.viewHistory') }}
                  </button>
                  <button class="MuiButton-root MuiButton-text MuiButton-sizeSmall" :disabled="busyAction === 'local'" @click="handleImport">
                    {{ i18n.t('settings.modals.backup.actions.importBackup') }}
                  </button>
                </template>
                <template v-else>
                  <button class="MuiButton-root MuiButton-contained MuiButton-sizeSmall" @click="handleBackup('webdav')">
                    {{ i18n.t('settings.modals.backup.actions.backup') }}
                  </button>
                  <button class="MuiButton-root MuiButton-outlined MuiButton-sizeSmall" @click="openHistory('webdav')">
                    {{ i18n.t('settings.modals.backup.actions.viewHistory') }}
                  </button>
                  <button class="MuiButton-root MuiButton-text MuiButton-sizeSmall" @click="webdavDialogOpen = true">
                    {{ i18n.t('settings.modals.backup.manual.configureWebdav') }}
                  </button>
                </template>
              </div>
            </div>
          </li>
        </ul>
      </div>
    </div>

    <BackupHistoryViewer
      :open="historyOpen"
      :source="historySource"
      :page="historyPage"
      @onSourceChange="(s: BackupSource) => historySource = s"
      @onPageChange="(p: number) => historyPage = p"
      @onClose="historyOpen = false"
    />
    <BackupWebdavDialog
      :open="webdavDialogOpen"
      @onClose="webdavDialogOpen = false"
      @onBackupSuccess="openHistory('webdav')"
      :setBusy="setWebdavBusy"
    />
  </BaseDialog>
</template>
