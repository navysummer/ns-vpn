<template>
  <transition name="fade">
    <n-button
      v-if="show"
      quaternary
      circle
      :style="buttonStyle"
      @click="onClick"
    >
      <template #icon>
        <ChevronUp />
      </template>
    </n-button>
  </transition>
</template>

<script setup lang="ts">
import type { CSSProperties } from 'vue'
import { computed } from 'vue'
import { ChevronUp } from '@vicons/ionicons5'

interface Props {
  onClick: () => void
  show: boolean
  sx?: Record<string, string | number>
}

const props = defineProps<Props>()

const emit = defineEmits<{
  click: []
}>()

const onClick = () => {
  emit('click')
  props.onClick()
}

const buttonStyle = computed((): CSSProperties => ({
  position: 'absolute',
  bottom: '20px',
  right: '20px',
  backgroundColor: 'rgba(255,255,255,0.1)',
  visibility: props.show ? 'visible' : 'hidden',
  ...(props.sx as CSSProperties ?? {}),
}))
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
