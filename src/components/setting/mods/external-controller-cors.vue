<script setup lang="ts">
import { ref, computed } from 'vue'
import i18n from 'i18next'

import { BaseDialog, Switch } from '@/components/base'
import { useClash } from '@/hooks/use-clash'
import { restartCore } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'

const DEV_URLS = ['tauri://localhost', 'http://tauri.localhost', 'http://localhost:3000']

const buttonStyle = {
  borderRadius: '8px',
  textTransform: 'none',
  boxShadow: '0 2px 4px rgba(0,0,0,0.1)',
}

const addButtonStyle = { ...buttonStyle, backgroundColor: '#4CAF50', color: 'white' }
const deleteButtonStyle = { ...buttonStyle, backgroundColor: '#FF5252', color: 'white' }

const { clash, mutateClash, patchClash } = useClash()
const open = ref(false)
const lastKey = ref(0)
const saving = ref(false)

interface AllowOriginItem { key: number; value: string }

const corsConfig = ref<{ allowPrivateNetwork: boolean; allowOrigins: AllowOriginItem[] }>({
  allowPrivateNetwork: true,
  allowOrigins: [],
})

defineExpose({
  open: () => {
    const cors = clash?.['external-controller-cors']
    const origins = cors?.['allow-origins'] ?? []
    lastKey.value = 0
    corsConfig.value = {
      allowPrivateNetwork: cors?.['allow-private-network'] ?? true,
      allowOrigins: origins
        .filter((origin: string) => !DEV_URLS.includes(origin.trim()))
        .map((origin: string) => {
          lastKey.value += 1
          return { key: lastKey.value, value: origin }
        }),
    }
    open.value = true
  },
  close: () => { open.value = false },
})

const handleAddOrigin = () => {
  lastKey.value += 1
  corsConfig.value.allowOrigins.push({ key: lastKey.value, value: '' })
}

const handleUpdateOrigin = (index: number, value: string) => {
  corsConfig.value.allowOrigins[index] = { ...corsConfig.value.allowOrigins[index], value }
}

const handleDeleteOrigin = (index: number) => {
  corsConfig.value.allowOrigins.splice(index, 1)
}

const handleSave = async () => {
  saving.value = true
  try {
    const fullOrigins = [...corsConfig.value.allowOrigins.map((o) => o.value), ...DEV_URLS]
    await patchClash({
      'external-controller-cors': {
        'allow-private-network': corsConfig.value.allowPrivateNetwork,
        'allow-origins': [...new Set(fullOrigins)].filter((o: string) => o.trim() !== ''),
      },
    })
    await restartCore()
    await mutateClash()
    open.value = false
    showNotice.success('shared.feedback.notifications.common.saveSuccess')
  } catch {
    showNotice.error('shared.feedback.notifications.common.saveFailed')
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <BaseDialog
    :open="open"
    :title="i18n.t('settings.sections.externalCors.title')"
    :contentSx="{ width: '500px' }"
    :okBtn="saving ? i18n.t('shared.statuses.saving') : i18n.t('shared.actions.save')"
    :cancelBtn="i18n.t('shared.actions.cancel')"
    @onClose="open = false"
    @onCancel="open = false"
    @onOk="handleSave"
  >
    <ul class="MuiList-root" style="width: 90%; padding: 16px; list-style: none;">
      <li style="padding: 8px 0;">
        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
          <span>{{ i18n.t('settings.sections.externalCors.fields.allowPrivateNetwork') }}</span>
          <Switch
            edge="end"
            :checked="corsConfig.allowPrivateNetwork"
            @change="(e: any) => corsConfig.allowPrivateNetwork = e.target.checked"
          />
        </div>
      </li>

      <hr class="MuiDivider-root" style="margin: 16px 0;" />

      <li style="padding: 8px 0;">
        <div style="width: 100%;">
          <div style="margin-bottom: 8px; font-weight: bold;">
            {{ i18n.t('settings.sections.externalCors.fields.allowedOrigins') }}
          </div>
          <div v-for="({ key, value: origin }, index) in corsConfig.allowOrigins" :key="key" style="display: flex; align-items: center; margin-bottom: 8px;">
            <input
              :value="origin"
              @input="handleUpdateOrigin(index, ($event.target as HTMLInputElement).value)"
              :placeholder="i18n.t('settings.sections.externalCors.placeholders.origin')"
              style="flex: 1; margin-right: 16px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; font-size: 14px;"
            />
            <button
              class="MuiButton-root MuiButton-contained MuiButton-sizeSmall"
              :disabled="corsConfig.allowOrigins.length <= 0"
              style="background-color: #FF5252; color: white; border-radius: 8px;"
              @click="handleDeleteOrigin(index)"
            >
              <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
            </button>
          </div>
          <button
            class="MuiButton-root MuiButton-contained MuiButton-sizeSmall"
            style="background-color: #4CAF50; color: white; border-radius: 8px;"
            @click="handleAddOrigin"
          >
            {{ i18n.t('settings.sections.externalCors.actions.add') }}
          </button>

          <div style="margin-top: 12px; padding: 8px; background-color: #f5f5f5; border-radius: 4px;">
            <div style="color: #666; font-size: 12px; font-style: italic;">
              {{ i18n.t('settings.sections.externalCors.messages.alwaysIncluded', { urls: DEV_URLS.join(', ') }) }}
            </div>
          </div>
        </div>
      </li>
    </ul>
  </BaseDialog>
</template>
