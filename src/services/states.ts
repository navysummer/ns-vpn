import { ref } from 'vue'

const themeMode = ref<'light' | 'dark'>('light')
const loadingCache = ref<Set<string>>(new Set())
const updateState = ref(false)

export function useThemeMode() {
  return themeMode
}

export function useSetThemeMode() {
  return (mode: 'light' | 'dark') => {
    themeMode.value = mode
  }
}

export function useLoadingCache() {
  return loadingCache
}

export function useSetLoadingCache() {
  return (cache: Set<string>) => {
    loadingCache.value = cache
  }
}

export function useUpdateState() {
  return updateState
}

export function useSetUpdateState() {
  return (state: boolean) => {
    updateState.value = state
  }
}

export const ThemeModeProvider = {
  install: () => {},
}

export const LoadingCacheProvider = {
  install: () => {},
}

export const UpdateStateProvider = {
  install: () => {},
}
