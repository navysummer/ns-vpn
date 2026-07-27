<template>
  <EnhancedCard
    :title="t('home.components.clashInfo.title')"
    iconColor="warning"
  >
    <template #icon>
      <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M22 9V7h-2V5c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-2h2v-2h-2v-2h2v-2h-2V9h2zm-4 10H4V5h14v14zM6 13h5v4H6v-4zm6-6h4v3h-4V7zM6 7h5v5H6V7zm6 4h4v6h-4v-6z"/></svg>
    </template>
    <div v-if="!clashConfig" :style="{ height: '24px' }" />
    <div v-else :style="{ display: 'flex', flexDirection: 'column', gap: '12px' }">
      <div :style="{ display: 'flex', justifyContent: 'space-between' }">
        <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">{{ t('home.components.clashInfo.fields.coreVersion') }}</span>
        <span :style="{ fontSize: '14px', fontWeight: 500 }">{{ clashVersion || '-' }}</span>
      </div>
      <hr :style="{ border: 'none', borderTop: '1px solid var(--border-color)', margin: 0 }" />
      <div :style="{ display: 'flex', justifyContent: 'space-between' }">
        <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">{{ t('home.components.clashInfo.fields.systemProxyAddress') }}</span>
        <span :style="{ fontSize: '14px', fontWeight: 500 }">{{ systemProxyAddress }}</span>
      </div>
      <hr :style="{ border: 'none', borderTop: '1px solid var(--border-color)', margin: 0 }" />
      <div :style="{ display: 'flex', justifyContent: 'space-between' }">
        <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">{{ t('home.components.clashInfo.fields.mixedPort') }}</span>
        <span :style="{ fontSize: '14px', fontWeight: 500 }">{{ displayedMixedPort }}</span>
      </div>
      <hr :style="{ border: 'none', borderTop: '1px solid var(--border-color)', margin: 0 }" />
      <div :style="{ display: 'flex', justifyContent: 'space-between' }">
        <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">{{ t('home.components.clashInfo.fields.uptime') }}</span>
        <span :style="{ fontSize: '14px', fontWeight: 500 }">{{ formattedUptime }}</span>
      </div>
      <hr :style="{ border: 'none', borderTop: '1px solid var(--border-color)', margin: 0 }" />
      <div :style="{ display: 'flex', justifyContent: 'space-between' }">
        <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">{{ t('home.components.clashInfo.fields.rulesCount') }}</span>
        <span :style="{ fontSize: '14px', fontWeight: 500 }">{{ rules.length }}</span>
      </div>
    </div>
  </EnhancedCard>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

import { useClash } from '@/hooks/use-clash'
import { useDisplayedMixedPort } from '@/hooks/use-displayed-mixed-port'
import {
  useClashConfigData,
  useRulesData,
  useSystemData,
  useUptimeData,
} from '@/providers/app-data-context'

import EnhancedCard from './enhanced-card.vue'

const { t } = useI18n()
const { version: clashVersion } = useClash()
const { clashConfig } = useClashConfigData()
const displayedMixedPort = useDisplayedMixedPort()
const { rules } = useRulesData()
const { uptime } = useUptimeData()
const { systemProxyAddress } = useSystemData()

const formatUptime = (uptimeMs: number) => {
  const hours = Math.floor(uptimeMs / 3600000)
  const minutes = Math.floor((uptimeMs % 3600000) / 60000)
  const seconds = Math.floor((uptimeMs % 60000) / 1000)
  return `${hours}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`
}

const formattedUptime = computed(() => formatUptime(uptime.value))
</script>
