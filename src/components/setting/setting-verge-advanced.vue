<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'

import { DialogRef, TooltipIcon } from '@/components/base'
import { updateLastCheckTime } from '@/hooks/use-update'
import {
  exitApp,
  exportDiagnosticInfo,
  openAppDir,
  openCoreDir,
  openDevTools,
  openLogsDir,
} from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { checkUpdateSafe as checkUpdate } from '@/services/update'
import { version } from '@root/package.json'

import BackupViewer from './mods/backup-viewer.vue'
import ConfigViewer from './mods/config-viewer.vue'
import HotkeyViewer from './mods/hotkey-viewer.vue'
import LayoutViewer from './mods/layout-viewer.vue'
import LiteModeViewer from './mods/lite-mode-viewer.vue'
import MiscViewer from './mods/misc-viewer.vue'
import { SettingItem, SettingList } from './mods/setting-comp'
import ThemeViewer from './mods/theme-viewer.vue'
import UpdateViewer from './mods/update-viewer.vue'

defineProps<{
  onError?: (err: Error) => void
}>()

const configRef = ref<DialogRef>()
const hotkeyRef = ref<DialogRef>()
const miscRef = ref<DialogRef>()
const themeRef = ref<DialogRef>()
const layoutRef = ref<DialogRef>()
const updateRef = ref<DialogRef>()
const backupRef = ref<DialogRef>()
const liteModeRef = ref<DialogRef>()

const onCheckUpdate = async () => {
  try {
    const info = await checkUpdate()
    updateLastCheckTime()
    if (!info?.available) {
      showNotice.success(
        'settings.components.verge.advanced.notifications.latestVersion',
      )
    } else {
      updateRef.value?.open()
    }
  } catch (err: any) {
    showNotice.error(err)
  }
}

const onExportDiagnosticInfo = async () => {
  await exportDiagnosticInfo()
  showNotice.success('shared.feedback.notifications.common.copySuccess', 1000)
}

const copyVersion = () => {
  navigator.clipboard.writeText(`v${version}`).then(() => {
    showNotice.success(
      'settings.components.verge.advanced.notifications.versionCopied',
      1000,
    )
  })
}
</script>

<template>
  <SettingList :title="i18n.t('settings.components.verge.advanced.title')">
    <ThemeViewer ref="themeRef" />
    <ConfigViewer ref="configRef" />
    <HotkeyViewer ref="hotkeyRef" />
    <MiscViewer ref="miscRef" />
    <LayoutViewer ref="layoutRef" />
    <UpdateViewer ref="updateRef" />
    <BackupViewer ref="backupRef" />
    <LiteModeViewer ref="liteModeRef" />

    <SettingItem
      @click="backupRef?.open()"
      :label="i18n.t('settings.components.verge.advanced.fields.backupSetting')"
    >
      <template #extra>
        <TooltipIcon
          :title="i18n.t('settings.components.verge.advanced.tooltips.backupInfo')"
        />
      </template>
    </SettingItem>

    <SettingItem
      @click="configRef?.open()"
      :label="i18n.t('settings.components.verge.advanced.fields.runtimeConfig')"
    />

    <SettingItem
      @click="openAppDir"
      :label="i18n.t('settings.components.verge.advanced.fields.openConfDir')"
    >
      <template #extra>
        <TooltipIcon
          :title="i18n.t('settings.components.verge.advanced.tooltips.openConfDir')"
        />
      </template>
    </SettingItem>

    <SettingItem
      @click="openCoreDir"
      :label="i18n.t('settings.components.verge.advanced.fields.openCoreDir')"
    />

    <SettingItem
      @click="openLogsDir"
      :label="i18n.t('settings.components.verge.advanced.fields.openLogsDir')"
    />

    <SettingItem
      @click="onCheckUpdate"
      :label="i18n.t('settings.components.verge.advanced.fields.checkUpdates')"
    />

    <SettingItem
      @click="openDevTools"
      :label="i18n.t('settings.components.verge.advanced.fields.openDevTools')"
    />

    <SettingItem
      :label="i18n.t('settings.components.verge.advanced.fields.liteModeSettings')"
      @click="liteModeRef?.open()"
    >
      <template #extra>
        <TooltipIcon
          :title="i18n.t('settings.components.verge.advanced.tooltips.liteMode')"
        />
      </template>
    </SettingItem>

    <SettingItem
      @click="exitApp()"
      :label="i18n.t('settings.components.verge.advanced.fields.exit')"
    />

    <SettingItem
      :label="i18n.t('settings.components.verge.advanced.fields.exportDiagnostics')"
    >
      <template #extra>
        <TooltipIcon @click="onExportDiagnosticInfo" />
      </template>
    </SettingItem>

    <SettingItem
      :label="i18n.t('settings.components.verge.advanced.fields.vergeVersion')"
    >
      <template #extra>
        <TooltipIcon
          :title="i18n.t('settings.components.verge.advanced.actions.copyVersion')"
          @click="copyVersion"
        />
      </template>
      <span style="padding: 7px 8px 7px 0;">v{{ version }}</span>
    </SettingItem>
  </SettingList>
</template>
