import { type Ref, computed } from 'vue'

const TRAFFIC_UNITS = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB']

export interface ConnectionRowView {
  id: string
  host: string
  process: string
  network: string
  type: string
  source: string
  destination: string
  upload: string
  download: string
  uploadSpeed: string
  downloadSpeed: string
  chains: string
  rule: string
  rulePayload: string
  sniffHost: string
  dlSpeed: number
  upSpeed: number
  dlTotal: number
  upTotal: number
}

export interface ConnectionRaw {
  id: string
  metadata: {
    host?: string
    process?: string
    network?: string
    type?: string
    sourceIP?: string
    sourcePort?: string
    destinationIP?: string
    destinationPort?: string
    dnsMode?: string
    specialProxy?: string
    specialRules?: string
    sniffHost?: string
  }
  upload?: number
  download?: number
  start?: string
  chains?: string[]
  rule?: string
  rulePayload?: string
  dlSpeed?: number
  upSpeed?: number
  dlTotal?: number
  upTotal?: number
}

const formatBytes = (bytes: number, decimals = 2): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  const idx = Math.min(i, TRAFFIC_UNITS.length - 1)
  const value = parseFloat((bytes / Math.pow(k, idx)).toFixed(decimals))
  return `${value} ${TRAFFIC_UNITS[idx]}`
}

export const createConnectionView = (conn: ConnectionRaw): ConnectionRowView => {
  const meta = conn.metadata || {}
  const formatSpeed = (speed: number = 0) => formatBytes(speed, 2)
  const formatTotal = (total: number = 0) => formatBytes(total, 2)

  return {
    id: conn.id,
    host: meta.host || `${meta.destinationIP || '-'}:${meta.destinationPort || '-'}`,
    process: meta.process || '-',
    network: meta.network || '-',
    type: meta.type || '-',
    source: meta.sourceIP ? `${meta.sourceIP}:${meta.sourcePort}` : '-',
    destination: meta.destinationIP ? `${meta.destinationIP}:${meta.destinationPort}` : '-',
    upload: formatTotal(conn.upload),
    download: formatTotal(conn.download),
    uploadSpeed: formatSpeed(conn.upSpeed),
    downloadSpeed: formatSpeed(conn.dlSpeed),
    chains: (conn.chains || []).join(' > '),
    rule: conn.rule || '-',
    rulePayload: conn.rulePayload || '-',
    sniffHost: meta.sniffHost || '-',
    dlSpeed: conn.dlSpeed || 0,
    upSpeed: conn.upSpeed || 0,
    dlTotal: conn.dlTotal || 0,
    upTotal: conn.upTotal || 0,
  }
}

export const getConnectionStartTime = (conn: { start?: string }): number => {
  return conn.start ? new Date(conn.start).getTime() : 0
}

export const formatConnectionChains = (chains?: string[]): string => {
  return (chains || []).join(' > ')
}

export const formatConnectionTraffic = (bytes: number): string => {
  return formatBytes(bytes)
}

export const getConnectionDestination = (conn: { metadata?: { destinationIP?: string; destinationPort?: string } }): string => {
  const m = conn.metadata || {}
  return m.destinationIP ? `${m.destinationIP}:${m.destinationPort}` : '-'
}

export const getConnectionProcess = (conn: { metadata?: { process?: string } }): string => {
  return conn.metadata?.process || '-'
}

export const getConnectionRule = (conn: { rule?: string; rulePayload?: string }): string => {
  return conn.rule || '-'
}

export const getConnectionSource = (conn: { metadata?: { sourceIP?: string; sourcePort?: string } }): string => {
  const m = conn.metadata || {}
  return m.sourceIP ? `${m.sourceIP}:${m.sourcePort}` : '-'
}

export const getConnectionTypeLabel = (conn: { metadata?: { type?: string } }): string => {
  return conn.metadata?.type || '-'
}

export const getConnectionHost = (conn: { metadata?: { host?: string; destinationIP?: string; destinationPort?: string } }): string => {
  const m = conn.metadata || {}
  return m.host || `${m.destinationIP || '-'}:${m.destinationPort || '-'}`
}

export const useConnectionRowViews = (
  connections: Ref<IConnectionsItem[]>,
) => {
  return computed(() => (connections.value || []).map(createConnectionView))
}
