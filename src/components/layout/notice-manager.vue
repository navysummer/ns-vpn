<template>
  <div
    class="notice-container"
    :style="containerStyle"
  >
    <template v-for="notice in currentNotices" :key="notice.id">
      <div
        class="notice-item"
        :style="{
          position: 'relative',
          width: '100%',
        }"
        @contextmenu.prevent="handleNoticeCopy(notice)"
      >
        <n-alert
          :type="notice.type"
          closable
          @close="handleClose(notice.id)"
          style="width: 100%"
        >
          {{ resolveNoticeMessage(notice, t) }}
        </n-alert>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useTranslation } from '@/composables/use-i18n'
const { t } = useTranslation()

import {
  subscribeNotices,
  hideNotice,
  getSnapshotNotices,
  showNotice,
} from '@/services/notice-service'
import type { TranslationKey } from '@/types/generated/i18n-keys'

type NoticePosition = NonNullable<IVergeConfig['notice_position']>
type NoticeItem = ReturnType<typeof getSnapshotNotices>[number]

const props = defineProps<{
  position?: NoticePosition | null
}>()

const VALID_POSITIONS: NoticePosition[] = [
  'top-left',
  'top-right',
  'bottom-left',
  'bottom-right',
]

const resolvePosition = (position?: NoticePosition | null): NoticePosition => {
  if (position && VALID_POSITIONS.includes(position)) {
    return position
  }
  return 'top-right'
}

const getAnchorOrigin = (position: NoticePosition) => {
  const [vertical, horizontal] = position.split('-') as [string, string]
  return { vertical, horizontal }
}

const resolveNoticeMessage = (notice: NoticeItem, tFn: Function): string => {
  const i18n = notice.i18n
  if (!i18n) return notice.message

  const params = (i18n.params ?? {}) as Record<string, unknown>
  const { prefixKey, prefixParams, prefix, message, ...restParams } = params

  const prefixKeyParams =
    prefixParams && typeof prefixParams === 'object'
      ? (prefixParams as Record<string, unknown>)
      : undefined

  const resolvedPrefix =
    typeof prefixKey === 'string'
      ? tFn(prefixKey as TranslationKey, {
          defaultValue: prefixKey,
          ...(prefixKeyParams ?? {}),
          ...restParams,
        })
      : typeof prefix === 'string'
        ? prefix
        : undefined

  const messageStr = typeof message === 'string' ? message : undefined

  const defaultValue =
    messageStr === undefined
      ? undefined
      : resolvedPrefix
        ? `${resolvedPrefix} ${messageStr}`
        : messageStr

  return tFn(i18n.key as TranslationKey, {
    defaultValue,
    ...restParams,
    ...(resolvedPrefix !== undefined ? { prefix: resolvedPrefix } : {}),
    ...(messageStr !== undefined ? { message: messageStr } : {}),
  })
}

const extractNoticeCopyText = (input: unknown): string | undefined => {
  if (input === null || input === undefined) return undefined
  if (typeof input === 'string') return input
  if (typeof input === 'number' || typeof input === 'boolean') {
    return String(input)
  }
  if (input instanceof Error) {
    return input.message || input.name
  }
  if (typeof input === 'object') {
    const maybeMessage = (input as { message?: unknown }).message
    if (typeof maybeMessage === 'string') return maybeMessage
  }
  try {
    return JSON.stringify(input)
  } catch {
    return String(input)
  }
}

const resolveNoticeCopyText = (
  notice: NoticeItem,
  tFn: Function,
): string | undefined => {
  if (
    notice.i18n?.key === 'shared.feedback.notices.prefixedRaw' ||
    notice.i18n?.key === 'shared.feedback.notices.raw'
  ) {
    const rawText = extractNoticeCopyText(notice.i18n?.params?.message)
    if (rawText) return rawText
  }
  return (
    extractNoticeCopyText(resolveNoticeMessage(notice, tFn)) ??
    extractNoticeCopyText(notice.message)
  )
}

const resolvedPosition = computed(() => resolvePosition(props.position))
const anchorOrigin = computed(() => getAnchorOrigin(resolvedPosition.value))
const currentNotices = getSnapshotNotices()

const handleClose = (id: number) => {
  hideNotice(id)
}

const handleNoticeCopy = async (notice: NoticeItem) => {
  const text = resolveNoticeCopyText(notice, t)
  if (!text) return
  try {
    await navigator.clipboard.writeText(text)
    showNotice.success('shared.feedback.notifications.common.copySuccess', 1000)
  } catch (error) {
    console.warn('[NoticeManager] copy to clipboard failed:', error)
  }
}

const containerStyle = computed(() => ({
  position: 'fixed' as const,
  top: anchorOrigin.value.vertical === 'top' ? '20px' : 'auto',
  bottom: anchorOrigin.value.vertical === 'bottom' ? '20px' : 'auto',
  left: anchorOrigin.value.horizontal === 'left' ? '20px' : 'auto',
  right: anchorOrigin.value.horizontal === 'right' ? '20px' : 'auto',
  zIndex: 1500,
  display: 'flex',
  flexDirection: 'column' as const,
  gap: '10px',
  maxWidth: '360px',
}))
</script>

<style scoped>
.notice-item {
  position: relative;
  width: 100%;
}
</style>
