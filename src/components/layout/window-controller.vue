<template>
  <template v-if="OS === 'linux' && !maximized">
    <div
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

  <div
    class="window-controls"
  >
    <!-- macOS -->
    <template v-if="OS === 'macos'">
      <n-button quaternary circle size="small" @click="close">
        <template #icon><Close /></template>
      </n-button>
      <n-button quaternary circle size="small" @click="minimize">
        <template #icon><Remove /></template>
      </n-button>
      <n-button quaternary circle size="small" @click="toggleMaximize">
        <template #icon>
          <Contract v-if="maximized" />
          <Expand v-else />
        </template>
      </n-button>
    </template>

    <!-- Windows -->
    <template v-else-if="OS === 'windows'">
      <n-button quaternary circle size="small" @click="minimize">
        <template #icon><Remove /></template>
      </n-button>
      <n-button quaternary circle size="small" @click="toggleMaximize">
        <template #icon>
          <Contract v-if="maximized" />
          <Expand v-else />
        </template>
      </n-button>
      <n-button quaternary circle size="small" class="close-button" @click="close">
        <template #icon><Close /></template>
      </n-button>
    </template>

    <!-- Linux -->
    <template v-else-if="OS === 'linux'">
      <n-button quaternary circle size="small" @click="minimize">
        <template #icon><Remove /></template>
      </n-button>
      <n-button quaternary circle size="small" @click="toggleMaximize">
        <template #icon>
          <Contract v-if="maximized" />
          <Expand v-else />
        </template>
      </n-button>
      <n-button quaternary circle size="small" class="close-button" @click="close">
        <template #icon><Close /></template>
      </n-button>
    </template>
  </div>
</template>

<script setup lang="ts">
import { Close, Remove, Contract, Expand } from '@vicons/ionicons5'
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

const OS = getSystem()
const { currentWindow, maximized, minimize, close, toggleFullscreen, toggleMaximize } = useWindowControls()

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

<style scoped>
.window-controls {
  display: flex;
  gap: 8px;
  align-items: center;
}

.window-controls :deep(.n-button) {
  cursor: default;
}

.close-button:hover {
  background-color: red !important;
  color: white !important;
}
</style>
