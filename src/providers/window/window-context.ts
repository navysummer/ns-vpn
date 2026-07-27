import { ref, provide, inject, onMounted } from 'vue'

const WindowKey = Symbol('window')

export function useWindowProvider() {
  const isMaximized = ref(false)
  const isFullscreen = ref(false)

  provide(WindowKey, { isMaximized, isFullscreen })
}

export function useWindowContext() {
  const context = inject(WindowKey)
  if (!context) {
    return { isMaximized: ref(false), isFullscreen: ref(false) }
  }
  return context as { isMaximized: typeof ref<boolean>; isFullscreen: typeof ref<boolean> }
}
