<template>
  <div
    v-if="show"
    class="window-resize-handles"
    data-tauri-drag-region="false"
    aria-hidden="true"
  >
    <div
      v-for="handle in RESIZE_HANDLES"
      :key="handle.direction"
      :class="`window-resize-handle window-resize-handle--${handle.position}`"
      :data-resize-direction="handle.direction"
      @pointerdown="startResizeDragging"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useWindowControls } from '@/hooks/use-window'
import getSystem from '@/utils/get-system'

const RESIZE_HANDLES = [
  { direction: 'North', position: 'north' },
  { direction: 'NorthEast', position: 'north-east' },
  { direction: 'East', position: 'east' },
  { direction: 'SouthEast', position: 'south-east' },
  { direction: 'South', position: 'south' },
  { direction: 'SouthWest', position: 'south-west' },
  { direction: 'West', position: 'west' },
  { direction: 'NorthWest', position: 'north-west' },
] as const

const { currentWindow, maximized } = useWindowControls()
const show = computed(() => getSystem() === 'linux' && !maximized)

const startResizeDragging = (event: PointerEvent) => {
  if (event.button !== 0) return
  event.preventDefault()
  const target = event.currentTarget as HTMLElement
  const direction = target.dataset.resizeDirection
  const handle = RESIZE_HANDLES.find((item) => item.direction === direction)
  if (handle) {
    void currentWindow
      .startResizeDragging(handle.direction)
      .catch((error: unknown) =>
        console.warn('[WindowResizeHandles] 调整窗口大小失败:', error),
      )
  }
}
</script>
