import { ref, shallowRef, onMounted, onUnmounted, computed } from 'vue'

import { useVerge } from '@/hooks/use-verge'
import delayManager, { type DelayUpdate } from '@/services/delay'
import {
  isInteractableMember,
  memberDetails,
  type ResolvedProxyMember,
} from '@/types/proxy-view'
import { useLockFn } from '@/composables/use-lock-fn'

const PRESET_PROXY_NAMES = [
  'DIRECT', 'REJECT', 'REJECT-DROP', 'PASS', 'COMPATIBLE',
]

const INITIAL_DELAY: DelayUpdate = { delay: -1, updatedAt: 0 }

export interface UseProxyDelayState {
  delayState: typeof delayState
  delayValue: number
  isPreset: boolean
  timeout: number
  onDelay: () => Promise<void>
}

export function useProxyDelayState(
  member: ResolvedProxyMember,
  groupName: string,
): UseProxyDelayState {
  const name = member.ref.name
  const details = memberDetails(member)
  const unresolved = member.kind === 'unresolved'
  const isPreset = unresolved || PRESET_PROXY_NAMES.includes(name)
  const delayState = shallowRef<DelayUpdate>(INITIAL_DELAY)
  const { verge } = useVerge()
  const timeout = verge?.default_latency_timeout || 10000

  onMounted(() => {
    if (isPreset) return
    delayManager.setListener(name, groupName, (update: DelayUpdate) => {
      delayState.value = update
    })
  })

  onUnmounted(() => {
    if (!isPreset) {
      delayManager.removeListener(name, groupName)
    }
  })

  const updateDelay = () => {
    if (unresolved) {
      delayState.value = INITIAL_DELAY
      return
    }
    const cachedUpdate = delayManager.getDelayUpdate(name, groupName)
    if (cachedUpdate) {
      delayState.value = { ...cachedUpdate }
      return
    }

    const fallbackDelay = delayManager.getDelayFix(member, groupName)
    if (fallbackDelay === -1) {
      delayState.value = { delay: -1, updatedAt: 0 }
      return
    }

    let updatedAt = 0
    const history = details?.history
    if (history && history.length > 0) {
      const lastRecord = history[history.length - 1]
      const parsed = Date.parse(lastRecord.time)
      if (!Number.isNaN(parsed)) {
        updatedAt = parsed
      }
    }

    delayState.value = { delay: fallbackDelay, updatedAt }
  }

  onMounted(() => {
    updateDelay()
  })

  const onDelay = useLockFn(async () => {
    if (!isInteractableMember(member)) return
    delayState.value = { delay: -2, updatedAt: Date.now() }
    delayState.value = await delayManager.checkDelay(member, groupName, timeout)
  })

  const delayValue = computed(() => delayState.value.delay)

  return {
    delayState,
    delayValue: delayValue.value,
    isPreset,
    timeout,
    onDelay,
  }
}
