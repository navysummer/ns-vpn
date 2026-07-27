import { ref, shallowRef } from 'vue'
import { getClashInfo } from '@/services/cmds'

const clashInfoCache = shallowRef<IClashInfo | null>(null)
const clashInfoPending = ref(true)

export function useClash() {
  const mutateClashInfo = async () => {
    try {
      clashInfoPending.value = true
      const info = await getClashInfo()
      clashInfoCache.value = info
    } catch (err) {
      console.error(err)
    } finally {
      clashInfoPending.value = false
    }
  }

  if (!clashInfoCache.value && clashInfoPending.value) {
    mutateClashInfo()
  }

  return {
    clashInfo: clashInfoCache,
    pending: clashInfoPending,
    mutateClashInfo,
  }
}
