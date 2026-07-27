import { ref, onMounted, onUnmounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

export const useWindowDecorations = () => {
  const decorated = ref(true)

  onMounted(async () => {
    try {
      const win = getCurrentWindow()
      decorated.value = await win.isDecorated()
    } catch {
      decorated.value = true
    }
  })

  return { decorated }
}

export const useWindowControls = () => {
  const currentWindow = getCurrentWindow()
  const maximized = ref(false)

  onMounted(async () => {
    try {
      maximized.value = await currentWindow.isMaximized()
    } catch {}
  })

  const minimize = () => { currentWindow.minimize() }
  const close = () => { currentWindow.close() }
  const toggleFullscreen = async () => {
    const full = await currentWindow.isFullscreen()
    currentWindow.setFullscreen(!full)
  }
  const toggleMaximize = async () => {
    const max = await currentWindow.isMaximized()
    if (max) { currentWindow.unmaximize() } else { currentWindow.maximize() }
  }

  return { currentWindow, maximized, minimize, close, toggleFullscreen, toggleMaximize }
}
