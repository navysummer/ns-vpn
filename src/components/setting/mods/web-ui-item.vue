<template>
  <div v-if="editing || onlyEdit" class="web-ui-item-editing">
    <div class="web-ui-item-row">
      <n-input
        v-model:value="editValue"
        size="small"
        :placeholder="t('settings.modals.webUI.messages.supportedPlaceholders')"
        autocomplete="new-password"
      />
      <n-button size="small" quaternary circle @click="save">
        <template #icon><n-icon><CheckmarkOutline /></n-icon></template>
      </n-button>
      <n-button size="small" quaternary circle @click="cancel">
        <template #icon><n-icon><CloseOutline /></n-icon></template>
      </n-button>
    </div>
    <n-divider />
  </div>
  <div v-else class="web-ui-item-display">
    <div class="web-ui-item-row">
      <span
        class="web-ui-item-text"
        :title="value"
        :class="{ 'placeholder-dim': !value }"
      >
        <template v-for="(part, index) in highlightedParts" :key="index">
          <span v-if="isPlaceholder(part)" class="placeholder">{{ part }}</span>
          <span v-else>{{ part }}</span>
        </template>
      </span>
      <n-button size="small" quaternary circle @click="$emit('openUrl', value)">
        <template #icon><n-icon><OpenOutline /></n-icon></template>
      </n-button>
      <n-button size="small" quaternary circle @click="startEdit">
        <template #icon><n-icon><CreateOutline /></n-icon></template>
      </n-button>
      <n-button size="small" quaternary circle @click="$emit('delete')">
        <template #icon><n-icon><TrashOutline /></n-icon></template>
      </n-button>
    </div>
    <n-divider />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import i18n from 'i18next'
import {
  CheckmarkOutline,
  CloseOutline,
  CreateOutline,
  OpenOutline,
  TrashOutline,
} from '@vicons/ionicons5'

const props = defineProps<{
  value?: string
  onlyEdit?: boolean
  onChange: (value?: string) => void
  onOpenUrl?: (value?: string) => void
  onDelete?: () => void
  onCancel?: () => void
}>()

const emit = defineEmits<{
  change: [value?: string]
  openUrl: [value?: string]
  delete: []
}>()

const t = i18n.t.bind(i18n)
const editing = ref(false)
const editValue = ref(props.value)

const highlightedParts = computed(() => {
  const placeholderRegex = /(%host|%port|%secret)/g
  if (!props.value) return ['NULL']
  return props.value.split(placeholderRegex).filter((part) => part !== '')
})

const isPlaceholder = (part: string) =>
  part === '%host' || part === '%port' || part === '%secret'

const startEdit = () => {
  editing.value = true
  editValue.value = props.value
}

const save = () => {
  props.onChange(editValue.value)
  editing.value = false
}

const cancel = () => {
  props.onCancel?.()
  editing.value = false
}
</script>

<style scoped>
.web-ui-item-row {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 8px 0;
}
.web-ui-item-text {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.placeholder-dim {
  color: #888;
}
.placeholder {
  color: var(--primary-color, #007aff);
}
</style>
