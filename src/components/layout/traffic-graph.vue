<template>
  <canvas ref="canvasRef" style="width: 100%; height: 100%" />
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import type { Traffic } from 'tauri-plugin-mihomo-api'

const maxPoint = 30
const refLineAlpha = 1
const refLineWidth = 2
const upLineAlpha = 0.6
const upLineWidth = 4
const downLineAlpha = 1
const downLineWidth = 4
const sampleIntervalMs = 1000
const frameIntervalMs = 1000 / 15
const animationDurationMs = sampleIntervalMs

const zeroTraffic: Traffic = { up: 0, down: 0, upTotal: 0, downTotal: 0 }
const createDefaultList = () =>
  Array.from({ length: maxPoint + 2 }, () => ({ ...zeroTraffic }))

const hasTraffic = (traffic?: Traffic | null) =>
  (traffic?.up ?? 0) !== 0 || (traffic?.down ?? 0) !== 0

const hasRetainedTraffic = (list: Traffic[]) => list.some(hasTraffic)

export interface TrafficRef {
  appendData: (data: Traffic) => void
  toggleStyle: () => void
}

type TrafficValueKey = 'up' | 'down'

const canvasRef = ref<HTMLCanvasElement | null>(null)
const countRef = ref(0)
const styleRef = ref(true)
const listRef = ref<Traffic[]>(createDefaultList())
const cacheRef = ref<Traffic | null>(null)

let requestDrawFn: ((animate?: boolean) => void) = () => {}
let timer: ReturnType<typeof setTimeout> | null = null
let raf = 0
let frameTimer: ReturnType<typeof setTimeout> | null = null
let resizeObserver: ResizeObserver | null = null
let animationStart = 0
let lastFrameTime = 0

const appendData = (data: Traffic) => {
  cacheRef.value = data
}

const toggleStyle = () => {
  styleRef.value = !styleRef.value
  requestDrawFn(false)
}

const getPalette = () => {
  const doc = document.documentElement
  const style = getComputedStyle(doc)
  return {
    primary: style.getPropertyValue('--primary-main').trim() || '#5b5c9d',
    secondary: '#FC9B76',
    divider: style.getPropertyValue('--divider-color').trim() || 'rgba(0, 0, 0, 0.12)',
  }
}

const setupCanvas = () => {
  const canvas = canvasRef.value
  if (!canvas) return
  const context = canvas.getContext('2d')
  if (!context) return

  const palette = getPalette()
  const refLineColor = palette.divider
  const upLineColor = palette.secondary
  const downLineColor = palette.primary

  const cancelPendingDraw = () => {
    if (frameTimer !== null) {
      clearTimeout(frameTimer)
      frameTimer = null
    }
    if (raf) {
      cancelAnimationFrame(raf)
      raf = 0
    }
  }

  const drawGraph = (offset = countRef.value) => {
    const list = listRef.value
    const lineStyle = styleRef.value
    const width = canvas.width
    const height = canvas.height
    const dx = width / maxPoint
    const dy = height / 7
    const l1 = dy
    const l2 = dy * 4

    const countY = (v: number) => {
      const h = height
      if (v == 0) return h - 1
      if (v <= 10) return h - (v / 10) * dy
      if (v <= 100) return h - (v / 100 + 1) * dy
      if (v <= 1024) return h - (v / 1024 + 2) * dy
      if (v <= 10240) return h - (v / 10240 + 3) * dy
      if (v <= 102400) return h - (v / 102400 + 4) * dy
      if (v <= 1048576) return h - (v / 1048576 + 5) * dy
      if (v <= 10485760) return h - (v / 10485760 + 6) * dy
      return 1
    }

    const drawBezier = (list: Traffic[], valueKey: TrafficValueKey) => {
      if (list.length === 0) return
      const firstX = (dx * -1 - offset + 3) | 0
      const firstY = countY(list[0]?.[valueKey] ?? 0)
      context.moveTo(firstX, firstY)
      for (let i = 1; i < list.length; i++) {
        const p1x = (dx * (i - 1) - offset + 3) | 0
        const p1y = countY(list[i]?.[valueKey] ?? 0)
        const hasNext = i + 1 < list.length
        const p2x = hasNext ? (dx * i - offset + 3) | 0 : p1x
        const p2y = hasNext ? countY(list[i + 1]?.[valueKey] ?? 0) : p1y
        context.quadraticCurveTo(p1x, p1y, (p1x + p2x) / 2, (p1y + p2y) / 2)
      }
    }

    const drawLine = (list: Traffic[], valueKey: TrafficValueKey) => {
      if (list.length === 0) return
      context.moveTo((dx * -1 - offset) | 0, countY(list[0]?.[valueKey] ?? 0))
      for (let i = 1; i < list.length; i++) {
        context.lineTo(
          (dx * (i - 1) - offset) | 0,
          countY(list[i]?.[valueKey] ?? 0),
        )
      }
    }

    context.clearRect(0, 0, width, height)
    context.beginPath()
    context.globalAlpha = refLineAlpha
    context.lineWidth = refLineWidth
    context.strokeStyle = refLineColor
    context.moveTo(0, l1)
    context.lineTo(width, l1)
    context.moveTo(0, l2)
    context.lineTo(width, l2)
    context.stroke()
    context.closePath()

    context.beginPath()
    context.globalAlpha = upLineAlpha
    context.lineWidth = upLineWidth
    context.strokeStyle = upLineColor
    if (lineStyle) drawBezier(list, 'up')
    else drawLine(list, 'up')
    context.stroke()
    context.closePath()

    context.beginPath()
    context.globalAlpha = downLineAlpha
    context.lineWidth = downLineWidth
    context.strokeStyle = downLineColor
    if (lineStyle) drawBezier(list, 'down')
    else drawLine(list, 'down')
    context.stroke()
    context.closePath()
  }

  const drawAnimatedFrame = (timestamp: number) => {
    raf = 0
    const timeSinceLastFrame = timestamp - lastFrameTime
    if (timeSinceLastFrame < frameIntervalMs) {
      frameTimer = setTimeout(() => {
        frameTimer = null
        raf = requestAnimationFrame(drawAnimatedFrame)
      }, frameIntervalMs - timeSinceLastFrame)
      return
    }
    lastFrameTime = timestamp
    const dx = canvas.width / maxPoint
    const progress = Math.min(
      (timestamp - animationStart) / animationDurationMs,
      1,
    )
    const offset = progress * dx
    countRef.value = offset
    drawGraph(offset)
    if (progress < 1) {
      raf = requestAnimationFrame(drawAnimatedFrame)
      return
    }
    countRef.value = dx
  }

  requestDrawFn = (animate = false) => {
    cancelPendingDraw()
    if (!animate) {
      raf = requestAnimationFrame(() => {
        raf = 0
        drawGraph()
      })
      return
    }
    animationStart = performance.now()
    lastFrameTime = animationStart - frameIntervalMs
    raf = requestAnimationFrame(drawAnimatedFrame)
  }

  requestDrawFn(false)

  if (typeof ResizeObserver !== 'undefined') {
    resizeObserver = new ResizeObserver(() => requestDrawFn(false))
    resizeObserver.observe(canvas)
  }
}

const handleData = () => {
  const data = cacheRef.value ?? zeroTraffic
  cacheRef.value = null
  const list = listRef.value
  const shouldAppend = hasTraffic(data) || hasRetainedTraffic(list)
  if (shouldAppend) {
    if (list.length > maxPoint + 2) list.shift()
    list.push(data)
    countRef.value = 0
    requestDrawFn(true)
  }
  timer = setTimeout(handleData, sampleIntervalMs)
}

onMounted(() => {
  handleData()
  setupCanvas()
})

onUnmounted(() => {
  if (timer) clearTimeout(timer)
  if (ref) cancelAnimationFrame(raf)
  if (frameTimer) clearTimeout(frameTimer)
  resizeObserver?.disconnect()
  requestDrawFn = () => {}
})

defineExpose({ appendData, toggleStyle })
</script>
