<template>
  <template v-if="hasProviders">
    <n-button size="small" @click="open = true">
      <template #icon>
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 3c3.87 0 7 1.79 7 4s-3.13 4-7 4-7-1.79-7-4 3.13-4 7-4zm0 14c-3.87 0-7-1.79-7-4v-2c0 2.21 3.13 4 7 4s7-1.79 7-4v2c0 2.21-3.13 4-7 4z"/>
        </svg>
      </template>
      {{ t('rules.page.provider.trigger') }}
    </n-button>

    <n-modal
      :show="open"
      preset="card"
      :style="{ maxWidth: '600px' }"
      :mask-closable="false"
      :closeable="true"
      @update:show="handleClose"
    >
      <template #header>
        <div style="display: flex; justify-content: space-between; align-items: center;">
          <span style="font-size: 18px; font-weight: 600;">{{ t('rules.page.provider.dialogTitle') }}</span>
          <n-button size="small" type="primary" @click="updateAllProviders">
            {{ t('rules.page.provider.actions.updateAll') }}
          </n-button>
        </div>
      </template>

      <div style="min-height: 250px;">
        <div
          v-for="[key, provider] in sortedProviders"
          :key="key"
          class="provider-item"
        >
          <div style="flex: 1; padding: 8px 16px;">
            <div style="display: flex; justify-content: space-between; align-items: center;">
              <span style="display: flex; align-items: center; font-weight: 600; font-size: 1rem;">
                <span style="margin-right: 8px;">{{ key }}</span>
                <span class="type-box">{{ provider.ruleCount }}</span>
              </span>
              <span style="font-size: 0.875rem; color: var(--secondary-text);">
                <small>{{ t('shared.labels.updateAt') }}: </small>
                {{ formatTime(provider.updatedAt) }}
              </span>
            </div>
            <div style="display: flex;">
              <span class="type-box">{{ provider.vehicleType }}</span>
              <span class="type-box">{{ provider.behavior }}</span>
            </div>
          </div>
          <div style="width: 1px; background: var(--border-color);" />
          <div
            style="
              width: 40px;
              display: flex;
              justify-content: center;
              align-items: center;
            "
          >
            <n-button
              quaternary
              circle
              size="small"
              :disabled="updating[key]"
              :title="t('rules.page.provider.actions.update')"
              :aria-label="t('rules.page.provider.actions.update')"
              @click="updateProvider(key)"
            >
              <template #icon>
                <svg
                  viewBox="0 0 24 24"
                  width="16"
                  height="16"
                  fill="currentColor"
                  :class="{ spinning: updating[key] }"
                >
                  <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
                </svg>
              </template>
            </n-button>
          </div>
        </div>
      </div>

      <template #footer>
        <n-button @click="handleClose">
          {{ t('shared.actions.close') }}
        </n-button>
      </template>
    </n-modal>
  </template>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useTranslation } from '@/composables/use-i18n'
import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import { updateRuleProvider } from 'tauri-plugin-mihomo-api'

import { useAppRefreshers, useRulesData } from '@/providers/app-data-context'
import { showNotice } from '@/services/notice-service'

dayjs.extend(relativeTime)

const { t } = useTranslation()
const open = ref(false)
const { ruleProviders } = useRulesData()
const { refreshRules, refreshRuleProviders } = useAppRefreshers()
const updating = ref<Record<string, boolean>>({})
const updateAllLock = ref(false)

const hasProviders = computed(() => Object.keys(ruleProviders || {}).length > 0)

const sortedProviders = computed(() => {
  return Object.entries(ruleProviders || {}).sort(([a], [b]) => a.localeCompare(b))
})

const formatTime = (time: string) => {
  return dayjs(time).fromNow()
}

const updateProvider = async (name: string) => {
  if (updating.value[name]) return
  updating.value[name] = true
  try {
    await updateRuleProvider(name)
    await refreshRules()
    await refreshRuleProviders()
    showNotice.success('rules.feedback.notifications.provider.updateSuccess', { name })
  } catch (err) {
    showNotice.error('rules.feedback.notifications.provider.updateFailed', { name, message: String(err) })
  } finally {
    updating.value[name] = false
  }
}

const updateAllProviders = async () => {
  if (updateAllLock.value) return
  updateAllLock.value = true
  try {
    const allProviders = Object.keys(ruleProviders || {})
    if (allProviders.length === 0) {
      showNotice.info('rules.feedback.notifications.provider.none')
      return
    }

    const newUpdating = allProviders.reduce(
      (acc, key) => {
        acc[key] = true
        return acc
      },
      {} as Record<string, boolean>,
    )
    updating.value = newUpdating

    for (const name of allProviders) {
      try {
        await updateRuleProvider(name)
        updating.value[name] = false
      } catch (err) {
        console.error(`更新 ${name} 失败`, err)
      }
    }

    await refreshRules()
    await refreshRuleProviders()

    showNotice.success('rules.feedback.notifications.provider.allUpdated')
  } catch (err) {
    showNotice.error('rules.feedback.notifications.provider.genericError', { message: String(err) })
  } finally {
    updating.value = {}
    updateAllLock.value = false
  }
}

const handleClose = () => {
  open.value = false
}
</script>

<style scoped>
.provider-item {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
  border-radius: 8px;
  overflow: hidden;
  transition: all 0.2s;
  background-color: var(--bg-color);
}

.provider-item:hover {
  background-color: color-mix(in srgb, var(--primary-main) 15%, var(--bg-color));
  border-color: color-mix(in srgb, var(--primary-main) 30%, transparent);
}

.type-box {
  display: inline-block;
  border: 1px solid color-mix(in srgb, var(--primary-main) 50%, transparent);
  color: color-mix(in srgb, var(--primary-main) 80%, transparent);
  border-radius: 4px;
  font-size: 10px;
  margin-right: 4px;
  padding: 0 2px;
  line-height: 1.25;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.spinning {
  animation: spin 1s linear infinite;
}
</style>
