import { ref } from 'vue'

export function useLockFn<T extends (...args: any[]) => Promise<any>>(fn: T): T {
  const lockRef = ref(false)

  return ((...args: any[]) => {
    if (lockRef.value) return Promise.resolve() as any
    lockRef.value = true
    return fn(...args).finally(() => {
      lockRef.value = false
    })
  }) as T
}
