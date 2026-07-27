import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAppStore = defineStore('app', () => {
  const themeMode = ref<'light' | 'dark'>('light')

  function setThemeMode(mode: 'light' | 'dark') {
    themeMode.value = mode
  }

  function toggleThemeMode() {
    themeMode.value = themeMode.value === 'light' ? 'dark' : 'light'
  }

  return {
    themeMode,
    setThemeMode,
    toggleThemeMode,
  }
})
