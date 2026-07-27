<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'

import { BaseDialog, DialogRef, Switch } from '@/components/base'
import { useVerge } from '@/hooks/use-verge'
import { showNotice } from '@/services/notice-service'

import HotkeyInput from './hotkey-input.vue'

const HOTKEY_FUNC = [
  'open_or_close_dashboard',
  'clash_mode_rule',
  'clash_mode_global',
  'clash_mode_direct',
  'toggle_system_proxy',
  'toggle_tun_mode',
  'entry_lightweight_mode',
  'reactivate_profiles',
] as const

const HOTKEY_FUNC_LABELS: Record<string, string> = {
  open_or_close_dashboard: 'settings.modals.hotkey.functions.openOrCloseDashboard',
  clash_mode_rule: 'settings.modals.hotkey.functions.rule',
  clash_mode_global: 'settings.modals.hotkey.functions.global',
  clash_mode_direct: 'settings.modals.hotkey.functions.direct',
  toggle_system_proxy: 'settings.modals.hotkey.functions.toggleSystemProxy',
  toggle_tun_mode: 'settings.modals.hotkey.functions.toggleTunMode',
  entry_lightweight_mode: 'settings.modals.hotkey.functions.entryLightweightMode',
  reactivate_profiles: 'settings.modals.hotkey.functions.reactivateProfiles',
}

const { verge, patchVerge } = useVerge()
const open = ref(false)
const hotkeyMap = ref<Record<string, string[]>>({})
const enableGlobalHotkey = ref(verge?.enable_global_hotkey ?? true)

defineExpose<DialogRef>({
  open: () => {
    open.value = true
    const map: Record<string, string[]> = {}
    verge?.hotkeys?.forEach((text: string) => {
      const [func, key] = text.split(',').map((e) => e.trim())
      if (!func || !key) return
      map[func] = key.split('+').map((e) => e.trim()).map((k) => (k === 'PLUS' ? '+' : k))
    })
    hotkeyMap.value = map
  },
  close: () => { open.value = false },
})

const onSave = async () => {
  const hotkeys = Object.entries(hotkeyMap.value)
    .map(([func, keys]) => {
      if (!func || !keys?.length) return ''
      const key = keys.map((k) => k.trim()).filter(Boolean).map((k) => (k === '+' ? 'PLUS' : k)).join('+')
      if (!key) return ''
      return `${func},${key}`
    })
    .filter(Boolean)

  try {
    await patchVerge({ hotkeys, enable_global_hotkey: enableGlobalHotkey.value })
    open.value = false
  } catch (err) {
    showNotice.error(err)
  }
}
</script>

<template>
  <BaseDialog
    :open="open"
    :title="i18n.t('settings.modals.hotkey.title')"
    :contentSx="{ width: '450px', maxHeight: '380px' }"
    :okBtn="i18n.t('shared.actions.save')"
    :cancelBtn="i18n.t('shared.actions.cancel')"
    @onClose="open = false"
    @onCancel="open = false"
    @onOk="onSave"
  >
    <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;">
      <span class="MuiTypography-root">{{ i18n.t('settings.modals.hotkey.toggles.enableGlobal') }}</span>
      <Switch edge="end" :checked="enableGlobalHotkey" @change="(e: any) => enableGlobalHotkey = e.target.checked" />
    </div>

    <div v-for="func in HOTKEY_FUNC" :key="func" style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 8px;">
      <span class="MuiTypography-root">{{ i18n.t(HOTKEY_FUNC_LABELS[func]) }}</span>
      <HotkeyInput
        :value="hotkeyMap[func] ?? []"
        @change="(v: string[]) => hotkeyMap[func] = v"
      />
    </div>
  </BaseDialog>
</template>
