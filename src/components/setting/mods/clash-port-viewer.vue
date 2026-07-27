<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'

import { BaseDialog, Switch } from '@/components/base'
import { useDisplayedMixedPort } from '@/hooks/use-displayed-mixed-port'
import { useVerge } from '@/hooks/use-verge'
import { saveProxyPorts } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import getSystem from '@/utils/get-system'

const OS = getSystem()

const generateRandomPort = () =>
  Math.floor(Math.random() * (65535 - 1025 + 1)) + 1025

const { verge } = useVerge()
const displayedMixedPort = useDisplayedMixedPort()
const open = ref(false)
const loading = ref(false)

const mixedPort = ref(displayedMixedPort)
const socksPort = ref(verge?.verge_socks_port ?? 7898)
const socksEnabled = ref(verge?.verge_socks_enabled ?? false)
const httpPort = ref(verge?.verge_port ?? 7899)
const httpEnabled = ref(verge?.verge_http_enabled ?? false)
const redirPort = ref(verge?.verge_redir_port ?? 7895)
const redirEnabled = ref(verge?.verge_redir_enabled ?? false)
const tproxyPort = ref(verge?.verge_tproxy_port ?? 7896)
const tproxyEnabled = ref(verge?.verge_tproxy_enabled ?? false)

defineExpose({
  open: () => {
    mixedPort.value = displayedMixedPort
    socksPort.value = verge?.verge_socks_port ?? 7898
    socksEnabled.value = verge?.verge_socks_enabled ?? false
    httpPort.value = verge?.verge_port ?? 7899
    httpEnabled.value = verge?.verge_http_enabled ?? false
    redirPort.value = verge?.verge_redir_port ?? 7895
    redirEnabled.value = verge?.verge_redir_enabled ?? false
    tproxyPort.value = verge?.verge_tproxy_port ?? 7896
    tproxyEnabled.value = verge?.verge_tproxy_enabled ?? false
    open.value = true
  },
  close: () => { open.value = false },
})

const onSave = async () => {
  const portList = [
    mixedPort.value,
    socksEnabled.value ? socksPort.value : -1,
    httpEnabled.value ? httpPort.value : -1,
    redirEnabled.value ? redirPort.value : -1,
    tproxyEnabled.value ? tproxyPort.value : -1,
  ].filter((p) => p !== -1)

  if (new Set(portList).size !== portList.length) return

  const isValidPort = (port: number) => port >= 1 && port <= 65535
  const allPortsValid = [
    mixedPort.value,
    socksEnabled.value ? socksPort.value : 0,
    httpEnabled.value ? httpPort.value : 0,
    redirEnabled.value ? redirPort.value : 0,
    tproxyEnabled.value ? tproxyPort.value : 0,
  ].every((port) => port === 0 || isValidPort(port))

  if (!allPortsValid) return

  loading.value = true
  try {
    const outcome = await saveProxyPorts({
      mixedPort: mixedPort.value,
      socks: { enabled: socksEnabled.value, port: socksPort.value },
      http: { enabled: httpEnabled.value, port: httpPort.value },
      redir: { enabled: redirEnabled.value, port: redirPort.value },
      tproxy: { enabled: tproxyEnabled.value, port: tproxyPort.value },
    })
    if (outcome.status === 'conflict') {
      showNotice.error('settings.modals.clashPort.messages.portInUse', { port: outcome.port })
      return
    }
    open.value = false
    showNotice.success('settings.modals.clashPort.messages.saved')
  } catch (error) {
    showNotice.error('settings.modals.clashPort.messages.saveFailed', error)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <BaseDialog
    :open="open"
    :title="i18n.t('settings.modals.clashPort.title')"
    :contentSx="{ width: '400px' }"
    :okBtn="loading ? i18n.t('shared.statuses.saving') : i18n.t('shared.actions.save')"
    :cancelBtn="i18n.t('shared.actions.cancel')"
    @onClose="open = false"
    @onCancel="open = false"
    @onOk="onSave"
  >
    <ul class="MuiList-root" style="width: 100%; list-style: none; padding: 0;">
      <li style="padding: 4px 0; min-height: 36px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span class="MuiListItemText-primary" style="font-size: 12px;">{{ i18n.t('settings.modals.clashPort.fields.mixed') }}</span>
        </div>
        <div style="display: flex; align-items: center;">
          <input
            type="text"
            :value="mixedPort"
            @input="mixedPort = +($event.target as HTMLInputElement).value.replace(/\D+/, '').slice(0, 5)"
            style="width: 80px; margin-right: 4px; font-size: 12px; padding: 4px; border: 1px solid #ccc; border-radius: 4px;"
          />
          <button class="MuiIconButton-root MuiIconButton-sizeSmall" :title="i18n.t('settings.modals.clashPort.actions.random')" style="margin-right: 4px;" @click="mixedPort = generateRandomPort()">
            <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M10.59 9.17L5.41 4 4 5.41l5.17 5.17 1.42-1.41zM14.5 4l2.04 2.04L4 18.59 5.41 20 17.96 7.46 20 9.5V4h-5.5zm.33 9.41l-1.41 1.41 3.13 3.13L14.5 20H20v-5.5l-2.04 2.04-3.13-3.13z"/></svg>
          </button>
          <Switch size="small" :checked="true" disabled style="opacity: 0.7; margin-left: 4px;" />
        </div>
      </li>

      <li style="padding: 4px 0; min-height: 36px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span class="MuiListItemText-primary" style="font-size: 12px;">{{ i18n.t('settings.modals.clashPort.fields.socks') }}</span>
        </div>
        <div style="display: flex; align-items: center;">
          <input
            type="text"
            :value="socksPort"
            :disabled="!socksEnabled"
            @input="socksPort = +($event.target as HTMLInputElement).value.replace(/\D+/, '').slice(0, 5)"
            style="width: 80px; margin-right: 4px; font-size: 12px; padding: 4px; border: 1px solid #ccc; border-radius: 4px;"
          />
          <button class="MuiIconButton-root MuiIconButton-sizeSmall" :title="i18n.t('settings.modals.clashPort.actions.random')" :disabled="!socksEnabled" style="margin-right: 4px;" @click="socksPort = generateRandomPort()">
            <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M10.59 9.17L5.41 4 4 5.41l5.17 5.17 1.42-1.41zM14.5 4l2.04 2.04L4 18.59 5.41 20 17.96 7.46 20 9.5V4h-5.5zm.33 9.41l-1.41 1.41 3.13 3.13L14.5 20H20v-5.5l-2.04 2.04-3.13-3.13z"/></svg>
          </button>
          <Switch size="small" :checked="socksEnabled" @change="(_, c: boolean) => socksEnabled = c" style="margin-left: 4px;" />
        </div>
      </li>

      <li style="padding: 4px 0; min-height: 36px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span class="MuiListItemText-primary" style="font-size: 12px;">{{ i18n.t('settings.modals.clashPort.fields.http') }}</span>
        </div>
        <div style="display: flex; align-items: center;">
          <input
            type="text"
            :value="httpPort"
            :disabled="!httpEnabled"
            @input="httpPort = +($event.target as HTMLInputElement).value.replace(/\D+/, '').slice(0, 5)"
            style="width: 80px; margin-right: 4px; font-size: 12px; padding: 4px; border: 1px solid #ccc; border-radius: 4px;"
          />
          <button class="MuiIconButton-root MuiIconButton-sizeSmall" :title="i18n.t('settings.modals.clashPort.actions.random')" :disabled="!httpEnabled" style="margin-right: 4px;" @click="httpPort = generateRandomPort()">
            <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M10.59 9.17L5.41 4 4 5.41l5.17 5.17 1.42-1.41zM14.5 4l2.04 2.04L4 18.59 5.41 20 17.96 7.46 20 9.5V4h-5.5zm.33 9.41l-1.41 1.41 3.13 3.13L14.5 20H20v-5.5l-2.04 2.04-3.13-3.13z"/></svg>
          </button>
          <Switch size="small" :checked="httpEnabled" @change="(_, c: boolean) => httpEnabled = c" style="margin-left: 4px;" />
        </div>
      </li>

      <li v-if="OS !== 'windows'" style="padding: 4px 0; min-height: 36px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span class="MuiListItemText-primary" style="font-size: 12px;">{{ i18n.t('settings.modals.clashPort.fields.redir') }}</span>
        </div>
        <div style="display: flex; align-items: center;">
          <input
            type="text"
            :value="redirPort"
            :disabled="!redirEnabled"
            @input="redirPort = +($event.target as HTMLInputElement).value.replace(/\D+/, '').slice(0, 5)"
            style="width: 80px; margin-right: 4px; font-size: 12px; padding: 4px; border: 1px solid #ccc; border-radius: 4px;"
          />
          <button class="MuiIconButton-root MuiIconButton-sizeSmall" :title="i18n.t('settings.modals.clashPort.actions.random')" :disabled="!redirEnabled" style="margin-right: 4px;" @click="redirPort = generateRandomPort()">
            <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M10.59 9.17L5.41 4 4 5.41l5.17 5.17 1.42-1.41zM14.5 4l2.04 2.04L4 18.59 5.41 20 17.96 7.46 20 9.5V4h-5.5zm.33 9.41l-1.41 1.41 3.13 3.13L14.5 20H20v-5.5l-2.04 2.04-3.13-3.13z"/></svg>
          </button>
          <Switch size="small" :checked="redirEnabled" @change="(_, c: boolean) => redirEnabled = c" style="margin-left: 4px;" />
        </div>
      </li>

      <li v-if="OS === 'linux'" style="padding: 4px 0; min-height: 36px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span class="MuiListItemText-primary" style="font-size: 12px;">{{ i18n.t('settings.modals.clashPort.fields.tproxy') }}</span>
        </div>
        <div style="display: flex; align-items: center;">
          <input
            type="text"
            :value="tproxyPort"
            :disabled="!tproxyEnabled"
            @input="tproxyPort = +($event.target as HTMLInputElement).value.replace(/\D+/, '').slice(0, 5)"
            style="width: 80px; margin-right: 4px; font-size: 12px; padding: 4px; border: 1px solid #ccc; border-radius: 4px;"
          />
          <button class="MuiIconButton-root MuiIconButton-sizeSmall" :title="i18n.t('settings.modals.clashPort.actions.random')" :disabled="!tproxyEnabled" style="margin-right: 4px;" @click="tproxyPort = generateRandomPort()">
            <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M10.59 9.17L5.41 4 4 5.41l5.17 5.17 1.42-1.41zM14.5 4l2.04 2.04L4 18.59 5.41 20 17.96 7.46 20 9.5V4h-5.5zm.33 9.41l-1.41 1.41 3.13 3.13L14.5 20H20v-5.5l-2.04 2.04-3.13-3.13z"/></svg>
          </button>
          <Switch size="small" :checked="tproxyEnabled" @change="(_, c: boolean) => tproxyEnabled = c" style="margin-left: 4px;" />
        </div>
      </li>
    </ul>
  </BaseDialog>
</template>
