<template>
  <div :style="{ display: 'flex', flexDirection: 'column', width: '100%' }">
    <div
      :style="{
        display: 'flex',
        justifyContent: 'center',
        gap: '8px',
        padding: '8px 0',
        position: 'relative',
        zIndex: 2,
      }"
    >
      <div
        v-for="mode in CLASH_MODES"
        :key="mode"
        :style="buttonStyles(mode)"
        @click="onChangeMode(mode)"
      >
        <span v-html="MODE_ICONS[mode]" />
        <span
          :style="{
            fontSize: '14px',
            textTransform: 'capitalize',
            fontWeight: mode === currentMode ? 600 : 400,
          }"
        >
          {{ t(MODE_META[mode].label) }}
        </span>
      </div>
    </div>

    <div
      :style="{
        width: '100%',
        margin: '8px 0',
        position: 'relative',
        display: 'flex',
        justifyContent: 'center',
        overflow: 'visible',
      }"
    >
      <div
        :style="{
          width: '95%',
          textAlign: 'center',
          color: 'var(--text-secondary-color)',
          padding: '6px',
          borderRadius: '8px',
          border: '1px solid var(--primary-color)',
          backgroundColor: 'var(--bg-color)',
          wordBreak: 'break-word',
          hyphens: 'auto',
          fontSize: '12px',
        }"
      >
        {{ modeDescription }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { closeAllConnections } from 'tauri-plugin-mihomo-api'

import { useClashMode, useRuntimeConfig } from '@/hooks/use-clash'
import { useVerge } from '@/hooks/use-verge'
import {
  useAppRefreshers,
  useClashConfigData,
  useCoreDataStatus,
} from '@/providers/app-data-context'
import { patchClashMode } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { setCacheData } from '@/services/query-client'

const { t } = useI18n()
const { verge } = useVerge()
const { clashConfig } = useClashConfigData()
const { isCoreDataPending } = useCoreDataStatus()
const { refreshClashConfig } = useAppRefreshers()

const CLASH_MODES = ['rule', 'global', 'direct'] as const

const MODE_META: Record<string, { label: string; description: string }> = {
  rule: { label: 'home.components.clashMode.labels.rule', description: 'home.components.clashMode.descriptions.rule' },
  global: { label: 'home.components.clashMode.labels.global', description: 'home.components.clashMode.descriptions.global' },
  direct: { label: 'home.components.clashMode.labels.direct', description: 'home.components.clashMode.descriptions.direct' },
}

const MODE_ICONS: Record<string, string> = {
  rule: '<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M22 11h-5V6h-3v5h-4V3H7v8H1.5v2H7v8h3v-8h4v5h3v-5h5v-2z"/></svg>',
  global: '<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/></svg>',
  direct: '<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M4 9l4.5-4.5L13 9l-4.5 4.5L4 9zm-2 5h4.5l4.5 4.5L15.5 14H22v-2H15.5L11 7.5 6.5 12H2v2z"/></svg>',
}

const optimisticMode = ref<string | null>(null)
const { data: runtimeConfig, isPending: isRuntimeConfigPending } = useRuntimeConfig()
const { data: backendMode, isPending: isBackendModePending, refetch: refetchBackendMode } = useClashMode()

const toClashMode = (mode?: string | null) => {
  if (!mode) return undefined
  const n = mode.toLowerCase()
  return CLASH_MODES.includes(n as any) ? n : undefined
}

const controllerMode = computed(() => toClashMode(clashConfig.value?.mode))
const fallbackMode = computed(() => toClashMode(backendMode.value) ?? toClashMode(runtimeConfig.value?.mode))
const resolvedMode = computed(() => controllerMode.value ?? fallbackMode.value)
const currentMode = computed(() => optimisticMode.value ?? resolvedMode.value)

const modeDescription = computed(() => {
  if (currentMode.value) return t(MODE_META[currentMode.value].description)
  if (isCoreDataPending.value || isRuntimeConfigPending.value || isBackendModePending.value) return '\u00A0'
  return t('home.components.clashMode.errors.communication')
})

let changing = false
const onChangeMode = async (mode: string) => {
  if (changing || mode === currentMode.value) return
  changing = true
  if (verge.value?.auto_close_connection) closeAllConnections()
  optimisticMode.value = mode as any
  try {
    await patchClashMode(mode)
    setCacheData(['getClashConfig'], (old: any) => old ? { ...old, mode } : old)
    await Promise.allSettled([refreshClashConfig(), refetchBackendMode()])
  } catch (error: any) {
    optimisticMode.value = null
    showNotice.error(error)
  } finally {
    optimisticMode.value = null
    changing = false
  }
}

const buttonStyles = (mode: string) => ({
  cursor: 'pointer',
  padding: '9px 16px',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  gap: '8px',
  backgroundColor: mode === currentMode.value ? 'var(--primary-color)' : 'var(--bg-color)',
  color: mode === currentMode.value ? '#fff' : 'var(--text-color)',
  borderRadius: '12px',
  transition: 'all 0.2s ease-in-out',
  position: 'relative' as const,
  overflow: 'visible',
})
</script>
