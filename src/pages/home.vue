<template>
  <BasePage
    :title="t('home.page.title')"
    :contentStyle="{ padding: '16px' }"
  >
    <template #header>
      <div :style="{ display: 'flex', alignItems: 'center' }">
        <n-tooltip :title="t('home.page.tooltips.lightweightMode')" :trigger="'hover'">
          <template #trigger>
            <n-button quaternary circle size="small" @click="entry_lightweight_mode()">
              <template #icon>
                <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42C8.27 19.99 10.51 21 13 21c4.97 0 9-4.03 9-9s-4.03-9-9-9zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z"/></svg>
              </template>
            </n-button>
          </template>
        </n-tooltip>
        <n-tooltip :title="t('home.page.tooltips.manual')" :trigger="'hover'">
          <template #trigger>
            <n-button quaternary circle size="small" @click="toGithubDoc">
              <template #icon>
                <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
              </template>
            </n-button>
          </template>
        </n-tooltip>
        <n-tooltip :title="t('home.page.tooltips.settings')" :trigger="'hover'">
          <template #trigger>
            <n-button quaternary circle size="small" @click="openSettings">
              <template #icon>
                <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.07.62-.07.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>
              </template>
            </n-button>
          </template>
        </n-tooltip>
      </div>
    </template>
    <div
      :style="{
        display: 'grid',
        gridTemplateColumns: 'repeat(12, 1fr)',
        gap: '12px',
      }"
    >
      <template v-for="card in allCards" :key="card.key">
        <div v-if="card.component" :style="{ gridColumn: `span ${card.size}` }">
          <component :is="card.component" />
        </div>
      </template>
    </div>
    <n-modal v-model:show="settingsOpen" :mask-closable="true" preset="card" :title="t('home.page.settings.title')" :style="{ maxWidth: '400px' }" @update:show="(val: boolean) => { if (!val) settingsOpen = false }">
      <div :style="{ display: 'flex', flexDirection: 'column', gap: '8px' }">
        <n-checkbox v-model:checked="settingsCards.profile" @update:checked="(val: boolean) => handleToggle('profile', val)">
          {{ t('home.page.settings.cards.profile') }}
        </n-checkbox>
        <n-checkbox v-model:checked="settingsCards.proxy" @update:checked="(val: boolean) => handleToggle('proxy', val)">
          {{ t('home.page.settings.cards.currentProxy') }}
        </n-checkbox>
        <n-checkbox v-model:checked="settingsCards.network" @update:checked="(val: boolean) => handleToggle('network', val)">
          {{ t('home.page.settings.cards.network') }}
        </n-checkbox>
        <n-checkbox v-model:checked="settingsCards.mode" @update:checked="(val: boolean) => handleToggle('mode', val)">
          {{ t('home.page.settings.cards.proxyMode') }}
        </n-checkbox>
        <n-checkbox v-model:checked="settingsCards.traffic" @update:checked="(val: boolean) => handleToggle('traffic', val)">
          {{ t('home.page.settings.cards.traffic') }}
        </n-checkbox>
        <n-checkbox v-model:checked="settingsCards.test" @update:checked="(val: boolean) => handleToggle('test', val)">
          {{ t('home.page.settings.cards.tests') }}
        </n-checkbox>
        <n-checkbox v-model:checked="settingsCards.ip" @update:checked="(val: boolean) => handleToggle('ip', val)">
          {{ t('home.page.settings.cards.ip') }}
        </n-checkbox>
        <n-checkbox v-model:checked="settingsCards.clashinfo" @update:checked="(val: boolean) => handleToggle('clashinfo', val)">
          {{ t('home.page.settings.cards.clashInfo') }}
        </n-checkbox>
        <n-checkbox v-model:checked="settingsCards.systeminfo" @update:checked="(val: boolean) => handleToggle('systeminfo', val)">
          {{ t('home.page.settings.cards.systemInfo') }}
        </n-checkbox>
      </div>
      <template #footer>
        <div :style="{ display: 'flex', justifyContent: 'flex-end', gap: '8px' }">
          <n-button @click="settingsOpen = false">{{ t('shared.actions.cancel') }}</n-button>
          <n-button type="primary" @click="handleSave">{{ t('shared.actions.save') }}</n-button>
        </div>
      </template>
    </n-modal>
  </BasePage>
</template>

<script setup lang="ts">
import { ref, computed, reactive, watch, defineAsyncComponent } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NCheckbox, NModal, NTooltip } from 'naive-ui'

import { BasePage } from '@/components/base'
import ClashModeCard from '@/components/home/clash-mode-card.vue'
import CurrentProxyCard from '@/components/home/current-proxy-card.vue'
import EnhancedCard from '@/components/home/enhanced-card.vue'
import EnhancedTrafficStats from '@/components/home/enhanced-traffic-stats.vue'
import HomeProfileCard from '@/components/home/home-profile-card.vue'
import ProxyTunCard from '@/components/home/proxy-tun-card.vue'
import { useProfiles } from '@/hooks/use-profiles'
import { useVerge } from '@/hooks/use-verge'
import { entry_lightweight_mode, openWebUrl } from '@/services/cmds'

const { t } = useI18n()
const { verge, patchVerge } = useVerge()
const { current, mutateProfiles } = useProfiles()

const LazyTestCard = defineAsyncComponent(() => import('@/components/home/test-card.vue'))
const LazyIpInfoCard = defineAsyncComponent(() => import('@/components/home/ip-info-card.vue'))
const LazyClashInfoCard = defineAsyncComponent(() => import('@/components/home/clash-info-card.vue'))
const LazySystemInfoCard = defineAsyncComponent(() => import('@/components/home/system-info-card.vue'))

const DEFAULT_HOME_CARDS = {
  info: false, profile: true, proxy: true, network: true,
  mode: true, traffic: true, clashinfo: true, systeminfo: true,
  test: true, ip: true,
}

const homeCards = computed(() => (verge.value?.home_cards as any) ?? DEFAULT_HOME_CARDS)
const settingsOpen = ref(false)
const settingsCards = reactive({ ...DEFAULT_HOME_CARDS })

watch(settingsOpen, (val) => {
  if (val) Object.assign(settingsCards, homeCards.value)
})

const toGithubDoc = async () => openWebUrl('https://clash-verge-rev.github.io/index.html')

const openSettings = () => { settingsOpen.value = true }

const handleToggle = (key: string, val: boolean) => {
  settingsCards[key as keyof typeof settingsCards] = val
}

const handleSave = async () => {
  await patchVerge({ home_cards: { ...settingsCards } })
  settingsOpen.value = false
}

const NetworkSettingsCard = {
  components: { ProxyTunCard, EnhancedCard },
  template: '<EnhancedCard :title="t(\'home.page.cards.networkSettings\')" iconColor="primary"><template #icon><svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M20 2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-2 5h-3v3h-2V7h-3V5h3V2h2v3h3v2zM4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6z"/></svg></template><ProxyTunCard /></EnhancedCard>',
  setup() { const { t: translate } = useI18n(); return { t: translate } },
}

const ClashModeEnhancedCard = {
  components: { ClashModeCard, EnhancedCard },
  template: '<EnhancedCard :title="t(\'home.page.cards.proxyMode\')" iconColor="info"><template #icon><svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M22 11h-5V6h-3v5h-4V3H7v8H1.5v2H7v8h3v-8h4v5h3v-5h5v-2z"/></svg></template><ClashModeCard /></EnhancedCard>',
  setup() { const { t: translate } = useI18n(); return { t: translate } },
}

const renderCard = (cardKey: string, component: any, size = 6) => {
  if (!homeCards.value[cardKey]) return null
  return { key: cardKey, component, size }
}

const allCards = computed(() => [
  renderCard('profile', HomeProfileCard ? { components: { HomeProfileCard }, template: '<HomeProfileCard :current="current" @profile-updated="mutateProfiles" />', setup() { return { current, mutateProfiles } } } : null),
  renderCard('proxy', CurrentProxyCard),
  renderCard('network', NetworkSettingsCard),
  renderCard('mode', ClashModeEnhancedCard),
  renderCard('traffic', { components: { EnhancedCard, EnhancedTrafficStats }, template: '<EnhancedCard :title="t(\'home.page.cards.trafficStats\')" iconColor="secondary"><template #icon><svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M20.38 8.57l-1.23 1.85a8 8 0 01-.22 7.58H5.07A8 8 0 0115.58 6.85l1.85-1.23A10 10 0 003.35 19a2 2 0 001.72 1h13.85a2 2 0 001.74-1 10 10 0 00-.27-10.44z"/></svg></template><EnhancedTrafficStats /></EnhancedCard>', setup() { const { t: translate } = useI18n(); return { t: translate } } }, 12),
  renderCard('test', LazyTestCard),
  renderCard('ip', LazyIpInfoCard),
  renderCard('clashinfo', LazyClashInfoCard),
  renderCard('systeminfo', LazySystemInfoCard),
].filter(Boolean))
</script>
