<template>
  <div style="box-sizing: border-box; min-height: 56px; display: flex; align-items: center; gap: 8px; padding: 6px 48px 6px 12px; border-bottom: 1px solid var(--divider-color); position: relative; overflow: hidden;">
    <div
      style="min-width: 0; flex: 1; cursor: pointer; user-select: text;"
      @click="handleShowDetail"
    >
      <div style="font-size: 14px; line-height: 1.4; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
        {{ row.host }}
      </div>
      <div style="display: flex; flex-wrap: wrap; gap: 4px; margin-top: 4px; overflow: hidden;">
        <span style="box-sizing: border-box; max-width: 100%; padding: 0 4px; border: 1px solid rgba(128,128,128,0.35); border-radius: 4px; font-size: 10px; line-height: 1.375; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
          {{ row.network }}
        </span>
        <span style="box-sizing: border-box; max-width: 100%; padding: 0 4px; border: 1px solid rgba(128,128,128,0.35); border-radius: 4px; font-size: 10px; line-height: 1.375; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
          {{ row.type }}
        </span>
        <span v-if="row.process" style="box-sizing: border-box; max-width: 100%; padding: 0 4px; border: 1px solid rgba(128,128,128,0.35); border-radius: 4px; font-size: 10px; line-height: 1.375; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
          {{ row.process }}
        </span>
        <span v-if="row.chains" style="box-sizing: border-box; max-width: 100%; padding: 0 4px; border: 1px solid rgba(128,128,128,0.35); border-radius: 4px; font-size: 10px; line-height: 1.375; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
          {{ row.chains }}
        </span>
        <span style="box-sizing: border-box; max-width: 100%; padding: 0 4px; border: 1px solid rgba(128,128,128,0.35); border-radius: 4px; font-size: 10px; line-height: 1.375; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
          <ConnectionRelativeTime :start="row.time" />
        </span>
        <span v-if="showTraffic" style="box-sizing: border-box; max-width: 100%; padding: 0 4px; border: 1px solid rgba(128,128,128,0.35); border-radius: 4px; font-size: 10px; line-height: 1.375; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
          {{ row.uploadSpeedText }} / {{ row.downloadSpeedText }}
        </span>
      </div>
    </div>
    <button
      v-if="!closed"
      class="MuiIconButton-root MuiIconButton-sizeSmall"
      style="position: absolute; right: 8px; top: 50%; transform: translateY(-50%);"
      @click="onDelete"
      :title="t('connections.components.actions.closeConnection')"
      :aria-label="t('connections.components.actions.closeConnection')"
    >
      <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { closeConnection } from 'tauri-plugin-mihomo-api'
import ConnectionRelativeTime from './connection-relative-time.vue'
import type { ConnectionRowView } from './connection-row-view'

const props = defineProps<{
  row: ConnectionRowView
  closed: boolean
  onShowDetail: (id: string) => void
}>()

const { t } = useI18n()

const showTraffic = computed(() => props.row.uploadSpeed >= 100 || props.row.downloadSpeed >= 100)

const handleShowDetail = () => props.onShowDetail(props.row.id)

let deleteLock = false
const onDelete = async () => {
  if (deleteLock) return
  deleteLock = true
  try { await closeConnection(props.row.id) }
  finally { deleteLock = false }
}
</script>
