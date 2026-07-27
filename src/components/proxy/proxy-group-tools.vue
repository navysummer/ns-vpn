<template>
  <div :style="{ display: 'flex', justifyContent: 'flex-end', alignItems: 'center', gap: '4px', height: '36px', flex: 1, marginLeft: '16px' }">
    <div v-if="textState === 'filter'" :style="{ flex: '1 1 auto' }">
      <BaseSearchBox
        :defaultValue="filterText"
        :matchCase="filterMatchCase"
        :matchWholeWord="filterMatchWholeWord"
        :useRegularExpression="filterUseRegularExpression"
        @click="(e: MouseEvent) => { e.preventDefault(); e.stopPropagation() }"
        @search="(text: string, state: any) => applyFilter(state)"
      />
    </div>
    <div v-if="textState === 'url'" :style="{ flex: '1 1 auto' }">
      <n-input
        size="small"
        :value="testUrl"
        :placeholder="t('proxies.page.placeholders.delayCheckUrl')"
        @click="(e: MouseEvent) => { e.preventDefault(); e.stopPropagation() }"
        @update:value="(val: string) => onHeadState({ testUrl: val })"
      />
    </div>
    <n-button quaternary circle size="small" :title="t('proxies.page.tooltips.locate')" @click="handleLocation">
      <template #icon>
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm8.94 3c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06C6.83 3.52 3.52 6.83 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c4.17-.46 7.48-3.77 7.94-7.94H23v-2h-2.06zM12 19c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z"/></svg>
      </template>
    </n-button>
    <n-button quaternary circle size="small" :title="t('proxies.page.tooltips.delayCheck')" @click="handleDelayCheck">
      <template #icon>
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M15 9H9v2h6V9zm-2 4H9v2h4v-2zm5-10H6c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H6V5h12v14z"/></svg>
      </template>
    </n-button>
    <n-button quaternary circle size="small" :title="sortTooltip" @click="handleSort">
      <template #icon>
        <svg v-if="sortType === 0" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M3 18h6v-2H3v2zM3 6v2h18V6H3zm0 7h12v-2H3v2z"/></svg>
        <svg v-else-if="sortType === 1" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"/></svg>
        <svg v-else viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M3 5h2v2H3V5zm4 2h14v2H7V7zm-4 6h2v-2H3v2zm4 2h14v-2H7v2zm-4 6h2v-2H3v2zm4 2h14v-2H7v2z"/></svg>
      </template>
    </n-button>
    <n-button quaternary circle size="small" :title="t('proxies.page.tooltips.delayCheckUrl')" @click="handleToggleUrl">
      <template #icon>
        <svg v-if="textState === 'url'" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M1 9l2 2c4.97-4.97 13.03-4.97 18 0l2-2C16.93 2.93 7.08 2.93 1 9zm8 8l3 3 3-3c-1.65-1.66-4.34-1.66-6 0zm-4-4l2 2c2.76-2.76 7.24-2.76 10 0l2-2C15.14 9.14 8.87 9.14 5 13z"/></svg>
        <svg v-else viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
      </template>
    </n-button>
    <n-button quaternary circle size="small" :title="showType ? t('proxies.page.tooltips.showBasic') : t('proxies.page.tooltips.showDetail')" @click="handleToggleType">
      <template #icon>
        <svg v-if="showType" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z"/></svg>
        <svg v-else viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27z"/></svg>
      </template>
    </n-button>
    <n-button quaternary circle size="small" :title="t('proxies.page.tooltips.filter')" @click="handleToggleFilter">
      <template #icon>
        <svg v-if="textState === 'filter'" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M10 18h4v-2h-4v2zM3 6v2h18V6H3zm3 7h12v-2H6v2z"/></svg>
        <svg v-else viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M10 18h4v-2h-4v2zM3 6v2h18V6H3zm3 7h12v-2H6v2z"/></svg>
      </template>
    </n-button>
  </div>
</template>

<script setup lang="ts">
import { computed, watch, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NInput } from 'naive-ui'

import { BaseSearchBox } from '@/components/base'
import { useVerge } from '@/hooks/use-verge'
import delayManager from '@/services/delay'

const props = defineProps<{
  sx?: any
  url?: string
  groupName: string
  headState: any
  onLocation: () => void
  onCheckDelay: () => void
  onHeadState: (val: any) => void
}>()

const { t } = useI18n()
const { verge } = useVerge()
const defaultLatencyUrl = computed(() => verge.value?.default_latency_test?.trim() || 'http://cp.cloudflare.com/generate_204')

const { showType, sortType, filterText, textState, testUrl, filterMatchCase, filterMatchWholeWord, filterUseRegularExpression } = computed(() => props.headState)

watch([() => props.groupName, () => testUrl.value, defaultLatencyUrl, () => props.url], () => {
  delayManager.setUrl(props.groupName, testUrl.value?.trim() || props.url || defaultLatencyUrl.value)
}, { immediate: true })

let debounceTimer: number | null = null
const applyFilter = (state: any) => {
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = window.setTimeout(() => {
    props.onHeadState({
      filterText: state.text,
      filterMatchCase: state.matchCase,
      filterMatchWholeWord: state.matchWholeWord,
      filterUseRegularExpression: state.useRegularExpression,
    })
  }, 600)
}

watch(() => props.headState.textState, (val) => {
  if (val !== 'filter' && debounceTimer) { clearTimeout(debounceTimer); debounceTimer = null }
})

onUnmounted(() => { if (debounceTimer) clearTimeout(debounceTimer) })

const sortTooltip = computed(() => {
  if (sortType.value === 0) return t('proxies.page.tooltips.sortDefault')
  if (sortType.value === 1) return t('proxies.page.tooltips.sortDelay')
  return t('proxies.page.tooltips.sortName')
})

const handleLocation = () => props.onLocation()
const handleDelayCheck = () => {
  if (testUrl.value?.trim() && textState.value !== 'filter') props.onHeadState({ textState: 'url' })
  props.onCheckDelay()
}
const handleSort = () => props.onHeadState({ sortType: ((sortType.value + 1) % 3) as any })
const handleToggleUrl = () => props.onHeadState({ textState: textState.value === 'url' ? null : 'url' })
const handleToggleType = () => props.onHeadState({ showType: !showType.value })
const handleToggleFilter = () => props.onHeadState({ textState: textState.value === 'filter' ? null : 'filter' })
</script>
