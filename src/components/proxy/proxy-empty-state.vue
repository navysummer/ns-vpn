<template>
  <div :style="{ width: '100%', height: '100%', display: 'flex', alignItems: 'center', justifyContent: 'center', padding: '16px' }">
    <n-alert type="warning" :style="{ width: '100%', maxWidth: '640px' }">
      <template #header>
        {{ title }}
      </template>
      {{ description }}
      <template #footer>
        <div :style="{ display: 'flex', gap: '8px', flexWrap: 'wrap', marginTop: '16px' }">
          <n-button v-if="showProfiles" size="small" type="primary" @click="router.push('/profile')">
            {{ t('proxies.page.empty.actions.openProfiles') }}
          </n-button>
          <template v-if="showCoreActions">
            <n-button size="small" type="primary" :loading="isRestarting" @click="handleRestart">
              {{ t('proxies.page.empty.actions.restartCore') }}
            </n-button>
            <n-button size="small" :disabled="isRestarting" @click="openLogsDir">
              {{ t('proxies.page.empty.actions.openLogs') }}
            </n-button>
          </template>
        </div>
      </template>
    </n-alert>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { NAlert, NButton } from 'naive-ui'

import { runStateQueryKey } from '@/hooks/use-system-state'
import { useAppRefreshers } from '@/providers/app-data-context'
import { openLogsDir, restartCore } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { revalidateQuery } from '@/services/query-client'

const props = defineProps<{
  reason: string
}>()

const { t } = useI18n()
const router = useRouter()
const { refreshProxy } = useAppRefreshers()
const isRestarting = ref(false)

const title = computed(() => {
  switch (props.reason) {
    case 'no-subscriptions': return t('proxies.page.empty.noSubscriptions.title')
    case 'inactive-subscription': return t('proxies.page.empty.inactiveSubscription.title')
    case 'core-unavailable': return t('proxies.page.empty.coreUnavailable.title')
    default: return t('proxies.page.empty.noProxyInfo.title')
  }
})

const description = computed(() => {
  switch (props.reason) {
    case 'no-subscriptions': return t('proxies.page.empty.noSubscriptions.description')
    case 'inactive-subscription': return t('proxies.page.empty.inactiveSubscription.description')
    case 'core-unavailable': return t('proxies.page.empty.coreUnavailable.description')
    default: return t('proxies.page.empty.noProxyInfo.description')
  }
})

const showProfiles = computed(() => ['no-subscriptions', 'inactive-subscription', 'no-proxy-info'].includes(props.reason))
const showCoreActions = computed(() => ['core-unavailable', 'no-proxy-info'].includes(props.reason))

const handleRestart = async () => {
  isRestarting.value = true
  try {
    await restartCore()
    await Promise.all([refreshProxy(), revalidateQuery(runStateQueryKey)])
  } catch (error: any) {
    showNotice.error(error)
  } finally {
    isRestarting.value = false
  }
}
</script>
