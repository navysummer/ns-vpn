<template>
  <div
    :style="boxStyle"
    @click="$emit('click', $event)"
    @dblclick="$emit('dblclick', $event)"
    @contextmenu.prevent="$emit('contextmenu', $event)"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed, inject } from 'vue'

const props = defineProps<{
  ariaSelected?: boolean
}>()

defineEmits<{
  click: [e: MouseEvent]
  dblclick: [e: MouseEvent]
  contextmenu: [e: MouseEvent]
}>()

const themeMode = inject<'light' | 'dark'>('themeMode', 'light')
const primaryColor = inject('primaryColor', '#1890ff')
const textPrimary = inject('textPrimary', '#000000')
const textSecondary = inject('textSecondary', 'rgba(0,0,0,0.6)')

const selected = computed(() => props.ariaSelected)

const boxStyle = computed(() => {
  const isLight = themeMode === 'light'
  const isSelected = selected.value
  const key = `${isLight ? 'light' : 'dark'}-${isSelected}`
  const backgroundColor = isLight ? '#ffffff' : '#282A36'

  const colorMap: Record<string, string> = {
    'light-true': textSecondary,
    'light-false': textSecondary,
    'dark-true': 'rgba(255,255,255,0.65)',
    'dark-false': 'rgba(255,255,255,0.65)',
  }

  const h2ColorMap: Record<string, string> = {
    'light-true': primaryColor,
    'light-false': textPrimary,
    'dark-true': primaryColor,
    'dark-false': textPrimary,
  }

  const borderSelectMap: Record<string, any> = {
    'light-true': { borderLeft: `3px solid ${primaryColor}`, width: 'calc(100% + 3px)', marginLeft: '-3px' },
    'light-false': { width: '100%' },
    'dark-true': { borderLeft: `3px solid ${primaryColor}`, width: 'calc(100% + 3px)', marginLeft: '-3px' },
    'dark-false': { width: '100%' },
  }

  return {
    position: 'relative',
    display: 'block',
    cursor: 'pointer',
    textAlign: 'left',
    padding: '8px 16px',
    boxSizing: 'border-box',
    backgroundColor,
    ...(borderSelectMap[key] || {}),
    borderRadius: '8px',
    color: colorMap[key] || textSecondary,
  }
})
</script>
