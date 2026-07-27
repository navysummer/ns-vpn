<template>
  <div
    class="MuiListItem-root MuiListItem-dense MuiListItem-padding"
    :style="itemStyle"
  >
    <div
      :style="{ cursor: sortable ? 'move' : '', flex: 1, minWidth: 0 }"
    >
      <div
        :style="{
          fontSize: '15px',
          fontWeight: 700,
          lineHeight: 1.5,
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          whiteSpace: 'nowrap',
          textDecoration: type === 'delete' ? 'line-through' : ''
        }"
        :title="ruleContent || '-'"
      >
        {{ ruleContent || '-' }}
      </div>
      <div
        style="
          width: 62%;
          overflow: hidden;
          display: flex;
          justify-content: space-between;
          padding-top: 2px;
          color: #ccc;
        "
      >
        <span
          :style="{
            display: 'inline-block',
            border: '1px solid',
            borderColor: 'rgba(25, 118, 210, 0.5)',
            color: 'rgba(25, 118, 210, 0.8)',
            borderRadius: 4,
            fontSize: 10,
            padding: '0 4px',
            lineHeight: 1.5,
            marginRight: 8
          }"
        >
          {{ ruleType }}
        </span>
        <span style="font-size: 13px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; color: var(--text-secondary);">
          {{ proxyPolicy }}
        </span>
      </div>
    </div>
    <button class="MuiIconButton-root MuiIconButton-sizeSmall" @click="$emit('delete')">
      <svg v-if="type === 'delete'" viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12.5 8c-2.65 0-5.05.99-6.9 2.6L2 7v9h9l-3.62-3.62c1.39-1.16 3.16-1.88 5.12-1.88 3.54 0 6.55 2.31 7.6 5.5l2.37-.78C21.08 11.03 17.15 8 12.5 8z"/></svg>
      <svg v-else viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed, inject } from 'vue'

const props = defineProps<{
  type: 'prepend' | 'original' | 'delete' | 'append'
  ruleRaw: string
  onDelete: () => void
}>()

defineEmits<{
  delete: []
}>()

const sortable = props.type === 'prepend' || props.type === 'append'

const rule = computed(() => props.ruleRaw.replace(',no-resolve', ''))
const ruleType = computed(() => rule.value.match(/^[^,]+/)?.[0] ?? '')
const proxyPolicy = computed(() => rule.value.match(/[^,]+$/)?.[0] ?? '')
const ruleContent = computed(() => {
  const r = rule.value
  const type = ruleType.value
  const policy = proxyPolicy.value
  return r.slice(type.length + 1, -policy.length - 1)
})

const themeMode = inject('themeMode', 'light')

const itemStyle = computed(() => {
  let background: string
  if (props.type === 'original') {
    background = themeMode === 'dark'
      ? 'rgba(30, 30, 30, 0.3)'
      : 'rgba(189, 189, 189, 0.3)'
  } else if (props.type === 'delete') {
    background = 'rgba(244, 67, 54, 0.3)'
  } else {
    background = 'rgba(76, 175, 80, 0.3)'
  }
  return {
    position: 'relative',
    background,
    height: '100%',
    margin: '8px 0',
    borderRadius: '8px',
    display: 'flex',
    alignItems: 'center',
    padding: '4px 16px',
  }
})
</script>
