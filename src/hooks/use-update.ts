import { ref } from 'vue'
import { getVersion } from 'tauri-plugin-mihomo-api'
import { check as checkUpdate } from '@tauri-apps/plugin-updater'

import { useQuery } from '@/services/query-client'
import { restartApp } from '@/services/cmds'

let lastCheckTimeRef = 0

export const updateLastCheckTime = (time?: number) => {
  lastCheckTimeRef = time ?? Date.now()
}

export const readLastCheckTime = () => lastCheckTimeRef

export const useUpdate = () => {
  const updating = ref(false)
  const checking = ref(false)
  const updateInfo = ref<any>(null)
  const showUpdateDialog = ref(false)

  const { data: versionData, refetch: mutateVersion } = useQuery({
    queryKey: ['getVersion'],
    queryFn: getVersion,
  })

  const checkForUpdate = async () => {
    checking.value = true
    try {
      const update = await checkUpdate()
      if (update?.shouldUpdate && update?.manifest) {
        updateInfo.value = update.manifest
        showUpdateDialog.value = true
      }
      updateLastCheckTime()
    } catch (err) {
      console.error('[useUpdate] check failed:', err)
    } finally {
      checking.value = false
    }
  }

  const doUpdate = async () => {
    updating.value = true
    try {
      const update = await checkUpdate()
      if (update?.shouldUpdate) {
        await update.downloadAndInstall()
        await relaunch()
      }
    } catch (err) {
      console.error('[useUpdate] update failed:', err)
    } finally {
      updating.value = false
    }
  }

  return {
    version: versionData?.version || '-',
    updating,
    checking,
    updateInfo,
    showUpdateDialog,
    checkForUpdate,
    doUpdate,
    mutateVersion,
  }
}
