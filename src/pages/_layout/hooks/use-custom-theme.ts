import { computed, watchEffect, ref } from 'vue'
import { useVerge } from '@/hooks/use-verge'
import { defaultDarkTheme, defaultTheme } from '@/pages/_theme'
import { useThemeMode, useSetThemeMode } from '@/services/states'

export const useCustomTheme = () => {
  const { verge } = useVerge()
  const { theme_mode, theme_setting } = verge ?? {}
  const mode = useThemeMode()
  const setMode = useSetThemeMode()
  const userBackgroundImage = computed(() => theme_setting?.background_image || '')
  const hasUserBackground = computed(() => !!userBackgroundImage.value)

  const theme = computed(() => {
    const setting = theme_setting || {}
    const dt = mode.value === 'light' ? defaultTheme : defaultDarkTheme

    return {
      primaryColor: setting.primary_color || dt.primary_color,
      primaryText: setting.primary_text || dt.primary_text,
      secondaryText: setting.secondary_text || dt.secondary_text,
      infoColor: setting.info_color || dt.info_color,
      errorColor: setting.error_color || dt.error_color,
      warningColor: setting.warning_color || dt.warning_color,
      successColor: setting.success_color || dt.success_color,
      backgroundColor: dt.background_color,
      fontFamily: setting.font_family || dt.font_family,
    }
  })

  const naiveTheme = computed(() => {
    const t = theme.value
    return {
      common: {
        primaryColor: t.primaryColor,
        primaryColorHover: t.primaryColor,
        primaryColorPressed: t.primaryColor,
        primaryColorSuppl: t.primaryColor,
        infoColor: t.infoColor,
        successColor: t.successColor,
        warningColor: t.warningColor,
        errorColor: t.errorColor,
        textColor1: t.primaryText,
        textColor2: t.secondaryText,
        bodyColor: t.backgroundColor,
        fontFamily: t.fontFamily,
      },
    }
  })

  watchEffect(() => {
    const rootEle = document.documentElement
    if (!rootEle) return

    const t = theme.value
    const isDark = mode.value !== 'light'
    const backgroundColor = isDark ? t.backgroundColor : '#ECECEC'
    const selectColor = isDark ? '#3E3E3E' : '#f5f5f5'
    const scrollColor = isDark ? '#555555' : '#90939980'
    const dividerColor = isDark ? 'rgba(255, 255, 255, 0.06)' : 'rgba(0, 0, 0, 0.06)'

    rootEle.style.setProperty('--divider-color', dividerColor)
    rootEle.style.setProperty('--background-color', backgroundColor)
    rootEle.style.setProperty('--selection-color', selectColor)
    rootEle.style.setProperty('--scroller-color', scrollColor)
    rootEle.style.setProperty('--primary-main', t.primaryColor)
    rootEle.style.setProperty('--primary-text', t.primaryText)
    rootEle.style.setProperty('--secondary-text', t.secondaryText)
    rootEle.style.setProperty('--info-color', t.infoColor)
    rootEle.style.setProperty('--error-color', t.errorColor)
    rootEle.style.setProperty('--warning-color', t.warningColor)
    rootEle.style.setProperty('--success-color', t.successColor)
    rootEle.style.setProperty('--window-border-color', isDark ? '#1E1E1E' : '#cccccc')
    rootEle.style.setProperty('--scrollbar-bg', isDark ? '#2E303D' : '#f1f1f1')
    rootEle.style.setProperty('--scrollbar-thumb', isDark ? '#555555' : '#c1c1c1')
    rootEle.style.setProperty(
      '--user-background-image',
      hasUserBackground.value ? `url('${userBackgroundImage.value}')` : 'none',
    )
    rootEle.style.setProperty(
      '--background-blend-mode',
      setting.background_blend_mode || 'normal',
    )
    rootEle.style.setProperty(
      '--background-opacity',
      setting.background_opacity !== undefined ? String(setting.background_opacity) : '1',
    )

    const scrollerStyle = `
      ::-webkit-scrollbar { width: 8px; height: 8px; background-color: var(--scrollbar-bg); }
      ::-webkit-scrollbar-thumb { background-color: var(--scrollbar-thumb); border-radius: 4px; }
    `
    let styleEl = document.querySelector('style#verge-theme') as HTMLStyleElement
    if (!styleEl) {
      styleEl = document.createElement('style')
      styleEl.id = 'verge-theme'
      document.head.appendChild(styleEl)
    }
    styleEl.innerHTML = scrollerStyle
  })

  return { theme, naiveTheme }
}
