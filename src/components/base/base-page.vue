<template>
  <BaseErrorBoundary>
    <div class="base-page">
      <header data-tauri-drag-region="true" style="user-select: none">
        <span
          data-tauri-drag-region="true"
          style="font-size: 20px; font-weight: 700"
        >
          {{ title }}
        </span>
        <slot name="header" />
      </header>

      <div
        :class="full ? 'base-container no-padding' : 'base-container'"
        :style="{ backgroundColor: isDark ? '#1e1f27' : '#ffffff' }"
      >
        <section
          :style="{
            backgroundColor: isDark ? '#1e1f27' : 'var(--background-color)',
          }"
        >
          <div class="base-content" :style="contentStyle">
            <slot />
          </div>
        </section>
      </div>
    </div>
  </BaseErrorBoundary>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import BaseErrorBoundary from './base-error-boundary.vue'

interface Props {
  title?: string
  header?: any
  contentStyle?: Record<string, string | number | undefined>
  full?: boolean
}

const props = defineProps<Props>()
const isDark = computed(() => document.documentElement.getAttribute('theme') !== 'light')
</script>
