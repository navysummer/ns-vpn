<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'

import { BaseDialog, DialogRef, Switch, TooltipIcon } from '@/components/base'
import { useVerge } from '@/hooks/use-verge'
import { entry_lightweight_mode } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'

const { verge, patchVerge } = useVerge()

const open = ref(false)
const values = ref({
  autoEnterLiteMode: false,
  autoEnterLiteModeDelay: 10,
})

defineExpose<DialogRef>({
  open: () => {
    open.value = true
    values.value = {
      autoEnterLiteMode: verge?.enable_auto_light_weight_mode ?? false,
      autoEnterLiteModeDelay: verge?.auto_light_weight_minutes ?? 10,
    }
  },
  close: () => { open.value = false },
})

const onSave = async () => {
  try {
    await patchVerge({
      enable_auto_light_weight_mode: values.value.autoEnterLiteMode,
      auto_light_weight_minutes: values.value.autoEnterLiteModeDelay,
    })
    open.value = false
  } catch (err) {
    showNotice.error(err)
  }
}
</script>

<template>
  <BaseDialog
    :open="open"
    :title="i18n.t('settings.modals.liteMode.title')"
    :contentSx="{ width: '450px' }"
    :okBtn="i18n.t('shared.actions.save')"
    :cancelBtn="i18n.t('shared.actions.cancel')"
    @onClose="open = false"
    @onCancel="open = false"
    @onOk="onSave"
  >
    <ul class="MuiList-root" style="list-style: none; padding: 0;">
      <li style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
        <span class="MuiListItemText-primary">{{ i18n.t('settings.modals.liteMode.actions.enterNow') }}</span>
        <span
          class="MuiTypography-root MuiTypography-button"
          style="cursor: pointer; color: primary.main; text-decoration: underline;"
          @click="async () => await entry_lightweight_mode()"
        >
          {{ i18n.t('shared.actions.enable') }}
        </span>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1; max-width: fit-content;">
          <span>{{ i18n.t('settings.modals.liteMode.toggles.autoEnter') }}</span>
        </div>
        <TooltipIcon :title="i18n.t('settings.modals.liteMode.tooltips.autoEnter')" />
        <Switch
          edge="end"
          :checked="values.autoEnterLiteMode"
          @change="(_, c: boolean) => values.autoEnterLiteMode = c"
          style="margin-left: auto;"
        />
      </li>

      <template v-if="values.autoEnterLiteMode">
        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.liteMode.fields.delay') }}</span>
          </div>
          <div style="display: flex; align-items: center; width: 150px;">
            <input
              type="number"
              :value="values.autoEnterLiteModeDelay"
              @input="values.autoEnterLiteModeDelay = parseInt(($event.target as HTMLInputElement).value) || 1"
              style="width: 100px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;"
            />
            <span style="margin-left: 4px;">{{ i18n.t('shared.units.minutes') }}</span>
          </div>
        </li>

        <li style="padding: 5px 2px;">
          <span class="MuiTypography-root MuiTypography-body2" style="color: text.secondary; font-style: italic;">
            {{ i18n.t('settings.modals.liteMode.messages.autoEnterHint', { n: values.autoEnterLiteModeDelay }) }}
          </span>
        </li>
      </template>
    </ul>
  </BaseDialog>
</template>
