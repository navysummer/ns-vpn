<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'
import { closeAllConnections, upgradeCore } from 'tauri-plugin-mihomo-api'

import { BaseDialog, DialogRef } from '@/components/base'
import { useClash, useClashInfo } from '@/hooks/use-clash'
import { useVerge } from '@/hooks/use-verge'
import { changeClashCore, restartCore } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'

const VALID_CORE = [
  { name: 'Mihomo', core: 'verge-mihomo', chipKey: 'settings.modals.clashCore.variants.release' },
  { name: 'Mihomo Alpha', core: 'verge-mihomo-alpha', chipKey: 'settings.modals.clashCore.variants.alpha' },
]

const { verge, mutateVerge } = useVerge()
const { mutateVersion } = useClash()
const { invalidateClashConfig } = useClashInfo()

const open = ref(false)
const upgrading = ref(false)
const restarting = ref(false)
const changingCore = ref<string | null>(null)

defineExpose<DialogRef>({
  open: () => { open.value = true },
  close: () => { open.value = false },
})

const clash_core = verge?.clash_core ?? 'verge-mihomo'

const onCoreChange = async (core: string) => {
  if (core === clash_core) return
  try {
    changingCore.value = core
    closeAllConnections()
    const errorMsg = await changeClashCore(core)
    if (errorMsg) {
      showNotice.error(errorMsg)
      changingCore.value = null
      return
    }
    mutateVerge()
    await new Promise((resolve) => setTimeout(resolve, 500))
    invalidateClashConfig()
    mutateVersion()
  } catch (err) {
    showNotice.error(err)
  } finally {
    changingCore.value = null
  }
}

const onRestart = async () => {
  try {
    restarting.value = true
    await restartCore()
    showNotice.success(i18n.t('settings.feedback.notifications.clash.restartSuccess'))
    restarting.value = false
  } catch (err) {
    restarting.value = false
    showNotice.error(err)
  }
}

const onUpgrade = async () => {
  try {
    upgrading.value = true
    await upgradeCore()
    upgrading.value = false
    mutateVersion()
    showNotice.success(i18n.t('settings.feedback.notifications.clash.versionUpdated'))
  } catch (err: any) {
    upgrading.value = false
    const errMsg = err?.response?.data?.message ?? String(err)
    const showMsg = errMsg.includes('already using latest version')
      ? i18n.t('settings.feedback.notifications.clash.alreadyLatestVersion')
      : errMsg
    showNotice.info(showMsg)
  }
}
</script>

<template>
  <BaseDialog
    :open="open"
    :contentSx="{ pb: 0, width: '400px', height: '180px', overflowY: 'auto', userSelect: 'text', marginTop: '-8px' }"
    :disableOk="true"
    :cancelBtn="i18n.t('shared.actions.close')"
    @onClose="open = false"
    @onCancel="open = false"
  >
    <template #title>
      <div style="display: flex; justify-content: space-between;">
        <span>{{ i18n.t('settings.sections.clash.form.fields.clashCore') }}</span>
        <div>
          <button
            class="MuiButton-root MuiButton-contained MuiButton-sizeSmall"
            :disabled="restarting || changingCore !== null"
            style="margin-right: 8px;"
            @click="onUpgrade"
          >
            <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor" style="margin-right: 4px;"><path d="M13 5.41V4h1c.55 0 1-.45 1-1s-.45-1-1-1h-4c-.55 0-1 .45-1 1s.45 1 1 1h1v1.41c-3.93.5-7 3.88-7 7.93 0 4.42 3.58 8 8 8s8-3.58 8-8c0-4.05-3.07-7.43-7-7.93zM12 20c-3.31 0-6-2.69-6-6s2.69-6 6-6 6 2.69 6 6-2.69 6-6 6z"/></svg>
            {{ i18n.t('shared.actions.upgrade') }}
          </button>
          <button
            class="MuiButton-root MuiButton-contained MuiButton-sizeSmall"
            :disabled="upgrading"
            @click="onRestart"
          >
            <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor" style="margin-right: 4px;"><path d="M12 5V1L7 6l5 5V7c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6H4c0 4.42 3.58 8 8 8s8-3.58 8-8-3.58-8-8-8z"/></svg>
            {{ i18n.t('shared.actions.restart') }}
          </button>
        </div>
      </div>
    </template>

    <ul class="MuiList-root" style="list-style: none; padding: 0;">
      <li v-for="each in VALID_CORE" :key="each.core">
        <button
          class="MuiListItemButton-root"
          :class="{ 'Mui-selected': each.core === clash_core }"
          :disabled="changingCore !== null || restarting || upgrading"
          @click="onCoreChange(each.core)"
          style="display: flex; align-items: center; width: 100%; padding: 8px; border: none; background: transparent; cursor: pointer; text-align: left;"
        >
          <div class="MuiListItemText-root" style="flex: 1;">
            <span class="MuiListItemText-primary">{{ each.name }}</span>
            <span class="MuiListItemText-secondary">/{{ each.core }}</span>
          </div>
          <span v-if="changingCore === each.core" class="MuiCircularProgress-root" style="width: 20px; height: 20px; margin-right: 8px;">
            <svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="10" fill="none" stroke="currentColor" stroke-width="3" stroke-dasharray="31.4 31.4" stroke-linecap="round"/></svg>
          </span>
          <span v-else class="MuiChip-root MuiChip-sizeSmall">{{ i18n.t(each.chipKey) }}</span>
        </button>
      </li>
    </ul>
  </BaseDialog>
</template>
