<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'

import { DialogRef, Switch, TooltipIcon } from '@/components/base'
import ProxyControlSwitches from '@/components/shared/proxy-control-switches.vue'
import { useVerge } from '@/hooks/use-verge'

import GuardState from './mods/guard-state.vue'
import { SettingList, SettingItem } from './mods/setting-comp'
import SysproxyViewer from './mods/sysproxy-viewer.vue'
import TunViewer from './mods/tun-viewer.vue'

defineProps<{
  onError?: (err: Error) => void
}>()

const { verge, mutateVerge, patchVerge } = useVerge()

const { enable_auto_launch, enable_silent_start } = verge ?? {}

const sysproxyRef = ref<DialogRef>()
const tunRef = ref<DialogRef>()

const onSwitchFormat = (
  _e: any,
  value: boolean,
) => value
const onChangeData = (patch: Partial<IVergeConfig>) => {
  mutateVerge({ ...verge, ...patch }, false)
}
</script>

<template>
  <SettingList :title="i18n.t('settings.sections.system.title')">
    <SysproxyViewer ref="sysproxyRef" />
    <TunViewer ref="tunRef" />

    <ProxyControlSwitches
      :label="i18n.t('settings.sections.system.toggles.tunMode')"
      :onError="onError"
    />

    <ProxyControlSwitches
      :label="i18n.t('settings.sections.system.toggles.systemProxy')"
      :onError="onError"
    />

    <SettingItem :label="i18n.t('settings.sections.system.fields.autoLaunch')">
      <GuardState
        :value="enable_auto_launch ?? false"
        valueProps="checked"
        :onCatch="onError"
        :onFormat="onSwitchFormat"
        @change="(e) => { onChangeData({ enable_auto_launch: e }) }"
        @guard="async (e: any) => {
          try {
            onChangeData({ enable_auto_launch: e })
            await patchVerge({ enable_auto_launch: e })
            return Promise.resolve()
          } catch (error) {
            onChangeData({ enable_auto_launch: !e })
            return Promise.reject(error)
          }
        }"
      >
        <Switch edge="end" />
      </GuardState>
    </SettingItem>

    <SettingItem
      :label="i18n.t('settings.sections.system.fields.silentStart')"
    >
      <template #extra>
        <TooltipIcon
          :title="i18n.t('settings.sections.system.tooltips.silentStart')"
        />
      </template>
      <GuardState
        :value="enable_silent_start ?? false"
        valueProps="checked"
        :onCatch="onError"
        :onFormat="onSwitchFormat"
        @change="(e) => onChangeData({ enable_silent_start: e })"
        @guard="(e) => patchVerge({ enable_silent_start: e })"
      >
        <Switch edge="end" />
      </GuardState>
    </SettingItem>
  </SettingList>
</template>
