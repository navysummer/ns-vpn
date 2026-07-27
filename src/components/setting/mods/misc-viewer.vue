<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'

import { BaseDialog, DialogRef, Switch, TooltipIcon } from '@/components/base'
import { useVerge } from '@/hooks/use-verge'
import { showNotice } from '@/services/notice-service'

const { verge, patchVerge } = useVerge()
const open = ref(false)
const values = ref({
  appLogLevel: 'warn',
  appLogMaxSize: 8,
  appLogMaxCount: 12,
  autoCloseConnection: true,
  autoCheckUpdate: true,
  enableBuiltinEnhanced: true,
  proxyLayoutColumn: 6,
  enableAutoDelayDetection: false,
  autoDelayDetectionIntervalMinutes: 5,
  defaultLatencyTest: '',
  autoLogClean: 2,
  defaultLatencyTimeout: 10000,
})

defineExpose<DialogRef>({
  open: () => {
    open.value = true
    values.value = {
      appLogLevel: verge?.app_log_level ?? 'warn',
      appLogMaxSize: verge?.app_log_max_size ?? 128,
      appLogMaxCount: verge?.app_log_max_count ?? 8,
      autoCloseConnection: verge?.auto_close_connection ?? true,
      autoCheckUpdate: verge?.auto_check_update ?? true,
      enableBuiltinEnhanced: verge?.enable_builtin_enhanced ?? true,
      proxyLayoutColumn: verge?.proxy_layout_column || 6,
      enableAutoDelayDetection: verge?.enable_auto_delay_detection ?? false,
      autoDelayDetectionIntervalMinutes: verge?.auto_delay_detection_interval_minutes ?? 5,
      defaultLatencyTest: verge?.default_latency_test || '',
      autoLogClean: verge?.auto_log_clean || 0,
      defaultLatencyTimeout: verge?.default_latency_timeout || 10000,
    }
  },
  close: () => { open.value = false },
})

const onSave = async () => {
  try {
    await patchVerge({
      app_log_level: values.value.appLogLevel,
      app_log_max_size: values.value.appLogMaxSize,
      app_log_max_count: values.value.appLogMaxCount,
      auto_close_connection: values.value.autoCloseConnection,
      auto_check_update: values.value.autoCheckUpdate,
      enable_builtin_enhanced: values.value.enableBuiltinEnhanced,
      proxy_layout_column: values.value.proxyLayoutColumn,
      enable_auto_delay_detection: values.value.enableAutoDelayDetection,
      auto_delay_detection_interval_minutes: values.value.autoDelayDetectionIntervalMinutes,
      default_latency_test: values.value.defaultLatencyTest,
      default_latency_timeout: values.value.defaultLatencyTimeout,
      auto_log_clean: values.value.autoLogClean as any,
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
    :title="i18n.t('settings.modals.misc.title')"
    :contentSx="{ width: '450px' }"
    :okBtn="i18n.t('shared.actions.save')"
    :cancelBtn="i18n.t('shared.actions.cancel')"
    @onClose="open = false"
    @onCancel="open = false"
    @onOk="onSave"
  >
    <ul class="MuiList-root" style="list-style: none; padding: 0;">
      <li style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
        <div class="MuiListItemText-root">
          <span>{{ i18n.t('settings.modals.misc.fields.appLogLevel') }}</span>
        </div>
        <select
          :value="values.appLogLevel"
          @change="(e: any) => values.appLogLevel = (e.target as HTMLSelectElement).value"
          class="MuiSelect-root MuiSelect-sizeSmall"
          style="width: 100px; padding: 7.5px 0;"
        >
          <option v-for="i in ['trace', 'debug', 'info', 'warn', 'error', 'silent']" :key="i" :value="i">
            {{ i[0].toUpperCase() + i.slice(1).toLowerCase() }}
          </option>
        </select>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
        <div class="MuiListItemText-root" style="max-width: fit-content;">
          <span>{{ i18n.t('settings.modals.misc.fields.appLogMaxSize') }}</span>
        </div>
        <div style="display: flex; align-items: center;">
          <input
            type="number"
            :value="values.appLogMaxSize"
            @input="values.appLogMaxSize = Math.max(1, parseInt(($event.target as HTMLInputElement).value) || 128)"
            style="width: 100px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;"
          />
          <span style="margin-left: 4px;">{{ i18n.t('shared.units.kilobytes') }}</span>
        </div>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
        <div class="MuiListItemText-root" style="max-width: fit-content;">
          <span>{{ i18n.t('settings.modals.misc.fields.appLogMaxCount') }}</span>
        </div>
        <div style="display: flex; align-items: center;">
          <input
            type="number"
            :value="values.appLogMaxCount"
            @input="values.appLogMaxCount = Math.max(1, parseInt(($event.target as HTMLInputElement).value) || 1)"
            style="width: 100px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;"
          />
          <span style="margin-left: 4px;">{{ i18n.t('shared.units.files') }}</span>
        </div>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="max-width: fit-content;">
          <span>{{ i18n.t('settings.modals.misc.fields.autoCloseConnections') }}</span>
        </div>
        <TooltipIcon :title="i18n.t('settings.modals.misc.tooltips.autoCloseConnections')" />
        <Switch edge="end" :checked="values.autoCloseConnection" @change="(_, c: boolean) => values.autoCloseConnection = c" style="margin-left: auto;" />
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
        <div class="MuiListItemText-root">
          <span>{{ i18n.t('settings.modals.misc.fields.autoCheckUpdate') }}</span>
        </div>
        <Switch edge="end" :checked="values.autoCheckUpdate" @change="(_, c: boolean) => values.autoCheckUpdate = c" />
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="max-width: fit-content;">
          <span>{{ i18n.t('settings.modals.misc.fields.enableBuiltinEnhanced') }}</span>
        </div>
        <TooltipIcon :title="i18n.t('settings.modals.misc.tooltips.enableBuiltinEnhanced')" />
        <Switch edge="end" :checked="values.enableBuiltinEnhanced" @change="(_, c: boolean) => values.enableBuiltinEnhanced = c" style="margin-left: auto;" />
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
        <div class="MuiListItemText-root">
          <span>{{ i18n.t('settings.modals.misc.fields.proxyLayoutColumns') }}</span>
        </div>
        <select
          :value="values.proxyLayoutColumn"
          @change="(e: any) => values.proxyLayoutColumn = Number((e.target as HTMLSelectElement).value)"
          class="MuiSelect-root MuiSelect-sizeSmall"
          style="width: 160px; padding: 7.5px 0;"
        >
          <option :value="6">{{ i18n.t('settings.modals.misc.options.proxyLayoutColumns.auto') }}</option>
          <option v-for="i in [1, 2, 3, 4, 5]" :key="i" :value="i">{{ i }}</option>
        </select>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
        <div class="MuiListItemText-root">
          <span>{{ i18n.t('settings.modals.misc.fields.autoLogClean') }}</span>
        </div>
        <select
          :value="values.autoLogClean"
          @change="(e: any) => values.autoLogClean = Number((e.target as HTMLSelectElement).value)"
          class="MuiSelect-root MuiSelect-sizeSmall"
          style="width: 160px; padding: 7.5px 0;"
        >
          <option :value="0">{{ i18n.t('settings.modals.misc.options.autoLogClean.never') }}</option>
          <option :value="1">{{ i18n.t('settings.modals.misc.options.autoLogClean.retainDays', { n: 1 }) }}</option>
          <option :value="2">{{ i18n.t('settings.modals.misc.options.autoLogClean.retainDays', { n: 7 }) }}</option>
          <option :value="3">{{ i18n.t('settings.modals.misc.options.autoLogClean.retainDays', { n: 30 }) }}</option>
          <option :value="4">{{ i18n.t('settings.modals.misc.options.autoLogClean.retainDays', { n: 90 }) }}</option>
        </select>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="max-width: fit-content;">
          <span>{{ i18n.t('settings.modals.misc.fields.autoDelayDetection') }}</span>
        </div>
        <TooltipIcon :title="i18n.t('settings.modals.misc.tooltips.autoDelayDetection')" />
        <Switch edge="end" :checked="values.enableAutoDelayDetection" @change="(_, c: boolean) => values.enableAutoDelayDetection = c" style="margin-left: auto;" />
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
        <div class="MuiListItemText-root" style="max-width: fit-content;">
          <span>{{ i18n.t('settings.modals.misc.fields.autoDelayDetectionInterval') }}</span>
        </div>
        <div style="display: flex; align-items: center;">
          <input
            type="number"
            :value="values.autoDelayDetectionIntervalMinutes"
            :disabled="!values.enableAutoDelayDetection"
            @input="(e: any) => {
              const parsed = parseInt((e.target as HTMLInputElement).value, 10)
              values.autoDelayDetectionIntervalMinutes = Number.isFinite(parsed) && parsed > 0 ? parsed : 1
            }"
            style="width: 120px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;"
          />
          <span style="margin-left: 4px;">{{ i18n.t('shared.units.minutes') }}</span>
        </div>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="max-width: fit-content;">
          <span>{{ i18n.t('settings.modals.misc.fields.defaultLatencyTest') }}</span>
        </div>
        <TooltipIcon :title="i18n.t('settings.modals.misc.tooltips.defaultLatencyTest')" />
        <input
          :value="values.defaultLatencyTest"
          @input="values.defaultLatencyTest = ($event.target as HTMLInputElement).value"
          placeholder="http://cp.cloudflare.com/generate_204"
          style="width: 250px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; margin-left: auto;"
        />
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
        <div class="MuiListItemText-root">
          <span>{{ i18n.t('settings.modals.misc.fields.defaultLatencyTimeout') }}</span>
        </div>
        <div style="display: flex; align-items: center;">
          <input
            type="number"
            :value="values.defaultLatencyTimeout"
            @input="values.defaultLatencyTimeout = parseInt(($event.target as HTMLInputElement).value)"
            placeholder="10000"
            style="width: 200px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;"
          />
          <span style="margin-left: 4px;">{{ i18n.t('shared.units.milliseconds') }}</span>
        </div>
      </li>
    </ul>
  </BaseDialog>
</template>
