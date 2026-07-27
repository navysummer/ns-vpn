

import { uninstallService } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'

import { useSystemState } from './use-system-state'

export const useServiceUninstaller = () => {
  const { mutateSystemState } = useSystemState()

  const uninstallServiceAndStartSidecar = async () => {
    let uninstallError: unknown
    showNotice.info('settings.statuses.clashService.uninstalling')
    try {
      await uninstallService()
      showNotice.success(
        'settings.feedback.notifications.clashService.uninstallSuccess',
      )
    } catch (error) {
      uninstallError = error
    }

    try {
      await mutateSystemState()
    } catch (error) {
      if (!uninstallError) throw error
    }

    if (uninstallError) throw uninstallError
  }

  return { uninstallServiceAndStartSidecar }
}
