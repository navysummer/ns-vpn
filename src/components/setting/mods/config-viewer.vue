<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'

import { DialogRef } from '@/components/base'
import EditorViewer from '@/components/profile/editor-viewer.vue'
import { getRuntimeYaml } from '@/services/cmds'

const open = ref(false)
const loading = ref(false)
const runtimeConfig = ref('')

defineExpose<DialogRef>({
  open: () => {
    runtimeConfig.value = ''
    loading.value = true
    open.value = true
    getRuntimeYaml()
      .then((data: any) => {
        runtimeConfig.value = data ?? '# Error getting runtime yaml\n'
      })
      .catch(() => {
        runtimeConfig.value = '# Error getting runtime yaml\n'
      })
      .finally(() => {
        loading.value = false
      })
  },
  close: () => { open.value = false },
})
</script>

<template>
  <EditorViewer
    v-if="open"
    :open="true"
    :value="runtimeConfig"
    :readOnly="true"
    language="yaml"
    path="runtime-config.yaml"
    :loading="loading"
    @onClose="open = false"
  >
    <template #title>
      <div style="display: flex; align-items: center; gap: 16px;">
        <span>{{ i18n.t('settings.components.verge.advanced.fields.runtimeConfig') }}</span>
        <span class="MuiChip-root MuiChip-sizeSmall">{{ i18n.t('shared.labels.readOnly') }}</span>
      </div>
    </template>
  </EditorViewer>
</template>
