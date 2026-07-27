<template>
  <n-modal :show="open" @update:show="onClose" :mask-closable="true" preset="dialog" :title="t('profiles.modals.logViewer.title')">
    <div style="width: 400px; height: 300px; overflow-x: hidden; user-select: text; padding-bottom: 8px;">
      <div v-for="[level, log] in logInfo" :key="`${level}-${log}`">
        <div style="color: var(--text-secondary); display: flex; align-items: center; gap: 8px;">
          <span
            class="MuiChip-root MuiChip-sizeSmall MuiChip-outlined"
            :class="level === 'error' || level === 'exception' ? 'MuiChip-colorError' : ''"
            style="font-size: 0.75rem; padding: 0 4px; border: 1px solid; border-radius: 12px;"
          >
            {{ level }}
          </span>
          <span>{{ log }}</span>
        </div>
        <hr style="margin: 4px 0; border: none; border-bottom: 1px solid var(--divider-color);" />
      </div>
      <div v-if="logInfo.length === 0">
        <BaseEmpty />
      </div>
    </div>
    <template #footer>
      <n-button @click="onClose" quaternary>{{ t('shared.actions.close') }}</n-button>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import BaseEmpty from '@/components/base/base-empty.vue'

defineProps<{
  open: boolean
  logInfo: [string, string][]
  onClose: () => void
}>()

const { t } = useI18n()
</script>
