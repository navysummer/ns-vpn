<template>
  <div
    ref="scrollParentRef"
    :class="className"
    :style="{
      overflowY: 'auto',
      contain: 'strict',
      width: '100%',
      height: '100%',
      overflowAnchor: 'none',
      ...style,
    }"
  >
    <!-- Sticky group headers -->
    <div
      v-for="section in visibleGroupSections"
      :key="getItemKey(items[section.groupIndex], section.groupIndex)"
      :style="{
        position: 'sticky',
        top: '0px',
        left: 0,
        width: '100%',
        zIndex: 10,
        pointerEvents: 'auto',
      }"
    >
      <slot
        name="group"
        :item="items[section.groupIndex]"
        :index="section.groupIndex"
        :sticky="isGroupSticky(section.groupIndex, 1)"
      />
    </div>

    <!-- Regular items -->
    <div
      v-for="(item, idx) in items"
      :key="getItemKey(item, idx)"
      :data-index="idx"
      :style="{
        visibility: isGroupItem(item, idx) ? 'hidden' : 'visible',
      }"
    >
      <template v-if="isGroupItem(item, idx)">
        <slot name="group" :item="item" :index="idx" :sticky="false" />
      </template>
      <template v-else>
        <slot name="item" :item="item" :index="idx" />
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface ScrollToIndexOptions {
  align?: 'auto' | 'center' | 'end' | 'start'
  behavior?: ScrollBehavior
}

interface Props<TItem = any> {
  initialOffset?: number
  items: TItem[]
  isGroupItem: (item: TItem, index: number) => boolean
  getItemKey: (item: TItem, index: number) => string | number
  estimateGroupItemHeight: number
  estimateItemHeight: number
  className?: string
  style?: Record<string, string | number | undefined>
  overscan?: number
}

const props = defineProps<Props>()

const scrollParentRef = ref<HTMLDivElement | null>(null)

const groupIndexes = computed(() =>
  props.items.reduce<number[]>((indexes, item, index) => {
    if (props.isGroupItem(item, index)) indexes.push(index)
    return indexes
  }, []),
)

const groupSections = computed(() =>
  groupIndexes.value.map((groupIndex, index) => ({
    groupIndex,
    nextGroupIndex: groupIndexes.value[index + 1] ?? props.items.length,
  })),
)

const visibleGroupSections = computed(() => {
  const scroller = scrollParentRef.value
  if (!scroller || !groupSections.value.length) return []
  return groupSections.value
})

const findGroupSectionIndex = (itemIndex: number) => {
  let low = 0
  let high = groupIndexes.value.length - 1
  let matchedIndex = -1
  while (low <= high) {
    const middle = Math.floor((low + high) / 2)
    if (groupIndexes.value[middle] <= itemIndex) {
      matchedIndex = middle
      low = middle + 1
    } else {
      high = middle - 1
    }
  }
  return matchedIndex
}

const isGroupSticky = (groupIndex: number, tolerance = 0) => {
  const scroller = scrollParentRef.value
  if (!scroller) return false
  const groupEl = scroller.querySelector(`[data-index="${groupIndex}"]`)
  if (!groupEl) return false
  return scroller.scrollTop > (groupEl as HTMLElement).offsetTop + tolerance
}

const getScrollElement = () => scrollParentRef.value

const isItemScrolledPastStart = (index: number, tolerance = 0) => {
  return isGroupSticky(index, tolerance)
}

const scrollToIndex = (index: number, options?: ScrollToIndexOptions) => {
  if (!scrollParentRef.value) return
  const el = scrollParentRef.value.querySelector(`[data-index="${index}"]`)
  if (el) {
    el.scrollIntoView({
      block: options?.align === 'center' ? 'center' : options?.align === 'end' ? 'end' : 'nearest',
      behavior: options?.behavior as ScrollBehavior,
    })
  }
}

defineExpose({
  getScrollElement,
  isItemScrolledPastStart,
  scrollToIndex,
})
</script>
