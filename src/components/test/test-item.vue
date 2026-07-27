<template>
  <div
    :style="{
      position: 'relative',
    }"
  >
    <TestBox
      @contextmenu="handleContextMenu"
    >
      <div style="position: relative;">
        <template v-if="itemData.icon && itemData.icon.trim() !== ''">
          <div style="display: flex; justify-content: center;">
            <img
              v-if="itemData.icon.trim().startsWith('http')"
              :src="iconCachePath === '' ? itemData.icon : iconCachePath"
              height="40"
            />
            <img
              v-else-if="itemData.icon.trim().startsWith('data')"
              :src="itemData.icon"
              height="40"
            />
            <img
              v-else-if="itemData.icon.trim().startsWith('<svg')"
              :src="`data:image/svg+xml;base64,${btoa(itemData.icon)}`"
              height="40"
            />
          </div>
        </template>
        <template v-else>
          <div style="display: flex; justify-content: center;">
            <svg viewBox="0 0 24 24" width="40" height="40" fill="currentColor">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/>
            </svg>
          </div>
        </template>

        <div style="display: flex; justify-content: center;">{{ itemData.name }}</div>
      </div>

      <div style="border-top: 1px solid var(--divider-color); margin-top: 8px;" />

      <div
        style="
          display: flex;
          justify-content: center;
          margin-top: 8px;
          color: var(--primary-main);
        "
      >
        <template v-if="delay === -2">
          <span class="widget">
            <BaseLoading />
          </span>
        </template>

        <template v-else-if="delay === -1">
          <span
            class="widget the-check"
            @click.stop="onDelay"
          >
            {{ t('tests.components.item.actions.test') }}
          </span>
        </template>

        <template v-else-if="delay >= 0">
          <span
            class="widget the-delay"
            :style="{ color: delayManager.formatDelayColor(delay) }"
            @click.stop="onDelay"
          >
            {{ delayManager.formatDelay(delay) }}
          </span>
        </template>
      </div>
    </TestBox>

    <n-dropdown
      trigger="manual"
      :show="!!anchorEl"
      :options="menuOptions"
      :x="position.left"
      :y="position.top"
      @clickoutside="handleCloseMenu"
      @select="handleMenuSelect"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useTranslation } from '@/composables/use-i18n'

import { BaseLoading } from '@/components/base'
import { useIconCache } from '@/hooks/use-icon-cache'
import { cmdTestDelay } from '@/services/cmds'
import delayManager from '@/services/delay'
import { subscribeVergeEvents } from '@/services/events'
import { showNotice } from '@/services/notice-service'

import TestBox from './test-box.vue'

interface Props {
  id: string
  itemData: IVergeTestItem
  onEdit: () => void
  onDelete: (uid: string) => void
}

const props = defineProps<Props>()

const { t } = useTranslation()
const anchorEl = ref<HTMLElement | null>(null)
const position = ref({ left: 0, top: 0 })
const delay = ref(-1)
const { uid, name, icon, url } = props.itemData
const iconCachePath = useIconCache({ icon, cacheKey: uid })
const testLock = ref(false)

const onDelay = async () => {
  if (testLock.value) return
  testLock.value = true
  try {
    delay.value = -2
    const result = await cmdTestDelay(url)
    delay.value = result
  } finally {
    testLock.value = false
  }
}

const handleContextMenu = (event: MouseEvent) => {
  const { clientX, clientY } = event
  position.value = { left: clientX, top: clientY }
  anchorEl.value = event.currentTarget as HTMLElement
  event.preventDefault()
}

const handleCloseMenu = () => {
  anchorEl.value = null
}

const onEditTest = () => {
  anchorEl.value = null
  props.onEdit()
}

const onDelete = async () => {
  anchorEl.value = null
  try {
    props.onDelete(uid)
  } catch (err: any) {
    showNotice.error(err)
  }
}

const menuOptions = computed(() => [
  {
    label: t('shared.actions.edit'),
    key: 'edit',
  },
  {
    label: t('shared.actions.delete'),
    key: 'delete',
  },
])

const handleMenuSelect = (key: string) => {
  if (key === 'edit') onEditTest()
  if (key === 'delete') onDelete()
}

let unsubscribe: (() => void) | null = null

onMounted(() => {
  unsubscribe = subscribeVergeEvents({ 'verge://test-all': () => onDelay() })
})

onUnmounted(() => {
  if (unsubscribe) unsubscribe()
})
</script>

<style scoped>
.widget {
  padding: 3px 6px;
  font-size: 14px;
  border-radius: 4px;
}

.widget.the-check:hover {
  background-color: color-mix(in srgb, var(--primary-main) 15%, transparent);
}

.widget.the-delay:hover {
  background-color: color-mix(in srgb, var(--primary-main) 15%, transparent);
}
</style>
