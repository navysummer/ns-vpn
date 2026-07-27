import { ref, computed } from 'vue'
import {
  closeConnection,
  getConnections,
  selectNodeForGroup,
} from 'tauri-plugin-mihomo-api'

import { useProfiles } from '@/hooks/use-profiles'
import { useVerge } from '@/hooks/use-verge'
import { syncTrayProxySelection } from '@/services/cmds'
import { debugLog } from '@/utils/debug'

const cleanupConnections = async (previousProxy: string) => {
  try {
    const { connections } = await getConnections()
    const cleanupPromises = (connections ?? [])
      .filter((conn) => conn.chains.includes(previousProxy))
      .map((conn) => closeConnection(conn.id))

    if (cleanupPromises.length > 0) {
      await Promise.allSettled(cleanupPromises)
      debugLog(`[ProxySelection] cleaned ${cleanupPromises.length} connections`)
    }
  } catch (error) {
    console.warn('[ProxySelection] connection cleanup failed:', error)
  }
}

interface ProxySelectionOptions {
  onSuccess?: () => void
  onError?: (error: any) => void
  enableConnectionCleanup?: boolean
}

interface ProxyChangeRequest {
  groupName: string
  proxyName: string
  previousProxy?: string
  skipConfigSave: boolean
}

export const useProxySelection = (options: ProxySelectionOptions = {}) => {
  const { current, patchCurrent } = useProfiles()
  const { verge } = useVerge()
  const pendingRequestRef = ref<ProxyChangeRequest | null>(null)
  const isProcessingRef = ref(false)

  const { onSuccess, onError, enableConnectionCleanup = true } = options

  const config = computed(() => ({
    autoCloseConnection: verge.value?.auto_close_connection ?? false,
    enableConnectionCleanup,
  }))

  const syncTraySelection = () => {
    syncTrayProxySelection().catch((error) => {
      console.error('[ProxySelection] tray sync failed:', error)
    })
  }

  const persistSelection = (groupName: string, proxyName: string, skipConfigSave: boolean) => {
    if (!current.value || skipConfigSave) return

    const selected = current.value.selected ? [...current.value.selected] : []
    const index = selected.findIndex((item) => item.name === groupName)

    if (index < 0) {
      selected.push({ name: groupName, now: proxyName })
    } else {
      selected[index] = { name: groupName, now: proxyName }
    }

    patchCurrent({ selected }).catch((error) => {
      console.error('[ProxySelection] failed to save proxy selection:', error)
    })
  }

  const executeChange = async (request: ProxyChangeRequest) => {
    const { groupName, proxyName, previousProxy, skipConfigSave } = request
    debugLog(`[ProxySelection] proxy switch: ${groupName} -> ${proxyName}`)

    try {
      await selectNodeForGroup(groupName, proxyName)
      onSuccess?.()
      syncTraySelection()
      persistSelection(groupName, proxyName, skipConfigSave)
      debugLog(`[ProxySelection] proxy and state sync complete: ${groupName} -> ${proxyName}`)

      if (
        config.value.enableConnectionCleanup &&
        config.value.autoCloseConnection &&
        previousProxy
      ) {
        cleanupConnections(previousProxy)
      }
    } catch (error) {
      console.error(`[ProxySelection] proxy switch failed: ${groupName} -> ${proxyName}`, error)
      onError?.(error)
    }
  }

  const flushChangeQueue = async () => {
    if (isProcessingRef.value) return
    isProcessingRef.value = true

    try {
      while (pendingRequestRef.value) {
        const request = pendingRequestRef.value
        pendingRequestRef.value = null
        await executeChange(request)
      }
    } finally {
      isProcessingRef.value = false
      if (pendingRequestRef.value) {
        flushChangeQueue()
      }
    }
  }

  const changeProxy = (
    groupName: string,
    proxyName: string,
    previousProxy?: string,
    skipConfigSave: boolean = false,
  ) => {
    pendingRequestRef.value = {
      groupName,
      proxyName,
      previousProxy,
      skipConfigSave,
    }
    flushChangeQueue()
  }

  const handleSelectChange = (
    groupName: string,
    previousProxy?: string,
    skipConfigSave: boolean = false,
  ) =>
    (event: { target: { value: string } }) => {
      const newProxy = event.target.value
      changeProxy(groupName, newProxy, previousProxy, skipConfigSave)
    }

  const handleProxyGroupChange = (
    group: { name: string; now?: string },
    proxy: { name: string },
  ) => {
    changeProxy(group.name, proxy.name, group.now)
  }

  return {
    changeProxy,
    handleSelectChange,
    handleProxyGroupChange,
  }
}
