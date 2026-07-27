<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import i18n from 'i18next'
import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import customParseFormat from 'dayjs/plugin/customParseFormat'
import { save } from '@tauri-apps/plugin-dialog'

import { BaseDialog, BaseLoadingOverlay } from '@/components/base'
import { useVerge } from '@/hooks/use-verge'
import {
  deleteLocalBackup,
  deleteWebdavBackup,
  exportLocalBackup,
  listLocalBackup,
  listWebDavBackup,
  restartApp,
  restoreLocalBackup,
  restoreWebDavBackup,
} from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import {
  buildWebdavSignature,
  getWebdavStatus,
  setWebdavStatus,
} from '@/services/webdav-status'

dayjs.extend(customParseFormat)
dayjs.extend(relativeTime)

const DATE_FORMAT = 'YYYY-MM-DD_HH-mm-ss'
const FILENAME_PATTERN = /\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2}/

type BackupSource = 'local' | 'webdav'

const props = defineProps<{
  open: boolean
  source: BackupSource
  page: number
  onSourceChange: (source: BackupSource) => void
  onPageChange: (page: number) => void
  onClose: () => void
}>()

interface BackupRow {
  filename: string
  platform: string
  backup_time: dayjs.Dayjs | null
  display_time: string
  sort_value: number
}

const { verge } = useVerge()
const rows = ref<BackupRow[]>([])
const loading = ref(false)
const isRestoring = ref(false)
const isRestarting = ref(false)
const isConfirming = ref(false)
const pendingConfirmation = ref<{
  action: 'delete' | 'restore'
  filename: string
  source: BackupSource
} | null>(null)

const isLocal = computed(() => props.source === 'local')
const isWebDavConfigured = computed(() =>
  Boolean(verge?.webdav_url && verge?.webdav_username && verge?.webdav_password),
)
const webdavSignature = computed(() => buildWebdavSignature(verge))
const webdavStatus = computed(() => getWebdavStatus(webdavSignature.value))
const shouldSkipWebDav = computed(() => !isLocal.value && !isWebDavConfigured.value)
const pageSize = 8
const isBusy = computed(() => loading.value || isRestoring.value || isRestarting.value || isConfirming.value)
const total = computed(() => rows.value.length)
const pageCount = computed(() => Math.max(1, Math.ceil(total.value / pageSize)))
const currentPage = computed(() => Math.min(props.page, pageCount.value - 1))
const pagedRows = computed(() =>
  rows.value.slice(
    currentPage.value * pageSize,
    currentPage.value * pageSize + pageSize,
  )
)

const buildRow = (item: any): BackupRow | null => {
  const { filename, last_modified } = item
  if (!filename.toLowerCase().endsWith('.zip')) return null

  const platform =
    (filename.includes('-') && filename.split('-')[0]) ||
    i18n.t('settings.modals.backup.history.unknownPlatform', {
      defaultValue: 'unknown',
    })
  const match = filename.match(FILENAME_PATTERN)
  const parsedFromName = match ? dayjs(match[0], DATE_FORMAT, true) : null
  const parsedFromModified =
    last_modified && dayjs(last_modified).isValid()
      ? dayjs(last_modified)
      : null
  const backupTime = parsedFromName?.isValid()
    ? parsedFromName
    : parsedFromModified

  return {
    filename,
    platform,
    backup_time: backupTime ?? null,
    display_time:
      backupTime?.format('YYYY-MM-DD HH:mm') ??
      parsedFromModified?.format('YYYY-MM-DD HH:mm') ??
      i18n.t('settings.modals.backup.history.unknownTime', {
        defaultValue: 'Unknown time',
      }),
    sort_value:
      backupTime?.valueOf() ??
      parsedFromModified?.valueOf() ??
      Number.NEGATIVE_INFINITY,
  }
}

const fetchRows = async (options?: { force?: boolean }) => {
  if (!props.open) return
  if (shouldSkipWebDav.value) {
    rows.value = []
    return
  }
  if (!isLocal.value && webdavStatus.value === 'failed' && !options?.force) {
    rows.value = []
    return
  }

  loading.value = true
  try {
    const list = isLocal.value
      ? await listLocalBackup()
      : await listWebDavBackup()
    if (!isLocal.value) {
      setWebdavStatus(webdavSignature.value, 'ready')
    }
    rows.value = list
      .map((item: any) => buildRow(item))
      .filter((item): item is BackupRow => item !== null)
      .sort((a: BackupRow, b: BackupRow) =>
        a.sort_value === b.sort_value
          ? b.filename.localeCompare(a.filename)
          : b.sort_value - a.sort_value,
      )
  } catch (error) {
    if (!isLocal.value) {
      setWebdavStatus(webdavSignature.value, 'failed')
    }
    console.error(error)
    rows.value = []
    showNotice.error(error)
  } finally {
    loading.value = false
  }
}

watch(() => props.open, (val) => {
  if (val) fetchRows()
})

const summary = computed(() => {
  if (shouldSkipWebDav.value || (!isLocal.value && webdavStatus.value === 'failed')) {
    return i18n.t('settings.modals.backup.manual.webdav')
  }
  if (!total.value) return i18n.t('settings.modals.backup.history.empty')
  const recent =
    rows.value[0]?.backup_time?.fromNow() ?? rows.value[0]?.display_time ?? ''
  return i18n.t('settings.modals.backup.history.summary', {
    count: total.value,
    recent,
  })
})

const handleDelete = (filename: string) => {
  if (isRestarting.value) return
  pendingConfirmation.value = { action: 'delete', filename, source: props.source }
}

const handleRestore = (filename: string) => {
  if (isRestoring.value || isRestarting.value) return
  pendingConfirmation.value = { action: 'restore', filename, source: props.source }
}

const handleConfirmAction = async () => {
  if (!pendingConfirmation.value) return
  const { action, filename, source: actionSource } = pendingConfirmation.value
  const actionIsLocal = actionSource === 'local'
  isConfirming.value = true
  if (action === 'restore') {
    isRestoring.value = true
  }
  try {
    if (action === 'delete') {
      if (actionIsLocal) {
        await deleteLocalBackup(filename)
      } else {
        await deleteWebdavBackup(filename)
      }
      pendingConfirmation.value = null
      await fetchRows()
    } else {
      if (actionIsLocal) {
        await restoreLocalBackup(filename)
      } else {
        await restoreWebDavBackup(filename)
      }
      pendingConfirmation.value = null
      showNotice.success('settings.modals.backup.messages.restoreSuccess')
      isRestarting.value = true
      window.setTimeout(() => {
        restartApp().catch((err: unknown) => {
          isRestarting.value = false
          showNotice.error(err)
        })
      }, 1000)
    }
  } catch (error) {
    console.error(error)
    showNotice.error(error)
  } finally {
    isConfirming.value = false
    isRestoring.value = false
  }
}

const handleExport = async (filename: string) => {
  if (isRestarting.value) return
  if (!isLocal.value) return
  const savePath = await save({ defaultPath: filename })
  if (!savePath || Array.isArray(savePath)) return
  try {
    await exportLocalBackup(filename, savePath)
    showNotice.success('settings.modals.backup.messages.localBackupExported')
  } catch (ignoreError: unknown) {
    showNotice.error('settings.modals.backup.messages.localBackupExportFailed')
  }
}

const handleRefresh = () => {
  if (isRestarting.value) return
  fetchRows({ force: true })
}

const closeConfirmDialog = () => {
  if (isConfirming.value) return
  pendingConfirmation.value = null
}

const confirmTitle = computed(() =>
  pendingConfirmation.value?.action === 'delete'
    ? i18n.t('settings.modals.backup.actions.deleteBackup')
    : i18n.t('settings.modals.backup.actions.restoreBackup')
)
const confirmMessage = computed(() =>
  pendingConfirmation.value?.action === 'delete'
    ? i18n.t('settings.modals.backup.messages.confirmDelete')
    : i18n.t('settings.modals.backup.messages.confirmRestore')
)
</script>

<template>
  <BaseDialog
    :open="open"
    :title="i18n.t('settings.modals.backup.history.title')"
    :contentSx="{ width: '520px' }"
    :disableOk="true"
    :cancelBtn="i18n.t('shared.actions.close')"
    @onCancel="onClose"
    @onClose="onClose"
  >
    <div style="position: relative; min-height: 320px;">
      <BaseLoadingOverlay :isLoading="isBusy" />
      <div style="display: flex; flex-direction: column; gap: 16px;">
        <div style="display: flex; align-items: center; justify-content: space-between;">
          <div class="MuiTabs-root">
            <div style="display: flex;">
              <button
                class="MuiTab-root"
                :class="{ 'Mui-selected': source === 'local' }"
                :disabled="isBusy"
                @click="onSourceChange('local'); onPageChange(0)"
              >
                {{ i18n.t('settings.modals.backup.tabs.local') }}
              </button>
              <button
                class="MuiTab-root"
                :class="{ 'Mui-selected': source === 'webdav' }"
                :disabled="isBusy"
                @click="onSourceChange('webdav'); onPageChange(0)"
              >
                {{ i18n.t('settings.modals.backup.tabs.webdav') }}
              </button>
            </div>
          </div>
          <button class="MuiIconButton-root MuiIconButton-sizeSmall" :disabled="isBusy" @click="handleRefresh">
            <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6H4c0 4.42 3.58 8 8 8s8-3.58 8-8-3.58-8-8-8z"/></svg>
          </button>
        </div>
        <span class="MuiTypography-root MuiTypography-body2" style="color: text.secondary;">{{ summary }}</span>

        <ul class="MuiList-root" style="list-style: none; padding: 0;">
          <li v-if="pagedRows.length === 0" class="MuiListItem-root">
            <div class="MuiListItemText-root">
              <span class="MuiListItemText-primary">{{ i18n.t('settings.modals.backup.history.empty') }}</span>
            </div>
          </li>
          <li v-for="row in pagedRows" :key="`${row.platform}-${row.filename}`" class="MuiListItem-root MuiListItem-divider">
            <div class="MuiListItemText-root">
              <span class="MuiTypography-root MuiTypography-body2" style="word-break: break-all; font-weight: 500;">{{ row.filename }}</span>
              <div style="display: flex; align-items: center; justify-content: space-between;">
                <span class="MuiTypography-root MuiTypography-caption" style="color: text.secondary;">{{ row.platform }} · {{ row.display_time }}</span>
                <div style="display: flex; gap: 4px; align-items: center;">
                  <button v-if="isLocal" class="MuiIconButton-root MuiIconButton-sizeSmall" :disabled="isBusy" @click="handleExport(row.filename)">
                    <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/></svg>
                  </button>
                  <button class="MuiIconButton-root MuiIconButton-sizeSmall" :disabled="isBusy" @click="handleDelete(row.filename)">
                    <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
                  </button>
                  <button class="MuiIconButton-root MuiIconButton-sizeSmall" :disabled="isBusy" @click="handleRestore(row.filename)">
                    <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42C8.27 19.99 10.51 21 13 21c4.97 0 9-4.03 9-9s-4.03-9-9-9zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z"/></svg>
                  </button>
                </div>
              </div>
            </div>
          </li>
        </ul>

        <div v-if="pageCount > 1" style="display: flex; justify-content: flex-end; align-items: center; gap: 8px;">
          <span class="MuiTypography-root MuiTypography-caption">{{ currentPage + 1 }} / {{ pageCount }}</span>
          <div style="display: flex; gap: 8px;">
            <button
              class="MuiButton-root MuiButton-text MuiButton-sizeSmall"
              :disabled="isBusy || currentPage === 0"
              @click="onPageChange(Math.max(0, currentPage - 1))"
            >
              {{ i18n.t('shared.actions.previous') }}
            </button>
            <button
              class="MuiButton-root MuiButton-text MuiButton-sizeSmall"
              :disabled="isBusy || currentPage >= pageCount - 1"
              @click="onPageChange(Math.min(pageCount - 1, currentPage + 1))"
            >
              {{ i18n.t('shared.actions.next') }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <BaseDialog
      :open="pendingConfirmation !== null"
      :title="confirmTitle"
      :okBtn="i18n.t('shared.actions.confirm')"
      :cancelBtn="i18n.t('shared.actions.cancel')"
      :contentSx="{ width: '420px' }"
      :loading="isConfirming"
      @onCancel="closeConfirmDialog"
      @onClose="closeConfirmDialog"
      @onOk="handleConfirmAction"
    >
      <span class="MuiTypography-root MuiTypography-body2" style="word-break: break-word;">
        {{ confirmMessage }}
      </span>
      <span v-if="pendingConfirmation?.filename" class="MuiTypography-root MuiTypography-caption" style="color: text.secondary; display: block; margin-top: 8px; word-break: break-all;">
        {{ pendingConfirmation.filename }}
      </span>
    </BaseDialog>
  </BaseDialog>
</template>
