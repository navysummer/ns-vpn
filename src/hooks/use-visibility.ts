import { TauriEvent } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { ref, onMounted, onUnmounted } from 'vue'

const isDocumentVisible = () =>
  typeof document === 'undefined' || document.visibilityState === 'visible'

export const useVisibility = () => {
  const visible = ref(isDocumentVisible())

  onMounted(() => {
    let mounted = true
    let visibilityTimer: ReturnType<typeof setTimeout> | null = null
    const appWindow = getCurrentWindow()

    const updateVisible = async () => {
      const windowVisible = await appWindow.isVisible().catch(() => true)
      if (mounted) {
        visible.value = isDocumentVisible() && windowVisible
      }
    }

    const updateVisibleSoon = () => {
      if (visibilityTimer) {
        window.clearTimeout(visibilityTimer)
      }
      visibilityTimer = window.setTimeout(() => {
        visibilityTimer = null
        updateVisible()
      }, 50)
    }

    const handleVisibleEvent = () => {
      updateVisible()
    }

    const handlePointerDown = () => { visible.value = true }

    document.addEventListener('focus', handleVisibleEvent)
    document.addEventListener('pointerdown', handlePointerDown)
    document.addEventListener('visibilitychange', handleVisibleEvent)
    window.addEventListener('focus', handleVisibleEvent)

    const unlistenFocusChanged = appWindow.onFocusChanged(updateVisibleSoon)
    const unlistenCloseRequested = appWindow.listen(
      TauriEvent.WINDOW_CLOSE_REQUESTED,
      () => {
        visible.value = false
        updateVisibleSoon()
      },
    )
    updateVisible()

    onUnmounted(() => {
      mounted = false
      if (visibilityTimer) {
        window.clearTimeout(visibilityTimer)
        visibilityTimer = null
      }
      document.removeEventListener('focus', handleVisibleEvent)
      document.removeEventListener('pointerdown', handlePointerDown)
      document.removeEventListener('visibilitychange', handleVisibleEvent)
      window.removeEventListener('focus', handleVisibleEvent)
      unlistenFocusChanged.then((unlisten) => unlisten())
      unlistenCloseRequested.then((unlisten) => unlisten())
    })
  })

  return visible
}
