import { ref, shallowRef } from 'vue'
import { getSystemProxy, getRuntimeState, RunState } from '@/services/cmds'

const sysproxyCache = shallowRef<any>(null)
const runstateCache = shallowRef<RunState | null>(null)

export function useSystemState() {
  const mutateSysproxy = async () => {
    try {
      sysproxyCache.value = await getSystemProxy()
    } catch (err) {
      console.error(err)
    }
  }

  const mutateRunstate = async () => {
    try {
      runstateCache.value = await getRuntimeState()
    } catch (err) {
      console.error(err)
    }
  }

  if (!sysproxyCache.value) mutateSysproxy()
  if (!runstateCache.value) mutateRunstate()

  return {
    sysproxy: sysproxyCache,
    runstate: runstateCache,
    mutateSysproxy,
    mutateRunstate,
  }
}
