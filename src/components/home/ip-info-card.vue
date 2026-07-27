<template>
  <EnhancedCard
    :title="t('home.components.ipInfo.title')"
    iconColor="info"
  >
    <template #icon>
      <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z"/></svg>
    </template>
    <template #action>
      <n-button quaternary circle size="small" @click="mutate">
        <template #icon>
          <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
        </template>
      </n-button>
    </template>
    <div v-if="isLoading" :style="{ display: 'flex', flexDirection: 'column', gap: '8px' }">
      <n-skeleton text :width="'60%'" :height="30" />
      <n-skeleton text :width="'80%'" :height="24" />
      <n-skeleton text :width="'70%'" :height="24" />
      <n-skeleton text :width="'50%'" :height="24" />
    </div>
    <div v-else-if="error" :style="{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', height: '100%' }">
      <span :style="{ fontSize: '16px', color: 'var(--error-color)' }">{{ error instanceof Error ? error.message : t('home.components.ipInfo.errors.load') }}</span>
      <n-button :style="{ marginTop: '16px' }" @click="mutate">{{ t('shared.actions.retry') }}</n-button>
    </div>
    <div v-else :style="{ height: '100%', display: 'flex', flexDirection: 'column' }">
      <div :style="{ display: 'flex', flex: 1, overflow: 'hidden' }">
        <div :style="{ width: '40%', overflow: 'hidden' }">
          <div :style="{ display: 'flex', alignItems: 'center', marginBottom: '8px', overflow: 'hidden' }">
            <span :style="{ fontSize: '24px', marginRight: '8px', width: '28px', textAlign: 'center', flexShrink: 0 }">{{ getCountryFlag(ipInfo?.country_code) }}</span>
            <span :style="{ fontSize: '16px', fontWeight: 500, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', maxWidth: '100%' }">{{ ipInfo?.country || t('home.components.ipInfo.labels.unknown') }}</span>
          </div>
          <div :style="{ display: 'flex', alignItems: 'center', marginBottom: '8px' }">
            <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)', flexShrink: 0 }">{{ t('home.components.ipInfo.labels.ip') }}:</span>
            <div :style="{ display: 'flex', alignItems: 'center', marginLeft: '8px', overflow: 'hidden', maxWidth: 'calc(100% - 30px)' }">
              <span :style="{ fontSize: '12px', fontFamily: 'monospace', overflow: 'hidden', textOverflow: 'ellipsis', wordBreak: 'break-all' }">{{ showIp ? ipInfo?.ip : '••••••••••' }}</span>
              <n-button quaternary circle size="small" @click="toggleShowIp">
                <template #icon>
                  <svg v-if="showIp" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z"/></svg>
                  <svg v-else viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z"/></svg>
                </template>
              </n-button>
            </div>
          </div>
          <InfoItem :label="t('home.components.ipInfo.labels.asn')" :value="ipInfo?.asn ? `AS${ipInfo.asn}` : 'N/A'" />
        </div>
        <div :style="{ width: '60%', overflow: 'auto' }">
          <InfoItem :label="t('home.components.ipInfo.labels.isp')" :value="ipInfo?.organization" />
          <InfoItem :label="t('home.components.ipInfo.labels.org')" :value="ipInfo?.asn_organization" />
          <InfoItem :label="t('home.components.ipInfo.labels.location')" :value="[ipInfo?.city, ipInfo?.region].filter(Boolean).join(', ')" />
          <InfoItem :label="t('home.components.ipInfo.labels.timezone')" :value="ipInfo?.timezone" />
        </div>
      </div>
      <div :style="{ marginTop: 'auto', paddingTop: '4px', borderTop: '1px solid var(--border-color)', display: 'flex', justifyContent: 'space-between', alignItems: 'center', opacity: 0.7, fontSize: '11px' }">
        <span :style="{ fontSize: '11px' }">{{ t('home.components.ipInfo.labels.autoRefresh') }}{{ countdown.type === 'countdown' ? `: ${countdown.remainingSeconds}s` : '...' }}</span>
        <span :style="{ textOverflow: 'ellipsis', overflow: 'hidden', whiteSpace: 'nowrap', fontSize: '11px' }">{{ `${ipInfo?.country_code ?? 'N/A'}, ${ipInfo?.longitude?.toFixed(2) ?? 'N/A'}, ${ipInfo?.latitude?.toFixed(2) ?? 'N/A'}` }}</span>
      </div>
    </div>
  </EnhancedCard>
</template>

<script setup lang="ts">
import { ref, computed, h, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NSkeleton } from 'naive-ui'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

import EnhancedCard from './enhanced-card.vue'
import { getIpInfo } from '@/services/api'
import { useQuery } from '@/services/query-client'

const IP_REFRESH_SECONDS = 300
const COUNTDOWN_TICK_INTERVAL = 5000
const IP_INFO_CACHE_KEY = 'cv_ip_info_cache'

const { t } = useI18n()
const showIp = ref(false)
const appWindow = getCurrentWebviewWindow()

const countdown = ref<{ type: string; remainingSeconds?: number }>({ type: 'countdown', remainingSeconds: IP_REFRESH_SECONDS })

const { data: ipInfo, error, isLoading, refetch: mutate } = useQuery({
  queryKey: [IP_INFO_CACHE_KEY],
  queryFn: getIpInfo,
  staleTime: Infinity,
  retry: 1,
  retryDelay: 30000,
})

const toggleShowIp = () => { showIp.value = !showIp.value }

const getCountryFlag = (countryCode?: string) => {
  if (!countryCode) return ''
  const codePoints = countryCode.toUpperCase().split('').map((char) => 127397 + char.charCodeAt(0))
  return String.fromCodePoint(...codePoints)
}

const InfoItem = (props: { label: string; value?: string }) =>
  h('div', { style: { marginBottom: '5px', display: 'flex', alignItems: 'flex-start' } }, [
    h('span', { style: { fontSize: '14px', color: 'var(--text-secondary-color)', minWidth: '60px', marginRight: '4px', flexShrink: 0, textAlign: 'right' } }, props.label + ':'),
    h('span', { style: { fontSize: '14px', marginLeft: '4px', overflow: 'hidden', textOverflow: 'ellipsis', wordBreak: 'break-word', flexGrow: 1 } }, props.value || 'Unknown'),
  ])

let active = true
let timer: number | null = null

const onCountdownTick = async () => {
  if (!active) return
  const now = Date.now()
  const ts = ipInfo.value?.lastFetchTs
  if (!ts) return
  const elapsed = Math.floor((now - ts) / 1000)
  const remaining = IP_REFRESH_SECONDS - elapsed
  if (remaining <= 0) {
    if (navigator.onLine && countdown.value.type !== 'revalidating') {
      const visible = await appWindow.isVisible()
      if (visible) {
        countdown.value = { type: 'revalidating' }
        try { await mutate() }
        finally { countdown.value = { type: 'countdown', remainingSeconds: IP_REFRESH_SECONDS } }
      }
    }
  } else {
    countdown.value = { type: 'countdown', remainingSeconds: remaining }
  }
}

onMounted(() => {
  timer = window.setInterval(onCountdownTick, COUNTDOWN_TICK_INTERVAL)
})

onUnmounted(() => {
  active = false
  if (timer !== null) clearInterval(timer)
})
</script>
