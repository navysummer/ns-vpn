import { ref } from 'vue'

export function useLockFn<P extends any[], R = void>(
  fn: (...args: P) => Promise<R>,
): (...args: P) => Promise<R> {
  const locked = ref(false)

  return async (...args: P): Promise<R> => {
    if (locked.value) return undefined as unknown as R
    locked.value = true
    try {
      return await fn(...args)
    } finally {
      locked.value = false
    }
  }
}
