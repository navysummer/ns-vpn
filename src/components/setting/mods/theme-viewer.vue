<template>
  <n-modal v-model:show="show" title="Theme">
    <n-card closable @close="show = false" :style="{ width: '600px', maxWidth: '90vw' }">
      <template #header>
        <span>{{ t('settings.modals.themeEditor.title') }}</span>
      </template>
      <n-thing>
        <template #description>
          <div :style="{ display: 'flex', gap: '12px', flexDirection: 'column' }">
            <n-button @click="handleEditTheme('light')">
              {{ t('settings.sections.appearance.light') }}
            </n-button>
            <n-button @click="handleEditTheme('dark')">
              {{ t('settings.sections.appearance.dark') }}
            </n-button>
          </div>
        </template>
      </n-thing>
      <template #footer>
        <n-button @click="show = false">{{ t('shared.actions.close') }}</n-button>
      </template>
    </n-card>
  </n-modal>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useTranslation } from '@/composables/use-i18n'
import { showNotice } from '@/services/notice-service'

const { t } = useTranslation()
const show = ref(false)

const handleEditTheme = (mode: string) => {
  showNotice.info('settings.modals.themeEditor.editHint')
}

defineExpose({
  open: () => { show.value = true },
  close: () => { show.value = false },
})
</script>
