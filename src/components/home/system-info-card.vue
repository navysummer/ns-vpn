<template>
  <EnhancedCard
    :title="t('home.components.systemInfo.title')"
    iconColor="error"
  >
    <template #icon>
      <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
    </template>
    <template #action>
      <n-button quaternary circle size="small" @click="goToSettings">
        <template #icon>
          <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.07.62-.07.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>
        </template>
      </n-button>
    </template>
    <div v-if="!verge" />
    <div v-else :style="{ display: 'flex', flexDirection: 'column', gap: '12px' }">
      <div :style="{ display: 'flex', justifyContent: 'space-between' }">
        <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">{{ t('home.components.systemInfo.fields.osInfo') }}</span>
        <span :style="{ fontSize: '14px', fontWeight: 500 }">{{ osInfo }}</span>
      </div>
      <hr :style="{ border: 'none', borderTop: '1px solid var(--border-color)', margin: 0 }" />
      <div :style="{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }">
        <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">{{ t('home.components.systemInfo.fields.autoLaunch') }}</span>
        <n-tag
          size="small"
          :color="autoLaunchEnabled ? 'success' : 'default'"
          :variant="autoLaunchEnabled ? 'filled' : 'outline'"
          :style="{ cursor: 'pointer' }"
          @click="toggleAutoLaunch"
        >
          {{ autoLaunchEnabled ? t('shared.statuses.enabled') : t('shared.statuses.disabled') }}
        </n-tag>
      </div>
      <hr :style="{ border: 'none', borderTop: '1px solid var(--border-color)', margin: 0 }" />
      <div :style="{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }">
        <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">{{ t('home.components.systemInfo.fields.runningMode') }}</span>
        <span
          :style="{
            ...runningModeStyle,
            fontWeight: 500,
            display: 'flex',
            alignItems: 'center',
            gap: '4px',
            fontSize: '14px',
            cursor: (isSidecarMode || (isAdminMode && isSidecarMode)) ? 'pointer' : 'default',
            textDecoration: (isSidecarMode || (isAdminMode && isSidecarMode)) ? 'underline' : 'none',
          }"
          @click="handleRunningModeClick"
        >
          <span v-html="modeIcon" />
          {{ modeText }}
        </span>
      </div>
      <hr :style="{ border: 'none', borderTop: '1px solid var(--border-color)', margin: 0 }" />
      <div :style="{ display: 'flex', justifyContent: 'space-between' }">
        <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">{{ t('home.components.systemInfo.fields.lastCheckUpdate') }}</span>
        <span
          :style="{ cursor: 'pointer', textDecoration: 'underline', fontWeight: 500, fontSize: '14px' }"
          @click="onCheckUpdate"
        >
          {{ lastCheckUpdateText }}
        </span>
      </div>
      <hr :style="{ border: 'none', borderTop: '1px solid var(--border-color)', margin: 0 }" />
      <div :style="{ display: 'flex', justifyContent: 'space-between' }">
        <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">{{ t('home.components.systemInfo.fields.vergeVersion') }}</span>
        <span :style="{ fontSize: '14px', fontWeight: 500 }">v{{ appVersion }}</span>
      </div>
    </div>
  </EnhancedCard>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { NButton, NTag } from 'naive-ui'

import EnhancedCard from './enhanced-card.vue'
import { useServiceInstaller } from '@/hooks/use-service-installer'
import { useSystemState } from '@/hooks/use-system-state'
import { useUpdate, updateLastCheckTime, readLastCheckTime } from '@/hooks/use-update'
import { useVerge } from '@/hooks/use-verge'
import { getSystemInfo } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { version as appVersion } from '@root/package.json'

const { t } = useI18n()
const router = useRouter()
const { verge, patchVerge } = useVerge()
const { isAdminMode, isSidecarMode, mutateSystemState } = useSystemState()
const { installServiceAndRestartCore } = useServiceInstaller()
const { checkUpdate: triggerCheckUpdate, lastCheckUpdate } = useUpdate(true)

const osInfo = ref('')

const lastCheckUpdateText = computed(() => lastCheckUpdate.value ? new Date(lastCheckUpdate.value).toLocaleString() : '-')

onMounted(() => {
  getSystemInfo().then((info: any) => {
    const sysName = info.system_name
    let sysVersion = info.system_version
    if (sysName && sysVersion.toLowerCase().startsWith(sysName.toLowerCase())) {
      sysVersion = sysVersion.substring(sysName.length).trim()
    }
    osInfo.value = `${sysName} ${sysVersion}`
  }).catch(console.error)
})

if (verge.value?.auto_check_update && readLastCheckTime() === null) {
  updateLastCheckTime()
  setTimeout(() => { triggerCheckUpdate().catch(console.error) }, 5000)
}

const goToSettings = () => router.push('/settings')

const toggleAutoLaunch = async () => {
  if (!verge.value) return
  try { await patchVerge({ enable_auto_launch: !verge.value.enable_auto_launch }) }
  catch (err) { console.error('切换开机自启动状态失败:', err) }
}

const handleRunningModeClick = async () => {
  if (isSidecarMode.value || (isAdminMode.value && isSidecarMode.value)) {
    await installServiceAndRestartCore()
    await mutateSystemState()
  }
}

let checkingUpdate = false
const onCheckUpdate = async () => {
  if (checkingUpdate) return
  checkingUpdate = true
  try {
    const result = await triggerCheckUpdate()
    const info = result.data
    if (!info?.available) {
      showNotice.success('settings.components.verge.advanced.notifications.latestVersion')
    } else {
      showNotice.info('shared.feedback.notifications.updateAvailable', 2000)
      goToSettings()
    }
  } catch (err: any) {
    showNotice.error(err)
  } finally {
    checkingUpdate = false
  }
}

const autoLaunchEnabled = computed(() => verge.value?.enable_auto_launch || false)
const runningModeStyle = computed(() => ({
  cursor: (isSidecarMode.value || (isAdminMode.value && isSidecarMode.value)) ? 'pointer' : 'default',
  textDecoration: (isSidecarMode.value || (isAdminMode.value && isSidecarMode.value)) ? 'underline' : 'none',
}))

const modeIcon = computed(() => {
  if (isAdminMode.value) {
    if (!isSidecarMode.value) {
      return '<svg viewBox="0 0 24 24" width="16" height="16" fill="var(--primary-color)" style="display:inline-block"><path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z"/></svg><svg viewBox="0 0 24 24" width="16" height="16" fill="var(--success-color)" style="display:inline-block;margin-left:4px"><path d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zM8 14H6v-2h2v2zm0-4H6V8h2v2zm8 4h-2v-2h2v2zm0-4h-2V8h2v2zm4 4h-2v-2h2v2zm0-4h-2V8h2v2z"/></svg>'
    }
    return '<svg viewBox="0 0 24 24" width="16" height="16" fill="var(--primary-color)" style="display:inline-block"><path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z"/></svg>'
  }
  if (isSidecarMode.value) {
    return '<svg viewBox="0 0 24 24" width="16" height="16" fill="var(--info-color)" style="display:inline-block"><path d="M20 2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14l4 4V4c0-1.1-.9-2-2-2zm-2 10H6v-2h12v2z"/></svg>'
  }
  return '<svg viewBox="0 0 24 24" width="16" height="16" fill="var(--success-color)" style="display:inline-block"><path d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zM8 14H6v-2h2v2zm0-4H6V8h2v2zm8 4h-2v-2h2v2zm0-4h-2V8h2v2zm4 4h-2v-2h2v2zm0-4h-2V8h2v2z"/></svg>'
})

const modeText = computed(() => {
  if (isAdminMode.value) {
    return !isSidecarMode.value ? t('home.components.systemInfo.badges.adminServiceMode') : t('home.components.systemInfo.badges.adminMode')
  }
  return isSidecarMode.value ? t('home.components.systemInfo.badges.sidecarMode') : t('home.components.systemInfo.badges.serviceMode')
})
</script>
