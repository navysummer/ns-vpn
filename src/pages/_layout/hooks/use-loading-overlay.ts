import { ref, watch } from 'vue'
import { hideInitialOverlay } from '../utils/initial-loading-overlay'

export const useLoadingOverlay = (themeReady: boolean | { value: boolean }) => {
  const done = ref(false)
  const themeReadyVal = typeof themeReady === 'boolean' ? themeReady : themeReady.value

  watch(themeReady, (ready) => {
    if (!ready || done.value) return
    done.value = true
    const timer = hideInitialOverlay()
    if (timer !== undefined) {
      // Timer cleanup is handled inside hideInitialOverlay
    }
  }, { immediate: true })
}
