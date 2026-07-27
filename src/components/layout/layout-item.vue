<template>
  <div
    :ref="setNodeRef"
    :style="[listItemStyle, isDragging ? { opacity: 0.78 } : {}]"
    class="layout-item-wrapper"
  >
    <div
      class="layout-item-button"
      :class="{ selected: match }"
      :style="buttonStyle"
      :title="navCollapsed ? children : undefined"
      :aria-label="navCollapsed ? children : undefined"
      v-bind="draggable ? attributes : {}"
      @pointerdown="handlePointerDown"
      @click="navigateTo"
    >
      <div
        v-if="effectiveMenuIcon === 'monochrome' || !effectiveMenuIcon"
        class="layout-item-icon"
        :style="{ color: 'var(--primary-text)', cursor: draggable ? 'grab' : 'inherit', marginLeft: '6px' }"
      >
        <n-icon v-if="icon[0]">
          <component :is="icon[0]" />
        </n-icon>
      </div>
      <div
        v-if="effectiveMenuIcon === 'colorful'"
        class="layout-item-icon"
        :style="{ cursor: draggable ? 'grab' : 'inherit' }"
      >
        <n-icon v-if="icon[1]">
          <component :is="icon[1]" />
        </n-icon>
      </div>
      <span
        class="layout-item-text"
        :style="{ textAlign: 'center', marginLeft: effectiveMenuIcon === 'disable' ? '' : '-35px' }"
      >
        {{ children }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DraggableAttributes, DraggableSyntheticListeners } from '@dnd-kit/core'
import { useRoute, useRouter } from 'vue-router'
import { computed } from 'vue'

import { useVerge } from '@/hooks/use-verge'

interface SortableProps {
  setNodeRef?: (element: HTMLElement | null) => void
  attributes?: DraggableAttributes
  listeners?: DraggableSyntheticListeners
  style?: Record<string, string | number | undefined>
  isDragging?: boolean
  disabled?: boolean
}

const props = defineProps<{
  to: string
  children: string
  icon: any[]
  sortable?: SortableProps
}>()

const { verge } = useVerge()
const { menu_icon } = verge ?? {}
const navCollapsed = verge?.collapse_navbar ?? false

const route = useRoute()
const router = useRouter()
const match = computed(() => route.path === props.to)

const effectiveMenuIcon = computed(() =>
  navCollapsed && menu_icon === 'disable' ? 'monochrome' : menu_icon,
)

const { setNodeRef, attributes, listeners, style, isDragging, disabled } =
  props.sortable ?? {}

const draggable = Boolean(props.sortable) && !disabled
const { onPointerDown, ...otherListeners } = draggable
  ? (listeners ?? ({} as DraggableSyntheticListeners))
  : ({} as DraggableSyntheticListeners)

const handlePointerDown = (event: PointerEvent) => {
  onPointerDown?.(event)
}

const navigateTo = () => {
  router.push(props.to)
}

const listItemStyle = computed(() => ({
  padding: '4px 0px',
  maxWidth: '250px',
  margin: '0 auto',
  ...(style ?? {}),
}))

const isDark = computed(() => document.documentElement.getAttribute('theme') !== 'light')

const buttonStyle = computed(() => {
  const mode = isDark.value ? 'dark' : 'light'
  const primaryMain = getComputedStyle(document.documentElement).getPropertyValue('--primary-main').trim() || '#007AFF'
  const bgcolor = mode === 'light'
    ? `${primaryMain}26`
    : `${primaryMain}59`
  const color = mode === 'light' ? '#1f1f1f' : '#ffffff'

  return {
    borderRadius: '8px',
    marginLeft: '10px',
    paddingLeft: '8px',
    paddingRight: '8px',
    marginRight: '10px',
    cursor: draggable ? 'grab' : 'pointer',
    display: 'flex',
    alignItems: 'center',
    width: 'calc(100% - 20px)',
    boxSizing: 'border-box' as const,
    border: 'none',
    background: 'transparent',
    fontFamily: 'inherit',
    fontSize: 'inherit',
    lineHeight: '48px',
    minHeight: '48px',
    textDecoration: 'none',
    color: 'var(--primary-text)',
    ...(match.value ? { backgroundColor: bgcolor } : {}),
  }
})
</script>

<style scoped>
.layout-item-wrapper {
  padding: 4px 0px;
  max-width: 250px;
  margin: 0 auto;
}

.layout-item-button {
  border-radius: 8px;
  margin-left: 10px;
  padding-left: 8px;
  padding-right: 8px;
  margin-right: 10px;
  display: flex;
  align-items: center;
  width: calc(100% - 20px);
  box-sizing: border-box;
  border: none;
  background: transparent;
  font-family: inherit;
  line-height: 48px;
  min-height: 48px;
  text-decoration: none;
  color: var(--primary-text);
}

.layout-item-button.selected {
  background-color: var(--background-color-alpha);
}

.layout-item-button:hover {
  background-color: var(--background-color-alpha);
}

.layout-item-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 36px;
}

.layout-item-text {
  flex: 1;
  font-weight: 500;
  color: var(--primary-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
