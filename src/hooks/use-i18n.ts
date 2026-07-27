import { ref } from 'vue'
import i18n from 'i18next'

import {
  changeLanguage,
  resolveLanguage,
  supportedLanguages,
} from '@/services/i18n'

export const useI18n = () => {
  const isLoading = ref(false)

  const switchLanguage = async (language: string) => {
    const targetLanguage = resolveLanguage(language)

    if (!supportedLanguages.includes(targetLanguage)) {
      console.warn(`Unsupported language: ${language}`)
      return
    }

    if (i18n.language === targetLanguage) {
      return
    }

    isLoading.value = true
    try {
      await changeLanguage(targetLanguage)
    } catch (error) {
      console.error('Failed to change language:', error)
    } finally {
      isLoading.value = false
    }
  }

  return {
    currentLanguage: i18n.language,
    supportedLanguages,
    switchLanguage,
    isLoading,
    t: i18n.t.bind(i18n),
  }
}
