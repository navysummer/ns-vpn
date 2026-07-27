<template>
  <EnhancedCard
    iconColor="info"
  >
    <template #icon>
      <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96z"/></svg>
    </template>
    <template #title>
      <span v-if="!current" :style="{ fontSize: '18px', fontWeight: 500 }">{{ t('profiles.page.title') }}</span>
      <button
        v-else-if="current.home"
        :style="{ ...titleLinkStyle, fontSize: '18px', fontWeight: 500 }"
        :title="current.name"
        @click="current.home && openWebUrl(current.home)"
      >
        <span :style="{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', flex: 1 }">{{ current.name }}</span>
        <svg viewBox="0 0 24 24" width="14" height="14" fill="currentColor" :style="{ marginLeft: '4px', opacity: 0.7, flexShrink: 0 }"><path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/></svg>
      </button>
      <span v-else :style="{ fontSize: '18px', fontWeight: 500 }">{{ current.name }}</span>
    </template>
    <template #action>
      <n-button v-if="current" size="small" @click="goToProfiles" :style="{ borderRadius: '12px' }">
        {{ t('layout.components.navigation.tabs.profiles') }}
        <template #suffix>
          <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M20 2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-2 5h-3v3h-2V7h-3V5h3V2h2v3h3v2zM4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6z"/></svg>
        </template>
      </n-button>
    </template>
    <div v-if="!current" :style="{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', padding: '19px 0', cursor: 'pointer', borderRadius: '12px' }" @click="goToProfiles">
      <svg viewBox="0 0 24 24" width="60" height="60" fill="var(--primary-color)" :style="{ marginBottom: '16px' }"><path d="M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96z"/></svg>
      <div :style="{ fontSize: '20px', fontWeight: 500, marginBottom: '8px' }">{{ t('profiles.page.actions.import') }} {{ t('profiles.page.title') }}</div>
      <div :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">{{ t('profiles.components.card.labels.clickToImport') }}</div>
    </div>
    <div v-else :style="{ display: 'flex', flexDirection: 'column', gap: '16px' }">
      <div v-if="current.url" :style="{ display: 'flex', alignItems: 'center', gap: '8px' }">
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M20 2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-2 5h-3v3h-2V7h-3V5h3V2h2v3h3v2zM4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6z"/></svg>
        <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)', display: 'flex', alignItems: 'center', minWidth: 0 }">
          <span :style="{ flexShrink: 0 }">{{ t('shared.labels.from') }}: </span>
          <button v-if="current.home" :style="{ display: 'inline-flex', alignItems: 'center', minWidth: 0, maxWidth: 'calc(100% - 40px)', marginLeft: '4px', fontWeight: 500, color: 'var(--primary-color)', background: 'none', border: 'none', cursor: 'pointer', padding: 0, fontSize: '14px', overflow: 'hidden' }" :title="parseUrl(current.url)" @click="current.home && openWebUrl(current.home)">
            <span :style="{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', minWidth: 0, flex: 1 }">{{ parseUrl(current.url) }}</span>
            <svg viewBox="0 0 24 24" width="12" height="12" fill="currentColor" :style="{ marginLeft: '4px', opacity: 0.7, flexShrink: 0 }"><path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/></svg>
          </button>
          <span v-else :style="{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', minWidth: 0, flex: 1, marginLeft: '4px', fontWeight: 500 }" :title="parseUrl(current.url)">{{ parseUrl(current.url) }}</span>
        </span>
      </div>
      <div v-if="current.updated" :style="{ display: 'flex', alignItems: 'center', gap: '8px' }">
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor" :style="{ cursor: 'pointer', animation: updating ? 'spin 1.5s linear infinite' : 'none' }" @click="onUpdateProfile"><path d="M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46C19.54 15.03 20 13.57 20 12c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74C4.46 8.97 4 10.43 4 12c0 4.42 3.58 8 8 8v3l4-4-4-4v3z"/></svg>
        <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)', cursor: 'pointer' }" @click="onUpdateProfile">
          {{ t('shared.labels.updateTime') }}: <span :style="{ fontWeight: 500 }">{{ dayjs(current.updated * 1000).format('YYYY-MM-DD HH:mm') }}</span>
        </span>
      </div>
      <template v-if="current.extra">
        <div :style="{ display: 'flex', alignItems: 'center', gap: '8px' }">
          <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M20.38 8.57l-1.23 1.85a8 8 0 01-.22 7.58H5.07A8 8 0 0115.58 6.85l1.85-1.23A10 10 0 003.35 19a2 2 0 001.72 1h13.85a2 2 0 001.74-1 10 10 0 00-.27-10.44z"/></svg>
          <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">
            {{ t('shared.labels.usedTotal') }}: <span :style="{ fontWeight: 500 }">{{ parseTraffic(usedTraffic) }} / {{ parseTraffic(current.extra.total) }}</span>
          </span>
        </div>
        <div v-if="current.extra.expire > 0" :style="{ display: 'flex', alignItems: 'center', gap: '8px' }">
          <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm-1-5h2v2h-2v-2zm0-10h2v8h-2V5z"/></svg>
          <span :style="{ fontSize: '14px', color: 'var(--text-secondary-color)' }">
            {{ t('shared.labels.expireTime') }}: <span :style="{ fontWeight: 500 }">{{ parseExpire(current.extra.expire) }}</span>
          </span>
        </div>
        <div :style="{ marginTop: '8px' }">
          <div :style="{ fontSize: '12px', color: 'var(--text-secondary-color)', marginBottom: '4px' }">{{ trafficPercentage }}%</div>
          <div :style="{ height: '8px', borderRadius: '4px', backgroundColor: 'var(--primary-color-alpha)', overflow: 'hidden' }">
            <div :style="{ height: '100%', borderRadius: '4px', backgroundColor: 'var(--primary-color)', width: trafficPercentage + '%', transition: 'width 0.3s ease' }" />
          </div>
        </div>
      </template>
    </div>
  </EnhancedCard>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { NButton } from 'naive-ui'
import dayjs from 'dayjs'

import EnhancedCard from './enhanced-card.vue'
import { useAppRefreshers } from '@/providers/app-data-context'
import { openWebUrl, updateProfile } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import parseTraffic from '@/utils/parse-traffic'

const props = defineProps<{
  current: any
  onProfileUpdated?: () => void
}>()

const { t } = useI18n()
const router = useRouter()
const { refreshAll } = useAppRefreshers()

const updating = ref(false)

const parseUrl = (url?: string) => {
  if (!url) return '-'
  if (url.startsWith('http')) return new URL(url).host
  return 'local'
}

const parseExpire = (expire?: number) => {
  if (!expire) return '-'
  return dayjs(expire * 1000).format('YYYY-MM-DD')
}

const usedTraffic = computed(() => {
  if (!props.current?.extra) return 0
  return props.current.extra.upload + props.current.extra.download
})

const trafficPercentage = computed(() => {
  if (!props.current?.extra?.total || props.current.extra.total <= 0) return 0
  return Math.min(Math.round((usedTraffic.value / props.current.extra.total) * 100), 100)
})

const onUpdateProfile = async () => {
  if (!props.current?.uid) return
  updating.value = true
  try {
    await updateProfile(props.current.uid, props.current.option)
    props.onProfileUpdated?.()
    refreshAll()
  } catch (err: any) {
    showNotice.error(err, 3000)
  } finally {
    updating.value = false
  }
}

const goToProfiles = () => router.push('/profile')

const titleLinkStyle = {
  color: 'inherit',
  textDecoration: 'none',
  display: 'flex',
  alignItems: 'center',
  minWidth: 0,
  maxWidth: '100%',
  background: 'none',
  border: 'none',
  cursor: 'pointer',
  padding: 0,
}
</script>
