<template>
  <div v-if="hasError" role="alert" style="padding: 16px">
    <h4>{{ t('shared.feedback.errors.unexpected') }}</h4>
    <pre>{{ errorMessage }}</pre>
    <details :title="t('shared.feedback.errors.stack')">
      <summary>{{ t('shared.feedback.errors.stack') }}</summary>
      <pre>{{ errorStack }}</pre>
    </details>
  </div>
  <slot v-else />
</template>

<script setup lang="ts">
import { ref, onErrorCaptured } from 'vue'
import { useTranslation } from '@/composables/use-i18n'
const { t } = useTranslation()

const hasError = ref(false)
const errorMessage = ref('')
const errorStack = ref<string | undefined>()

onErrorCaptured((err: unknown) => {
  hasError.value = true
  errorMessage.value = err instanceof Error ? err.message : String(err)
  errorStack.value = err instanceof Error ? err.stack : undefined
  return false
})
</script>
