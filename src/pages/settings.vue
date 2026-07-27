<script setup lang="ts">
import i18n from 'i18next'

import { BasePage } from '@/components/base'
import SettingClash from '@/components/setting/setting-clash.vue'
import SettingSystem from '@/components/setting/setting-system.vue'
import SettingVergeAdvanced from '@/components/setting/setting-verge-advanced.vue'
import SettingVergeBasic from '@/components/setting/setting-verge-basic.vue'
import { openWebUrl } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { useThemeMode } from '@/services/states'

const mode = useThemeMode()
const isDark = mode === 'light' ? false : true

const onError = (err: any) => {
  showNotice.error(err)
}

const toGithubRepo = async () => {
  await openWebUrl('https://github.com/clash-verge-rev/clash-verge-rev')
}

const toGithubDoc = async () => {
  await openWebUrl('https://clash-verge-rev.github.io/index.html')
}

const toTelegramChannel = async () => {
  await openWebUrl('https://t.me/clash_verge_re')
}
</script>

<template>
  <BasePage
    :title="i18n.t('settings.page.title')"
  >
    <template #header>
      <div class="MuiButtonGroup-root MuiButtonGroup-contained">
        <button
          class="MuiIconButton-root MuiIconButton-sizeMedium"
          :title="i18n.t('settings.page.actions.manual')"
          @click="toGithubDoc"
        >
          <svg viewBox="0 0 24 24" fill="currentColor" width="1em" height="1em"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13 15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z"/></svg>
        </button>
        <button
          class="MuiIconButton-root MuiIconButton-sizeMedium"
          :title="i18n.t('settings.page.actions.telegram')"
          @click="toTelegramChannel"
        >
          <svg viewBox="0 0 24 24" fill="currentColor" width="1em" height="1em"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm4.64 6.8c-.15 1.58-.8 5.42-1.13 7.19-.14.75-.42 1-.68 1.03-.58.05-1.02-.38-1.58-.75-.88-.58-1.38-.94-2.23-1.5-.99-.65-.35-1.01.22-1.59.15-.15 2.71-2.48 2.76-2.69.01-.03.01-.14-.07-.2-.08-.06-.19-.04-.27-.02-.12.02-1.96 1.25-5.54 3.66-.52.36-1 .53-1.42.52-.47-.01-1.37-.26-2.04-.48-.82-.27-1.47-.42-1.41-.88.03-.24.36-.49.99-.74 3.92-1.71 6.54-2.84 7.85-3.37 3.74-1.52 4.52-1.78 5.03-1.79.11 0 .37.03.53.17.14.12.18.29.2.42-.02.11.01.44 0 .68z"/></svg>
        </button>
        <button
          class="MuiIconButton-root MuiIconButton-sizeMedium"
          :title="i18n.t('settings.page.actions.github')"
          @click="toGithubRepo"
        >
          <svg viewBox="0 0 24 24" fill="currentColor" width="1em" height="1em"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/></svg>
        </button>
      </div>
    </template>

    <div class="MuiGrid-root MuiGrid-container" style="gap: 12px; display: flex; flex-wrap: wrap;">
      <div class="MuiGrid-root MuiGrid-item" style="flex: 1; min-width: calc(50% - 12px);">
        <div
          class="setting-card"
          :style="{
            borderRadius: '16px',
            marginBottom: '12px',
            backgroundColor: isDark ? '#282a36' : '#ffffff',
          }"
        >
          <SettingSystem :onError="onError" />
        </div>
        <div
          class="setting-card"
          :style="{
            borderRadius: '16px',
            backgroundColor: isDark ? '#282a36' : '#ffffff',
          }"
        >
          <SettingClash :onError="onError" />
        </div>
      </div>
      <div class="MuiGrid-root MuiGrid-item" style="flex: 1; min-width: calc(50% - 12px);">
        <div
          class="setting-card"
          :style="{
            borderRadius: '16px',
            marginBottom: '12px',
            backgroundColor: isDark ? '#282a36' : '#ffffff',
          }"
        >
          <SettingVergeBasic :onError="onError" />
        </div>
        <div
          class="setting-card"
          :style="{
            borderRadius: '16px',
            backgroundColor: isDark ? '#282a36' : '#ffffff',
          }"
        >
          <SettingVergeAdvanced :onError="onError" />
        </div>
      </div>
    </div>
  </BasePage>
</template>
