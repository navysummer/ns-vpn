<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'

import { BaseDialog, DialogRef, Switch } from '@/components/base'
import { useClashInfo } from '@/hooks/use-clash'
import { useVerge } from '@/hooks/use-verge'
import { showNotice } from '@/services/notice-service'

const [open, setOpen] = [ref(false), (v: boolean) => { open.value = v }]
const copySuccess = ref<string | null>(null)
const isSaving = ref(false)

const { clashInfo, patchInfo } = useClashInfo()
const { verge, patchVerge } = useVerge()
const controller = ref(clashInfo?.server || '')
const secret = ref(clashInfo?.secret || '')
const enableController = ref(verge?.enable_external_controller ?? false)

defineExpose({
  open: () => {
    open.value = true
    controller.value = clashInfo?.server || ''
    secret.value = clashInfo?.secret || ''
    enableController.value = verge?.enable_external_controller ?? false
  },
  close: () => { open.value = false },
})

const onSave = async () => {
  try {
    isSaving.value = true
    await patchVerge({ enable_external_controller: enableController.value })
    if (enableController.value) {
      if (!controller.value.trim()) {
        showNotice.error('settings.sections.externalController.messages.addressRequired')
        return
      }
      if (!secret.value.trim()) {
        showNotice.error('settings.sections.externalController.messages.secretRequired')
        return
      }
      await patchInfo({ 'external-controller': controller.value, secret: secret.value })
    } else {
      await patchInfo({ 'external-controller': '' })
    }
    showNotice.success('shared.feedback.notifications.common.saveSuccess')
    open.value = false
  } catch (err) {
    showNotice.error('shared.feedback.notifications.common.saveFailed', err, 4000)
  } finally {
    isSaving.value = false
  }
}

const handleCopyToClipboard = async (text: string, type: string) => {
  try {
    await navigator.clipboard.writeText(text)
    copySuccess.value = type
    setTimeout(() => { copySuccess.value = null })
  } catch (err) {
    console.warn('[ControllerViewer] copy to clipboard failed:', err)
    showNotice.error('settings.sections.externalController.messages.copyFailed')
  }
}
</script>

<template>
  <BaseDialog
    :open="open"
    :title="i18n.t('settings.sections.externalController.title')"
    :contentSx="{ width: '400px' }"
    :cancelBtn="i18n.t('shared.actions.cancel')"
    @onClose="open = false"
    @onCancel="open = false"
    @onOk="onSave"
  >
    <template #okBtn>
      <template v-if="isSaving">
        <span style="display: flex; align-items: center; gap: 8px;">
          <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><circle cx="12" cy="12" r="10" fill="none" stroke="currentColor" stroke-width="3" stroke-dasharray="31.4 31.4" stroke-linecap="round"/></svg>
          {{ i18n.t('shared.statuses.saving') }}
        </span>
      </template>
      <template v-else>{{ i18n.t('shared.actions.save') }}</template>
    </template>

    <ul class="MuiList-root" style="list-style: none; padding: 0;">
      <li style="padding: 5px 2px; display: flex; justify-content: space-between;">
        <div class="MuiListItemText-root">
          <span>{{ i18n.t('settings.sections.externalController.fields.enable') }}</span>
        </div>
        <Switch
          edge="end"
          :checked="enableController"
          @change="(e: any) => enableController = e.target.checked"
          :disabled="isSaving"
        />
      </li>

      <li style="padding: 5px 2px; display: flex; justify-content: space-between;">
        <div class="MuiListItemText-root">
          <span>{{ i18n.t('settings.sections.externalController.fields.address') }}</span>
        </div>
        <div style="display: flex; align-items: center; gap: 8px;">
          <input
            :value="controller"
            @input="controller = ($event.target as HTMLInputElement).value"
            :placeholder="i18n.t('settings.sections.externalController.placeholders.address')"
            :disabled="isSaving || !enableController"
            style="width: 175px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;"
          />
          <button
            class="MuiIconButton-root MuiIconButton-sizeSmall"
            :disabled="isSaving || !enableController"
            :title="i18n.t('settings.sections.externalController.tooltips.copy')"
            @click="handleCopyToClipboard(controller, 'controller')"
          >
            <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/></svg>
          </button>
        </div>
      </li>

      <li style="padding: 5px 2px; display: flex; justify-content: space-between;">
        <div class="MuiListItemText-root">
          <span>{{ i18n.t('settings.sections.externalController.fields.secret') }}</span>
        </div>
        <div style="display: flex; align-items: center; gap: 8px;">
          <input
            :value="secret"
            @input="secret = ($event.target as HTMLInputElement).value"
            :placeholder="i18n.t('settings.sections.externalController.placeholders.secret')"
            :disabled="isSaving || !enableController"
            style="width: 175px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;"
          />
          <button
            class="MuiIconButton-root MuiIconButton-sizeSmall"
            :disabled="isSaving || !enableController"
            :title="i18n.t('settings.sections.externalController.tooltips.copy')"
            @click="handleCopyToClipboard(secret, 'secret')"
          >
            <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/></svg>
          </button>
        </div>
      </li>
    </ul>

    <div v-if="copySuccess !== null" class="MuiSnackbar-root" style="position: fixed; bottom: 24px; right: 24px;">
      <div class="MuiAlert-root MuiAlert-standardSuccess">
        <template v-if="copySuccess === 'controller'">
          {{ i18n.t('settings.sections.externalController.messages.controllerCopied') }}
        </template>
        <template v-else>
          {{ i18n.t('settings.sections.externalController.messages.secretCopied') }}
        </template>
      </div>
    </div>
  </BaseDialog>
</template>
