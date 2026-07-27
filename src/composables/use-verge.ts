import { ref, shallowRef, triggerRef } from 'vue'
import { getVergeConfig, patchVergeConfig } from '@/services/cmds'

const vergeCache = shallowRef<IVergeConfig | undefined>(undefined)
const vergePending = ref(true)
const vergeError = ref(false)

export function useVerge() {
  const mutateVerge = async () => {
    try {
      vergePending.value = true
      const config = await getVergeConfig()
      vergeCache.value = config
      vergeError.value = false
    } catch (err) {
      console.error(err)
      vergeError.value = true
    } finally {
      vergePending.value = false
    }
  }

  const patchVerge = async (payload: Partial<IVergeConfig>) => {
    if (!vergeCache.value) return
    const newConfig = { ...vergeCache.value, ...payload }
    vergeCache.value = newConfig as IVergeConfig
    await patchVergeConfig(payload as IVergeConfig)
  }

  if (!vergeCache.value && vergePending.value) {
    mutateVerge()
  }

  return {
    verge: vergeCache,
    pending: vergePending,
    error: vergeError,
    mutateVerge,
    patchVerge,
  }
}
