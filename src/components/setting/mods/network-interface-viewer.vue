<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'

import { BaseDialog, BaseEmpty, DialogRef } from '@/components/base'
import { useNetworkInterfaces } from '@/hooks/use-network'
import { showNotice } from '@/services/notice-service'

const open = ref(false)
const isV4 = ref(true)

defineExpose<DialogRef>({
  open: () => { open.value = true },
  close: () => { open.value = false },
})

const { networkInterfaces, loading } = useNetworkInterfaces()
const isEmpty = networkInterfaces.length === 0
</script>

<template>
  <BaseDialog
    :open="open"
    :contentSx="{ width: '450px' }"
    :disableOk="true"
    :cancelBtn="i18n.t('shared.actions.close')"
    @onClose="open = false"
    @onCancel="open = false"
  >
    <template #title>
      <div style="display: flex; justify-content: space-between;">
        <span>{{ i18n.t('settings.modals.networkInterface.title') }}</span>
        <div>
          <button class="MuiButton-root MuiButton-contained MuiButton-sizeSmall" @click="isV4 = !isV4">
            {{ isV4 ? 'Ipv6' : 'Ipv4' }}
          </button>
        </div>
      </div>
    </template>

    <div v-if="loading && isEmpty" style="display: flex; justify-content: center; padding: 32px 0;">
      <svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor"><circle cx="12" cy="12" r="10" fill="none" stroke="currentColor" stroke-width="3" stroke-dasharray="31.4 31.4" stroke-linecap="round"/></svg>
    </div>
    <div v-else-if="isEmpty" style="min-height: 160px;">
      <BaseEmpty />
    </div>
    <div v-else>
      <div v-for="item in networkInterfaces" :key="item.name">
        <h4>{{ item.name }}</h4>
        <div>
          <div v-for="address in item.addr" :key="(isV4 ? address.V4?.ip : address.V6?.ip) || ''">
            <div v-if="isV4 ? address.V4?.ip : address.V6?.ip" style="display: flex; justify-content: space-between; margin: 8px 0;">
              <span>{{ i18n.t('settings.modals.networkInterface.fields.ipAddress') }}</span>
              <div style="border-radius: 8px; padding: 2px 2px 2px 8px; display: flex; align-items: center;">
                <span style="user-select: text;">{{ isV4 ? address.V4?.ip : address.V6?.ip }}</span>
                <button class="MuiIconButton-root MuiIconButton-sizeSmall" @click="async () => {
                  await writeText(isV4 ? address.V4?.ip || '' : address.V6?.ip || '')
                  showNotice.success('shared.feedback.notifications.common.copySuccess')
                }">
                  <svg viewBox="0 0 24 24" width="18px" height="18px" fill="currentColor"><path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/></svg>
                </button>
              </div>
            </div>
          </div>
          <div style="display: flex; justify-content: space-between; margin: 8px 0;">
            <span>{{ i18n.t('settings.modals.networkInterface.fields.macAddress') }}</span>
            <div style="border-radius: 8px; padding: 2px 2px 2px 8px; display: flex; align-items: center;">
              <span style="user-select: text;">{{ item.mac_addr ?? '' }}</span>
              <button class="MuiIconButton-root MuiIconButton-sizeSmall" @click="async () => {
                await writeText(item.mac_addr ?? '')
                showNotice.success('shared.feedback.notifications.common.copySuccess')
              }">
                <svg viewBox="0 0 24 24" width="18px" height="18px" fill="currentColor"><path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/></svg>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </BaseDialog>
</template>
