import { ref, watch, onUnmounted } from 'vue'

import delayManager, { type DelaySnapshot } from '@/services/delay'

const NO_DELAYS: DelaySnapshot = { of: () => -1 }

export const useGroupDelays = (getGroup: () => string | null) => {
  const delays = ref(NO_DELAYS)
  let unsubscribe: (() => void) | null = null

  watch(
    getGroup,
    (newGroup, oldGroup) => {
      unsubscribe?.()
      unsubscribe = null
      if (!newGroup) {
        delays.value = NO_DELAYS
        return
      }
      const name = newGroup
      unsubscribe = delayManager.addGroupListener(name, () => {
        delays.value = delayManager.groupDelays(name)
      })
      delays.value = delayManager.groupDelays(name)
    },
    { immediate: true },
  )

  onUnmounted(() => {
    unsubscribe?.()
  })

  return delays
}

export const useGroupsDelays = (getGroups: () => readonly string[]) => {
  const delaysMap = ref<ReadonlyMap<string, DelaySnapshot>>(new Map())
  let cleanups: (() => void)[] = []

  watch(
    getGroups,
    (newGroups) => {
      for (const cleanup of cleanups) cleanup()
      const cacheKey = newGroups.join(' ')
      cleanups = newGroups.map((name) =>
        delayManager.addGroupListener(name, () => {
          delaysMap.value = delayManager.groupsDelays(cacheKey)
        }),
      )
      delaysMap.value = delayManager.groupsDelays(cacheKey)
    },
    { immediate: true },
  )

  onUnmounted(() => {
    for (const cleanup of cleanups) cleanup()
  })

  return delaysMap
}
