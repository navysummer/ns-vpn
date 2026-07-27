<template>
  <n-modal :show="open" @update:show="onClose" :mask-closable="true" preset="card" :title="title ?? t('profiles.modals.qrViewer.title')" style="max-width: 400px;">
    <div style="display: flex; justify-content: center; padding: 16px; background-color: #fff; border-radius: 4px;">
      <canvas ref="qrCanvasRef" />
    </div>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import QRCode from 'qrcode'

const props = defineProps<{
  open: boolean
  value: string
  title?: string
  onClose: () => void
}>()

const { t } = useI18n()
const qrCanvasRef = ref<HTMLCanvasElement | null>(null)

watch(() => props.open, async (val) => {
  if (val && props.value) {
    await nextTick()
    if (qrCanvasRef.value) {
      try {
        await QRCode.toCanvas(qrCanvasRef.value, props.value, { width: 256, margin: 2 })
      } catch { /* ignore */ }
    }
  }
})
</script>
