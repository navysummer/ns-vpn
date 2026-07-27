<template>
  <LightweightTrafficErrorBoundary>
    <div class="traffic-container">
      <div
        v-if="trafficGraph && pageVisible"
        style="width: 100%; height: 60px; margin-bottom: 6px"
        @click="trafficRef?.toggleStyle?.()"
      >
        <TrafficGraph :ref="setTrafficRef" />
      </div>

      <div class="traffic-rows">
        <div
          :title="t('home.components.traffic.metrics.uploadSpeed')"
          class="traffic-row"
        >
          <n-icon size="16" :color="(traffic?.up || 0) > 0 ? '#FC9B76' : '#aaa'" style="margin-right: 8px">
            <ArrowUp />
          </n-icon>
          <span class="traffic-val" style="color: #FC9B76">{{ up }}</span>
          <span class="traffic-unit">{{ upUnit }}/s</span>
        </div>

        <div
          :title="t('home.components.traffic.metrics.downloadSpeed')"
          class="traffic-row"
        >
          <n-icon size="16" :color="(traffic?.down || 0) > 0 ? '#007AFF' : '#aaa'" style="margin-right: 8px">
            <ArrowDown />
          </n-icon>
          <span class="traffic-val" style="color: #007AFF">{{ down }}</span>
          <span class="traffic-unit">{{ downUnit }}/s</span>
        </div>

        <div
          v-if="displayMemory"
          :title="t('home.components.traffic.metrics.memoryUsage')"
          class="traffic-row"
          style="cursor: auto; color: #aaa"
        >
          <n-icon size="16" style="margin-right: 8px">
            <MemoryIcon />
          </n-icon>
          <span class="traffic-val">{{ inuse }}</span>
          <span class="traffic-unit">{{ inuseUnit }}</span>
        </div>
      </div>
    </div>
  </LightweightTrafficErrorBoundary>
</template>

<script setup lang="ts">
import { ref, watch, h } from 'vue'
import { ArrowUp, ArrowDown } from '@vicons/ionicons5'

import LightweightTrafficErrorBoundary from '@/components/shared/traffic-error-boundary.vue'
import { useMemoryData } from '@/hooks/use-memory-data'
import { useTrafficData } from '@/hooks/use-traffic-data'
import { useVerge } from '@/hooks/use-verge'
import { useVisibility } from '@/hooks/use-visibility'
import parseTraffic from '@/utils/parse-traffic'
import { useTranslation } from '@/composables/use-i18n'

import TrafficGraph from './traffic-graph.vue'
import type { TrafficRef } from './traffic-graph.vue'

const { t } = useTranslation()

const MemoryIcon = () =>
  h('svg', {
    width: '16',
    height: '16',
    viewBox: '0 0 24 24',
    fill: 'none',
    stroke: 'currentColor',
    'stroke-width': '2',
    'stroke-linecap': 'round',
    'stroke-linejoin': 'round',
  }, [
    h('rect', { x: '4', y: '4', width: '16', height: '16', rx: '2', ry: '2' }),
    h('rect', { x: '9', y: '9', width: '6', height: '6' }),
    h('line', { x1: '9', y1: '1', x2: '9', y2: '4' }),
    h('line', { x1: '15', y1: '1', x2: '15', y2: '4' }),
    h('line', { x1: '9', y1: '20', x2: '9', y2: '23' }),
    h('line', { x1: '15', y1: '20', x2: '15', y2: '23' }),
  ])

const { verge } = useVerge()
const trafficGraph = verge?.traffic_graph ?? true
const displayMemory = verge?.enable_memory_usage ?? true

const trafficRef = ref<TrafficRef | null>(null)
const pageVisible = useVisibility()

const setTrafficRef = (el: any) => {
  trafficRef.value = el
}

const { response: { data: traffic } } = useTrafficData({ enabled: pageVisible })
const { response: { data: memory } } = useMemoryData({ enabled: displayMemory && pageVisible })

watch(traffic, (val) => {
  if (trafficRef.value) {
    trafficRef.value.appendData({
      up: val?.up || 0,
      down: val?.down || 0,
      upTotal: val?.upTotal || 0,
      downTotal: val?.downTotal || 0,
    })
  }
}, { deep: true })

const [up, upUnit] = parseTraffic(traffic?.up || 0)
const [down, downUnit] = parseTraffic(traffic?.down || 0)
const [inuse, inuseUnit] = parseTraffic(memory?.inuse || 0)
</script>

<style scoped>
.traffic-container {
  position: relative;
}

.traffic-rows {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.traffic-row {
  display: flex;
  align-items: center;
  white-space: nowrap;
}

.traffic-val {
  flex: 1 1 56px;
  user-select: none;
  text-align: center;
  font-size: 14px;
}

.traffic-unit {
  flex: 0 1 27px;
  user-select: none;
  font-size: 12px;
  text-align: right;
  color: var(--secondary-text);
}
</style>
