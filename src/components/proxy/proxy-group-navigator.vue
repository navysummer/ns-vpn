<template>
  <div v-if="processedGroups.length > 0" :style="containerStyle">
    <n-tooltip
      v-for="{ name, displayChar } in processedGroups"
      :key="name"
      :title="name"
      placement="left"
      :trigger="'hover'"
    >
      <template #trigger>
        <button
          :style="buttonStyle"
          @click="handleGroupClick(name)"
          @mouseenter="handleGroupHover(name)"
          @focus="handleGroupHover(name)"
          @mouseleave="handleButtonLeave"
          @blur="handleButtonLeave"
        >
          {{ displayChar }}
        </button>
      </template>
    </n-tooltip>
  </div>
</template>

<script lang="ts">
export const DEFAULT_HOVER_DELAY = 280
</script>

<script setup lang="ts">
import { ref, computed, watch, onUnmounted } from 'vue'
import { NTooltip } from 'naive-ui'

const props = withDefaults(defineProps<{
  proxyGroupNames: string[]
  onGroupLocation: (groupName: string) => void
  enableHoverJump?: boolean
  hoverDelay?: number
}>(), {
  enableHoverJump: true,
  hoverDelay: 280,
})

const lastHovered = ref<string | null>(null)
let hoverTimer: ReturnType<typeof setTimeout> | null = null

const clearHoverTimer = () => {
  if (hoverTimer) { clearTimeout(hoverTimer); hoverTimer = null }
}

watch(() => props.enableHoverJump, () => {
  if (!props.enableHoverJump) { clearHoverTimer(); lastHovered.value = null }
})

onUnmounted(() => clearHoverTimer())

const getGroupDisplayChar = (groupName: string) => {
  if (!groupName) return '?'
  return Array.from(groupName)[0] || '?'
}

const processedGroups = computed(() =>
  props.proxyGroupNames
    .filter(name => name && name.trim())
    .map(name => ({ name, displayChar: getGroupDisplayChar(name) })),
)

const handleGroupClick = (groupName: string) => {
  clearHoverTimer()
  lastHovered.value = groupName
  props.onGroupLocation(groupName)
}

const handleGroupHover = (groupName: string) => {
  if (!props.enableHoverJump || lastHovered.value === groupName) return
  clearHoverTimer()
  hoverTimer = setTimeout(() => {
    hoverTimer = null
    lastHovered.value = groupName
    props.onGroupLocation(groupName)
  }, props.hoverDelay)
}

const handleButtonLeave = () => {
  clearHoverTimer()
  lastHovered.value = null
}

const containerStyle = {
  position: 'absolute' as const,
  right: '2px',
  top: '50%',
  transform: 'translateY(-50%)',
  zIndex: 10,
  display: 'flex',
  flexDirection: 'column' as const,
  gap: '2px',
  borderRadius: '4px',
  padding: '2px',
  maxHeight: '70vh',
  overflowY: 'auto' as const,
  minWidth: 'auto',
}

const buttonStyle = {
  minWidth: '28px',
  minHeight: '28px',
  width: '28px',
  height: '28px',
  fontSize: '12px',
  fontWeight: 600,
  padding: 0,
  borderRadius: '4px',
  color: 'var(--text-secondary-color)',
  textAlign: 'center' as const,
  justifyContent: 'center',
  textTransform: 'none' as const,
  border: 'none',
  background: 'transparent',
  cursor: 'pointer',
}
</script>
