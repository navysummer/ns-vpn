<template>
  <BasePage
    full
    :contentStyle="{ height: '100%' }"
  >
    <template #title>
      <span v-if="isChainMode" :style="{ display: 'inline-flex', alignItems: 'center', gap: '6px' }" data-tauri-drag-region="true">
        {{ t('proxies.page.title.chainMode') }}
        <TooltipIcon :title="chainWarning" :icon="'warning'" color="warning" :style="{ padding: '2px' }" />
      </span>
      <span v-else>{{ t('proxies.page.title.default') }}</span>
    </template>
    <template #header>
      <div :style="{ display: 'flex', alignItems: 'center', gap: '8px' }">
        <ProviderButton />
        <div :style="{ display: 'flex', gap: '0' }">
          <n-button
            v-for="mode in MODES"
            :key="mode"
            size="small"
            :type="mode === curMode ? 'primary' : 'default'"
            :style="{ textTransform: 'capitalize' }"
            @click="onChangeMode(mode)"
          >
            {{ t(`proxies.page.modes.${mode}`) }}
          </n-button>
        </div>
        <n-button
          size="small"
          :type="isChainMode ? 'primary' : 'default'"
          :style="{ marginLeft: '8px' }"
          @click="onToggleChainMode"
        >
          <template #icon>
            <svg v-if="isChainMode" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M17 7h-4v2h4c1.65 0 3 1.35 3 3s-1.35 3-3 3h-4v2h4c2.76 0 5-2.24 5-5s-2.24-5-5-5zm-6 8H7c-1.65 0-3-1.35-3-3s1.35-3 3-3h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-2z"/></svg>
            <svg v-else viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M17 7h-4v2h4c1.65 0 3 1.35 3 3s-1.35 3-3 3h-4v2h4c2.76 0 5-2.24 5-5s-2.24-5-5-5zm-6 8H7c-1.65 0-3-1.35-3-3s1.35-3 3-3h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-2z"/></svg>
          </template>
          {{ t('proxies.page.actions.toggleChain') }}
        </n-button>
      </div>
    </template>
    <ProxyGroups
      :mode="curMode ?? 'rule'"
      :isChainMode="isChainMode"
      :chainConfigData="chainConfigData"
    />
  </BasePage>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton } from 'naive-ui'
import { closeAllConnections } from 'tauri-plugin-mihomo-api'

import { BasePage, TooltipIcon } from '@/components/base'
import ProviderButton from '@/components/proxy/provider-button.vue'
import ProxyGroups from '@/components/proxy/proxy-groups.vue'
import { useVerge } from '@/hooks/use-verge'
import { useAppRefreshers, useClashConfigData } from '@/providers/app-data-context'
import { getRuntimeProxyChainConfig, patchClashMode, updateProxyChainConfigInRuntime } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'

const MODES = ['rule', 'global', 'direct'] as const

const { t } = useI18n()
const { clashConfig } = useClashConfigData()
const { refreshClashConfig } = useAppRefreshers()
const { verge } = useVerge()

const isChainMode = ref(false)
try { isChainMode.value = localStorage.getItem('proxy-chain-mode-enabled') === 'true' } catch {}

const chainConfigData = ref<string | null>(null)

const normalizedMode = computed(() => clashConfig.value?.mode?.toLowerCase())
const curMode = computed(() => {
  const m = normalizedMode.value
  return m && MODES.includes(m as any) ? m as typeof MODES[number] : undefined
})

const chainWarning = t('proxies.page.chain.warning')

let changingMode = false
const onChangeMode = async (mode: string) => {
  if (changingMode) return
  changingMode = true
  if (mode !== curMode.value && verge.value?.auto_close_connection) closeAllConnections()
  try {
    await patchClashMode(mode)
    refreshClashConfig()
  } catch (error: any) {
    showNotice.error(error)
  } finally { changingMode = false }
}

let togglingChain = false
const onToggleChainMode = async () => {
  if (togglingChain) return
  togglingChain = true
  const newChainMode = !isChainMode.value
  isChainMode.value = newChainMode
  localStorage.setItem('proxy-chain-mode-enabled', newChainMode.toString())
  if (!newChainMode) {
    try {
      await updateProxyChainConfigInRuntime(null)
    } catch (error) {
      console.error('Failed to clear chain configuration:', error)
    }
  }
  togglingChain = false
}

watch(isChainMode, async (val) => {
  if (!val) { chainConfigData.value = null; return }
  try {
    const exitNode = localStorage.getItem('proxy-chain-exit-node')
    if (!exitNode) { chainConfigData.value = ''; return }
    const data = await getRuntimeProxyChainConfig(exitNode)
    chainConfigData.value = data || ''
  } catch (error) {
    console.error('Failed to get runtime proxy chain config:', error)
    chainConfigData.value = ''
  }
}, { immediate: true })

watch(normalizedMode, (val) => {
  if (val && !MODES.includes(val as any)) onChangeMode('rule')
})
</script>
