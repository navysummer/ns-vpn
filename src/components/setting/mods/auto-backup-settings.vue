<script setup lang="ts">
import { computed, ref } from 'vue'
import i18n from 'i18next'

import { Switch } from '@/components/base'
import { useVerge } from '@/hooks/use-verge'
import { showNotice } from '@/services/notice-service'

const MIN_INTERVAL_HOURS = 1
const MAX_INTERVAL_HOURS = 168

const { verge, patchVerge } = useVerge()

const derivedValues = computed(() => ({
  scheduleEnabled: verge?.enable_auto_backup_schedule ?? false,
  intervalHours: verge?.auto_backup_interval_hours ?? 24,
  changeEnabled: verge?.auto_backup_on_change ?? true,
}))

const pendingValues = ref<{
  scheduleEnabled: boolean
  intervalHours: number
  changeEnabled: boolean
} | null>(null)

const values = computed(() => {
  if (!pendingValues.value) return derivedValues.value
  if (
    pendingValues.value.scheduleEnabled === derivedValues.value.scheduleEnabled &&
    pendingValues.value.intervalHours === derivedValues.value.intervalHours &&
    pendingValues.value.changeEnabled === derivedValues.value.changeEnabled
  ) {
    return derivedValues.value
  }
  return pendingValues.value
})

const intervalInputDraft = ref<string | null>(null)

const disabled = !verge
const scheduleDisabled = computed(() => disabled || !values.value.scheduleEnabled)

const applyPatch = async (
  partial: Partial<{ scheduleEnabled: boolean; intervalHours: number; changeEnabled: boolean }>,
  payload: Partial<IVergeConfig>,
) => {
  const nextValues = { ...values.value, ...partial }
  pendingValues.value = nextValues
  try {
    await patchVerge(payload)
  } catch (error) {
    showNotice.error(error)
    pendingValues.value = null
  }
}

const handleScheduleToggle = (
  _: any,
  checked: boolean,
) => {
  applyPatch(
    { scheduleEnabled: checked },
    {
      enable_auto_backup_schedule: checked,
      auto_backup_interval_hours: values.value.intervalHours,
    },
  )
}

const handleChangeToggle = (
  _: any,
  checked: boolean,
) => {
  applyPatch({ changeEnabled: checked }, { auto_backup_on_change: checked })
}

const handleIntervalInputChange = (event: Event) => {
  intervalInputDraft.value = (event.target as HTMLInputElement).value
}

const commitIntervalInput = () => {
  const rawValue = intervalInputDraft.value ?? values.value.intervalHours.toString()
  const trimmed = rawValue.trim()
  if (trimmed === '') {
    intervalInputDraft.value = null
    return
  }

  const parsed = Number(trimmed)
  if (!Number.isFinite(parsed)) {
    intervalInputDraft.value = null
    return
  }

  const clamped = Math.min(
    MAX_INTERVAL_HOURS,
    Math.max(MIN_INTERVAL_HOURS, Math.round(parsed)),
  )

  if (clamped === values.value.intervalHours) {
    intervalInputDraft.value = null
    return
  }

  applyPatch(
    { intervalHours: clamped },
    { auto_backup_interval_hours: clamped },
  )
  intervalInputDraft.value = null
}
</script>

<template>
  <li class="MuiListItem-root MuiListItem-divider MuiListItem-dense" style="list-style: none; padding: 10px 0;">
    <div style="display: flex; align-items: center; width: 100%; gap: 8px;">
      <div class="MuiListItemText-root">
        <span class="MuiListItemText-primary">{{ i18n.t('settings.modals.backup.auto.scheduleLabel') }}</span>
        <span class="MuiListItemText-secondary">{{ i18n.t('settings.modals.backup.auto.scheduleHelper') }}</span>
      </div>
      <Switch
        edge="end"
        :checked="values.scheduleEnabled"
        @change="handleScheduleToggle"
        :disabled="disabled"
      />
    </div>
  </li>

  <li class="MuiListItem-root MuiListItem-divider MuiListItem-dense" style="list-style: none; padding: 10px 0;">
    <div style="display: flex; align-items: center; width: 100%; gap: 16px;">
      <div class="MuiListItemText-root">
        <span class="MuiListItemText-primary">{{ i18n.t('settings.modals.backup.auto.intervalLabel') }}</span>
      </div>
      <div style="display: flex; align-items: center; min-width: 160px;">
        <input
          type="number"
          :value="intervalInputDraft ?? values.intervalHours.toString()"
          :disabled="scheduleDisabled"
          :min="MIN_INTERVAL_HOURS"
          :max="MAX_INTERVAL_HOURS"
          style="width: 100px; padding: 4px 8px; border: 1px solid #ccc; border-radius: 4px;"
          @input="handleIntervalInputChange"
          @blur="commitIntervalInput"
          @keydown="(e: any) => { if (e.key === 'Enter') { e.preventDefault(); commitIntervalInput() } }"
        />
        <span style="margin-left: 4px;">{{ i18n.t('shared.units.hours') }}</span>
      </div>
    </div>
  </li>

  <li class="MuiListItem-root MuiListItem-divider MuiListItem-dense" style="list-style: none; padding: 10px 0;">
    <div style="display: flex; align-items: center; width: 100%; gap: 8px;">
      <div class="MuiListItemText-root">
        <span class="MuiListItemText-primary">{{ i18n.t('settings.modals.backup.auto.changeLabel') }}</span>
        <span class="MuiListItemText-secondary">{{ i18n.t('settings.modals.backup.auto.changeHelper') }}</span>
      </div>
      <Switch
        edge="end"
        :checked="values.changeEnabled"
        @change="handleChangeToggle"
        :disabled="disabled"
      />
    </div>
  </li>
</template>
