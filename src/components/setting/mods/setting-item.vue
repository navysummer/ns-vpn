<template>
  <li v-if="clickable" :style="{ listStyle: 'none', padding: 0 }">
    <button
      :disabled="isLoading"
      @click="handleClick"
      :style="{ display: 'flex', alignItems: 'center', width: '100%', padding: '8px 16px', border: 'none', background: 'transparent', cursor: 'pointer', textAlign: 'left' }"
    >
      <div :style="{ flex: 1 }">
        <div :style="{ display: 'flex', alignItems: 'center', fontSize: '14px' }">
          <span>{{ label }}</span>
          <span v-if="extra" style="marginLeft: 8px"><slot name="extra">{{ extra }}</slot></span>
        </div>
        <div v-if="secondary" :style="{ fontSize: '12px', color: 'var(--text-secondary-color)' }">{{ secondary }}</div>
      </div>
      <svg v-if="isLoading" viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><circle cx="12" cy="12" r="10" fill="none" stroke="currentColor" stroke-width="3" stroke-dasharray="31.4 31.4" stroke-linecap="round"/></svg>
      <svg v-else viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/></svg>
    </button>
  </li>
  <li v-else :style="{ listStyle: 'none', padding: '5px 0' }">
    <div :style="{ flex: 1 }">
      <div :style="{ display: 'flex', alignItems: 'center', fontSize: '14px' }">
        <span>{{ label }}</span>
        <span v-if="extra" style="marginLeft: 8px"><slot name="extra">{{ extra }}</slot></span>
      </div>
      <div v-if="secondary" :style="{ fontSize: '12px', color: 'var(--text-secondary-color)' }">{{ secondary }}</div>
    </div>
    <slot />
  </li>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import isAsyncFunction from '@/utils/is-async-function'

const props = defineProps<{
  label: any
  extra?: any
  secondary?: any
  onClick?: () => void | Promise<any>
}>()

const isLoading = ref(false)
const clickable = !!props.onClick

const handleClick = () => {
  if (props.onClick) {
    if (isAsyncFunction(props.onClick)) {
      isLoading.value = true
      props.onClick()!.finally(() => isLoading.value = false)
    } else {
      props.onClick()
    }
  }
}
</script>
