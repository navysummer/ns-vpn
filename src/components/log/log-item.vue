<template>
  <div class="log-item">
    <div>
      <span class="time" v-html="renderHighlightText(value.time || '')" />
      <span
        class="type"
        :data-type="(value.type || '').toLowerCase()"
        v-html="renderHighlightText(value.type)"
      />
    </div>
    <div>
      <span class="data" v-html="renderHighlightText(value.payload)" />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { SearchState } from '@/components/base'

interface Props {
  value: ILogItem
  searchState?: SearchState
}

defineProps<Props>()

const escapeHtml = (text: string) => {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
}

const renderHighlightText = (text: string, searchState?: SearchState): string => {
  if (!searchState?.text.trim()) return escapeHtml(text)

  try {
    const searchText = searchState.text
    let pattern: string

    if (searchState.useRegularExpression) {
      try {
        new RegExp(searchText)
        pattern = searchText
      } catch {
        pattern = searchText.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
      }
    } else {
      const escaped = searchText.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
      pattern = searchState.matchWholeWord ? `\\b${escaped}\\b` : escaped
    }

    const flags = searchState.matchCase ? 'g' : 'gi'
    const regex = new RegExp(pattern, flags)

    const escapedText = escapeHtml(text)
    const escapedSearch = searchText.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')

    // Use regex on escaped text to find matches, but we need the original flags for case sensitivity
    // Build a pattern from the original search terms
    const escapedPattern = pattern
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')

    const resultRegex = new RegExp(`(${escapedPattern})`, flags)
    return escapedText.replace(resultRegex, '<span class="highlight">$1</span>')
  } catch {
    return escapeHtml(text)
  }
}
</script>

<style scoped>
.log-item {
  padding: 8px 0;
  margin: 0 12px;
  line-height: 1.35;
  border-bottom: 1px solid var(--divider-color);
  font-size: 0.875rem;
  user-select: text;
}

.log-item :deep(.time) {
  color: var(--secondary-text);
}

.log-item :deep(.type) {
  display: inline-block;
  margin-left: 8px;
  text-align: center;
  border-radius: 2px;
  text-transform: uppercase;
  font-weight: 600;
}

.log-item :deep(.type[data-type="error"]),
.log-item :deep(.type[data-type="err"]) {
  color: var(--error-color);
}

.log-item :deep(.type[data-type="warning"]),
.log-item :deep(.type[data-type="warn"]) {
  color: var(--warning-color);
}

.log-item :deep(.type[data-type="info"]),
.log-item :deep(.type[data-type="inf"]) {
  color: var(--primary-main);
}

.log-item :deep(.data) {
  color: var(--text-primary);
  overflow-wrap: anywhere;
}

.log-item :deep(.highlight) {
  background-color: var(--highlight-bg, #ffeb3b90);
  border-radius: 2px;
  padding: 0 2px;
}

[theme="dark"] .log-item :deep(.highlight) {
  background-color: #ffeb3b40;
}
</style>
