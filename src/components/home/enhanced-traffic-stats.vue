<template>
  <div>
    <div :style="{ display: 'flex', flexDirection: 'column', gap: '8px' }">
      <div v-if="trafficGraph && pageVisible" :style="{ height: '130px', cursor: 'pointer', border: '1px solid var(--border-color)', borderRadius: '12px', overflow: 'hidden' }" @click="trafficRef?.toggleStyle()">
        <div :style="{ height: '100%', position: 'relative' }">
          <EnhancedCanvasTrafficGraph ref="trafficRef" />
        </div>
      </div>
      <div :style="{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '8px' }">
        <div
          v-for="card in statCards"
          :key="card.title"
          :style="{
            display: 'flex',
            alignItems: 'center',
            borderRadius: '12px',
            backgroundColor: card.bgColor,
            border: `1px solid ${card.borderColor}`,
            padding: '8px',
            transition: 'all 0.2s ease-in-out',
          }"
        >
          <div :style="{ marginRight: '8px', display: 'flex', alignItems: 'center', justifyContent: 'center', width: '32px', height: '32px', borderRadius: '50%', backgroundColor: card.iconBg, color: card.iconColor }">
            <span v-html="card.icon" />
          </div>
          <div :style="{ flexGrow: 1, minWidth: 0 }">
            <div :style="{ fontSize: '12px', color: 'var(--text-secondary-color)', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }">{{ card.title }}</div>
            <div :style="{ display: 'flex', alignItems: 'baseline' }">
              <span :style="{ fontSize: '16px', fontWeight: 'bold', marginRight: '4px' }">{{ card.value }}</span>
              <span :style="{ fontSize: '12px', color: 'var(--text-secondary-color)' }">{{ card.unit }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, useTemplateRef } from 'vue'
import { useI18n } from 'vue-i18n'

import EnhancedCanvasTrafficGraph from './enhanced-canvas-traffic-graph.vue'
import { useConnectionSummaryData } from '@/hooks/use-connection-data'
import { useMemoryData } from '@/hooks/use-memory-data'
import { useTrafficData } from '@/hooks/use-traffic-data'
import { useVerge } from '@/hooks/use-verge'
import { useVisibility } from '@/hooks/use-visibility'
import parseTraffic from '@/utils/parse-traffic'

const { t } = useI18n()
const { verge } = useVerge()
const trafficRef = ref()
const pageVisible = useVisibility()

const trafficGraph = computed(() => verge.value?.traffic_graph ?? true)
const displayMemory = computed(() => verge.value?.enable_memory_usage ?? true)

const { response: { data: traffic } } = useTrafficData({ enabled: pageVisible })
const { response: { data: memory } } = useMemoryData({ enabled: displayMemory.value && pageVisible })
const { response: { data: connectionSummary } } = useConnectionSummaryData({ enabled: pageVisible })

const parsedData = computed(() => {
  const [up, upUnit] = parseTraffic(traffic.value?.up || 0)
  const [down, downUnit] = parseTraffic(traffic.value?.down || 0)
  const [inuse, inuseUnit] = parseTraffic(memory.value?.inuse || 0)
  const [uploadTotal, uploadTotalUnit] = parseTraffic(traffic.value?.upTotal || 0)
  const [downloadTotal, downloadTotalUnit] = parseTraffic(traffic.value?.downTotal || 0)
  return { up, upUnit, down, downUnit, inuse, inuseUnit, uploadTotal, uploadTotalUnit, downloadTotal, downloadTotalUnit, connectionsCount: connectionSummary?.activeConnectionCount }
})

const colorMap: Record<string, string> = {
  primary: 'var(--primary-color)',
  secondary: 'var(--secondary-color)',
  error: 'var(--error-color)',
  warning: 'var(--warning-color)',
  info: 'var(--info-color)',
  success: 'var(--success-color)',
}

const statCards = computed(() => {
  const cards: any[] = [
    { icon: '<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M13 5.5c0 .83-.67 1.5-1.5 1.5S10 6.33 10 5.5 10.67 4 11.5 4 13 4.67 13 5.5zM10 19h4v-8h-4v8zm-6-7c-.83 0-1.5.67-1.5 1.5S3.17 15 4 15s1.5-.67 1.5-1.5S4.83 12 4 12zm16 0c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5S20.83 12 20 12z"/></svg>', title: t('home.components.traffic.metrics.uploadSpeed'), value: parsedData.value.up, unit: `${parsedData.value.upUnit}/s`, color: 'secondary' },
    { icon: '<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M13 5.5c0 .83-.67 1.5-1.5 1.5S10 6.33 10 5.5 10.67 4 11.5 4 13 4.67 13 5.5zM10 19h4v-8h-4v8zm-6-7c-.83 0-1.5.67-1.5 1.5S3.17 15 4 15s1.5-.67 1.5-1.5S4.83 12 4 12zm16 0c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5S20.83 12 20 12z"/></svg>', title: t('home.components.traffic.metrics.downloadSpeed'), value: parsedData.value.down, unit: `${parsedData.value.downUnit}/s`, color: 'primary' },
    { icon: '<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M3.9 12c0-1.71 1.39-3.1 3.1-3.1h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-1.9H7c-1.71 0-3.1-1.39-3.1-3.1zM8 13h8v-2H8v2zm9-6h-4v1.9h4c1.71 0 3.1 1.39 3.1 3.1s-1.39 3.1-3.1 3.1h-4V17h4c2.76 0 5-2.24 5-5s-2.24-5-5-5z"/></svg>', title: t('home.components.traffic.metrics.activeConnections'), value: parsedData.value.connectionsCount, unit: '', color: 'success' },
    { icon: '<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z"/></svg>', title: t('shared.labels.uploaded'), value: parsedData.value.uploadTotal, unit: parsedData.value.uploadTotalUnit, color: 'secondary' },
    { icon: '<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z"/></svg>', title: t('shared.labels.downloaded'), value: parsedData.value.downloadTotal, unit: parsedData.value.downloadTotalUnit, color: 'primary' },
  ]
  if (displayMemory.value) {
    cards.push({
      icon: '<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M20 8h-2.81c-.45-.78-1.07-1.45-1.82-1.96L17 4.41 15.59 3l-2.17 2.17C12.96 5.06 12.49 5 12 5c-.49 0-.96.06-1.41.17L8.41 3 7 4.41l1.62 1.63C7.88 6.55 7.26 7.22 6.81 8H4v2h2.09c-.05.33-.09.66-.09 1v1H4v2h2v1c0 .34.04.67.09 1H4v2h2.81c1.04 1.79 2.97 3 5.19 3s4.15-1.21 5.19-3H20v-2h-2.09c.05-.33.09-.66.09-1v-1h2v-2h-2v-1c0-.34-.04-.67-.09-1H20V8zm-6 8h-4v-2h4v2zm0-4h-4v-2h4v2z"/></svg>',
      title: t('home.components.traffic.metrics.memoryUsage'),
      value: parsedData.value.inuse,
      unit: parsedData.value.inuseUnit,
      color: 'error',
    })
  }
  return cards.map((card: any) => {
    const mainColor = colorMap[card.color] || colorMap.primary
    return { ...card, bgColor: mainColor + '0D', borderColor: mainColor + '26', iconBg: mainColor + '1A', iconColor: mainColor }
  })
})
</script>
