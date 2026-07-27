import './assets/styles/index.scss'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import { router } from './router'
import { initializeLanguage } from './services/i18n'
import { preloadAppData, resolveThemeMode, getPreloadConfig } from './services/preload'
import { disableWebViewShortcuts } from './utils/disable-webview-shortcuts'

import { MihomoWebSocket } from 'tauri-plugin-mihomo-api'

if (!window.ResizeObserver) {
  window.ResizeObserver = ResizeObserver
}

disableWebViewShortcuts()

const app = createApp(App)
const pinia = createPinia()
app.use(pinia)
app.use(router)

const bootstrap = async () => {
  const appDataPromise = preloadAppData()
  const { initialThemeMode } = await appDataPromise

  const store = await import('./stores/app')
  store.useAppStore().setThemeMode(initialThemeMode)

  app.mount('#root')
}

bootstrap().catch((error) => {
  console.error('[main.ts] App bootstrap failed, falling back to default language:', error)
  initializeLanguage('zh')
    .catch((fallbackError) => {
      console.error('[main.ts] Fallback language initialization failed:', fallbackError)
    })
    .finally(() => {
      const mode = resolveThemeMode(getPreloadConfig())
      const store = useAppStore()
      store.setThemeMode(mode)
      app.mount('#root')
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
