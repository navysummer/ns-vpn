<template>
  <div
    v-if="open && detail"
    style="position: fixed; bottom: 16px; right: 16px; z-index: 1400; max-width: 520px; max-height: 480px; overflow-y: auto; background-color: var(--bg-paper); color: var(--text-primary); border-radius: 4px; box-shadow: 0 4px 20px rgba(0,0,0,0.15); padding: 16px;"
  >
    <div style="user-select: text; color: var(--text-secondary);">
      <div v-for="info in information" :key="info.label">
        <b>{{ info.label }}</b>
        <span style="word-break: break-all; color: var(--text-primary);">: {{ info.value }}</span>
      </div>
      <div v-if="!closed" style="text-align: right; margin-top: 12px;">
        <button
          class="MuiButton-root MuiButton-contained"
          :title="t('connections.components.actions.closeConnection')"
          @click="onDelete(); onClose()"
        >
          {{ t('connections.components.actions.closeConnection') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import { closeConnection } from 'tauri-plugin-mihomo-api'
import parseTraffic from '@/utils/parse-traffic'

dayjs.extend(relativeTime)

const { t } = useI18n()
const open = ref(false)
const detail = ref<IConnectionsItem | null>(null)
const closed = ref(false)

const onClose = () => {
  open.value = false
  detail.value = null
  closed.value = false
}

const information = computed(() => {
  if (!detail.value) return []
  const data = detail.value
  const { metadata, rulePayload } = data
  const chains = [...data.chains].reverse().join(' / ')
  const rule = rulePayload ? `${data.rule}(${rulePayload})` : data.rule
  const hostAddress = metadata.host || metadata.destinationIP || metadata.remoteDestination
  const host = `${hostAddress}:${metadata.destinationPort}`
  const Destination = metadata.destinationIP ? metadata.destinationIP : metadata.remoteDestination

  return [
    { label: t('connections.components.fields.host'), value: host },
    { label: t('shared.labels.downloaded'), value: parseTraffic(data.download).join(' ') },
    { label: t('shared.labels.uploaded'), value: parseTraffic(data.upload).join(' ') },
    { label: t('connections.components.fields.dlSpeed'), value: parseTraffic(data.curDownload ?? -1).join(' ') + '/s' },
    { label: t('connections.components.fields.ulSpeed'), value: parseTraffic(data.curUpload ?? -1).join(' ') + '/s' },
    { label: t('connections.components.fields.chains'), value: chains },
    { label: t('connections.components.fields.rule'), value: rule },
    { label: t('connections.components.fields.process'), value: `${metadata.process}${metadata.processPath ? `(${metadata.processPath})` : ''}` },
    { label: t('connections.components.fields.time'), value: dayjs(data.start).fromNow() },
    { label: t('connections.components.fields.source'), value: `${metadata.sourceIP}:${metadata.sourcePort}` },
    { label: t('connections.components.fields.destination'), value: Destination },
    { label: t('connections.components.fields.destinationPort'), value: `${metadata.destinationPort}` },
    { label: t('connections.components.fields.type'), value: `${metadata.type}(${metadata.network})` },
  ]
})

const openDetail = (d: IConnectionsItem, isClosed: boolean) => {
  if (open.value) return
  detail.value = d
  closed.value = isClosed
  open.value = true
}

let deleteLock = false
const onDelete = async () => {
  if (deleteLock || !detail.value) return
  deleteLock = true
  try { await closeConnection(detail.value.id) }
  finally { deleteLock = false }
}

defineExpose({ open: openDetail, close: onClose })
</script>
