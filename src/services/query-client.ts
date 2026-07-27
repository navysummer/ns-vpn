import { ref, shallowRef, triggerRef, onUnmounted, type Ref, type ShallowRef } from 'vue'

type QueryKey = string | readonly unknown[]
type QueryDataUpdater<T> =
  | T
  | undefined
  | ((current: T | undefined) => T | undefined)

type QueryOptions<T> = {
  queryKey: QueryKey
  queryFn: () => Promise<T> | T
  enabled?: boolean
  initialData?: T | (() => T | undefined)
  placeholderData?: T | (() => T | undefined)
  staleTime?: number
  retry?: number | false
  retryDelay?: number | ((attempt: number) => number)
  refetchInterval?: number | false
  refetchIntervalInBackground?: boolean
  revalidateOnMount?: boolean
}

type QueryResult<T> = {
  data: Readonly<Ref<T | undefined>>
  error: Readonly<Ref<Error | undefined>>
  isLoading: Readonly<Ref<boolean>>
  isValidating: Readonly<Ref<boolean>>
  isFetching: Readonly<Ref<boolean>>
  isPending: Readonly<Ref<boolean>>
  mutate: (data?: T | Promise<T>) => Promise<T | undefined>
  refetch: () => Promise<{ data: T | undefined }>
}

const serializeQueryKey = (queryKey: QueryKey) =>
  Array.isArray(queryKey) ? queryKey.join('__') : queryKey

const queryCache = new Map<string, unknown>()

export const getCacheData = <T>(queryKey: QueryKey): T | undefined => {
  return queryCache.get(serializeQueryKey(queryKey)) as T | undefined
}

const updateCachedData = <T>(
  queryKey: QueryKey,
  updaterOrData: QueryDataUpdater<T>,
) => {
  const current = getCacheData<T>(queryKey)
  const next =
    typeof updaterOrData === 'function'
      ? (updaterOrData as (current: T | undefined) => T | undefined)(current)
      : updaterOrData
  const cacheKey = serializeQueryKey(queryKey)
  if (next === undefined) {
    queryCache.delete(cacheKey)
  } else {
    queryCache.set(cacheKey, next)
  }
  return next
}

export const setCacheData = <T>(
  queryKey: QueryKey,
  updaterOrData: QueryDataUpdater<T>,
) => {
  return updateCachedData(queryKey, updaterOrData)
}

export const setCacheDataAsync = async <T>(
  queryKey: QueryKey,
  updaterOrData: QueryDataUpdater<T>,
) => {
  return updateCachedData(queryKey, updaterOrData)
}

export const revalidateQuery = async <T>(queryKey: QueryKey) => {
  const cacheKey = serializeQueryKey(queryKey)
  const data = queryCache.get(cacheKey) as T | undefined
  return data
}

export const revalidateQueries = (queryKeys: readonly QueryKey[]) =>
  queryKeys.map(revalidateQuery)

export const removeCacheData = (queryKey: QueryKey) => {
  const cacheKey = serializeQueryKey(queryKey)
  queryCache.delete(cacheKey)
}

export const fetchCacheData = async <T>(
  queryKey: QueryKey,
  queryFn: () => Promise<T> | T,
) => {
  const data = await queryFn()
  setCacheData(queryKey, data)
  return data
}

export function useQuery<T>(options: QueryOptions<T>): QueryResult<T> {
  const {
    queryKey,
    queryFn,
    enabled = true,
    initialData,
    staleTime = 0,
    retry = 3,
    retryDelay = 5000,
    refetchInterval,
  } = options

  const data = shallowRef<T | undefined>(
    initialData
      ? typeof initialData === 'function'
        ? (initialData as () => T | undefined)()
        : initialData
      : undefined,
  ) as ShallowRef<T | undefined>

  const error = ref<Error | undefined>(undefined)
  const isLoading = ref(true)
  const isValidating = ref(false)
  const isFetching = ref(false)
  const isPending = ref(true)

  const retryDelayFn =
    typeof retryDelay === 'function' ? retryDelay : () => retryDelay

  let retryCount = 0
  let timer: ReturnType<typeof setTimeout> | null = null
  let intervalTimer: ReturnType<typeof setInterval> | null = null

  const fetch = async (): Promise<T | undefined> => {
    if (!enabled) return undefined

    isValidating.value = true
    isFetching.value = true

    try {
      const result = await queryFn()
      data.value = result
      error.value = undefined
      retryCount = 0
      queryCache.set(serializeQueryKey(queryKey), result)
      return result
    } catch (err) {
      const e = err instanceof Error ? err : new Error(String(err))
      error.value = e

      if (retry !== false && retryCount < (retry || 0)) {
        retryCount++
        const delay = retryDelayFn(retryCount - 1)
        await new Promise((resolve) => {
          timer = setTimeout(resolve, delay)
        })
        return fetch()
      }

      return undefined
    } finally {
      isValidating.value = false
      isFetching.value = false
      isLoading.value = false
      isPending.value = false
    }
  }

  const refetch = async (): Promise<{ data: T | undefined }> => {
    const result = await fetch()
    return { data: result }
  }

  const mutate = async (newData?: T | Promise<T>): Promise<T | undefined> => {
    if (newData !== undefined) {
      const resolved = await Promise.resolve(newData)
      data.value = resolved
      queryCache.set(serializeQueryKey(queryKey), resolved)
    }
    return data.value
  }

  if (enabled) {
    fetch()
  }

  if (refetchInterval && refetchInterval > 0) {
    intervalTimer = setInterval(() => {
      fetch()
    }, refetchInterval)
  }

  onUnmounted(() => {
    if (timer) clearTimeout(timer)
    if (intervalTimer) clearInterval(intervalTimer)
  })

  return {
    data: data as Readonly<Ref<T | undefined>>,
    error: error as Readonly<Ref<Error | undefined>>,
    isLoading: isLoading as Readonly<Ref<boolean>>,
    isValidating: isValidating as Readonly<Ref<boolean>>,
    isFetching: isFetching as Readonly<Ref<boolean>>,
    isPending: isPending as Readonly<Ref<boolean>>,
    mutate,
    refetch,
  }
}
