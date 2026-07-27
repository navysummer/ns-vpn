import { ref, shallowRef } from 'vue'
import { getProfiles } from '@/services/cmds'

const profilesCache = shallowRef<IProfilesConfig | undefined>(undefined)
const profilesPending = ref(true)

export function useProfiles() {
  const mutateProfiles = async () => {
    try {
      profilesPending.value = true
      profilesCache.value = await getProfiles()
    } catch (err) {
      console.error(err)
    } finally {
      profilesPending.value = false
    }
  }

  if (!profilesCache.value && profilesPending.value) {
    mutateProfiles()
  }

  const current = profilesCache.value?.current ?? null

  return {
    profiles: profilesCache,
    current,
    pending: profilesPending,
    mutateProfiles,
  }
}
