<template>
  <div style="display: flex; flex-direction: column; flex: 1; min-height: 0; position: relative;">
    <div
      ref="scrollContainerRef"
      style="flex: 1; min-height: 0; overflow: auto; -webkit-overflow-scrolling: touch; overscroll-behavior: contain; border-radius: 8px;"
      @scroll="onScroll"
    >
      <div :style="{ minWidth: '100%', width: tableWidth + 'px' }">
        <div style="position: sticky; top: 0; z-index: 2;">
          <div :style="{ display: 'flex', borderBottom: `1px solid ${borderColor}` }">
            <div
              v-for="column in visibleColumns"
              :key="column.field"
              :style="{
                display: 'flex',
                alignItems: 'center',
                position: 'relative',
                boxSizing: 'border-box',
                flex: `0 0 ${column.size}px`,
                minWidth: column.minWidth + 'px',
                maxWidth: column.maxWidth !== undefined ? column.maxWidth + 'px' : 'none',
                fontSize: 13,
                fontWeight: 600,
                color: textSecondary,
                userSelect: 'none',
              }"
            >
              <button
                type="button"
                :style="{
                  flex: 1,
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: column.align === 'right' ? 'flex-end' : 'flex-start',
                  gap: 4,
                  padding: 8,
                  border: 0,
                  background: 'transparent',
                  color: 'inherit',
                  font: 'inherit',
                  textAlign: column.align === 'right' ? 'right' : 'left',
                  cursor: 'pointer',
                }"
                @click="toggleSorting(column.field)"
              >
                {{ column.headerName }}
                <span v-if="sorting?.id === column.field">{{ sorting.desc ? '▼' : '▲' }}</span>
              </button>
              <div
                :style="{
                  cursor: 'col-resize',
                  position: 'absolute',
                  right: 0,
                  top: 0,
                  width: RESIZE_HANDLE_WIDTH + 'px',
                  height: '100%',
                  transform: 'translateX(50%)',
                }"
                @mousedown="(e) => handleResizeMouseDown(column, e)"
                @touchstart.prevent="(e) => handleResizeTouchStart(column, e)"
              />
            </div>
          </div>
        </div>
        <div :style="{ position: 'relative', height: totalRowsHeight + 'px' }">
          <div
            v-for="(row, offset) in visibleRows"
            :key="row.id"
            :style="{
              display: 'flex',
              position: 'absolute',
              top: (firstVisibleRow + offset) * ROW_HEIGHT + 'px',
              left: 0,
              right: 0,
              height: ROW_HEIGHT + 'px',
              cursor: 'pointer',
              borderBottom: `1px solid ${borderColor}`,
            }"
            @click="onShowDetail(row.id)"
          >
            <div
              v-for="column in visibleColumns"
              :key="column.field"
              :style="{
                boxSizing: 'border-box',
                flex: `0 0 ${column.size}px`,
                minWidth: column.minWidth + 'px',
                maxWidth: column.maxWidth !== undefined ? column.maxWidth + 'px' : 'none',
                padding: '8px',
                fontSize: 13,
                display: 'flex',
                alignItems: 'center',
                justifyContent: column.align === 'right' ? 'flex-end' : 'flex-start',
                whiteSpace: 'nowrap',
                overflow: 'hidden',
                textOverflow: 'ellipsis',
              }"
            >
              <template v-if="column.field === 'time'">
                <ConnectionRelativeTime :start="row.start" />
              </template>
              <template v-else>
                {{ getCellValue(column.field, row) }}
              </template>
            </div>
          </div>
        </div>
      </div>
    </div>
      <ConnectionColumnManager
        :open="columnManagerOpen"
        :columns="managerColumns"
        @close="onCloseWrapper"
        @order-change="handleManagerOrderChange"
        @reset="handleResetColumns"
      />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import ConnectionRelativeTime from './connection-relative-time.vue'
import ConnectionColumnManager, { type ConnectionColumnOption } from './connection-column-manager.vue'
import {
  formatConnectionChains,
  formatConnectionTraffic,
  getConnectionDestination,
  getConnectionHost,
  getConnectionProcess,
  getConnectionRule,
  getConnectionSource,
  getConnectionStartTime,
  getConnectionTypeLabel,
} from './connection-row-view'

const ROW_HEIGHT = 40
const RESIZE_HANDLE_WIDTH = 6
const OVERSCAN_ROWS = 6

const props = defineProps<{
  connections: IConnectionsItem[]
  onShowDetail: (id: string) => void
  columnManagerOpen: boolean
  onCloseColumnManager?: () => void
}>()

const { t } = useI18n()

type ColumnField = 'host' | 'download' | 'upload' | 'dlSpeed' | 'ulSpeed' | 'chains' | 'rule' | 'process' | 'time' | 'source' | 'remoteDestination' | 'type'

interface ColumnDef {
  field: ColumnField
  headerName: string
  width: number
  minWidth: number
  align?: 'left' | 'right'
}

const baseColumns: ColumnDef[] = [
  { field: 'host', headerName: t('connections.components.fields.host'), width: 180, minWidth: 140 },
  { field: 'download', headerName: t('shared.labels.downloaded'), width: 76, minWidth: 60, align: 'right' },
  { field: 'upload', headerName: t('shared.labels.uploaded'), width: 76, minWidth: 60, align: 'right' },
  { field: 'dlSpeed', headerName: t('connections.components.fields.dlSpeed'), width: 76, minWidth: 60, align: 'right' },
  { field: 'ulSpeed', headerName: t('connections.components.fields.ulSpeed'), width: 76, minWidth: 60, align: 'right' },
  { field: 'chains', headerName: t('connections.components.fields.chains'), width: 280, minWidth: 160 },
  { field: 'rule', headerName: t('connections.components.fields.rule'), width: 220, minWidth: 160 },
  { field: 'process', headerName: t('connections.components.fields.process'), width: 180, minWidth: 140 },
  { field: 'time', headerName: t('connections.components.fields.time'), width: 100, minWidth: 80, align: 'right' },
  { field: 'source', headerName: t('connections.components.fields.source'), width: 160, minWidth: 120 },
  { field: 'remoteDestination', headerName: t('connections.components.fields.destination'), width: 160, minWidth: 120 },
  { field: 'type', headerName: t('connections.components.fields.type'), width: 120, minWidth: 80 },
]

const columnVisibilityModel = ref<Record<string, boolean>>({})
const columnOrder = ref<string[]>([])
const columnWidths = ref<Record<string, number>>({})
const sorting = ref<{ id: ColumnField; desc: boolean } | null>(null)
const scrollTop = ref(0)
const containerHeight = ref(0)
const scrollContainerRef = ref<HTMLDivElement | null>(null)

const getCellValue = (field: ColumnField, conn: IConnectionsItem): string => {
  switch (field) {
    case 'host': return getConnectionHost(conn)
    case 'download': return formatConnectionTraffic(conn.download ?? 0)
    case 'upload': return formatConnectionTraffic(conn.upload ?? 0)
    case 'dlSpeed': return `${formatConnectionTraffic(conn.curDownload ?? 0)}/s`
    case 'ulSpeed': return `${formatConnectionTraffic(conn.curUpload ?? 0)}/s`
    case 'chains': return formatConnectionChains(conn.chains)
    case 'rule': return getConnectionRule(conn)
    case 'process': return getConnectionProcess(conn)
    case 'source': return getConnectionSource(conn)
    case 'remoteDestination': return getConnectionDestination(conn)
    case 'type': return getConnectionTypeLabel(conn)
    default: return ''
  }
}

const visibleColumns = computed(() => {
  const ordered = columnOrder.value.length > 0
    ? columnOrder.value.map(id => baseColumns.find(c => c.field === id)).filter(Boolean) as ColumnDef[]
    : baseColumns

  return ordered
    .filter(c => (columnVisibilityModel.value[c.field] ?? true) !== false)
    .map(c => ({
      ...c,
      size: columnWidths.value[c.field] ?? c.width,
    }))
})

const tableWidth = computed(() => visibleColumns.value.reduce((sum, c) => sum + c.size, 0))

const sortedConnections = computed(() => {
  if (!sorting.value) return props.connections
  const dir = sorting.value.desc ? -1 : 1
  return [...props.connections].sort((a, b) => {
    const aVal = getCellValue(sorting.value!.id, a)
    const bVal = getCellValue(sorting.value!.id, b)
    if (typeof aVal === 'number' || typeof bVal === 'number') {
      return ((Number(aVal) || 0) - (Number(bVal) || 0)) * dir
    }
    return String(aVal).localeCompare(String(bVal)) * dir
  })
})

const totalRowsHeight = computed(() => sortedConnections.value.length * ROW_HEIGHT)

const bodyScrollTop = computed(() => Math.max(0, scrollTop.value - ROW_HEIGHT))
const firstVisibleRow = computed(() => Math.max(0, Math.min(sortedConnections.value.length, Math.max(0, Math.floor(bodyScrollTop.value / ROW_HEIGHT) - OVERSCAN_ROWS))))
const lastVisibleRow = computed(() => Math.min(sortedConnections.value.length, Math.ceil((bodyScrollTop.value + containerHeight.value) / ROW_HEIGHT) + OVERSCAN_ROWS))
const visibleRows = computed(() => sortedConnections.value.slice(firstVisibleRow.value, lastVisibleRow.value))

const onScroll = (e: Event) => {
  const el = e.currentTarget as HTMLDivElement
  scrollTop.value = el.scrollTop
  containerHeight.value = el.clientHeight
}

const toggleSorting = (field: ColumnField) => {
  if (!sorting.value || sorting.value.id !== field) { sorting.value = { id: field, desc: false }; return }
  if (!sorting.value.desc) { sorting.value = { id: field, desc: true }; return }
  sorting.value = null
}

onMounted(() => {
  if (scrollContainerRef.value) {
    containerHeight.value = scrollContainerRef.value.clientHeight
  }
})

const borderColor = 'var(--divider-color)'
const textSecondary = 'var(--text-secondary)'

const onCloseWrapper = () => props.onCloseColumnManager?.()

const handleResizeMouseDown = (column: ColumnDef & { size: number }, event: MouseEvent) => {
  event.preventDefault()
  event.stopPropagation()
  startResize(column.field, event.clientX, column.size, column.minWidth)
}

const handleResizeTouchStart = (column: ColumnDef & { size: number }, event: TouchEvent) => {
  event.stopPropagation()
  const touch = event.touches[0]
  if (!touch) return
  startResize(column.field, touch.clientX, column.size, column.minWidth)
}

const startResize = (field: ColumnField, startClientX: number, startWidth: number, minWidth: number) => {
  const handleMove = (clientX: number) => {
    columnWidths.value = { ...columnWidths.value, [field]: Math.max(minWidth, startWidth + clientX - startClientX) }
  }
  const handleMouseMove = (e: MouseEvent) => handleMove(e.clientX)
  const handleMouseUp = () => { window.removeEventListener('mousemove', handleMouseMove); window.removeEventListener('mouseup', handleMouseUp) }
  window.addEventListener('mousemove', handleMouseMove)
  window.addEventListener('mouseup', handleMouseUp)
}

const managerColumns = computed<ConnectionColumnOption[]>(() => {
  const ordered = columnOrder.value.length > 0
    ? columnOrder.value.map(id => baseColumns.find(c => c.field === id)).filter(Boolean) as ColumnDef[]
    : baseColumns
  return ordered.map(c => ({
    id: c.field,
    label: c.headerName,
    visible: (columnVisibilityModel.value[c.field] ?? true) !== false,
    toggleVisibility: (visible: boolean) => {
      const current = { ...columnVisibilityModel.value }
      const visibleCount = baseColumns.reduce((count, col) => {
        if (col.field === c.field) return count + (visible ? 1 : 0)
        return count + ((current[col.field] ?? true) !== false ? 1 : 0)
      }, 0)
      if (visibleCount === 0) return
      if (!visible) current[c.field] = false
      else delete current[c.field]
      columnVisibilityModel.value = current
    },
  }))
})

const handleManagerOrderChange = (order: string[]) => {
  columnOrder.value = order
}

const handleResetColumns = () => {
  columnVisibilityModel.value = {}
  columnOrder.value = []
  columnWidths.value = {}
  sorting.value = null
}
</script>
