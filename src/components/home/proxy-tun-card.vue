<template>
  <div :style="{ display: 'flex', flexDirection: 'column', width: '100%' }">
    <div :style="{ display: 'flex', justifyContent: 'center', gap: '8px', position: 'relative', zIndex: 2 }">
      <div
        :style="tabButtonStyle('system')"
        @click="handleTabChange('system')"
      >
        <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M20 18c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2H0v2h24v-2h-4zM4 6h16v10H4V6z"/></svg>
        <span :style="{ fontSize: '14px', fontWeight: activeTab === 'system' ? 600 : 400 }">{{ t('settings.sections.system.toggles.systemProxy') }}</span>
        <span v-if="systemProxyConfigState" :style="{ width: '8px', height: '8px', borderRadius: '50%', backgroundColor: activeTab === 'system' ? '#fff' : 'var(--success-color)', position: 'absolute', top: '8px', right: '8px' }" />
      </div>
      <div
        :style="tabButtonStyle('tun')"
        @click="handleTabChange('tun')"
      >
        <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M22 9V7h-2V5c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-2h2v-2h-2v-2h2v-2h-2V9h2zM4 5h14v14H4V5z"/></svg>
        <span :style="{ fontSize: '14px', fontWeight: activeTab === 'tun' ? 600 : 400 }">{{ t('settings.sections.system.toggles.tunMode') }}</span>
        <span v-if="enable_tun_mode && isTunModeAvailable" :style="{ width: '8px', height: '8px', borderRadius: '50%', backgroundColor: activeTab === 'tun' ? '#fff' : 'var(--success-color)', position: 'absolute', top: '8px', right: '8px' }" />
      </div>
    </div>

    <div :style="{ width: '100%', margin: '8px 0', position: 'relative', display: 'flex', justifyContent: 'center', overflow: 'visible' }">
      <div :style="{ width: '95%', textAlign: 'center', color: 'var(--text-secondary-color)', padding: '6px', borderRadius: '8px', border: '1px solid var(--primary-color)', backgroundColor: 'var(--bg-color)', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '4px', fontSize: '12px' }">
        {{ tabDescription.text }}
        <n-tooltip :title="tabDescription.tooltip" :trigger="'hover'">
          <template #trigger>
            <svg viewBox="0 0 24 24" width="14" height="14" fill="currentColor" :style="{ opacity: 0.7, flexShrink: 0 }"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13 15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z"/></svg>
          </template>
        </n-tooltip>
      </div>
    </div>

    <div :style="{ padding: '8px', backgroundColor: 'var(--primary-color-alpha)', borderRadius: '12px' }">
      <ProxyControlSwitches
        :onError="handleError"
        :label="activeTab === 'system' ? t('settings.sections.system.toggles.systemProxy') : t('settings.sections.system.toggles.tunMode')"
        :noRightPadding="true"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NTooltip } from 'naive-ui'

import ProxyControlSwitches from '@/components/shared/proxy-control-switches.vue'
import { useSystemProxyState } from '@/hooks/use-system-proxy-state'
import { useSystemState } from '@/hooks/use-system-state'
import { useVerge } from '@/hooks/use-verge'
import { showNotice } from '@/services/notice-service'

const LOCAL_STORAGE_TAB_KEY = 'clash-verge-proxy-active-tab'

const { t } = useI18n()
const activeTab = ref(localStorage.getItem(LOCAL_STORAGE_TAB_KEY) || 'system')
const { verge } = useVerge()
const { isTunModeAvailable } = useSystemState()
const { configState: systemProxyConfigState } = useSystemProxyState()
const enable_tun_mode = computed(() => verge.value?.enable_tun_mode)

const handleError = (err: unknown) => showNotice.error(err)

const handleTabChange = (tab: string) => {
  activeTab.value = tab
  localStorage.setItem(LOCAL_STORAGE_TAB_KEY, tab)
}

const tabDescription = computed(() => {
  if (activeTab.value === 'system') {
    return {
      text: systemProxyConfigState.value ? t('home.components.proxyTun.status.systemProxyEnabled') : t('home.components.proxyTun.status.systemProxyDisabled'),
      tooltip: t('home.components.proxyTun.tooltips.systemProxy'),
    }
  }
  return {
    text: !isTunModeAvailable.value ? t('home.components.proxyTun.status.tunModeServiceRequired') : enable_tun_mode.value ? t('home.components.proxyTun.status.tunModeEnabled') : t('home.components.proxyTun.status.tunModeDisabled'),
    tooltip: t('home.components.proxyTun.tooltips.tunMode'),
  }
})

const tabButtonStyle = (tab: string) => ({
  cursor: 'pointer',
  padding: '8px 16px',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  gap: '8px',
  backgroundColor: activeTab.value === tab ? 'var(--primary-color)' : 'var(--bg-color)',
  color: activeTab.value === tab ? '#fff' : 'var(--text-color)',
  borderRadius: '12px',
  flex: 1,
  maxWidth: '160px',
  transition: 'all 0.2s ease-in-out',
  position: 'relative' as const,
})
</script>
