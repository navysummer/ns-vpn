<template>
  <div
    v-if="hasError"
    :style="{
      padding: '16px',
      minHeight: fallback ? '60px' : '200px',
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center',
      justifyContent: 'center',
      border: fallback ? 'none' : '1px dashed #f44336',
      borderRadius: fallback ? '4px' : '8px',
      background: fallback ? 'rgba(244,67,54,0.05)' : 'rgba(244,67,54,0.1)',
    }"
  >
    <template v-if="!fallback">
      <svg viewBox="0 0 24 24" width="48" height="48" fill="#f44336" style="margin-bottom: 16px;"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 12c-.55 0-1-.45-1-1V8c0-.55.45-1 1-1s1 .45 1 1v5c0 .55-.45 1-1 1zm-1 3h2v-2h-2v2z"/></svg>
      <h6 style="margin: 0 0 8px 0;">{{ t('shared.feedback.errors.trafficStats') }}</h6>
      <p style="margin: 0 0 8px 0; color: var(--text-secondary); text-align: center; font-size: 14px;">
        {{ t('shared.feedback.errors.trafficStatsDescription') }}
      </p>
      <div :style="{ padding: '8px 16px', background: 'rgba(244,67,54,0.1)', borderRadius: '4px', marginBottom: '16px', maxWidth: '400px', border: '1px solid rgba(244,67,54,0.3)' }">
        <p style="margin: 0; font-size: 13px;">
          <strong>{{ t('shared.feedback.errors.label') }}:</strong>
          {{ error?.message || t('shared.feedback.errors.unknown') }}
        </p>
        <p v-if="retryCount > 0" style="margin: 4px 0 0; font-size: 12px;">
          {{ t('shared.labels.retryAttempts') }}: {{ retryCount }}/{{ maxRetries }}
        </p>
      </div>
      <div style="display: flex; gap: 8px; margin-bottom: 16px;">
        <button
          v-if="canRetry"
          class="MuiButton-root MuiButton-contained MuiButton-sizeSmall"
          @click="handleRetry"
        >
          <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor" style="margin-right: 4px;"><path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
          {{ t('shared.actions.retry') }}
        </button>
        <button
          class="MuiButton-root MuiButton-outlined MuiButton-sizeSmall"
          @click="handleRefresh"
        >
          {{ t('shared.actions.refreshPage') }}
        </button>
        <button
          class="MuiButton-root MuiButton-text MuiButton-sizeSmall"
          @click="showDetails = !showDetails"
        >
          <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor" style="margin-right: 4px;"><path d="M20 8h-2.81c-.45-.78-1.07-1.45-1.82-1.96L17 4.41 15.59 3l-2.17 2.17C12.96 5.06 12.49 5 12 5s-.96.06-1.41.17L8.41 3 7 4.41l1.62 1.63C7.88 6.55 7.26 7.22 6.81 8H4v2h2.09c-.05.33-.09.66-.09 1v1H4v2h2v1c0 .34.04.67.09 1H4v2h2.81c1.04 1.79 2.97 3 5.19 3s4.15-1.21 5.19-3H20v-2h-2.09c.05-.33.09-.66.09-1v-1h2v-2h-2v-1c0-.34-.04-.67-.09-1H20V8zm-6 8h-4v-2h4v2zm0-4h-4v-2h4v2z"/></svg>
          {{ showDetails ? t('shared.actions.hideDetails') : t('shared.actions.showDetails') }}
        </button>
      </div>
      <transition name="collapse">
        <div v-if="showDetails" :style="{ width: '100%', maxWidth: '600px', padding: '16px', background: 'var(--bg-paper)', borderRadius: '4px', border: '1px solid var(--divider-color)' }">
          <p style="margin: 0 0 8px; font-weight: 600; font-size: 14px;">{{ t('shared.feedback.errors.details') }}:</p>
          <pre style="margin: 0; white-space: pre-wrap; word-break: break-word; font-family: monospace; font-size: 0.75rem; color: var(--text-secondary);">{{ error?.stack }}</pre>
          <template v-if="errorInfo?.componentStack">
            <p style="margin: 16px 0 8px; font-weight: 600; font-size: 14px;">{{ t('shared.feedback.errors.componentStack') }}:</p>
            <pre style="margin: 0; white-space: pre-wrap; word-break: break-word; font-family: monospace; font-size: 0.75rem; color: var(--text-secondary);">{{ errorInfo.componentStack }}</pre>
          </template>
        </div>
      </transition>
    </template>
    <template v-else>
      <svg viewBox="0 0 24 24" width="20" height="20" fill="#f44336" style="margin-right: 8px;"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 12c-.55 0-1-.45-1-1V8c0-.55.45-1 1-1s1 .45 1 1v5c0 .55-.45 1-1 1zm-1 3h2v-2h-2v2z"/></svg>
      <span style="font-size: 12px;">{{ t('shared.feedback.errors.trafficUnavailable') }}</span>
    </template>
  </div>
  <slot v-else />
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'

const props = defineProps<{
  fallback?: boolean
  onError?: (error: Error, errorInfo: any) => void
}>()

const { t } = useI18n()
const hasError = ref(false)
const error = ref<Error | null>(null)
const errorInfo = ref<{ componentStack?: string } | null>(null)
const showDetails = ref(false)
const retryCount = ref(0)
const maxRetries = 3

const canRetry = computed(() => retryCount.value < maxRetries)

const handleRetry = () => {
  if (retryCount.value < maxRetries) {
    retryCount.value++
    hasError.value = false
    error.value = null
    errorInfo.value = null
    showDetails.value = false
  }
}

const handleRefresh = () => { window.location.reload() }

defineExpose({
  captureError: (err: Error, info?: any) => {
    hasError.value = true
    error.value = err
    errorInfo.value = info || null
    if (props.onError) props.onError(err, info)
  }
})
</script>
