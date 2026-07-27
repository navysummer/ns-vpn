<template>
  <div
    :style="{
      width: '100%',
      height: '100%',
      position: 'relative',
      backgroundColor: 'var(--action-hover-color)',
      borderRadius: '8px',
      cursor: 'pointer',
      overflow: 'hidden',
    }"
    @click="toggleStyle"
    @mousemove="handleMouseMove"
    @mouseleave="handleMouseLeave"
  >
    <canvas
      ref="canvasRef"
      :style="{ width: '100%', height: '100%', display: 'block' }"
      @click="toggleStyle"
    />

    <canvas
      v-if="tooltipData.visible"
      ref="hoverCanvasRef"
      :style="{
        position: 'absolute',
        inset: 0,
        width: '100%',
        height: '100%',
        display: 'block',
        pointerEvents: 'none',
      }"
    />

    <div
      :style="{
        position: 'absolute',
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        pointerEvents: 'none',
      }"
    >
      <div
        :style="{
          position: 'absolute',
          top: '6px',
          left: '40px',
          fontSize: '11px',
          fontWeight: 'bold',
          color: 'var(--text-secondary-color)',
          cursor: 'pointer',
          pointerEvents: 'all',
          padding: '4px 8px',
          borderRadius: '4px',
          backgroundColor: 'rgba(0,0,0,0.05)',
        }"
        @click.stop="handleTimeRangeClick"
      >
        {{ getTimeRangeText() }}
      </div>

      <div
        :style="{
          position: 'absolute',
          top: '6px',
          right: '8px',
          display: 'flex',
          flexDirection: 'column',
          gap: '4px',
        }"
      >
        <div :style="{ fontSize: '11px', fontWeight: 'bold', color: colors.up, textAlign: 'right' }">
          {{ t('home.components.traffic.legends.upload') }}
        </div>
        <div :style="{ fontSize: '11px', fontWeight: 'bold', color: colors.down, textAlign: 'right' }">
          {{ t('home.components.traffic.legends.download') }}
        </div>
      </div>

      <div
        :style="{
          position: 'absolute',
          bottom: '6px',
          right: '8px',
          fontSize: '10px',
          color: 'var(--text-disabled-color)',
          opacity: 0.7,
        }"
      >
        {{ chartStyle === 'bezier' ? t('home.components.traffic.chartStyles.smooth') : t('home.components.traffic.chartStyles.linear') }}
      </div>

      <div
        :style="{
          position: 'absolute',
          bottom: '6px',
          left: '8px',
          fontSize: '9px',
          color: 'var(--text-disabled-color)',
          opacity: 0.6,
          lineHeight: 1.2,
        }"
      >
        {{ t('home.components.traffic.diagnostics', { points: displayData.length, compressed: samplerStats.compressedBufferSize, fps: currentFPS }) }}
      </div>

      <div
        v-if="tooltipData.visible"
        :style="{
          position: 'absolute',
          left: tooltipData.x + 8 + 'px',
          top: tooltipData.y - 8 + 'px',
          backgroundColor: 'var(--bg-color)',
          border: '1px solid var(--border-color)',
          borderRadius: '4px',
          padding: '4px 8px',
          fontSize: '10px',
          lineHeight: 1.2,
          zIndex: 1000,
          pointerEvents: 'none',
          transform: tooltipData.x > 200 ? 'translateX(-100%)' : 'translateX(0)',
          boxShadow: '0 4px 12px rgba(0,0,0,0.15)',
          whiteSpace: 'nowrap',
        }"
      >
        <div :style="{ color: 'var(--text-secondary-color)', marginBottom: '1px' }">{{ tooltipData.timestamp }}</div>
        <div :style="{ color: 'var(--secondary-color)', fontWeight: 500 }">↑ {{ tooltipData.upSpeed }}</div>
        <div :style="{ color: 'var(--primary-color)', fontWeight: 500 }">↓ {{ tooltipData.downSpeed }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, useTemplateRef, useReducer } from 'vue'
import { useI18n } from 'vue-i18n'

import { useTrafficGraphDataEnhanced } from '@/hooks/use-traffic-monitor'
import { useVerge } from '@/hooks/use-verge'
import { debugLog } from '@/utils/debug'
import parseTraffic from '@/utils/parse-traffic'
import { formatTrafficHourMinute, formatTrafficMinuteSecond, formatTrafficName } from '@/utils/traffic-sampler'

interface ITrafficDataPoint {
  up: number
  down: number
  timestamp?: number
}

const MAX_POINTS = 300
const TARGET_FPS = 15
const LINE_WIDTH_UP = 2.5
const LINE_WIDTH_DOWN = 2.5
const LINE_WIDTH_GRID = 0.5
const ALPHA_GRADIENT = 0.15
const ALPHA_LINE = 0.9
const PADDING_TOP = 16
const PADDING_RIGHT = 16
const PADDING_BOTTOM = 32
const PADDING_LEFT = 35
const STALE_DATA_THRESHOLD = 2500

const GRAPH_CONFIG = {
  maxPoints: MAX_POINTS,
  targetFPS: TARGET_FPS,
  lineWidth: { up: LINE_WIDTH_UP, down: LINE_WIDTH_DOWN, grid: LINE_WIDTH_GRID },
  alpha: { gradient: ALPHA_GRADIENT, line: ALPHA_LINE },
  padding: { top: PADDING_TOP, right: PADDING_RIGHT, bottom: PADDING_BOTTOM, left: PADDING_LEFT },
}

type TimeRange = 1 | 5 | 10
type ChartStyle = 'bezier' | 'line'

interface TooltipData {
  x: number; y: number; upSpeed: string; downSpeed: string; timestamp: string
  visible: boolean; dataIndex: number; highlightY: number
}

const { t } = useI18n()
const verge = useVerge()
const pause_render_traffic_stats_on_blur = computed(() => verge.verge?.pause_render_traffic_stats_on_blur ?? true)
const { dataPoints, requestRange, samplerStats } = useTrafficGraphDataEnhanced()

const timeRange = ref<TimeRange>(10)
const chartStyle = ref<ChartStyle>('bezier')
const tooltipData = ref<TooltipData>({ x: 0, y: 0, upSpeed: '', downSpeed: '', timestamp: '', visible: false, dataIndex: -1, highlightY: 0 })
const tooltipDataRef = ref<TooltipData>({ x: 0, y: 0, upSpeed: '', downSpeed: '', timestamp: '', visible: false, dataIndex: -1, highlightY: 0 })
const canvasRef = useTemplateRef<HTMLCanvasElement>('canvasRef')
const hoverCanvasRef = useTemplateRef<HTMLCanvasElement>('hoverCanvasRef')
const displayData = ref<ITrafficDataPoint[]>([])
const currentFPS = ref(TARGET_FPS)
const isDocumentVisible = ref(typeof document !== 'undefined' ? !document.hidden : true)
const isWindowFocused = ref(typeof document !== 'undefined' ? !document.hidden : true)
const lastDataTimestamp = ref(0)
const dataStale = ref(false)

let drawFrameId: number | undefined
let hoverFrameId: number | undefined
let mouseMoveFrameId: number | undefined
let pendingMousePosition: { clientX: number; clientY: number } | null = null
let debounceTimeoutId: number | null = null

const colors = computed(() => ({
  up: 'var(--secondary-color)',
  down: 'var(--primary-color)',
  grid: 'var(--border-color)',
  text: 'var(--text-secondary-color)',
  background: 'var(--bg-color)',
}))

watch(dataPoints, (newData) => {
  if (debounceTimeoutId !== null) clearTimeout(debounceTimeoutId)
  debounceTimeoutId = window.setTimeout(() => {
    displayData.value = newData
  }, 50)
}, { deep: true })

watch(timeRange, (val) => { requestRange(val) }, { immediate: true })

watch(displayData, (data) => {
  if (data.length === 0) {
    lastDataTimestamp.value = 0
    dataStale.value = false
    currentFPS.value = TARGET_FPS
    return
  }
  const latest = data[data.length - 1]?.timestamp
  if (latest) {
    lastDataTimestamp.value = latest
    dataStale.value = Date.now() - latest > STALE_DATA_THRESHOLD
  }
})

onMounted(() => {
  const handleFocus = () => { isWindowFocused.value = true; scheduleDrawGraph() }
  const handleBlur = () => { isWindowFocused.value = false; scheduleDrawGraph() }
  const handleVisibility = () => {
    isDocumentVisible.value = !document.hidden
    if (isDocumentVisible.value || !pause_render_traffic_stats_on_blur.value) currentFPS.value = TARGET_FPS
    scheduleDrawGraph()
  }
  window.addEventListener('focus', handleFocus)
  window.addEventListener('blur', handleBlur)
  document.addEventListener('visibilitychange', handleVisibility)

  const canvas = canvasRef.value
  if (canvas && typeof ResizeObserver !== 'undefined') {
    const observer = new ResizeObserver(() => scheduleDrawGraph())
    observer.observe(canvas)
  }

  onUnmounted(() => {
    window.removeEventListener('focus', handleFocus)
    window.removeEventListener('blur', handleBlur)
    document.removeEventListener('visibilitychange', handleVisibility)
    if (drawFrameId !== undefined) cancelAnimationFrame(drawFrameId)
    if (hoverFrameId !== undefined) cancelAnimationFrame(hoverFrameId)
    if (mouseMoveFrameId !== undefined) cancelAnimationFrame(mouseMoveFrameId)
  })
})

const calculateY = (value: number, height: number, topValue: number, bottomValue: number) => {
  const padding = GRAPH_CONFIG.padding
  const topY = padding.top + 10
  const bottomY = height - padding.bottom - 5
  if (topValue === bottomValue) return bottomY
  const ratio = (value - bottomValue) / (topValue - bottomValue)
  return bottomY - ratio * (bottomY - topY)
}

const computeYScale = (data: ITrafficDataPoint[]) => {
  if (data.length === 0) return { topValue: 1024, bottomValue: 0 }
  let maxValue = 0
  let minValue = Infinity
  for (let i = 0; i < data.length; i++) {
    const up = data[i].up; const down = data[i].down
    if (up > maxValue) maxValue = up
    if (down > maxValue) maxValue = down
    if (up < minValue) minValue = up
    if (down < minValue) minValue = down
  }
  if (!isFinite(minValue)) minValue = 0
  if (maxValue === 0) return { topValue: 1024, bottomValue: 0 }
  const range = maxValue - minValue
  if (range === 0) return { topValue: maxValue * 1.2, bottomValue: 0 }
  const pct = 0.1
  return { topValue: maxValue + range * pct, bottomValue: Math.max(0, minValue - range * pct) }
}

const yScale = computed(() => computeYScale(displayData.value))

const handleMouseMove = (event: MouseEvent) => {
  if (displayData.value.length === 0) return
  pendingMousePosition = { clientX: event.clientX, clientY: event.clientY }
  if (mouseMoveFrameId !== undefined) return
  mouseMoveFrameId = requestAnimationFrame(() => {
    mouseMoveFrameId = undefined
    if (!pendingMousePosition) return
    const canvas = canvasRef.value
    if (!canvas || displayData.value.length === 0) return
    const rect = canvas.getBoundingClientRect()
    const mouseX = pendingMousePosition.clientX - rect.left
    const mouseY = pendingMousePosition.clientY - rect.top
    const padding = GRAPH_CONFIG.padding
    const effectiveWidth = rect.width - padding.left - padding.right
    if (effectiveWidth <= 0) return
    const relativeMouseX = mouseX - padding.left
    const ratio = Math.max(0, Math.min(1, relativeMouseX / effectiveWidth))
    const dataIndex = Math.round(ratio * (displayData.value.length - 1))
    if (dataIndex < 0 || dataIndex >= displayData.value.length) return
    const point = displayData.value[dataIndex]
    const [upValue, upUnit] = parseTraffic(point.up)
    const [downValue, downUnit] = parseTraffic(point.down)
    const timeStr = point.timestamp ? formatTrafficName(point.timestamp) : t('home.components.traffic.unknownTime')
    const { topValue: tvH, bottomValue: bvH } = yScale.value
    const upY = calculateY(point.up, rect.height, tvH, bvH)
    const downY = calculateY(point.down, rect.height, tvH, bvH)
    const highlightY = Math.max(point.up, point.down) === point.up ? upY : downY
    tooltipData.value = {
      x: mouseX, y: mouseY,
      upSpeed: `${upValue}${upUnit}/s`, downSpeed: `${downValue}${downUnit}/s`,
      timestamp: timeStr, visible: true, dataIndex, highlightY,
    }
    pendingMousePosition = null
  })
}

const handleMouseLeave = () => {
  pendingMousePosition = null
  if (mouseMoveFrameId !== undefined) {
    cancelAnimationFrame(mouseMoveFrameId)
    mouseMoveFrameId = undefined
  }
  if (tooltipData.value.visible) tooltipData.value = { ...tooltipData.value, visible: false }
}

const getYAxisTicks = (topValue: number, bottomValue: number, height: number) => {
  const formatTrafficValue = (bytes: number): string => {
    if (bytes === 0) return '0'
    if (bytes < 1024) return `${Math.round(bytes)}B`
    if (bytes < 1024 * 1024) return `${Math.round(bytes / 1024)}KB`
    return `${(bytes / (1024 * 1024)).toFixed(1)}MB`
  }
  const padding = GRAPH_CONFIG.padding
  const topY = padding.top + 10
  const bottomY = height - padding.bottom - 5
  const middleY = (topY + bottomY) / 2
  const middleValue = (bottomValue + topValue) / 2
  return [
    { value: bottomValue, label: formatTrafficValue(bottomValue), y: bottomY },
    { value: middleValue, label: formatTrafficValue(middleValue), y: middleY },
    { value: topValue, label: formatTrafficValue(topValue), y: topY },
  ]
}

const drawYAxis = (ctx: CanvasRenderingContext2D, width: number, height: number, topValue: number, bottomValue: number) => {
  const padding = GRAPH_CONFIG.padding
  const ticks = getYAxisTicks(topValue, bottomValue, height)
  if (ticks.length === 0) return
  ctx.save()
  ticks.forEach((tick, index) => {
    const isBottom = index === 0; const isTop = index === ticks.length - 1
    if (isBottom || isTop) {
      ctx.strokeStyle = colors.value.grid
      ctx.lineWidth = isBottom ? 0.8 : 0.4
      ctx.globalAlpha = isBottom ? 0.25 : 0.15
      ctx.beginPath(); ctx.moveTo(padding.left, tick.y); ctx.lineTo(width - padding.right, tick.y); ctx.stroke()
    }
    ctx.fillStyle = colors.value.text
    ctx.font = "8px -apple-system, BlinkMacSystemFont, 'Segoe UI', Arial, sans-serif"
    ctx.globalAlpha = 0.9; ctx.textAlign = 'right'; ctx.textBaseline = 'middle'
    if (tick.label !== '0') {
      const labelWidth = ctx.measureText(tick.label).width
      ctx.globalAlpha = 0.15; ctx.fillStyle = colors.value.background
      ctx.fillRect(padding.left - labelWidth - 8, tick.y - 5, labelWidth + 4, 10)
    }
    ctx.globalAlpha = 0.9; ctx.fillStyle = colors.value.text
    ctx.fillText(tick.label, padding.left - 4, tick.y)
  })
  ctx.restore()
}

const getTimeDisplayStrategy = (timeRangeMinutes: TimeRange) => {
  switch (timeRangeMinutes) {
    case 1: return { maxLabels: 6, formatTime: formatTrafficMinuteSecond, intervalSeconds: 10, minPixelDistance: 35 }
    case 5: return { maxLabels: 6, formatTime: formatTrafficHourMinute, intervalSeconds: 30, minPixelDistance: 38 }
    case 10: default: return { maxLabels: 8, formatTime: formatTrafficHourMinute, intervalSeconds: 60, minPixelDistance: 40 }
  }
}

const drawTimeAxis = (ctx: CanvasRenderingContext2D, width: number, height: number, data: ITrafficDataPoint[]) => {
  if (data.length === 0) return
  const padding = GRAPH_CONFIG.padding
  const effectiveWidth = width - padding.left - padding.right
  const timeAxisY = height - padding.bottom + 14
  const strategy = getTimeDisplayStrategy(timeRange.value)
  ctx.save(); ctx.fillStyle = colors.value.text
  ctx.font = "10px -apple-system, BlinkMacSystemFont, 'Segoe UI', Arial, sans-serif"
  ctx.globalAlpha = 0.7
  const targetLabels = Math.min(strategy.maxLabels, data.length)
  const step = Math.max(1, Math.floor(data.length / (targetLabels - 1)))
  const minPixelDistance = strategy.minPixelDistance || 45
  const actualStep = Math.max(step, Math.ceil((data.length * minPixelDistance) / effectiveWidth))
  const timePoints: Array<{ index: number; x: number; label: string }> = []
  if (data.length > 0 && data[0].timestamp) timePoints.push({ index: 0, x: padding.left, label: strategy.formatTime(data[0].timestamp) })
  for (let i = actualStep; i < data.length - actualStep; i += actualStep) {
    const point = data[i]; if (!point.timestamp) continue
    const x = padding.left + (i / (data.length - 1)) * effectiveWidth
    timePoints.push({ index: i, x, label: strategy.formatTime(point.timestamp) })
  }
  if (data.length > 1 && data[data.length - 1].timestamp) {
    const lastX = width - padding.right; const lastPoint = timePoints[timePoints.length - 1]
    if (!lastPoint || lastX - lastPoint.x >= minPixelDistance)
      timePoints.push({ index: data.length - 1, x: lastX, label: strategy.formatTime(data[data.length - 1].timestamp) })
  }
  timePoints.forEach((point, index) => {
    ctx.textAlign = index === 0 ? 'left' : index === timePoints.length - 1 ? 'right' : 'center'
    ctx.fillText(point.label, point.x, timeAxisY)
  })
  ctx.restore()
}

const drawGrid = (ctx: CanvasRenderingContext2D, width: number, height: number) => {
  const padding = GRAPH_CONFIG.padding
  const effectiveWidth = width - padding.left - padding.right
  const effectiveHeight = height - padding.top - padding.bottom
  ctx.save(); ctx.strokeStyle = colors.value.grid; ctx.lineWidth = GRAPH_CONFIG.lineWidth.grid; ctx.globalAlpha = 0.7
  const hLines = 4; const vLines = 6
  for (let i = 1; i <= hLines; i++) {
    const y = padding.top + (effectiveHeight / (hLines + 1)) * i
    ctx.beginPath(); ctx.moveTo(padding.left, y); ctx.lineTo(width - padding.right, y); ctx.stroke()
  }
  for (let i = 1; i <= vLines; i++) {
    const x = padding.left + (effectiveWidth / (vLines + 1)) * i
    ctx.beginPath(); ctx.moveTo(x, padding.top); ctx.lineTo(x, height - padding.bottom); ctx.stroke()
  }
  ctx.restore()
}

const drawTrafficLine = (
  ctx: CanvasRenderingContext2D, data: ITrafficDataPoint[], valueKey: 'up' | 'down',
  width: number, height: number, color: string, withGradient: boolean, topValue: number, bottomValue: number,
) => {
  if (data.length < 2) return
  const padding = GRAPH_CONFIG.padding
  const effectiveWidth = width - padding.left - padding.right
  const lastIndex = data.length - 1
  const getX = (index: number) => padding.left + (index / lastIndex) * effectiveWidth
  const getY = (index: number) => calculateY(data[index][valueKey], height, topValue, bottomValue)
  ctx.save()
  if (withGradient && chartStyle.value === 'bezier') {
    const gradient = ctx.createLinearGradient(0, padding.top, 0, height - padding.bottom)
    const alphaHex = Math.round(GRAPH_CONFIG.alpha.gradient * 255).toString(16).padStart(2, '0')
    gradient.addColorStop(0, `${color}${alphaHex}`); gradient.addColorStop(1, `${color}00`)
    ctx.beginPath(); ctx.moveTo(getX(0), getY(0))
    for (let i = 1; i < data.length; i++) {
      if (chartStyle.value === 'bezier') {
        const cx = (getX(i) + getX(Math.min(i + 1, lastIndex))) / 2
        const cy = (getY(i) + getY(Math.min(i + 1, lastIndex))) / 2
        ctx.quadraticCurveTo(getX(i), getY(i), cx, cy)
      } else ctx.lineTo(getX(i), getY(i))
    }
    ctx.lineTo(getX(lastIndex), height - padding.bottom); ctx.lineTo(getX(0), height - padding.bottom)
    ctx.closePath(); ctx.fillStyle = gradient; ctx.fill()
  }
  ctx.beginPath(); ctx.strokeStyle = color; ctx.lineWidth = GRAPH_CONFIG.lineWidth.up
  ctx.lineCap = 'round'; ctx.lineJoin = 'round'; ctx.globalAlpha = GRAPH_CONFIG.alpha.line
  ctx.moveTo(getX(0), getY(0))
  for (let i = 1; i < data.length; i++) {
    if (chartStyle.value === 'bezier') {
      const cx = (getX(i) + getX(Math.min(i + 1, lastIndex))) / 2
      const cy = (getY(i) + getY(Math.min(i + 1, lastIndex))) / 2
      ctx.quadraticCurveTo(getX(i), getY(i), cx, cy)
    } else ctx.lineTo(getX(i), getY(i))
  }
  ctx.stroke(); ctx.restore()
}

const syncCanvasSize = (canvas: HTMLCanvasElement) => {
  const ctx = canvas.getContext('2d'); if (!ctx) return null
  const rect = canvas.getBoundingClientRect()
  const dpr = window.devicePixelRatio || 1
  const cssWidth = rect.width; const cssHeight = rect.height
  const pixelWidth = Math.max(1, Math.floor(cssWidth * dpr))
  const pixelHeight = Math.max(1, Math.floor(cssHeight * dpr))
  canvas.style.width = '100%'; canvas.style.height = '100%'
  if (canvas.width !== pixelWidth || canvas.height !== pixelHeight) {
    canvas.width = pixelWidth; canvas.height = pixelHeight
    ctx.setTransform(1, 0, 0, 1, 0, 0); ctx.scale(dpr, dpr)
  }
  return { ctx, cssWidth, cssHeight }
}

const clearCanvas = (canvas: HTMLCanvasElement | null) => {
  if (!canvas) return
  const synced = syncCanvasSize(canvas)
  if (!synced) return; synced.ctx.clearRect(0, 0, synced.cssWidth, synced.cssHeight)
}

const drawGraph = () => {
  const canvas = canvasRef.value
  if (!canvas || displayData.value.length === 0) { clearCanvas(canvasRef.value); clearCanvas(hoverCanvasRef.value); return }
  const synced = syncCanvasSize(canvas); if (!synced) return
  const { ctx, cssWidth, cssHeight } = synced
  ctx.clearRect(0, 0, cssWidth, cssHeight)
  const { topValue, bottomValue } = yScale.value
  drawYAxis(ctx, cssWidth, cssHeight, topValue, bottomValue)
  drawGrid(ctx, cssWidth, cssHeight)
  drawTimeAxis(ctx, cssWidth, cssHeight, displayData.value)
  drawTrafficLine(ctx, displayData.value, 'down', cssWidth, cssHeight, colors.value.down, true, topValue, bottomValue)
  drawTrafficLine(ctx, displayData.value, 'up', cssWidth, cssHeight, colors.value.up, true, topValue, bottomValue)
  clearCanvas(hoverCanvasRef.value)
}

const drawHoverOverlay = () => {
  const canvas = hoverCanvasRef.value
  if (!canvas || displayData.value.length < 2) { clearCanvas(canvas); return }
  const synced = syncCanvasSize(canvas); if (!synced) return
  const { ctx, cssWidth, cssHeight } = synced
  ctx.clearRect(0, 0, cssWidth, cssHeight)
  const currentTooltip = tooltipDataRef.value
  if (currentTooltip.visible && currentTooltip.dataIndex >= 0) {
    const padding = GRAPH_CONFIG.padding
    const effectiveWidth = cssWidth - padding.left - padding.right
    const dataX = padding.left + (currentTooltip.dataIndex / (displayData.value.length - 1)) * effectiveWidth
    ctx.save(); ctx.strokeStyle = colors.value.text; ctx.lineWidth = 1; ctx.globalAlpha = 0.6; ctx.setLineDash([4, 4])
    ctx.beginPath(); ctx.moveTo(dataX, padding.top); ctx.lineTo(dataX, cssHeight - padding.bottom); ctx.stroke()
    ctx.beginPath(); ctx.moveTo(padding.left, currentTooltip.highlightY)
    ctx.lineTo(cssWidth - padding.right, currentTooltip.highlightY); ctx.stroke()
    ctx.restore()
  }
}

const shouldSkipGraphDraw = () => {
  if (!isDocumentVisible.value) return true
  if (!isWindowFocused.value && pause_render_traffic_stats_on_blur.value) return true
  if (lastDataTimestamp.value > 0 && Date.now() - lastDataTimestamp.value > STALE_DATA_THRESHOLD) return true
  return dataStale.value
}

const scheduleHoverDraw = () => {
  if (hoverFrameId !== undefined) return
  hoverFrameId = requestAnimationFrame(() => { hoverFrameId = undefined; drawHoverOverlay() })
}

const scheduleDrawGraph = () => {
  if (drawFrameId !== undefined) return
  drawFrameId = requestAnimationFrame(() => {
    drawFrameId = undefined
    if (shouldSkipGraphDraw()) return
    drawGraph()
    drawHoverOverlay()
  })
}

watch(tooltipData, (val) => {
  tooltipDataRef.value = val
  scheduleHoverDraw()
}, { deep: true })

watch(displayData, () => scheduleDrawGraph(), { deep: true })

const handleTimeRangeClick = (e: MouseEvent) => {
  e.stopPropagation()
  timeRange.value = timeRange.value === 1 ? 5 : timeRange.value === 5 ? 10 : 1
}

const toggleStyle = () => { chartStyle.value = chartStyle.value === 'bezier' ? 'line' : 'bezier' }

const getTimeRangeText = () => t('home.components.traffic.patterns.minutes', { time: timeRange.value })
</script>
