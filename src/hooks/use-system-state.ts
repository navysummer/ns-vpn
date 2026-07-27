import { useQuery } from '@/services/query-client'
import { getSystemProxy, getRuntimeState } from '@/services/cmds'

export const runStateQueryKey = ['getRuntimeState'] as const

export const useSystemState = () => {
  const { data: sysproxy, refetch: mutateSysproxy } = useQuery({
    queryKey: ['getSystemProxy'],
    queryFn: getSystemProxy,
  })

  const { data: runstate, refetch: mutateRunstate } = useQuery({
    queryKey: runStateQueryKey,
    queryFn: getRuntimeState,
  })

  return {
    sysproxy,
    runstate,
    mutateSysproxy,
    mutateRunstate,
  }
}
