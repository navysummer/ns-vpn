import { ref, shallowRef, computed } from 'vue'

import type { ProxyViewV1 } from '@/types/proxy-view'
import { useQuery } from '@/services/query-client'
import { getProxyView, getRuntimeConfig } from '@/services/cmds'
import { getRules } from 'tauri-plugin-mihomo-api'
import { useSystemState } from '@/hooks/use-system-state'

export function useProxiesData() {
  const { data: proxyView, refetch: refreshProxy, isPending } = useQuery({
    queryKey: ['getProxyView'],
    queryFn: getProxyView,
  })
  const isProxyViewPending = computed(() => isPending.value)
  const isProxyViewError = false
  return { proxyView, isProxyViewPending, isProxyViewError }
}

export function useClashConfigData() {
  const { data: clashConfig, refetch } = useQuery({
    queryKey: ['getRuntimeConfig'],
    queryFn: getRuntimeConfig,
  })
  return { clashConfig }
}

export function useRulesData() {
  const { data: rules, refetch } = useQuery({
    queryKey: ['getRules'],
    queryFn: getRules,
  })
  return { rules }
}

export function useSystemData() {
  const { runningMode } = useSystemState()
  const isProxyViewError = false
  return { runningMode, isProxyViewError }
}

export function useUptimeData() {
  const uptime = ref(0)
  return { uptime }
}

export function useCoreDataStatus() {
  const isCoreDataPending = computed(() => false)
  return { isCoreDataPending }
}

export function useAppRefreshers() {
  const refreshProxy = async () => {}
  const refreshClashConfig = async () => {}
  const refreshRules = async () => {}
  const refreshSysproxy = async () => {}
  const refreshRuleProviders = async () => {}
  const refreshAll = async () => {}
  return {
    refreshProxy,
    refreshClashConfig,
    refreshRules,
    refreshSysproxy,
    refreshRuleProviders,
    refreshAll,
  }
}
