<template>
  <n-modal v-model:show="show" title="Tunnels">
    <n-card closable @close="show = false" :style="{ width: '600px', maxWidth: '90vw' }">
      <template #header>
        <span>{{ t('settings.sections.tunnels.title') }}</span>
      </template>
      <n-empty :description="t('shared.statuses.comingSoon')" />
      <template #footer>
        <n-button @click="show = false">{{ t('shared.actions.close') }}</n-button>
      </template>
    </n-card>
  </n-modal>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useTranslation } from '@/composables/use-i18n'

const { t } = useTranslation()
const show = ref(false)

defineExpose({
  open: () => { show.value = true },
  close: () => { show.value = false },
})
</script>
