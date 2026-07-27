import { computed } from 'vue'
import i18n from 'i18next'

export function useTranslation() {
  const t = (key: string, options?: any) => {
    return i18n.t(key, options)
  }

  return { t, i18n }
}
