import { onMounted, onUnmounted } from 'vue'

import { runStateQueryKey } from '@/hooks/use-system-state'
import type { RunState } from '@/services/cmds'
import { subscribeVergeEvents } from '@/services/events'
import { revalidateQueries, setCacheDataAsync } from '@/services/query-client'

export const useLayoutEvents = (
  handleNotice: (payload: [string, string]) => void,
) => {
  let unsub: (() => void) | null = null

  onMounted(() => {
    const revalidateKeys = (keys: readonly string[]) => {
      void revalidateQueries(keys.map((key) => [key]))
    }

    unsub = subscribeVergeEvents({
      'verge://refresh-clash-config': () => {
        revalidateKeys([
          'getProxyView',
          'getVersion',
          'getClashConfig',
          'getClashInfo',
          'getClashMode',
          'getRuntimeConfig',
          'getRules',
          'getRuleProviders',
        ])
      },
      'verge://refresh-verge-config': () => {
        revalidateKeys(['getVergeConfig', 'getSystemProxy', 'getAutotemProxy'])
      },
      'verge://run-state-changed': (payload) => {
        void setCacheDataAsync<RunState>(runStateQueryKey, payload)
      },
      'verge://notice-message': handleNotice,
    })
  })

  onUnmounted(() => {
    unsub?.()
  })
}
