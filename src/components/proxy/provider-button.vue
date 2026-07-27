<template>
  <template v-if="providers.length > 0 || providerUnavailable">
    <n-button
      size="small"
      :type="providerUnavailable ? 'warning' : 'default'"
      :disabled="providerUnavailable"
      @click="open = true"
    >
      <template #icon>
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M20 2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-2 5h-3v3h-2V7h-3V5h3V2h2v3h3v2zM4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6z"/></svg>
      </template>
      {{ t('proxies.page.provider.title') }}
    </n-button>

    <n-modal v-model:show="open" preset="card" :style="{ maxWidth: '600px', width: '100%' }" :title="t('proxies.page.provider.title')" @update:show="(val: boolean) => { if (!val) open = false }">
      <template #header-extra>
        <n-button size="small" type="primary" @click="updateAllProviders">
          {{ t('proxies.page.provider.actions.updateAll') }}
        </n-button>
      </template>
      <div :style="{ minHeight: '250px' }">
        <div v-for="provider in providers" :key="provider.name" :style="{ display: 'flex', alignItems: 'center', padding: '12px 16px', marginBottom: '8px', borderRadius: '12px', backgroundColor: 'var(--bg-color)', transition: 'all 0.2s' }">
          <div :style="{ flex: 1 }">
            <div :style="{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }">
              <div :style="{ display: 'flex', alignItems: 'center', overflow: 'hidden' }">
                <span :style="{ marginRight: '8px', fontSize: '16px', fontWeight: 500, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }" :title="provider.name">{{ provider.name }}</span>
                <span :style="typeBoxStyle">{{ provider.proxyRecordIds.length }}</span>
                <span :style="typeBoxStyle">{{ provider.vehicleType }}</span>
              </div>
              <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">
                <small>{{ t('shared.labels.updateAt') }}: </small>{{ provider.updatedAt ? dayjs(provider.updatedAt).fromNow() : '-' }}
              </span>
            </div>
            <template v-if="provider.subscriptionInfo">
              <div :style="{ marginBottom: '8px', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }">
                <span :title="t('shared.labels.usedTotal')">{{ parseTraffic((provider.subscriptionInfo.upload || 0) + (provider.subscriptionInfo.download || 0)) }} / {{ parseTraffic(provider.subscriptionInfo.total || 0) }}</span>
                <span :title="t('shared.labels.expireTime')">{{ provider.subscriptionInfo.expire ? parseExpire(provider.subscriptionInfo.expire) : '-' }}</span>
              </div>
              <div :style="{ height: '6px', borderRadius: '3px', backgroundColor: 'var(--border-color)', overflow: 'hidden' }">
                <div :style="{ height: '100%', borderRadius: '3px', backgroundColor: 'var(--primary-color)', width: Math.min(Math.round(((provider.subscriptionInfo.download || 0) + (provider.subscriptionInfo.upload || 0)) * 100 / (provider.subscriptionInfo.total || 1)), 100) + '%' }" />
              </div>
            </template>
          </div>
          <div :style="{ borderLeft: '1px solid var(--border-color)', height: '40px', margin: '0 8px' }" />
          <n-button
            quaternary circle size="small"
            :disabled="updating[provider.name]"
            @click="updateProvider(provider.name)"
          >
            <template #icon>
              <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor" :style="{ animation: updating[provider.name] ? 'spin 1s linear infinite' : 'none' }"><path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
            </template>
          </n-button>
        </div>
      </div>
      <template #footer>
        <n-button @click="open = false">{{ t('shared.actions.close') }}</n-button>
      </template>
    </n-modal>
  </template>
</template>

<script setup lang="ts">
import { ref, computed, reactive } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NModal } from 'naive-ui'
import dayjs from 'dayjs'
import { updateProxyProvider } from 'tauri-plugin-mihomo-api'

import { useAppRefreshers, useProxiesData } from '@/providers/app-data-context'
import { showNotice } from '@/services/notice-service'
import parseTraffic from '@/utils/parse-traffic'

const { t } = useI18n()
const open = ref(false)
const { proxyView } = useProxiesData()
const { refreshProxy } = useAppRefreshers()
const updating = reactive<Record<string, boolean>>({})

const providers = computed(() => proxyView.value?.providers ?? [])
const providerUnavailable = computed(() => proxyView.value?.providerState === 'unavailable')

const parseExpire = (expire?: number) => {
  if (!expire) return '-'
  return dayjs(expire * 1000).format('YYYY-MM-DD')
}

const typeBoxStyle = {
  display: 'inline-block',
  border: '1px solid var(--secondary-color)',
  color: 'var(--secondary-color)',
  borderRadius: '4px',
  fontSize: '10px',
  marginRight: '4px',
  padding: '0 4px',
  lineHeight: 1.5,
}

let updatingAll = false
const updateAllProviders = async () => {
  if (updatingAll) return
  updatingAll = true
  const allProviders = providers.value.map((p: any) => p.name)
  if (allProviders.length === 0) { showNotice.info('proxies.feedback.notifications.provider.none'); return }
  allProviders.forEach((name: string) => { updating[name] = true })
  for (const name of allProviders) {
    try { await updateProxyProvider(name) }
    catch (err) { console.error(`更新 ${name} 失败`, err) }
    updating[name] = false
  }
  await refreshProxy()
  showNotice.success('proxies.feedback.notifications.provider.allUpdated')
  updatingAll = false
}

let updatingOne = false
const updateProvider = async (name: string) => {
  if (updatingOne || updating[name]) return
  updatingOne = true
  updating[name] = true
  try {
    await updateProxyProvider(name)
    await refreshProxy()
    showNotice.success('proxies.feedback.notifications.provider.updateSuccess', { name })
  } catch (err: any) {
    showNotice.error('proxies.feedback.notifications.provider.updateFailed', { name, message: String(err) })
  } finally {
    updating[name] = false
    updatingOne = false
  }
}
</script>
