import { ref } from 'vue'

type IClashLog = {
  enable: boolean
  logLevel: string
  logFilter: string
  logOrder: string
}

const defaultClashLog: IClashLog = {
  enable: true,
  logLevel: 'INFO',
  logFilter: 'all',
  logOrder: 'asc',
}

const STORAGE_KEY = 'clash-log'

function loadSetting(): IClashLog {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (raw) return JSON.parse(raw) as IClashLog
  } catch {}
  return defaultClashLog
}

function saveSetting(value: IClashLog) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(value))
}

const globalSetting = ref<IClashLog>(loadSetting())

export const useClashLog = () => {
  const set = (value: IClashLog) => {
    globalSetting.value = value
    saveSetting(value)
  }

  return [globalSetting, set] as const
}
