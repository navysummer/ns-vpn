import { ref } from 'vue'

type IConnectionSetting = { layout: 'table' | 'card' }

const defaultConnectionSetting: IConnectionSetting = { layout: 'table' }

const STORAGE_KEY = 'connections-setting'

function loadSetting(): IConnectionSetting {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (raw) return JSON.parse(raw) as IConnectionSetting
  } catch {}
  return defaultConnectionSetting
}

function saveSetting(value: IConnectionSetting) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(value))
}

const globalSetting = ref<IConnectionSetting>(loadSetting())

export const useConnectionSetting = () => {
  return globalSetting
}
