<template>
  <div ref="parentRef" :style="{ ...style, overflow: 'auto' }">
    <div :style="{ position: 'relative' }">
      <div
        v-for="index in count"
        :key="getItemKey ? getItemKey(index - 1) : index"
        :data-index="index - 1"
      >
        <slot name="item" :index="index - 1" />
      </div>
      <div v-if="footer != null" :style="{ height: footer + 'px' }" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

interface Props {
  count: number
  estimateSize: number
  overscan?: number
  getItemKey?: (index: number) => string | number
  style?: Record<string, string | number | undefined>
  footer?: number
  onScroll?: (e: Event) => void
}

const props = defineProps<Props>()

const parentRef = ref<HTMLDivElement | null>(null)

onMounted(() => {
  if (props.onScroll && parentRef.value) {
    parentRef.value.addEventListener('scroll', props.onScroll, { passive: true })
  }
})

onUnmounted(() => {
  if (props.onScroll && parentRef.value) {
    parentRef.value.removeEventListener('scroll', props.onScroll)
  }
})

const scrollToIndex = (
  index: number,
  options?: { align?: 'start' | 'center' | 'end' | 'auto'; behavior?: ScrollBehavior },
) => {
  if (!parentRef.value) return
  const el = parentRef.value.querySelector(`[data-index="${index}"]`)
  if (el) {
    el.scrollIntoView({ block: options?.align === 'center' ? 'center' : options?.align === 'end' ? 'end' : 'start', behavior: options?.behavior as ScrollBehavior })
  }
}

const scrollTo = (options: ScrollToOptions) => {
  parentRef.value?.scrollTo(options)
}

defineExpose({ scrollToIndex, scrollTo })
</script>
