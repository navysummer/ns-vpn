import './assets/styles/index.scss'

import { createApp, defineComponent, h } from 'vue'
import { createPinia } from 'pinia'
import { RouterView } from 'vue-router'
import { router } from './pages/_routers'
import { useWindowProvider } from './providers/window/window-context'
import { preloadAppData, resolveThemeMode, getPreloadConfig } from './services/preload'
import { FALLBACK_LANGUAGE, initializeLanguage } from './services/i18n'
import { useSetThemeMode } from './services/states'
import { MihomoWebSocket } from 'tauri-plugin-mihomo-api'
import { disableWebViewShortcuts } from './utils/disable-webview-shortcuts'
import BaseErrorBoundary from './components/base/base-error-boundary.vue'

const mainElementId = 'root'
const container = document.getElementById(mainElementId)

if (!container) {
  throw new Error(`No container '${mainElementId}' found to render application`)
}

disableWebViewShortcuts()

const App = defineComponent({
  setup() {
    useWindowProvider()
    return () => h(BaseErrorBoundary, null, { default: () => h(RouterView) })
  },
})

let appPromise: Promise<void> | null = null

const initializeApp = async () => {
  if (appPromise) return appPromise
  appPromise = (async () => {
    const pinia = createPinia()
    const app = createApp(App)
    app.use(pinia)
    app.use(router)
    app.mount(`#${mainElementId}`)
  })()
  return appPromise
}

const bootstrap = async () => {
  const appDataPromise = preloadAppData()

  const { initialThemeMode } = await appDataPromise
  useSetThemeMode()(initialThemeMode)
  await initializeApp()
}

bootstrap().catch((error) => {
  console.error('[main.ts] App bootstrap failed, falling back to default language:', error)
  initializeLanguage(FALLBACK_LANGUAGE)
    .catch((fallbackError) => {
      console.error('[main.ts] Fallback language initialization failed:', fallbackError)
    })
    .finally(() => {
      useSetThemeMode()(resolveThemeMode(getPreloadConfig()))
      initializeApp().catch(console.error)
    })
})

window.addEventListener('error', (event) => {
  console.error('[main.ts] Global error:', event.error)
})

window.addEventListener('unhandledrejection', (event) => {
  console.error('[main.ts] Unhandled promise rejection:', event.reason)
})

window.addEventListener('beforeunload', () => {
  MihomoWebSocket.cleanupAll()
})

window.addEventListener('DOMContentLoaded', () => {
  MihomoWebSocket.cleanupAll()
})
