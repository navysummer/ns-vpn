<template>
  <div
    class="rule-item"
  >
    <span
      style="
        color: var(--secondary-text);
        font-size: 0.875rem;
        line-height: 2;
        min-width: 30px;
        margin-right: 18px;
        text-align: center;
      "
    >
      {{ value.lineNo }}
    </span>

    <div style="user-select: text;">
      <span style="display: block; font-size: 1rem; font-weight: 600; color: var(--text-primary);">
        {{ value.payload || '-' }}
      </span>

      <span
        style="
          display: inline-block;
          font-size: 0.875rem;
          color: var(--secondary-text);
          margin-right: 24px;
          min-width: 120px;
        "
      >
        {{ value.type }}
      </span>

      <span
        style="display: inline-block; font-size: 0.875rem;"
        :style="{ color: parseColor(value.proxy) }"
      >
        {{ value.proxy }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Rule } from 'tauri-plugin-mihomo-api'

interface Props {
  value: Rule & { lineNo: number }
}

defineProps<Props>()

const COLOR_PATHS = [
  'primary',
  'secondary',
  'info',
  'warning',
  'success',
] as const

const CSS_VAR_MAP: Record<string, string> = {
  primary: 'var(--primary-main)',
  secondary: 'var(--secondary-text)',
  'info.main': 'var(--primary-main)',
  'warning.main': 'var(--warning-color)',
  'success.main': 'var(--success-color)',
  'error.main': 'var(--error-color)',
  'text.primary': 'var(--text-primary)',
}

const parseColor = (text: string) => {
  if (text === 'REJECT' || text === 'REJECT-DROP') return 'var(--error-color)'
  if (text === 'DIRECT') return 'var(--text-primary)'

  let sum = 0
  for (let i = 0; i < text.length; i++) {
    sum += text.charCodeAt(i)
  }
  const key = COLOR_PATHS[sum % COLOR_PATHS.length]
  return CSS_VAR_MAP[key] || 'var(--text-primary)'
}
</script>

<style scoped>
.rule-item {
  display: flex;
  padding: 4px 16px;
  color: var(--text-primary);
  border-bottom: 1px solid var(--divider-color);
}
</style>
