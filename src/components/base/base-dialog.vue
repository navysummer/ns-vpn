<template>
  <n-modal
    :show="open"
    preset="card"
    :title="title"
    :style="{ maxWidth: '600px' }"
    :mask-closable="false"
    :closeable="true"
    @update:show="onClose"
  >
    <div :style="contentSx">
      <slot />
    </div>
    <template v-if="!disableFooter" #footer>
      <div style="display: flex; justify-content: flex-end; gap: 8px">
        <n-button v-if="!disableCancel" @click="onCancel">
          {{ cancelBtn }}
        </n-button>
        <n-button
          v-if="!disableOk"
          type="primary"
          :loading="loading"
          @click="onOk"
        >
          {{ okBtn }}
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  title: string
  open: boolean
  okBtn?: string
  cancelBtn?: string
  disableEnforceFocus?: boolean
  disableOk?: boolean
  disableCancel?: boolean
  disableFooter?: boolean
  contentSx?: Record<string, string | number>
  loading?: boolean
  onOk?: () => void
  onCancel?: () => void
  onClose?: () => void
}

interface DialogRef {
  open: () => void
  close: () => void
}

const props = defineProps<Props>()
const show = ref(props.open)

const emit = defineEmits<{
  ok: []
  cancel: []
  close: []
}>()

const onOk = () => {
  props.onOk?.()
  emit('ok')
}

const onCancel = () => {
  props.onCancel?.()
  emit('cancel')
}

const onClose = () => {
  props.onClose?.()
  emit('close')
}

const dialogOpen = () => {
  show.value = true
}

const dialogClose = () => {
  show.value = false
}

defineExpose({ open: dialogOpen, close: dialogClose })
</script>
