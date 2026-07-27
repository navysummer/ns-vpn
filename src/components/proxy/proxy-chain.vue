<template>
  <div :style="{ height: '100%', padding: '16px', display: 'flex', flexDirection: 'column', border: '1px solid var(--border-color)', borderRadius: '12px', backgroundColor: 'var(--bg-color)' }">
    <div :style="{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '16px' }">
      <div :style="{ display: 'flex', alignItems: 'center', gap: '6px' }">
        <span :style="{ fontSize: '20px', fontWeight: 600 }">{{ t('proxies.page.chain.header') }}</span>
        <TooltipIcon :title="chainWarning" :icon="'warning'" color="warning" :style="{ padding: '2px' }" />
      </div>
      <div :style="{ display: 'flex', alignItems: 'center', gap: '8px' }">
        <n-button v-if="currentProxyChain.length > 0" quaternary circle size="small" @click="clearChain">
          <template #icon>
            <svg viewBox="0 0 24 24" width="16" height="16" fill="var(--error-color)"><path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
          </template>
        </n-button>
        <n-button
          size="small"
          :type="isConnected ? 'error' : 'success'"
          :disabled="isConnecting || (!isConnected && (currentProxyChain.length < 2 || currentProxyChain.some((p: any) => p.recordId === undefined) || (mode === 'global' && proxyView?.global === null) || (mode !== 'global' && !selectedGroup)))"
          :style="{ minWidth: '90px' }"
          @click="handleConnect"
        >
          <template #icon>
            <svg v-if="isConnected" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M17 7h-4v2h4c1.65 0 3 1.35 3 3s-1.35 3-3 3h-4v2h4c2.76 0 5-2.24 5-5s-2.24-5-5-5zm-6 8H7c-1.65 0-3-1.35-3-3s1.35-3 3-3h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-2z"/></svg>
            <svg v-else viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M17 7h-4v2h4c1.65 0 3 1.35 3 3s-1.35 3-3 3h-4v2h4c2.76 0 5-2.24 5-5s-2.24-5-5-5zm-6 8H7c-1.65 0-3-1.35-3-3s1.35-3 3-3h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-2z"/></svg>
          </template>
          {{ isConnecting ? t('proxies.page.actions.connecting') : isConnected ? t('proxies.page.actions.disconnect') : t('proxies.page.actions.connect') }}
        </n-button>
      </div>
    </div>

    <n-alert v-if="currentProxyChain.length === 1" type="warning" :style="{ marginBottom: '16px' }">
      {{ t('proxies.page.chain.minimumNodesHint') }}
    </n-alert>
    <n-alert v-else type="info" :style="{ marginBottom: '16px' }">
      {{ t('proxies.page.chain.instruction') }}
    </n-alert>

    <div :style="{ flex: 1, overflow: 'auto' }">
      <div v-if="currentProxyChain.length === 0" :style="{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100%', color: 'var(--text-secondary-color)' }">
        <span>{{ t('proxies.page.chain.empty') }}</span>
      </div>
      <div v-else :style="{ borderRadius: '8px', minHeight: '60px', padding: '8px' }">
        <div v-for="(proxy, index) in currentProxyChain" :key="proxy.id" :style="{ marginBottom: index < currentProxyChain.length - 1 ? '0' : '0' }">
          <div
            :style="{
              display: 'flex',
              alignItems: 'center',
              padding: '8px',
              backgroundColor: 'var(--bg-color)',
              borderRadius: '8px',
              border: getRoleBorder(index),
              opacity: proxy.recordId === undefined ? 0.55 : 1,
            }"
          >
            <div :style="{ display: 'flex', alignItems: 'center', marginRight: '8px', color: 'var(--text-secondary-color)', cursor: 'grab' }">
              <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M11 18c0 1.1-.9 2-2 2s-2-.9-2-2 .9-2 2-2 2 .9 2 2zm-2-8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0-6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm6 4c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"/></svg>
            </div>

            <n-tag
              size="small"
              :color="index === 0 ? 'success' : (index === currentProxyChain.length - 1 && currentProxyChain.length > 1 ? 'warning' : 'primary')"
              :style="{ marginRight: '8px', fontWeight: 700, color: '#fff' }"
            >
              {{ index === 0 ? t('proxies.page.chain.entryNode') : index === currentProxyChain.length - 1 && currentProxyChain.length > 1 ? t('proxies.page.chain.exitNode') : `${index + 1}` }}
            </n-tag>

            <span :style="{ flex: 1, fontWeight: 500, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', fontSize: '14px' }">{{ proxy.name }}</span>

            <n-tag v-if="proxy.type" size="small" variant="outline" :style="{ marginRight: '8px' }">{{ proxy.type }}</n-tag>

            <n-tag
              v-if="proxy.delay !== undefined"
              size="small"
              :color="proxy.delay > 0 && proxy.delay < 200 ? 'success' : proxy.delay > 0 && proxy.delay < 800 ? 'warning' : 'error'"
              :style="{ marginRight: '8px', fontSize: '11px', minWidth: '50px' }"
            >
              {{ proxy.delay > 0 ? `${proxy.delay}ms` : t('shared.labels.timeout') }}
            </n-tag>

            <n-button quaternary circle size="small" @click="handleRemoveProxy(proxy.id)">
              <template #icon>
                <svg viewBox="0 0 24 24" width="16" height="16" fill="var(--error-color)"><path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
              </template>
            </n-button>
          </div>
          <div v-if="index < currentProxyChain.length - 1" :style="{ display: 'flex', justifyContent: 'center', padding: '2px 0' }">
            <svg viewBox="0 0 24 24" width="20" height="20" fill="var(--primary-color)" :style="{ opacity: 0.7 }"><path d="M20 12l-1.41-1.41L13 16.17V4h-2v12.17l-5.58-5.59L4 12l8 8 8-8z"/></svg>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { NAlert, NButton, NTag } from 'naive-ui'
import { closeAllConnections, selectNodeForGroup } from 'tauri-plugin-mihomo-api'
import yaml from 'js-yaml'

import { TooltipIcon } from '@/components/base'
import { useRuntimeConfig } from '@/hooks/use-clash'
import { useAppRefreshers, useProxiesData } from '@/providers/app-data-context'
import { updateProxyChainConfigInRuntime } from '@/services/cmds'
import { selectGlobalChainNodes, selectRuleChainMembers } from '@/types/proxy-view'
import { rebindProxyChainItems } from './proxy-chain-model'
import { debugLog } from '@/utils/debug'

const props = withDefaults(defineProps<{
  proxyChain: any[]
  onUpdateChain: (chain: any[]) => void
  chainConfigData?: string | null
  onMarkUnsavedChanges?: () => void
  mode?: string
  selectedGroup?: string | null
}>(), {})

const { t } = useI18n()
const chainWarning = t('proxies.page.chain.warning')
const { proxyView } = useProxiesData()
const { refreshProxy } = useAppRefreshers()
const { data: runtimeConfig } = useRuntimeConfig(true)
const isConnecting = ref(false)

const candidates = computed(() => {
  if (!proxyView.value) return []
  if (props.mode === 'rule' && props.selectedGroup) {
    return selectRuleChainMembers(proxyView.value, props.selectedGroup).flatMap(
      ({ member }: any) => member.kind === 'node' ? [member.node] : [],
    )
  }
  if (!runtimeConfig.value) return []
  return selectGlobalChainNodes(proxyView.value, (runtimeConfig.value as any)?.proxies)
})

const currentProxyChain = computed(() => {
  if (!proxyView.value) return props.proxyChain.map((item: any) => ({ ...item, recordId: undefined, delay: undefined }))
  return rebindProxyChainItems(props.proxyChain, candidates.value, proxyView.value) || props.proxyChain
})

const isConnected = computed(() => {
  if (!proxyView.value || currentProxyChain.value.length === 0) return false
  const lastNode = currentProxyChain.value[currentProxyChain.value.length - 1]
  if (localStorage.getItem('proxy-chain-exit-node') === lastNode.name) return true
  if (currentProxyChain.value.length < 2) return false
  if (props.mode === 'global') return proxyView.value.global?.now === lastNode.name
  if (!props.selectedGroup) return false
  const proxyChainGroup = proxyView.value.groups.find((g: any) => g.name === props.selectedGroup)
  return proxyChainGroup?.now === lastNode.name
})

const getRoleBorder = (index: number) => {
  if (index === 0) return '1.5px solid var(--success-color)'
  if (index === currentProxyChain.value.length - 1 && currentProxyChain.value.length > 1) return '1.5px solid var(--warning-color)'
  return '1px solid var(--border-color)'
}

const handleRemoveProxy = (id: string) => {
  const newChain = currentProxyChain.value.filter((item: any) => item.id !== id)
  props.onUpdateChain(newChain)
  props.onMarkUnsavedChanges?.()
}

const clearChain = () => {
  updateProxyChainConfigInRuntime(null)
  localStorage.removeItem('proxy-chain-group')
  localStorage.removeItem('proxy-chain-exit-node')
  localStorage.removeItem('proxy-chain-items')
  props.onUpdateChain([])
}

const handleConnect = async () => {
  if (isConnected.value) {
    isConnecting.value = true
    try {
      await updateProxyChainConfigInRuntime(null)
      const targetGroup = props.mode === 'global' ? 'GLOBAL' : props.selectedGroup || localStorage.getItem('proxy-chain-group')
      if (targetGroup) {
        try { await selectNodeForGroup(targetGroup, 'DIRECT') }
        catch {
          if (currentProxyChain.value.length >= 1) {
            try { await selectNodeForGroup(targetGroup, currentProxyChain.value[0].name) } catch {}
          }
        }
      }
      localStorage.removeItem('proxy-chain-group')
      localStorage.removeItem('proxy-chain-exit-node')
      localStorage.removeItem('proxy-chain-items')
      await closeAllConnections()
      await refreshProxy()
      props.onUpdateChain([])
    } catch (error) {
      console.error('Failed to disconnect', error)
      alert(t('proxies.page.chain.disconnectFailed'))
    } finally { isConnecting.value = false }
    return
  }

  if (props.mode === 'global' && proxyView.value?.global === null) { alert(t('proxies.page.chain.connectFailed')); return }
  if (currentProxyChain.value.length < 2 || currentProxyChain.value.some(({ recordId }: any) => !recordId)) { alert(t('proxies.page.chain.minimumNodes')); return }

  isConnecting.value = true
  try {
    const chainProxies = currentProxyChain.value.map((node: any) => node.name)
    await updateProxyChainConfigInRuntime(chainProxies)
    const lastNode = currentProxyChain.value[currentProxyChain.value.length - 1]
    if (props.mode !== 'global' && !props.selectedGroup) throw new Error('规则模式下必须选择代理组')
    const targetGroup = props.mode === 'global' ? 'GLOBAL' : props.selectedGroup
    await selectNodeForGroup(targetGroup || 'GLOBAL', lastNode.name)
    localStorage.setItem('proxy-chain-group', targetGroup || 'GLOBAL')
    localStorage.setItem('proxy-chain-exit-node', lastNode.name)
    refreshProxy()
  } catch (error) {
    console.error('Failed to connect', error)
    alert(t('proxies.page.chain.connectFailed'))
  } finally { isConnecting.value = false }
}

watch(() => props.chainConfigData, (val) => {
  if (val) {
    try {
      const parsed = yaml.load(val) as any
      const timestamp = Date.now()
      const items = (parsed?.proxies?.map((p: any, i: number) => ({
        id: `${p.name}_${timestamp}_${i}`, name: p.name, type: p.type, delay: undefined,
      })) || [])
      if (items.length > 0) props.onUpdateChain(items)
    } catch (error) {
      console.error('Failed to process chain config data:', error)
    }
  }
}, { immediate: true })
</script>
