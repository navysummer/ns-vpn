<template>
  <template v-if="updateInfo?.available">
    <UpdateViewer ref="viewerRef" />
    <n-button
      type="error"
      size="small"
      :class="className"
      @click="viewerRef?.open()"
    >
      {{ t('shared.actions.new') }}
    </n-button>
  </template>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useTranslation } from '@/composables/use-i18n'
const { t } = useTranslation()
import { useUpdate } from '@/hooks/use-update'
import UpdateViewer from '../setting/mods/update-viewer.vue'

interface Props {
  className?: string
}

const props = defineProps<Props>()
const { className } = props
const { updateInfo } = useUpdate()
const viewerRef = ref<{ open: () => void; close: () => void } | null>(null)
</script>
