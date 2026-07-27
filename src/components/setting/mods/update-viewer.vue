<template>
  <n-modal v-model:show="show" title="Update">
    <n-card closable @close="show = false">
      <template #header>
        <n-space align="center" justify="space-between">
          <span>{{ t('settings.modals.update.title', { version: updateInfo?.version ?? '' }) }}</span>
          <n-button size="small" @click="openReleaseUrl">
            {{ t('settings.modals.update.actions.goToRelease') }}
          </n-button>
        </n-space>
      </template>
      <div v-if="updateInfo?.body" v-html="renderedBody" />
      <div v-else>
        {{ t('settings.modals.update.messages.available') }}
      </div>
      <n-progress v-if="updating" :value="progress" :indicator-placement="'inside'" />
      <template #footer>
        <n-space justify="end">
          <n-button @click="show = false">{{ t('shared.actions.cancel') }}</n-button>
          <n-button type="primary" :loading="updating" @click="onUpdate">
            {{ t('settings.modals.update.actions.update') }}
          </n-button>
        </n-space>
      </template>
    </n-card>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
import { open as openUrl } from '@tauri-apps/plugin-shell'
import type { DownloadEvent } from '@tauri-apps/plugin-updater'

import { useUpdate } from '@/hooks/use-update'
import { restartApp } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { useUpdateState, useSetUpdateState } from '@/services/states'
import { useTranslation } from '@/composables/use-i18n'

const { t } = useTranslation()
const show = ref(false)
const updating = ref(false)
const downloaded = ref(0)
const total = ref(0)

const progress = computed(() => {
  if (total.value <= 0) return 0
  return Math.min((downloaded.value / total.value) * 100, 100)
})

const updateState = useUpdateState()
const setUpdateState = useSetUpdateState()
const { updateInfo } = useUpdate()

const renderedBody = computed(() => {
  if (!updateInfo.value?.body) return ''
  return updateInfo.value.body
    .replace(/```/g, '<code>')
    .replace(/\n/g, '<br/>')
})

const openReleaseUrl = () => {
  openUrl(`https://github.com/clash-verge-rev/clash-verge-rev/releases/tag/v${updateInfo.value?.version}`)
}

const onUpdate = async () => {
  if (!updateInfo.value?.body) return
  if (updating.value) return
  updating.value = true
  setUpdateState(true)
  downloaded.value = 0
  total.value = 0

  try {
    await updateInfo.value.downloadAndInstall((event: DownloadEvent) => {
      if (event.event === 'Started') {
        total.value = event.data.contentLength ?? 0
        downloaded.value = 0
      }
      if (event.event === 'Progress') {
        downloaded.value += event.data.chunkLength
      }
      if (event.event === 'Finished' && total.value === 0) {
        total.value = downloaded.value
      }
    })
    await restartApp()
  } catch (err: any) {
    showNotice.error(err)
  } finally {
    setUpdateState(false)
    updating.value = false
  }
}

defineExpose({
  open: () => { show.value = true },
  close: () => { show.value = false },
})
</script>
