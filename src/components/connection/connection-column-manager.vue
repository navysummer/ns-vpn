<template>
  <n-modal :show="open" @update:show="onClose" :mask-closable="true" preset="card" style="max-width: 400px;" :title="t('connections.components.columnManager.title')">
    <div style="padding-top: 8px;">
      <div style="display: flex; flex-direction: column; gap: 8px;">
        <div
          v-for="column in columns"
          :key="column.id"
          style="display: flex; align-items: center; gap: 8px; padding: 4px 8px; border-radius: 4px; border: 1px solid var(--divider-color);"
        >
          <label style="display: flex; align-items: center; gap: 4px; cursor: pointer;">
            <input
              type="checkbox"
              :checked="column.visible"
              :disabled="column.visible && visibleCount <= 1"
              @change="column.toggleVisibility(($event.target as HTMLInputElement).checked)"
            />
          </label>
          <span style="flex: 1; font-size: 14px;">{{ column.label }}</span>
          <span style="cursor: grab; color: var(--text-secondary);">
            <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M11 18c0 1.1-.9 2-2 2s-2-.9-2-2 .9-2 2-2 2 .9 2 2zm-2-8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0-6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm6 4c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"/></svg>
          </span>
        </div>
      </div>
    </div>
    <template #footer>
      <div style="display: flex; justify-content: space-between; width: 100%;">
        <n-button text @click="handleReset">{{ t('shared.actions.resetToDefault') }}</n-button>
        <n-button @click="handleClose">{{ t('shared.actions.close') }}</n-button>
      </div>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

export interface ConnectionColumnOption {
  id: string
  label: string
  visible: boolean
  toggleVisibility: (visible: boolean) => void
}

const props = defineProps<{
  open: boolean
  columns: ConnectionColumnOption[]
  onClose: () => void
  onOrderChange: (order: string[]) => void
  onReset: () => void
}>()

const emit = defineEmits<{
  close: []
  orderChange: [order: string[]]
  reset: []
}>()

const { t } = useI18n()
const visibleCount = computed(() => props.columns.filter(c => c.visible).length)

const handleClose = () => props.onClose()
const handleReset = () => props.onReset()
</script>
