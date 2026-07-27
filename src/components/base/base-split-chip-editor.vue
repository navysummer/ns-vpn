<template>
  <div v-if="renderHeader" @click="renderHeader(modeToggle)" />
  <div v-else-if="showModeToggle">
    {{ modeToggle }}
  </div>

  <template v-if="mode === 'visual'">
    <div style="padding: 0 2px 5px">
      <div class="chip-list">
        <template v-if="items.length">
          <n-tag
            v-for="(item, index) in items"
            :key="item.key"
            size="small"
            closable
            :disabled="disabled"
            @close="handleRemoveItem(index)"
          >
            {{ item.value }}
          </n-tag>
        </template>
        <span v-else style="font-size: 14px; color: var(--secondary-text)">
          {{ resolvedLabels.empty }}
        </span>
      </div>
      <div style="display: flex; gap: 8px; margin-top: 8px; align-items: center">
        <n-input
          :value="draft"
          size="small"
          :disabled="disabled"
          :placeholder="placeholder"
          :status="error ? 'error' : undefined"
          style="flex: 1"
          @input="draft = $event"
          @keydown.enter.prevent="handleAddDraft"
        />
        <n-button
          size="small"
          secondary
          :disabled="disabled || !draft.trim()"
          @click="handleAddDraft"
        >
          {{ resolvedLabels.add }}
        </n-button>
      </div>
      <div v-if="helperText" :style="{ color: error ? 'var(--error-color)' : 'var(--secondary-text)', fontSize: '12px', marginTop: '4px' }">
        {{ helperText }}
      </div>
    </div>
  </template>
  <template v-else>
    <n-input
      type="textarea"
      :value="value"
      size="small"
      :disabled="disabled"
      :status="error ? 'error' : undefined"
      :rows="rows"
      style="width: 100%"
      @input="onChange($event)"
    />
  </template>
</template>

<script lang="ts">
const DEFAULT_SPLIT_PATTERN = /[,\n;\r]+/
</script>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useTranslation } from '@/composables/use-i18n'
const { t } = useTranslation()

type BaseSplitChipEditorMode = 'visual' | 'advanced'

interface BaseSplitChipEditorProps {
  value?: string
  onChange: (value: string) => void
  disabled?: boolean
  error?: boolean
  helperText?: string
  placeholder?: string
  rows?: number
  separator?: string
  splitPattern?: RegExp
  defaultMode?: BaseSplitChipEditorMode
  showModeToggle?: boolean
  ariaLabel?: string
  addLabel?: string
  emptyLabel?: string
  modeLabels?: Partial<Record<BaseSplitChipEditorMode, string>>
  renderHeader?: (modeToggle: any) => any
}

const splitValue = (value: string, splitPattern: RegExp) =>
  value
    .split(splitPattern)
    .map((item) => item.trim())
    .filter(Boolean)

const props = withDefaults(defineProps<BaseSplitChipEditorProps>(), {
  value: '',
  disabled: false,
  error: false,
  rows: 4,
  separator: ',',
  splitPattern: DEFAULT_SPLIT_PATTERN,
  defaultMode: 'visual',
  showModeToggle: true,
})

const mode = ref<BaseSplitChipEditorMode>(props.defaultMode)
const draft = ref('')

const resolvedLabels = computed(() => ({
  visual: props.modeLabels?.visual ?? t('shared.editorModes.visualization'),
  advanced: props.modeLabels?.advanced ?? t('shared.editorModes.advanced'),
  add: props.addLabel ?? t('shared.actions.new'),
  empty: props.emptyLabel ?? t('shared.statuses.empty'),
}))

const values = computed(() => splitValue(props.value, props.splitPattern))

const items = computed(() => {
  const counts = new Map<string, number>()
  return values.value.map((item) => {
    const nextCount = (counts.get(item) ?? 0) + 1
    counts.set(item, nextCount)
    return {
      key: `${item}-${nextCount}`,
      value: item,
    }
  })
})

const handleAddDraft = () => {
  const nextValues = splitValue(draft.value, props.splitPattern)
  if (!nextValues.length) return
  const nextValue = [...values.value, ...nextValues].join(props.separator)
  props.onChange(nextValue)
  draft.value = ''
}

const handleRemoveItem = (index: number) => {
  const nextValue = values.value.filter((_, itemIndex) => itemIndex !== index)
  props.onChange(nextValue.join(props.separator))
}

const nextMode = computed(() => mode.value === 'visual' ? 'advanced' : 'visual')
const modeToggle = computed(() => ({
  label: nextMode.value === 'visual' ? resolvedLabels.value.visual : resolvedLabels.value.advanced,
  onClick: () => {
    mode.value = nextMode.value
    if (nextMode.value === 'visual') {
      draft.value = ''
    }
  },
}))
</script>

<style scoped>
.chip-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  min-height: 32px;
}
</style>
