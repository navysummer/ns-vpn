import { ref, onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

export function useWindowDecorations() {
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
