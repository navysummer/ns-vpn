import { ref, shallowRef, onMounted, onUnmounted } from 'vue'
import { MihomoWebSocket } from 'tauri-plugin-mihomo-api'

import { useVisibility } from '@/hooks/use-visibility'
import { debugLog } from '@/utils/debug'
import { TrafficDataSampler, formatTrafficName } from '@/utils/traffic-sampler'

class ReferenceCounter {
  private count = 0
  private callbacks = new Set<() => void>()

  private notify() {
    this.callbacks.forEach((cb) => cb())
  }

  increment(): () => void {
    this.count++
    this.notify()
    let released = false
    return () => {
      if (released) return
      released = true
      this.count--
      this.notify()
    }
  }

  onChange(callback: () => void): () => void {
    this.callbacks.add(callback)
    return () => { this.callbacks.delete(callback) }
  }

  get current(): number { return this.count }
}

const trafficSubscriptions = new ReferenceCounter()

let globalWs: MihomoWebSocket | null = null

function ensureTrafficMonitor(cb: (data: any) => void) {
  if (!globalWs) {
    MihomoWebSocket.connect_traffic().then((ws) => {
      globalWs = ws
      ws.addListener((msg) => {
        if (msg.type !== 'Text') return
        try { cb(JSON.parse(msg.data)) } catch {}
      })
    })
  }
  return globalWs
}

export interface DataPoint {
  time: number
  upload: number
  download: number
}

export const useTrafficMonitorData = (options?: { enabled?: boolean }) => {
  const visible = useVisibility()
  const enabled = options?.enabled ?? true
  const uploadSpeed = ref(0)
  const downloadSpeed = ref(0)
  const dataPoints = shallowRef<DataPoint[]>([])

  const sampler = new TrafficDataSampler()

  onMounted(() => {
    if (!enabled) return
    const release = trafficSubscriptions.increment()
    ensureTrafficMonitor((data) => {
      sampler.addSample(data)
      dataPoints.value = sampler.getDataPoints()
      uploadSpeed.value = sampler.getUploadSpeed()
      downloadSpeed.value = sampler.getDownloadSpeed()
    })

    onUnmounted(() => {
      release()
    })
  })

  return {
    uploadSpeed,
    downloadSpeed,
    dataPoints,
  }
}

export const useTrafficMonitorEnhanced = () => {
  const { dataPoints, uploadSpeed, downloadSpeed } = useTrafficMonitorData()
  const samplerStats = ref({ total: 0, duration: 0 })

  return {
    dataPoints,
    requestRange: { min: 0, max: 100 },
    samplerStats,
  }
}

export const useTrafficGraphDataEnhanced = () => {
  return useTrafficMonitorEnhanced()
}
