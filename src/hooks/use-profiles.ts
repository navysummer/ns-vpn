import { ref } from 'vue'

import { getProfiles, patchProfilesConfig, patchProfile, viewProfile } from '@/services/cmds'
import { getCacheData, setCacheData, useQuery } from '@/services/query-client'

export const useProfiles = () => {
  const { data: profiles, refetch } = useQuery({
    queryKey: ['getProfiles'],
    queryFn: getProfiles,
  })

  const currentUid = ref<string | null>(null)

  const current = profiles?.current ?? null
  const chain = profiles?.chain ?? null

  const mutateProfiles = async () => {
    await refetch()
  }

  const patchCurrent = async (payload: Partial<IProfilesConfig>) => {
    if (!profiles) return
    const merged = { ...profiles, ...payload }
    setCacheData(['getProfiles'], merged)
    try {
      await patchProfilesConfig(payload as IProfilesConfig)
    } catch {
      await refetch()
    }
  }

  const activateProfile = async (uid: string) => {
    currentUid.value = uid
    await viewProfile(uid)
    await refetch()
  }

  return {
    profiles,
    current,
    chain,
    currentUid,
    mutateProfiles,
    patchCurrent,
    patchProfile,
    activateProfile,
  }
}
