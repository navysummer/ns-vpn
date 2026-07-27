<script setup lang="ts">
import { computed, ref } from 'vue'
import i18n from 'i18next'

import { BaseDialog, BaseLoadingOverlay } from '@/components/base'
import { useVerge } from '@/hooks/use-verge'
import { listWebDavBackup } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { buildWebdavSignature, setWebdavStatus } from '@/services/webdav-status'

import BackupConfigViewer from './backup-config-viewer.vue'

const props = defineProps<{
  open: boolean
  onClose: () => void
  onBackupSuccess?: () => void
  setBusy?: (loading: boolean) => void
}>()

const { verge } = useVerge()
const loading = ref(false)
const webdavSignature = computed(() => buildWebdavSignature(verge))

const handleLoading = (value: boolean) => {
  loading.value = value
  props.setBusy?.(value)
}

const refreshWebdav = async (options?: { silent?: boolean; signature?: string }) => {
  const signature = options?.signature ?? webdavSignature.value
  handleLoading(true)
  try {
    await listWebDavBackup()
    setWebdavStatus(signature, 'ready')
    if (!options?.silent) {
      showNotice.success('settings.modals.backup.messages.webdavRefreshSuccess')
    }
  } catch (error) {
    setWebdavStatus(signature, 'failed')
    showNotice.error('settings.modals.backup.messages.webdavRefreshFailed', { error })
  } finally {
    handleLoading(false)
  }
}

const refreshSilently = async (signature?: string) => refreshWebdav({ silent: true, signature })
</script>

<template>
  <BaseDialog
    :open="open"
    :title="i18n.t('settings.modals.backup.webdav.title')"
    :contentSx="{ width: '520px' }"
    :disableOk="true"
    :cancelBtn="i18n.t('shared.actions.close')"
    @onCancel="onClose"
    @onClose="onClose"
  >
    <div style="position: relative;">
      <BaseLoadingOverlay :isLoading="loading" />
      <BackupConfigViewer
        :setLoading="handleLoading"
        @onBackupSuccess="async () => { await refreshSilently(); onBackupSuccess?.() }"
        @onSaveSuccess="refreshSilently"
        @onRefresh="refreshWebdav"
        @onInit="refreshSilently"
      />
    </div>
  </BaseDialog>
</template>
