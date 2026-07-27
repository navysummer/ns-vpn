<script setup lang="ts">
import { ref, onMounted } from 'vue'
import i18n from 'i18next'
import { convertFileSrc } from '@tauri-apps/api/core'
import { join } from '@tauri-apps/api/path'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { exists } from '@tauri-apps/plugin-fs'

import { BaseDialog, DialogRef, Switch, TooltipIcon } from '@/components/base'
import { DEFAULT_HOVER_DELAY } from '@/components/proxy/proxy-group-navigator.vue'
import { useVerge } from '@/hooks/use-verge'
import { useWindowDecorations } from '@/hooks/use-window'
import { copyIconFile, getAppDir } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import getSystem from '@/utils/get-system'

import GuardState from './guard-state.vue'

const OS = getSystem()

const clampHoverDelay = (value: number) => {
  if (!Number.isFinite(value)) return DEFAULT_HOVER_DELAY
  return Math.min(5000, Math.max(0, Math.round(value)))
}

const getIcons = async (icon_dir: string, name: string) => {
  const updateTime = localStorage.getItem(`icon_${name}_update_time`) || ''
  const icon_png = await join(icon_dir, `${name}-${updateTime}.png`)
  const icon_ico = await join(icon_dir, `${name}-${updateTime}.ico`)
  return { icon_png, icon_ico }
}

const { verge, patchVerge, mutateVerge } = useVerge()
const open = ref(false)
const commonIcon = ref('')
const sysproxyIcon = ref('')
const tunIcon = ref('')
const { decorated, toggleDecorations } = useWindowDecorations()

onMounted(() => { initIconPath() })

async function initIconPath() {
  const appDir = await getAppDir()
  const icon_dir = await join(appDir, 'icons')
  const common = await getIcons(icon_dir, 'common')
  const sysproxy = await getIcons(icon_dir, 'sysproxy')
  const tun = await getIcons(icon_dir, 'tun')
  commonIcon.value = (await exists(common.icon_ico)) ? common.icon_ico : common.icon_png
  sysproxyIcon.value = (await exists(sysproxy.icon_ico)) ? sysproxy.icon_ico : sysproxy.icon_png
  tunIcon.value = (await exists(tun.icon_ico)) ? tun.icon_ico : tun.icon_png
}

defineExpose<DialogRef>({
  open: () => { open.value = true },
  close: () => { open.value = false },
})

const onSwitchFormat = (_e: any, value: boolean) => value
const onError = (err: any) => { showNotice.error(err) }
const onChangeData = (patch: Partial<IVergeConfig>) => {
  mutateVerge({ ...verge, ...patch }, false)
}
</script>

<template>
  <BaseDialog
    :open="open"
    :title="i18n.t('settings.components.verge.layout.title')"
    :contentSx="{ width: '450px' }"
    :disableOk="true"
    :cancelBtn="i18n.t('shared.actions.close')"
    @onClose="open = false"
    @onCancel="open = false"
  >
    <ul class="MuiList-root" style="list-style: none; padding: 0;">
      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.preferSystemTitlebar') }}</span>
        </div>
        <GuardState
          :value="decorated"
          valueProps="checked"
          :onCatch="onError"
          :onFormat="onSwitchFormat"
          @change="async () => { await toggleDecorations() }"
        >
          <Switch edge="end" />
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.trafficGraph') }}</span>
        </div>
        <GuardState
          :value="verge?.traffic_graph ?? true"
          valueProps="checked"
          :onCatch="onError"
          :onFormat="onSwitchFormat"
          @change="(e) => onChangeData({ traffic_graph: e })"
          @guard="(e) => patchVerge({ traffic_graph: e })"
        >
          <Switch edge="end" />
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.memoryUsage') }}</span>
        </div>
        <GuardState
          :value="verge?.enable_memory_usage ?? true"
          valueProps="checked"
          :onCatch="onError"
          :onFormat="onSwitchFormat"
          @change="(e) => onChangeData({ enable_memory_usage: e })"
          @guard="(e) => patchVerge({ enable_memory_usage: e })"
        >
          <Switch edge="end" />
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.proxyGroupIcon') }}</span>
        </div>
        <GuardState
          :value="verge?.enable_group_icon ?? true"
          valueProps="checked"
          :onCatch="onError"
          :onFormat="onSwitchFormat"
          @change="(e) => onChangeData({ enable_group_icon: e })"
          @guard="(e) => patchVerge({ enable_group_icon: e })"
        >
          <Switch edge="end" />
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.pauseRenderTrafficStatsOnBlur') }}</span>
        </div>
        <GuardState
          :value="verge?.pause_render_traffic_stats_on_blur ?? true"
          valueProps="checked"
          :onCatch="onError"
          :onFormat="onSwitchFormat"
          @change="(e) => onChangeData({ pause_render_traffic_stats_on_blur: e })"
          @guard="(e) => patchVerge({ pause_render_traffic_stats_on_blur: e })"
        >
          <Switch edge="end" />
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.toastPosition') }}</span>
        </div>
        <GuardState
          :value="verge?.notice_position ?? 'top-right'"
          :onCatch="onError"
          :onFormat="(e: any) => e.target.value"
          @change="(value: any) => onChangeData({ notice_position: value })"
          @guard="(value: any) => patchVerge({ notice_position: value })"
        >
          <select class="MuiSelect-root MuiSelect-sizeSmall" style="width: 180px; padding: 7.5px 0;">
            <option value="top-right">{{ i18n.t('settings.components.verge.layout.options.toastPosition.topRight') }}</option>
            <option value="top-left">{{ i18n.t('settings.components.verge.layout.options.toastPosition.topLeft') }}</option>
            <option value="bottom-right">{{ i18n.t('settings.components.verge.layout.options.toastPosition.bottomRight') }}</option>
            <option value="bottom-left">{{ i18n.t('settings.components.verge.layout.options.toastPosition.bottomLeft') }}</option>
          </select>
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <div style="display: flex; align-items: center; gap: 4px;">
            <span>{{ i18n.t('settings.components.verge.layout.fields.hoverNavigator') }}</span>
            <TooltipIcon :title="i18n.t('settings.components.verge.layout.tooltips.hoverNavigator')" />
          </div>
        </div>
        <GuardState
          :value="verge?.enable_hover_jump_navigator ?? true"
          valueProps="checked"
          :onCatch="onError"
          :onFormat="onSwitchFormat"
          @change="(e) => onChangeData({ enable_hover_jump_navigator: e })"
          @guard="(e) => patchVerge({ enable_hover_jump_navigator: e })"
        >
          <Switch edge="end" />
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <div style="display: flex; align-items: center; gap: 4px;">
            <span>{{ i18n.t('settings.components.verge.layout.fields.hoverNavigatorDelay') }}</span>
            <TooltipIcon :title="i18n.t('settings.components.verge.layout.tooltips.hoverNavigatorDelay')" />
          </div>
        </div>
        <GuardState
          :value="verge?.hover_jump_navigator_delay ?? DEFAULT_HOVER_DELAY"
          :waitTime="400"
          :onCatch="onError"
          :onFormat="(e: any) => clampHoverDelay(Number(e.target.value))"
          @change="(value: any) => onChangeData({ hover_jump_navigator_delay: clampHoverDelay(value) })"
          @guard="(value: any) => patchVerge({ hover_jump_navigator_delay: clampHoverDelay(value) })"
        >
          <input
            type="number"
            :value="verge?.hover_jump_navigator_delay ?? DEFAULT_HOVER_DELAY"
            :disabled="!(verge?.enable_hover_jump_navigator ?? true)"
            style="width: 120px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;"
          />
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.navIcon') }}</span>
        </div>
        <GuardState
          :value="verge?.menu_icon ?? 'monochrome'"
          :onCatch="onError"
          :onFormat="(e: any) => e.target.value"
          @change="(value: any) => onChangeData({ menu_icon: value })"
          @guard="(value: any) => patchVerge({ menu_icon: value })"
        >
          <select class="MuiSelect-root MuiSelect-sizeSmall" style="width: 140px; padding: 7.5px 0;">
            <option value="monochrome">{{ i18n.t('settings.components.verge.layout.options.icon.monochrome') }}</option>
            <option value="colorful">{{ i18n.t('settings.components.verge.layout.options.icon.colorful') }}</option>
            <option value="disable">{{ i18n.t('settings.components.verge.layout.options.icon.disable') }}</option>
          </select>
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.collapseNavBar') }}</span>
        </div>
        <GuardState
          :value="verge?.collapse_navbar ?? false"
          valueProps="checked"
          :onCatch="onError"
          :onFormat="onSwitchFormat"
          @change="(e) => onChangeData({ collapse_navbar: e })"
          @guard="(e) => patchVerge({ collapse_navbar: e })"
        >
          <Switch edge="end" />
        </GuardState>
      </li>

      <li v-if="OS === 'macos'" style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.trayIcon') }}</span>
        </div>
        <GuardState
          :value="verge?.tray_icon ?? 'monochrome'"
          :onCatch="onError"
          :onFormat="(e: any) => e.target.value"
          @change="(e: any) => onChangeData({ tray_icon: e })"
          @guard="(e: any) => patchVerge({ tray_icon: e })"
        >
          <select class="MuiSelect-root MuiSelect-sizeSmall" style="width: 140px; padding: 7.5px 0;">
            <option value="monochrome">{{ i18n.t('settings.components.verge.layout.options.icon.monochrome') }}</option>
            <option value="colorful">{{ i18n.t('settings.components.verge.layout.options.icon.colorful') }}</option>
          </select>
        </GuardState>
      </li>

      <li v-if="OS === 'macos'" style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.enableTraySpeed') }}</span>
        </div>
        <GuardState
          :value="verge?.enable_tray_speed ?? false"
          valueProps="checked"
          :onCatch="onError"
          :onFormat="onSwitchFormat"
          @change="(e) => onChangeData({ enable_tray_speed: e })"
          @guard="(e) => patchVerge({ enable_tray_speed: e })"
        >
          <Switch edge="end" />
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.proxyGroupsDisplayMode') }}</span>
        </div>
        <GuardState
          :value="verge?.tray_proxy_groups_display_mode ?? 'default'"
          :onCatch="onError"
          :onFormat="(e: any) => e.target.value"
          @change="(value: any) => onChangeData({ tray_proxy_groups_display_mode: value })"
          @guard="(value: any) => patchVerge({ tray_proxy_groups_display_mode: value })"
        >
          <select class="MuiSelect-root MuiSelect-sizeSmall" style="width: 140px; padding: 7.5px 0;">
            <option value="default">{{ i18n.t('settings.components.verge.layout.options.proxyGroupsDisplayMode.default') }}</option>
            <option value="inline">{{ i18n.t('settings.components.verge.layout.options.proxyGroupsDisplayMode.inline') }}</option>
            <option value="disable">{{ i18n.t('settings.components.verge.layout.options.proxyGroupsDisplayMode.disable') }}</option>
          </select>
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.showOutboundModesInline') }}</span>
        </div>
        <GuardState
          :value="verge?.tray_inline_outbound_modes ?? false"
          valueProps="checked"
          :onCatch="onError"
          :onFormat="onSwitchFormat"
          @change="(e) => onChangeData({ tray_inline_outbound_modes: e })"
          @guard="(e) => patchVerge({ tray_inline_outbound_modes: e })"
        >
          <Switch edge="end" />
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.commonTrayIcon') }}</span>
        </div>
        <GuardState
          :value="verge?.common_tray_icon"
          :onCatch="onError"
          @change="(e) => onChangeData({ common_tray_icon: e })"
          @guard="(e) => patchVerge({ common_tray_icon: e })"
        >
          <button
            class="MuiButton-root MuiButton-outlined MuiButton-sizeSmall"
            @click="async () => {
              if (verge?.common_tray_icon) {
                onChangeData({ common_tray_icon: false })
                patchVerge({ common_tray_icon: false })
              } else {
                const selected = await openDialog({
                  directory: false, multiple: false,
                  filters: [{ name: 'Tray Icon Image', extensions: ['png', 'ico'] }],
                })
                if (selected) {
                  await copyIconFile(`${selected}`, 'common')
                  await initIconPath()
                  onChangeData({ common_tray_icon: true })
                  patchVerge({ common_tray_icon: true })
                }
              }
            }"
          >
            <img v-if="verge?.common_tray_icon && commonIcon" height="20px" :src="convertFileSrc(commonIcon)" style="margin-right: 4px;" />
            {{ verge?.common_tray_icon ? i18n.t('shared.actions.clear') : i18n.t('settings.components.verge.basic.actions.browse') }}
          </button>
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.systemProxyTrayIcon') }}</span>
        </div>
        <GuardState
          :value="verge?.sysproxy_tray_icon"
          :onCatch="onError"
          @change="(e) => onChangeData({ sysproxy_tray_icon: e })"
          @guard="(e) => patchVerge({ sysproxy_tray_icon: e })"
        >
          <button
            class="MuiButton-root MuiButton-outlined MuiButton-sizeSmall"
            @click="async () => {
              if (verge?.sysproxy_tray_icon) {
                onChangeData({ sysproxy_tray_icon: false })
                patchVerge({ sysproxy_tray_icon: false })
              } else {
                const selected = await openDialog({
                  directory: false, multiple: false,
                  filters: [{ name: 'Tray Icon Image', extensions: ['png', 'ico'] }],
                })
                if (selected) {
                  await copyIconFile(`${selected}`, 'sysproxy')
                  await initIconPath()
                  onChangeData({ sysproxy_tray_icon: true })
                  patchVerge({ sysproxy_tray_icon: true })
                }
              }
            }"
          >
            <img v-if="verge?.sysproxy_tray_icon && sysproxyIcon" height="20px" :src="convertFileSrc(sysproxyIcon)" style="margin-right: 4px;" />
            {{ verge?.sysproxy_tray_icon ? i18n.t('shared.actions.clear') : i18n.t('settings.components.verge.basic.actions.browse') }}
          </button>
        </GuardState>
      </li>

      <li style="padding: 5px 2px; display: flex; align-items: center;">
        <div class="MuiListItemText-root" style="flex: 1;">
          <span>{{ i18n.t('settings.components.verge.layout.fields.tunTrayIcon') }}</span>
        </div>
        <GuardState
          :value="verge?.tun_tray_icon"
          :onCatch="onError"
          @change="(e) => onChangeData({ tun_tray_icon: e })"
          @guard="(e) => patchVerge({ tun_tray_icon: e })"
        >
          <button
            class="MuiButton-root MuiButton-outlined MuiButton-sizeSmall"
            @click="async () => {
              if (verge?.tun_tray_icon) {
                onChangeData({ tun_tray_icon: false })
                patchVerge({ tun_tray_icon: false })
              } else {
                const selected = await openDialog({
                  directory: false, multiple: false,
                  filters: [{ name: 'Tun Icon Image', extensions: ['png', 'ico'] }],
                })
                if (selected) {
                  await copyIconFile(`${selected}`, 'tun')
                  await initIconPath()
                  onChangeData({ tun_tray_icon: true })
                  patchVerge({ tun_tray_icon: true })
                }
              }
            }"
          >
            <img v-if="verge?.tun_tray_icon && tunIcon" height="20px" :src="convertFileSrc(tunIcon)" style="margin-right: 4px;" />
            {{ verge?.tun_tray_icon ? i18n.t('shared.actions.clear') : i18n.t('settings.components.verge.basic.actions.browse') }}
          </button>
        </GuardState>
      </li>
    </ul>
  </BaseDialog>
</template>
