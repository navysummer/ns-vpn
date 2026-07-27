<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'
import { invoke } from '@tauri-apps/api/core'
import { updateGeo, type LogLevel } from 'tauri-plugin-mihomo-api'

import { DialogRef, Switch, TooltipIcon } from '@/components/base'
import { useClash } from '@/hooks/use-clash'
import { useClashLog } from '@/hooks/use-clash-log'
import { useDisplayedMixedPort } from '@/hooks/use-displayed-mixed-port'
import { useVerge } from '@/hooks/use-verge'
import { invoke_uwp_tool } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import getSystem from '@/utils/get-system'

import ClashCoreViewer from './mods/clash-core-viewer.vue'
import ClashPortViewer from './mods/clash-port-viewer.vue'
import ControllerViewer from './mods/controller-viewer.vue'
import DnsViewer from './mods/dns-viewer.vue'
import HeaderConfiguration from './mods/external-controller-cors.vue'
import GuardState from './mods/guard-state.vue'
import NetworkInterfaceViewer from './mods/network-interface-viewer.vue'
import { SettingItem, SettingList } from './mods/setting-comp'
import TunnelsViewer from './mods/tunnels-viewer.vue'
import WebUIViewer from './mods/web-ui-viewer.vue'

const isWIN = getSystem() === 'windows'

const props = defineProps<{
  onError: (err: Error) => void
}>()

const { clash, version, mutateClash, patchClash } = useClash()
const { verge, patchVerge } = useVerge()
const displayedMixedPort = useDisplayedMixedPort()
const [, setClashLog] = useClashLog()

const {
  ipv6,
  'allow-lan': allowLan,
  'log-level': logLevel,
  'unified-delay': unifiedDelay,
} = clash ?? {}

const dnsSettingsEnabled = ref(verge?.enable_dns_settings ?? false)

const webRef = ref<DialogRef>()
const portRef = ref<DialogRef>()
const ctrlRef = ref<DialogRef>()
const coreRef = ref<DialogRef>()
const networkRef = ref<DialogRef>()
const dnsRef = ref<DialogRef>()
const corsRef = ref<DialogRef>()
const tunnelRef = ref<DialogRef>()

const onSwitchFormat = (_e: any, value: boolean) => value
const onChangeData = (patch: Partial<IConfigData>) => {
  mutateClash((old) => ({ ...old!, ...patch }), false)
}
const onUpdateGeo = async () => {
  try {
    await updateGeo()
    showNotice.success('settings.feedback.notifications.clash.geoDataUpdated')
  } catch (err: any) {
    showNotice.error(err)
  }
}

const handleDnsToggle = async (enable: boolean) => {
  try {
    dnsSettingsEnabled.value = enable
    await patchVerge({ enable_dns_settings: enable })
    await invoke('apply_dns_config', { apply: enable })
    setTimeout(() => {
      mutateClash()
    }, 500)
  } catch (err: any) {
    dnsSettingsEnabled.value = !enable
    showNotice.error(err)
    await patchVerge({ enable_dns_settings: !enable }).catch(() => {})
    throw err
  }
}
</script>

<template>
  <SettingList :title="i18n.t('settings.sections.clash.title')">
    <WebUIViewer ref="webRef" />
    <ClashPortViewer ref="portRef" />
    <ControllerViewer ref="ctrlRef" />
    <ClashCoreViewer ref="coreRef" />
    <NetworkInterfaceViewer ref="networkRef" />
    <DnsViewer ref="dnsRef" />
    <HeaderConfiguration ref="corsRef" />
    <TunnelsViewer ref="tunnelRef" />

    <SettingItem :label="i18n.t('settings.sections.clash.form.fields.allowLan')">
      <template #extra>
        <TooltipIcon
          :title="i18n.t('settings.sections.clash.form.tooltips.networkInterface')"
          color="inherit"
          @click="networkRef?.open()"
        />
      </template>
      <GuardState
        :value="allowLan ?? false"
        valueProps="checked"
        :onCatch="props.onError"
        :onFormat="onSwitchFormat"
        @change="(e) => onChangeData({ 'allow-lan': e })"
        @guard="(e) => patchClash({ 'allow-lan': e })"
      >
        <Switch edge="end" />
      </GuardState>
    </SettingItem>

    <SettingItem :label="i18n.t('settings.sections.clash.form.fields.dnsOverwrite')">
      <template #extra>
        <TooltipIcon @click="dnsRef?.open()" />
      </template>
      <Switch
        edge="end"
        :checked="dnsSettingsEnabled"
        @change="(_, checked: boolean) => handleDnsToggle(checked)"
      />
    </SettingItem>

    <SettingItem :label="i18n.t('settings.sections.clash.form.fields.ipv6')">
      <GuardState
        :value="ipv6 ?? false"
        valueProps="checked"
        :onCatch="props.onError"
        :onFormat="onSwitchFormat"
        @change="(e) => onChangeData({ ipv6: e })"
        @guard="(e) => patchClash({ ipv6: e })"
      >
        <Switch edge="end" />
      </GuardState>
    </SettingItem>

    <SettingItem :label="i18n.t('settings.sections.clash.form.fields.unifiedDelay')">
      <template #extra>
        <TooltipIcon
          :title="i18n.t('settings.sections.clash.form.tooltips.unifiedDelay')"
        />
      </template>
      <GuardState
        :value="unifiedDelay ?? false"
        valueProps="checked"
        :onCatch="props.onError"
        :onFormat="onSwitchFormat"
        @change="(e) => onChangeData({ 'unified-delay': e })"
        @guard="(e) => patchClash({ 'unified-delay': e })"
      >
        <Switch edge="end" />
      </GuardState>
    </SettingItem>

    <SettingItem :label="i18n.t('settings.sections.clash.form.fields.logLevel')">
      <template #extra>
        <TooltipIcon
          :title="i18n.t('settings.sections.clash.form.tooltips.logLevel')"
        />
      </template>
      <GuardState
        :value="logLevel === 'warn' ? 'warning' : (logLevel ?? 'info')"
        :onCatch="props.onError"
        :onFormat="(e: any) => e.target.value"
        @change="(e) => onChangeData({ 'log-level': e })"
        @guard="(e) => {
          setClashLog((pre: any) => ({
            ...pre!,
            logLevel: e.toUpperCase() as LogLevel,
          }))
          return patchClash({ 'log-level': e })
        }"
      >
        <select class="MuiSelect-root MuiSelect-sizeSmall" style="width: 100px; padding: 7.5px 0;">
          <option value="debug">{{ i18n.t('settings.sections.clash.form.options.logLevel.debug') }}</option>
          <option value="info">{{ i18n.t('settings.sections.clash.form.options.logLevel.info') }}</option>
          <option value="warning">{{ i18n.t('settings.sections.clash.form.options.logLevel.warning') }}</option>
          <option value="error">{{ i18n.t('settings.sections.clash.form.options.logLevel.error') }}</option>
          <option value="silent">{{ i18n.t('settings.sections.clash.form.options.logLevel.silent') }}</option>
        </select>
      </GuardState>
    </SettingItem>

    <SettingItem :label="i18n.t('settings.sections.clash.form.fields.portConfig')">
      <input
        class="MuiInput-root MuiInput-sizeSmall"
        :value="displayedMixedPort"
        readonly
        style="width: 100px; cursor: pointer; padding: 7.5px;"
        @click="(e: any) => { portRef?.open(); (e.target as HTMLElement).blur() }"
      />
    </SettingItem>

    <SettingItem
      :label="i18n.t('settings.sections.clash.form.fields.external')"
      @click="ctrlRef?.open()"
    >
      <template #extra>
        <TooltipIcon
          :title="i18n.t('settings.sections.externalCors.tooltips.open')"
          @click="(e: any) => { e.stopPropagation(); corsRef?.open() }"
        />
      </template>
    </SettingItem>

    <SettingItem
      @click="webRef?.open()"
      :label="i18n.t('settings.sections.clash.form.fields.webUI')"
    />

    <SettingItem :label="i18n.t('settings.sections.clash.form.fields.clashCore')">
      <template #extra>
        <TooltipIcon @click="coreRef?.open()" />
      </template>
      <span style="padding: 7px 8px 7px 0;">{{ version }}</span>
    </SettingItem>

    <SettingItem
      v-if="isWIN"
      @click="invoke_uwp_tool"
      :label="i18n.t('settings.sections.clash.form.fields.openUwpTool')"
    >
      <template #extra>
        <TooltipIcon
          :title="i18n.t('settings.sections.clash.form.tooltips.openUwpTool')"
        />
      </template>
    </SettingItem>

    <SettingItem
      @click="onUpdateGeo"
      :label="i18n.t('settings.sections.clash.form.fields.updateGeoData')"
    />

    <SettingItem
      :label="i18n.t('settings.sections.clash.form.fields.tunnels.title')"
      @click="tunnelRef?.open()"
    />
  </SettingList>
</template>
