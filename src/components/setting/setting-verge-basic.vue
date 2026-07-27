<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'
import { open } from '@tauri-apps/plugin-dialog'

import { DialogRef, TooltipIcon } from '@/components/base'
import { useVerge } from '@/hooks/use-verge'
import { navigationItems } from '@/pages/_navigation-meta'
import { copyClashEnv } from '@/services/cmds'
import { supportedLanguages } from '@/services/i18n'
import { showNotice } from '@/services/notice-service'
import getSystem from '@/utils/get-system'

import BackupViewer from './mods/backup-viewer.vue'
import ConfigViewer from './mods/config-viewer.vue'
import GuardState from './mods/guard-state.vue'
import HotkeyViewer from './mods/hotkey-viewer.vue'
import LayoutViewer from './mods/layout-viewer.vue'
import MiscViewer from './mods/misc-viewer.vue'
import { SettingItem, SettingList } from './mods/setting-comp'
import ThemeModeSwitch from './mods/theme-mode-switch.vue'
import ThemeViewer from './mods/theme-viewer.vue'
import UpdateViewer from './mods/update-viewer.vue'

const OS = getSystem()

const languageOptions = supportedLanguages.map((code: string) => {
  const labels: { [key: string]: string } = {
    en: 'English',
    ru: 'Русский',
    zh: '简体中文',
    fa: 'فارسی',
    tt: 'Татар',
    id: 'Bahasa Indonesia',
    ar: 'العربية',
    ko: '한국어',
    tr: 'Türkçe',
    de: 'Deutsch',
    es: 'Español',
    jp: '日本語',
    zhtw: '繁體中文',
  }
  const label = labels[code] || code
  return { code, label }
})

defineProps<{
  onError?: (err: Error) => void
}>()

const { verge, patchVerge, mutateVerge } = useVerge()
const {
  theme_mode,
  language,
  tray_event,
  env_type,
  startup_script,
  start_page,
} = verge ?? {}
const configRef = ref<DialogRef>()
const hotkeyRef = ref<DialogRef>()
const miscRef = ref<DialogRef>()
const themeRef = ref<DialogRef>()
const layoutRef = ref<DialogRef>()
const updateRef = ref<DialogRef>()
const backupRef = ref<DialogRef>()

const onChangeData = (patch: any) => {
  mutateVerge({ ...verge, ...patch }, false)
}

const onCopyClashEnv = async () => {
  await copyClashEnv()
  showNotice.success('shared.feedback.notifications.common.copySuccess', 1000)
}
</script>

<template>
  <SettingList :title="i18n.t('settings.components.verge.basic.title')">
    <ThemeViewer ref="themeRef" />
    <ConfigViewer ref="configRef" />
    <HotkeyViewer ref="hotkeyRef" />
    <MiscViewer ref="miscRef" />
    <LayoutViewer ref="layoutRef" />
    <UpdateViewer ref="updateRef" />
    <BackupViewer ref="backupRef" />

    <SettingItem :label="i18n.t('settings.components.verge.basic.fields.language')">
      <GuardState
        :value="language ?? 'en'"
        :onCatch="onError"
        :onFormat="(e: any) => e.target.value"
        @change="(e) => onChangeData({ language: e })"
        @guard="(e) => patchVerge({ language: e })"
      >
        <select class="MuiSelect-root MuiSelect-sizeSmall" style="width: 110px; padding: 7.5px 0;">
          <option v-for="{ code, label } in languageOptions" :key="code" :value="code">{{ label }}</option>
        </select>
      </GuardState>
    </SettingItem>

    <SettingItem
      :label="i18n.t('settings.components.verge.basic.fields.themeMode')"
    >
      <GuardState
        :value="theme_mode"
        :onCatch="onError"
        @change="(e) => onChangeData({ theme_mode: e })"
        @guard="(e) => patchVerge({ theme_mode: e })"
      >
        <ThemeModeSwitch />
      </GuardState>
    </SettingItem>

    <SettingItem v-if="OS !== 'linux'"
      :label="i18n.t('settings.components.verge.basic.fields.trayClickEvent')"
    >
      <GuardState
        :value="tray_event ?? 'main_window'"
        :onCatch="onError"
        :onFormat="(e: any) => e.target.value"
        @change="(e) => onChangeData({ tray_event: e })"
        @guard="(e) => patchVerge({ tray_event: e })"
      >
        <select class="MuiSelect-root MuiSelect-sizeSmall" style="width: 140px; padding: 7.5px 0;">
          <option value="main_window">{{ i18n.t('settings.components.verge.basic.trayOptions.showMainWindow') }}</option>
          <option value="tray_menu">{{ i18n.t('settings.components.verge.basic.trayOptions.showTrayMenu') }}</option>
          <option value="system_proxy">{{ i18n.t('settings.sections.system.toggles.systemProxy') }}</option>
          <option value="tun_mode">{{ i18n.t('settings.sections.system.toggles.tunMode') }}</option>
          <option value="disable">{{ i18n.t('settings.components.verge.basic.trayOptions.disable') }}</option>
        </select>
      </GuardState>
    </SettingItem>

    <SettingItem
      :label="i18n.t('settings.components.verge.basic.fields.copyEnvType')"
    >
      <template #extra>
        <TooltipIcon @click="onCopyClashEnv" />
      </template>
      <GuardState
        :value="env_type ?? (OS === 'windows' ? 'powershell' : 'bash')"
        :onCatch="onError"
        :onFormat="(e: any) => e.target.value"
        @change="(e) => onChangeData({ env_type: e })"
        @guard="(e) => patchVerge({ env_type: e })"
      >
        <select class="MuiSelect-root MuiSelect-sizeSmall" style="width: 140px; padding: 7.5px 0;">
          <option value="bash">Bash</option>
          <option value="fish">Fish</option>
          <option value="nushell">Nushell</option>
          <option value="cmd">CMD</option>
          <option value="powershell">PowerShell</option>
        </select>
      </GuardState>
    </SettingItem>

    <SettingItem
      :label="i18n.t('settings.components.verge.basic.fields.startPage')"
    >
      <GuardState
        :value="start_page ?? '/'"
        :onCatch="onError"
        :onFormat="(e: any) => e.target.value"
        @change="(e) => onChangeData({ start_page: e })"
        @guard="(e) => patchVerge({ start_page: e })"
      >
        <select class="MuiSelect-root MuiSelect-sizeSmall" style="width: 140px; padding: 7.5px 0;">
          <option v-for="page in Object.values(navigationItems)" :key="page.path" :value="page.path">
            {{ i18n.t(page.label) }}
          </option>
        </select>
      </GuardState>
    </SettingItem>

    <SettingItem
      :label="i18n.t('settings.components.verge.basic.fields.startupScript')"
    >
      <GuardState
        :value="startup_script ?? ''"
        :onCatch="onError"
        :onFormat="(e: any) => e.target.value"
        @change="(e) => onChangeData({ startup_script: e })"
        @guard="(e) => patchVerge({ startup_script: e })"
      >
        <div style="display: flex; align-items: center; width: 230px;">
          <input
            :value="startup_script"
            disabled
            style="flex: 1; border: none; background: transparent; padding: 4px 0;"
          />
          <button class="MuiButton-root" @click="async () => {
            const selected = await open({
              directory: false,
              multiple: false,
              filters: [{ name: 'Shell Script', extensions: ['sh', 'bat', 'ps1'] }],
            })
            if (selected) {
              onChangeData({ startup_script: `${selected}` })
              patchVerge({ startup_script: `${selected}` })
            }
          }">
            {{ i18n.t('settings.components.verge.basic.actions.browse') }}
          </button>
          <button v-if="startup_script" class="MuiButton-root" @click="async () => {
            onChangeData({ startup_script: '' })
            patchVerge({ startup_script: '' })
          }">
            {{ i18n.t('shared.actions.clear') }}
          </button>
        </div>
      </GuardState>
    </SettingItem>

    <SettingItem
      @click="themeRef?.open()"
      :label="i18n.t('settings.components.verge.basic.fields.themeSetting')"
    />

    <SettingItem
      @click="layoutRef?.open()"
      :label="i18n.t('settings.components.verge.basic.fields.layoutSetting')"
    />

    <SettingItem
      @click="miscRef?.open()"
      :label="i18n.t('settings.components.verge.basic.fields.misc')"
    />

    <SettingItem
      @click="hotkeyRef?.open()"
      :label="i18n.t('settings.components.verge.basic.fields.hotkeySetting')"
    />
  </SettingList>
</template>
