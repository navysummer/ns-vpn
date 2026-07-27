<template>
  <BasePage
    full
    :content-style="{
      height: '100%',
      display: 'flex',
      flexDirection: 'column',
      overflow: 'hidden',
      borderRadius: '8px',
      minHeight: 0,
    }"
  >
    <template #title>
      <span style="white-space: nowrap;">{{ t('connections.page.title') }}</span>
    </template>
    <template #header>
      <div style="display: flex; align-items: center; gap: 16px;">
        <div style="margin-left: 8px; margin-right: 8px;">
          {{ t('shared.labels.downloaded') }}: {{ parseTraffic(traffic?.downTotal || 0) }}
        </div>
        <div style="margin-left: 8px; margin-right: 8px;">
          {{ t('shared.labels.uploaded') }}: {{ parseTraffic(traffic?.upTotal || 0) }}
        </div>
        <button
          class="MuiIconButton-root MuiIconButton-sizeSmall"
          @click="toggleLayout"
        >
          <svg v-if="isTableLayout" viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z"/></svg>
          <svg v-else viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M3 3h18v2H3V3zm0 8h18v2H3v-2zm0 8h18v2H3v-2z"/></svg>
        </button>
        <button class="MuiButton-root MuiButton-contained MuiButton-sizeSmall" @click="onCloseAll">
          <span style="white-space: nowrap;">{{ t('shared.actions.closeAll') }}</span>
        </button>
      </div>
    </template>

    <div
      style="
        padding-top: 8px;
        margin-bottom: 4px;
        margin-left: 10px;
        margin-right: 10px;
        min-height: 36px;
        display: flex;
        align-items: center;
        gap: 8px;
        user-select: text;
        position: sticky;
        top: 0;
        z-index: 2;
      "
    >
      <div style="display: flex; margin-right: 8px; flex-basis: content; gap: 0;">
        <button
          class="MuiButton-root MuiButton-sizeSmall"
          :class="connectionsType === 'active' ? 'MuiButton-contained' : 'MuiButton-outlined'"
          style="border-radius: 0;"
          @click="selectConnectionsType('active')"
        >
          {{ t('connections.components.actions.active') }} {{ connections?.activeConnections.length }}
        </button>
        <button
          class="MuiButton-root MuiButton-sizeSmall"
          :class="connectionsType === 'closed' ? 'MuiButton-contained' : 'MuiButton-outlined'"
          style="border-radius: 0;"
          @click="selectConnectionsType('closed')"
        >
          {{ t('connections.components.actions.closed') }} {{ connections?.closedConnections.length }}
        </button>
      </div>
      <select
        v-if="!isTableLayout"
        v-model="curOrderOpt"
        style="padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color); font-size: 14px;"
      >
        <option v-for="option in ORDER_OPTIONS" :key="option.id" :value="option.id">
          {{ t(option.labelKey) }}
        </option>
      </select>
      <div style="flex: 1; display: flex; align-items: center;">
        <input
          type="text"
          :placeholder="t('shared.search')"
          style="flex: 1; padding: 6px 12px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color); box-sizing: border-box;"
          @input="handleSearch(($event.target as HTMLInputElement).value)"
        />
      </div>
      <button
        v-if="isTableLayout && filterConn.length > 0"
        class="MuiIconButton-root MuiIconButton-sizeSmall"
        :title="t('connections.components.columnManager.title')"
        :aria-label="t('connections.components.columnManager.title')"
        style="flex: 0 0 auto;"
        @click="isColumnManagerOpen = true"
      >
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M4 5v14h16V5H4zm6 12H6v-2h4v2zm0-4H6v-2h4v2zm0-4H6V7h4v2zm8 8h-6v-2h6v2zm0-4h-6v-2h6v2zm0-4h-6V7h6v2z"/></svg>
      </button>
    </div>

    <div v-if="filterConn.length === 0">
      <BaseEmpty />
    </div>
    <ConnectionTable
      v-else-if="isTableLayout"
      :connections="filterConn"
      :on-show-detail="showDetailById"
      :column-manager-open="isColumnManagerOpen"
      :on-close-column-manager="() => isColumnManagerOpen = false"
    />
    <div
      v-else
      style="flex: 1; border-radius: 8px; overflow-y: auto; -webkit-overflow-scrolling: touch; overscroll-behavior: contain;"
    >
      <ConnectionRowItem
        v-for="row in displayRows"
        :key="row.id"
        :row="row"
        :closed="connectionsType === 'closed'"
        :on-show-detail="showDetailById"
      />
    </div>

    <ConnectionDetail ref="detailRef" />

    <div
      v-if="connectionsType === 'closed' && filterConn.length > 0"
      style="position: absolute; right: 16px; bottom: 16px; z-index: 1050;"
    >
      <button
        class="MuiFab-root MuiFab-extended MuiFab-sizeMedium MuiFab-primary"
        style="display: flex; align-items: center; gap: 8px;"
        @click="clearClosedConnections()"
      >
        <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
        {{ t('shared.actions.clear') }}
      </button>
    </div>
  </BasePage>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { closeAllConnections } from 'tauri-plugin-mihomo-api'

import { BaseEmpty, BasePage, BaseSearchBox, BaseStyledSelect } from '@/components/base'
import ConnectionDetail from '@/components/connection/connection-detail.vue'
import ConnectionRowItem from '@/components/connection/connection-row-item.vue'
import { getConnectionStartTime, useConnectionRowViews } from '@/components/connection/connection-row-view'
import ConnectionTable from '@/components/connection/connection-table.vue'
import { useConnectionData } from '@/hooks/use-connection-data'
import { useConnectionSetting } from '@/hooks/use-connection-setting'
import { useTrafficData } from '@/hooks/use-traffic-data'
import { useVisibility } from '@/hooks/use-visibility'
import parseTraffic from '@/utils/parse-traffic'

type OrderFunc = (list: IConnectionsItem[]) => IConnectionsItem[]

const ORDER_OPTIONS = [
  {
    id: 'default',
    labelKey: 'connections.components.order.default',
    fn: (list: IConnectionsItem[]) => list.sort((a, b) => getConnectionStartTime(b) - getConnectionStartTime(a)),
  },
  {
    id: 'uploadSpeed',
    labelKey: 'connections.components.order.uploadSpeed',
    fn: (list: IConnectionsItem[]) => list.sort((a, b) => (b.curUpload ?? 0) - (a.curUpload ?? 0)),
  },
  {
    id: 'downloadSpeed',
    labelKey: 'connections.components.order.downloadSpeed',
    fn: (list: IConnectionsItem[]) => list.sort((a, b) => (b.curDownload ?? 0) - (a.curDownload ?? 0)),
  },
] as const

type OrderKey = (typeof ORDER_OPTIONS)[number]['id']

const { t } = useI18n()
const pageVisible = useVisibility()
const matchText = ref('')
const hasSearch = ref(false)
const curOrderOpt = ref<OrderKey>('default')
const connectionsType = ref<'active' | 'closed'>('active')

const { response: { data: connections }, clearClosedConnections } = useConnectionData({ enabled: pageVisible })
const { response: { data: traffic } } = useTrafficData({ enabled: pageVisible })
const setting = useConnectionSetting()

const isTableLayout = computed(() => setting?.layout === 'table')
const isColumnManagerOpen = ref(false)

const selectedConnections = computed(() =>
  connectionsType.value === 'active'
    ? (connections?.activeConnections ?? EMPTY_CONNECTIONS)
    : (connections?.closedConnections ?? EMPTY_CONNECTIONS),
)

const EMPTY_CONNECTIONS: IConnectionsItem[] = []

const filterConn = computed(() => {
  const orderFunc = ORDER_OPTIONS.find(o => o.id === curOrderOpt.value)?.fn
  if (isTableLayout.value && !hasSearch.value) return selectedConnections.value
  if (!hasSearch.value) return orderFunc ? orderFunc([...selectedConnections.value]) : [...selectedConnections.value]
  const matchConns = selectedConnections.value.filter((conn: IConnectionsItem) => {
    const { host, destinationIP, process } = conn.metadata
    return matchConn(host || '') || matchConn(destinationIP || '') || matchConn(process || '')
  })
  return orderFunc ? orderFunc(matchConns) : matchConns
})

const matchConn = (input: string) => input.toLowerCase().includes(matchText.value.toLowerCase())

const displayRows = useConnectionRowViews(
  isTableLayout.value ? EMPTY_CONNECTIONS : filterConn.value,
)

const detailRef = ref<InstanceType<typeof ConnectionDetail> | null>(null)

const selectConnectionsType = (type: 'active' | 'closed') => {
  if (type === connectionsType.value) return
  detailRef.value?.close()
  isColumnManagerOpen.value = false
  connectionsType.value = type
}

const showDetailById = (id: string) => {
  const connection = filterConn.value.find((item: IConnectionsItem) => item.id === id)
  if (connection) detailRef.value?.open(connection, connectionsType.value === 'closed')
}

let closeAllLock = false
const onCloseAll = async () => {
  if (closeAllLock) return
  closeAllLock = true
  try { await closeAllConnections() }
  finally { closeAllLock = false }
}

const handleSearch = (text: string) => {
  matchText.value = text
  hasSearch.value = text.length > 0
}

const toggleLayout = () => {
  setSetting((o: any) => o?.layout !== 'table' ? { ...o, layout: 'table' } : { ...o, layout: 'list' })
}
</script>
