<template>
  <div :style="{ width: '100%', paddingRight: noRightPadding ? '4px' : '16px' }">
    <div
      v-if="isSystemProxyMode"
      :style="{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        padding: '8px',
        paddingRight: '16px',
        borderRadius: '12px',
        backgroundColor: systemProxyIndicator ? 'rgba(76, 175, 80, 0.07)' : 'transparent',
        opacity: 1,
        transition: 'background-color 0.3s',
      }"
    >
      <div style="display: flex; align-items: center;">
        <svg v-if="systemProxyIndicator" viewBox="0 0 24 24" width="24" height="24" fill="#4caf50" style="margin-right: 8px;"><path d="M10 16.5l6-4.5-6-4.5v9zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"/></svg>
        <svg v-else viewBox="0 0 24 24" width="24" height="24" fill="rgba(0,0,0,0.38)" style="margin-right: 8px;"><path d="M10 16.5l6-4.5-6-4.5v9zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"/></svg>
        <span style="font-weight: 500; font-size: 15px;">{{ t('settings.sections.proxyControl.fields.systemProxy') }}</span>
        <button
          class="MuiIconButton-root MuiIconButton-sizeSmall"
          style="margin-left: 8px;"
          :title="t('settings.sections.proxyControl.tooltips.systemProxy')"
          @click="sysproxyRef?.open()"
        >
          <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.07.62-.07.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>
        </button>
      </div>
      <label>
        <input
          type="checkbox"
          role="switch"
          :checked="systemProxyIndicator"
          @change="handleToggle('systemProxy', ($event.target as HTMLInputElement).checked)"
        />
      </label>
    </div>

    <div
      v-if="isTunMode"
      :style="{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        padding: '8px',
        paddingRight: '16px',
        borderRadius: '12px',
        backgroundColor: enableTunMode ? 'rgba(76, 175, 80, 0.07)' : 'transparent',
        opacity: isTunModeAvailable ? 1 : 0.6,
        transition: 'background-color 0.3s',
      }"
    >
      <div style="display: flex; align-items: center;">
        <svg v-if="enableTunMode" viewBox="0 0 24 24" width="24" height="24" fill="#4caf50" style="margin-right: 8px;"><path d="M10 16.5l6-4.5-6-4.5v9zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"/></svg>
        <svg v-else viewBox="0 0 24 24" width="24" height="24" fill="rgba(0,0,0,0.38)" style="margin-right: 8px;"><path d="M10 16.5l6-4.5-6-4.5v9zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"/></svg>
        <span style="font-weight: 500; font-size: 15px;">{{ t('settings.sections.proxyControl.fields.tunMode') }}</span>
        <button
          class="MuiIconButton-root MuiIconButton-sizeSmall"
          style="margin-left: 8px;"
          :title="t('settings.sections.proxyControl.tooltips.tunMode')"
          @click="tunRef?.open()"
        >
          <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.07.62-.07.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>
        </button>
        <template v-if="!isTunModeAvailable">
          <button
            class="MuiIconButton-root MuiIconButton-sizeSmall"
            style="margin-left: 8px; color: #ed6c02;"
            :title="t('settings.sections.proxyControl.tooltips.tunUnavailable')"
          >
            <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z"/></svg>
          </button>
          <button
            class="MuiIconButton-root MuiIconButton-sizeSmall"
            style="margin-left: 8px;"
            :title="t('settings.sections.proxyControl.actions.installService')"
            @click="onInstallService"
          >
            <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M22.7 19l-9.1-9.1c.9-2.3.4-5-1.5-6.9-2-2-5-2.4-7.4-1.3L9 6 6 9 1.6 4.7C.4 7.1.9 10.1 2.9 12.1c1.9 1.9 4.6 2.4 6.9 1.5l9.1 9.1c.4.4 1 .4 1.4 0l2.3-2.3c.5-.4.5-1.1.1-1.4z"/></svg>
          </button>
        </template>
        <button
          v-if="isServiceInstallReady"
          class="MuiIconButton-root MuiIconButton-sizeSmall"
          style="margin-left: 8px;"
          :title="t('settings.sections.proxyControl.actions.uninstallService')"
          @click="onUninstallService"
        >
          <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
        </button>
      </div>
      <label>
        <input
          type="checkbox"
          role="switch"
          :checked="enableTunMode"
          :disabled="!isTunModeAvailable"
          @change="handleToggle('tun', ($event.target as HTMLInputElement).checked)"
        />
      </label>
    </div>

    <SysproxyViewer ref="sysproxyRef" />
    <TunViewer ref="tunRef" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { DialogRef } from '@/components/base'
import SysproxyViewer from '@/components/setting/mods/sysproxy-viewer.vue'
import TunViewer from '@/components/setting/mods/tun-viewer.vue'
import { useServiceInstaller } from '@/hooks/use-service-installer'
import { useServiceUninstaller } from '@/hooks/use-service-uninstaller'
import { useSystemProxyState } from '@/hooks/use-system-proxy-state'
import { useSystemState } from '@/hooks/use-system-state'
import { useVerge } from '@/hooks/use-verge'
import { showNotice } from '@/services/notice-service'

const props = defineProps<{
  label?: string
  onError?: (err: Error) => void
  noRightPadding?: boolean
}>()

const { t } = useI18n()
const { verge, mutateVerge, patchVerge } = useVerge()
const { installServiceAndRestartCore } = useServiceInstaller()
const { uninstallServiceAndStartSidecar } = useServiceUninstaller()
const { indicator: systemProxyIndicator, toggleSystemProxy } = useSystemProxyState()
const { runState, isTunModeAvailable, mutateSystemState } = useSystemState()
const isServiceInstallReady = computed(() => runState.value.serviceUsable)

const sysproxyRef = ref<InstanceType<typeof SysproxyViewer> | null>(null)
const tunRef = ref<InstanceType<typeof TunViewer> | null>(null)

const enableTunMode = computed(() => verge.value?.enable_tun_mode ?? false)

const isSystemProxyMode = computed(() =>
  props.label === t('settings.sections.system.toggles.systemProxy') || !props.label
)
const isTunMode = computed(() => props.label === t('settings.sections.system.toggles.tunMode'))

let tunToggleLock = false
const handleTunToggle = async (value: boolean) => {
  if (tunToggleLock) return
  tunToggleLock = true
  try {
    if (!isTunModeAvailable.value) {
      const msgKey = 'settings.sections.proxyControl.tooltips.tunUnavailable'
      showNotice.error(msgKey)
      throw new Error(t(msgKey))
    }
    await mutateVerge({ ...verge.value, enable_tun_mode: value }, false)
    await patchVerge({ enable_tun_mode: value })
  } finally { tunToggleLock = false }
}

let installLock = false
const onInstallService = async () => {
  if (installLock) return
  installLock = true
  try {
    await installServiceAndRestartCore()
    await mutateSystemState()
  } catch (err) { showNotice.error(err) }
  finally { installLock = false }
}

let uninstallLock = false
const onUninstallService = async () => {
  if (uninstallLock) return
  uninstallLock = true
  try {
    await uninstallServiceAndStartSidecar()
  } catch (err) { showNotice.error(err) }
  finally { uninstallLock = false }
}

const handleToggle = async (type: string, value: boolean) => {
  if (type === 'tun') await handleTunToggle(value)
  else await toggleSystemProxy(value)
}
</script>
