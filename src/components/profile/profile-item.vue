<template>
  <div style="position: relative;">
    <ProfileBox
      :aria-selected="selected"
      @click="onBoxClick"
      @contextmenu="onContextMenu"
    >
      <div
        v-if="activating"
        style="
          position: absolute;
          display: flex;
          justify-content: center;
          align-items: center;
          top: 10px;
          left: 10px;
          right: 10px;
          bottom: 2px;
          z-index: 10;
          backdrop-filter: blur(2px);
          background-color: rgba(0, 0, 0, 0.1);
        "
      >
        <div
          class="MuiCircularProgress-root MuiCircularProgress-indeterminate MuiCircularProgress-colorInherit"
          style="width: 20px; height: 20px; animation: pulse 1.5s ease-in-out infinite;"
        >
          <svg viewBox="22 22 44 44">
            <circle cx="44" cy="44" r="20.2" fill="none" stroke-width="3.6" style="stroke: currentColor; stroke-dasharray: 80px, 200px; stroke-dashoffset: 0; animation: mui-circular-progress 1.4s ease-in-out infinite;" />
          </svg>
        </div>
      </div>
      <div style="position: relative;">
        <div style="display: flex; justify-content: start;">
          <button
            v-if="batchMode"
            class="MuiIconButton-root MuiIconButton-sizeSmall"
            style="padding: 2px; margin-right: 4px; margin-left: -8px;"
            @click.stop="onSelectionChange?.()"
          >
            <svg v-if="isSelected" viewBox="0 0 24 24" width="20" height="20" fill="#1976d2"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM17.99 9l-1.41-1.42-6.59 6.59-2.58-2.57-1.42 1.41 4 3.99z"/></svg>
            <svg v-else viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M19 5v14H5V5h14m0-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z"/></svg>
          </button>
          <div
            ref="dragHandleRef"
            :style="{ display: 'flex', margin: 'auto 0', ...(batchMode ? { marginLeft: '-4px' } : {}) }"
          >
            <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor" style="cursor: move; margin-left: -6px;">
              <path d="M11 18c0 1.1-.9 2-2 2s-2-.9-2-2 .9-2 2-2 2 .9 2 2zm-2-8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0-6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm6 4c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"/>
            </svg>
          </div>
          <h2
            :style="{
              width: batchMode ? 'calc(100% - 56px)' : 'calc(100% - 36px)',
              fontSize: '18px',
              fontWeight: 600,
              lineHeight: '26px',
              margin: 0,
              overflow: 'hidden',
              textOverflow: 'ellipsis',
              whiteSpace: 'nowrap',
            }"
            :title="itemData.name"
          >
            {{ itemData.name || 'Profile' }}
          </h2>
        </div>
        <button
          v-if="hasUrl"
          class="MuiIconButton-root MuiIconButton-sizeSmall"
          :style="{
            position: 'absolute',
            padding: '3px',
            top: -1,
            right: -5,
            animation: loading ? '1s linear infinite round-anim' : 'none',
          }"
          :disabled="loading"
          @click.stop="onUpdateClick"
          :title="t('shared.actions.refresh')"
        >
          <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
        </button>
      </div>
      <div style="height: 26px; display: flex; align-items: center; justify-content: space-between;">
        <span
          v-if="description"
          style="font-size: 14px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;"
          :title="description"
        >
          {{ description }}
        </span>
        <span
          v-else-if="hasUrl"
          style="font-size: 14px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1;"
          :title="`${t('shared.labels.from')} ${from}`"
        >
          {{ from }}
        </span>
        <div v-if="hasUrl" style="display: flex; justify-content: flex-end; margin-left: auto;">
          <span
            :title="showNextUpdate ? t('profiles.components.profileItem.tooltips.showLast') : `${t('shared.labels.updateTime')}: ${parseExpire(updated)}\n${t('profiles.components.profileItem.tooltips.showNext')}`"
            style="
              font-size: 14px;
              text-align: right;
              cursor: pointer;
              display: inline-block;
              border-bottom: 1px dashed transparent;
              transition: all 0.2s;
              white-space: nowrap;
            "
            @click.stop="toggleUpdateTimeDisplay"
          >
            {{ showNextUpdate ? nextUpdateTime : (updated > 0 ? dayjs(updated * 1000).fromNow() : '') }}
          </span>
        </div>
      </div>
      <div v-if="hasExtra" style="height: 26px; display: flex; align-items: center; justify-content: space-between; font-size: 14px;">
        <span :title="t('shared.labels.usedTotal')">
          {{ parseTraffic(upload + download) }} / {{ parseTraffic(total) }}
        </span>
        <span :title="t('shared.labels.expireTime')">{{ expire }}</span>
      </div>
      <div v-else style="height: 26px; display: flex; align-items: center; justify-content: flex-end; font-size: 12px;">
        <span :title="t('shared.labels.updateTime')">{{ parseExpire(updated) }}</span>
      </div>
      <div
        class="MuiLinearProgress-root MuiLinearProgress-determinate"
        style="width: 100%; height: 4px; overflow: hidden; border-radius: 2px; opacity: total > 0 ? 1 : 0;"
      >
        <div
          class="MuiLinearProgress-bar MuiLinearProgress-barColorPrimary MuiLinearProgress-bar1Determinate"
          :style="{ width: `${progress}%`, height: '100%', backgroundColor: 'currentColor', transition: 'width 0.3s linear' }"
        />
      </div>
    </ProfileBox>

    <div
      v-if="anchorEl"
      class="MuiMenu-root MuiMenu-paper"
      :style="{ position: 'fixed', left: `${position.left}px`, top: `${position.top}px`, zIndex: 1300, minWidth: 120, backgroundColor: 'var(--bg-paper)', border: '1px solid var(--divider-color)', borderRadius: '4px', boxShadow: '0 5px 15px rgba(0,0,0,0.3)', padding: '4px 0' }"
      @click.stop
      @contextmenu.prevent="anchorEl = null"
    >
      <div
        v-for="item in currentMenu"
        :key="item.label"
        class="MuiMenuItem-root MuiMenuItem-dense"
        :style="{
          padding: '4px 16px',
          minWidth: 120,
          fontSize: '14px',
          cursor: item.disabled ? 'default' : 'pointer',
          opacity: item.disabled ? 0.5 : 1,
          color: item.label === menuLabels.delete ? '#f44336' : 'inherit',
        }"
        @click="item.handler"
      >
        {{ t(item.label) }}
      </div>
    </div>

    <EditorViewer
      v-if="fileOpen"
      :open="true"
      :value="profileDocument.value"
      language="yaml"
      :path="`profile:${uid}.yaml`"
      :loading="profileDocument.loading"
      :dirty="profileDocument.dirty"
      @change="profileDocument.setValue"
      @save="handleSaveProfileDocument"
      @close="fileOpen = false"
    />
    <RulesEditorViewer
      v-if="rulesOpen"
      :groups-uid="option?.groups ?? ''"
      :merge-uid="option?.merge ?? ''"
      :profile-uid="uid"
      :property="option?.rules ?? ''"
      :open="true"
      @save="onSave?.($event.prev, $event.curr)"
      @close="rulesOpen = false"
    />
    <ProxiesEditorViewer
      v-if="proxiesOpen"
      :profile-uid="uid"
      :property="option?.proxies ?? ''"
      :open="true"
      @save="onSave?.($event.prev, $event.curr)"
      @close="proxiesOpen = false"
    />
    <GroupsEditorViewer
      v-if="groupsOpen"
      :merge-uid="option?.merge ?? ''"
      :proxies-uid="option?.proxies ?? ''"
      :profile-uid="uid"
      :property="option?.groups ?? ''"
      :open="true"
      @save="onSave?.($event.prev, $event.curr)"
      @close="groupsOpen = false"
    />
    <EditorViewer
      v-if="mergeOpen"
      :open="true"
      :value="mergeDocument.value"
      language="yaml"
      :path="`merge:${option?.merge ?? ''}.yaml`"
      :loading="mergeDocument.loading"
      :dirty="mergeDocument.dirty"
      @change="mergeDocument.setValue"
      @save="handleSaveMergeDocument"
      @close="mergeOpen = false"
    />
    <EditorViewer
      v-if="scriptOpen"
      :open="true"
      :value="scriptDocument.value"
      language="javascript"
      :path="`script:${option?.script ?? ''}.js`"
      :loading="scriptDocument.loading"
      :dirty="scriptDocument.dirty"
      @change="scriptDocument.setValue"
      @save="handleSaveScriptDocument"
      @close="scriptOpen = false"
    />

    <BaseDialog
      :open="confirmOpen"
      :title="t('profiles.modals.confirmDelete.title')"
      :ok-btn="t('shared.actions.confirm')"
      :cancel-btn="t('shared.actions.cancel')"
      :content-sx="{ width: { xs: 320, sm: 420 }, userSelect: 'text' }"
      @cancel="confirmOpen = false"
      @close="confirmOpen = false"
      @ok="onDelete(); confirmOpen = false"
    >
      <p style="word-break: break-word; margin: 0;">{{ t('profiles.modals.confirmDelete.message') }}</p>
    </BaseDialog>

    <QrViewer
      v-if="qrOpen && itemData.url"
      :open="true"
      :value="`${itemData.url}${itemData.url.includes('?') ? '&' : '?'}name=${encodeURIComponent(name)}`"
      @close="qrOpen = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick, useTemplateRef } from 'vue'
import { useI18n } from 'vue-i18n'
import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import { open } from '@tauri-apps/plugin-shell'
import { useLockFn } from '@/hooks/use-lock-fn'

import BaseDialog from '@/components/base/base-dialog.vue'
import EditorViewer from '@/components/profile/editor-viewer.vue'
import GroupsEditorViewer from '@/components/profile/groups-editor-viewer.vue'
import RulesEditorViewer from '@/components/profile/rules-editor-viewer.vue'
import { useEditorDocument } from '@/hooks/use-editor-document'
import { getNextUpdateTime, readProfileFile, saveProfileFile, updateProfile, viewProfile } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { useLoadingCache, useSetLoadingCache } from '@/services/states'
import type { TranslationKey } from '@/types/generated/i18n-keys'
import { debugLog } from '@/utils/debug'
import parseTraffic from '@/utils/parse-traffic'

import ProfileBox from './profile-box.vue'
import ProxiesEditorViewer from './proxies-editor-viewer.vue'
import QrViewer from './qr-viewer.vue'

dayjs.extend(relativeTime)

const props = defineProps<{
  selected: boolean
  activating: boolean
  itemData: IProfileItem
  mutateProfiles: () => Promise<void>
  onSelect: (force: boolean) => void
  onEdit: () => void
  onSave?: (prev?: string, curr?: string) => void
  onDelete: () => void
  batchMode?: boolean
  isSelected?: boolean
  onSelectionChange?: () => void
  timerUpdateRevision: number
  completedUpdateRevision: number
  dragHandleRef?: (node: HTMLElement | null) => void
  dragHandleAttributes?: Record<string, any>
  dragHandleListeners?: Record<string, any>
}>()

const { t } = useI18n()
const anchorEl = ref<HTMLElement | null>(null)
const position = ref({ left: 0, top: 0 })
const loadingCache = useLoadingCache()
const setLoadingCache = useSetLoadingCache()

const showNextUpdate = ref(false)
const showNextUpdateRef = ref(false)
const nextUpdateTime = ref('')
const refreshTimeoutRef = ref<ReturnType<typeof setTimeout> | undefined>(undefined)

const setLoadingFn = (loading: boolean) => {
  setLoadingCache((cache: Set<string>) => {
    const next = new Set(cache)
    if (loading) next.add(props.itemData.uid)
    else next.delete(props.itemData.uid)
    return next
  })
}

const { uid, name = 'Profile', extra, updated = 0, option } = props.itemData

const hasUrl = computed(() => !!props.itemData.url)
const hasExtra = computed(() => !!extra)
const hasHome = computed(() => !!props.itemData.home)

const { upload = 0, download = 0, total = 0 } = extra ?? {}
const from = computed(() => parseUrl(props.itemData.url))
const description = computed(() => props.itemData.desc)
const expire = computed(() => parseExpire(extra?.expire))
const progress = computed(() => Math.min(Math.round(((download + upload) * 100) / (total + 0.01)) + 1, 100))

const loading = computed(() => loadingCache.value.has(props.itemData.uid))

const fileOpen = ref(false)
const rulesOpen = ref(false)
const proxiesOpen = ref(false)
const groupsOpen = ref(false)
const mergeOpen = ref(false)
const scriptOpen = ref(false)
const confirmOpen = ref(false)
const qrOpen = ref(false)

const profileDocument = useEditorDocument({ open: fileOpen, load: () => readProfileFile(uid) })
const mergeDocument = useEditorDocument({ open: mergeOpen, load: () => readProfileFile(option?.merge ?? '') })
const scriptDocument = useEditorDocument({ open: scriptOpen, load: () => readProfileFile(option?.script ?? '') })

const fetchNextUpdateTimeCallback = async (forceRefresh = false) => {
  if (props.itemData.option?.update_interval && props.itemData.option.update_interval > 0) {
    try {
      debugLog(`尝试获取配置 ${props.itemData.uid} 的下次更新时间`)
      const nextUpdate = await getNextUpdateTime(props.itemData.uid)
      if (nextUpdate) {
        const nextUpdateDate = dayjs(nextUpdate * 1000)
        const now = dayjs()
        if (nextUpdateDate.isBefore(now)) {
          nextUpdateTime.value = t('profiles.components.profileItem.status.lastUpdateFailed')
        } else {
          const diffMinutes = nextUpdateDate.diff(now, 'minute')
          if (diffMinutes < 60) {
            nextUpdateTime.value = diffMinutes <= 0
              ? `${t('profiles.components.profileItem.status.nextUp')} <1m`
              : `${t('profiles.components.profileItem.status.nextUp')} ${diffMinutes}m`
          } else {
            const hours = Math.floor(diffMinutes / 60)
            const mins = diffMinutes % 60
            nextUpdateTime.value = `${t('profiles.components.profileItem.status.nextUp')} ${hours}h ${mins}m`
          }
        }
      } else {
        nextUpdateTime.value = t('profiles.components.profileItem.status.noSchedule')
      }
    } catch {
      nextUpdateTime.value = t('profiles.components.profileItem.status.unknown')
    }
  } else {
    nextUpdateTime.value = t('profiles.components.profileItem.status.autoUpdateDisabled')
  }
}
const fetchNextUpdateTime = useLockFn(fetchNextUpdateTimeCallback)

const toggleUpdateTimeDisplay = (e: MouseEvent) => {
  e.stopPropagation()
  if (!showNextUpdate.value) fetchNextUpdateTime()
  showNextUpdate.value = !showNextUpdate.value
}

watch(showNextUpdate, (val) => { showNextUpdateRef.value = val })
watch([() => props.itemData.option?.update_interval, () => updated], () => {
  if (showNextUpdate.value) fetchNextUpdateTime()
})

watch(() => props.timerUpdateRevision, (val) => {
  if (val === 0 || !showNextUpdateRef.value) return
  if (refreshTimeoutRef.value !== undefined) clearTimeout(refreshTimeoutRef.value)
  refreshTimeoutRef.value = window.setTimeout(() => fetchNextUpdateTime(true), 1000)
})

watch(() => props.completedUpdateRevision, (val) => {
  if (val === 0 || !showNextUpdateRef.value) return
  fetchNextUpdateTime()
})

const forceRefresh = ref(0)
const incrementRefresh = () => { forceRefresh.value++ }

watch([() => props.itemData.url, () => props.updated], () => {
  if (!hasUrl.value) return
  let timer: ReturnType<typeof setTimeout> | undefined
  const handler = () => {
    const now = Date.now()
    const lastUpdate = updated * 1000
    if (now - lastUpdate >= 24 * 36e5) return
    const wait = now - lastUpdate >= 36e5 ? 30e5 : 5e4
    timer = setTimeout(() => { incrementRefresh(); handler() }, wait)
  }
  handler()
  onUnmounted(() => { if (timer) clearTimeout(timer) })
})

const onBoxClick = (e: MouseEvent) => {
  if (props.activating) { e.preventDefault(); e.stopPropagation(); return }
  props.onSelect(false)
}

const onContextMenu = (event: MouseEvent) => {
  position.value = { top: event.clientY, left: event.clientX }
  anchorEl.value = event.currentTarget as HTMLElement
  event.preventDefault()
}

const onUpdateClick = (e: MouseEvent) => {
  e.stopPropagation()
  if (props.activating || loading.value) return
  onUpdate(1)
}

const menuLabels: Record<string, TranslationKey> = {
  home: 'profiles.components.menu.home',
  select: 'profiles.components.menu.select',
  shareQrCode: 'profiles.components.menu.shareQrCode',
  editInfo: 'profiles.components.menu.editInfo',
  editFile: 'profiles.components.menu.editFile',
  editRules: 'profiles.components.menu.editRules',
  editProxies: 'profiles.components.menu.editProxies',
  editGroups: 'profiles.components.menu.editGroups',
  extendConfig: 'profiles.components.menu.extendConfig',
  extendScript: 'profiles.components.menu.extendScript',
  openFile: 'profiles.components.menu.openFile',
  update: 'profiles.components.menu.update',
  updateViaProxy: 'profiles.components.menu.updateViaProxy',
  delete: 'shared.actions.delete',
}

const onOpenHome = () => { anchorEl.value = null; open(props.itemData.home ?? '') }
const onEditInfo = () => { anchorEl.value = null; props.onEdit() }
const onShareQrCode = () => { anchorEl.value = null; qrOpen.value = true }
const onEditFileFn = () => { anchorEl.value = null; fileOpen.value = true }
const onEditRulesFn = () => { anchorEl.value = null; rulesOpen.value = true }
const onEditProxiesFn = () => { anchorEl.value = null; proxiesOpen.value = true }
const onEditGroupsFn = () => { anchorEl.value = null; groupsOpen.value = true }
const onEditMergeFn = () => { anchorEl.value = null; mergeOpen.value = true }
const onEditScriptFn = () => { anchorEl.value = null; scriptOpen.value = true }
const onForceSelect = () => { anchorEl.value = null; props.onSelect(true) }
const onOpenFile = useLockFn(async () => { anchorEl.value = null; try { await viewProfile(props.itemData.uid) } catch (err) { showNotice.error(err) } })
const onDeleteClick = () => {
  anchorEl.value = null
  if (props.batchMode) { props.onSelectionChange?.() } else { confirmOpen.value = true }
}

let updateLock = false
const onUpdate = async (type: 0 | 1 | 2) => {
  if (updateLock) return
  updateLock = true
  anchorEl.value = null
  setLoadingFn(true)
  const optionData: Partial<IProfileOption> = {}
  if (type === 0) { optionData.with_proxy = false; optionData.self_proxy = false }
  else if (type === 2) {
    if (props.itemData.option?.self_proxy) { optionData.with_proxy = false; optionData.self_proxy = true }
    else { optionData.with_proxy = true; optionData.self_proxy = false }
  }
  try {
    const payload = Object.keys(optionData).length > 0 ? optionData : undefined
    await updateProfile(props.itemData.uid, payload)
    void props.mutateProfiles()
  } finally { setLoadingFn(false); updateLock = false }
}

const urlModeMenu = computed(() => [
  ...(hasHome.value ? [{ label: menuLabels.home, handler: onOpenHome, disabled: false }] : []),
  { label: menuLabels.select, handler: onForceSelect, disabled: false },
  { label: menuLabels.shareQrCode, handler: onShareQrCode, disabled: false },
  { label: menuLabels.editInfo, handler: onEditInfo, disabled: false },
  { label: menuLabels.editFile, handler: onEditFileFn, disabled: false },
  { label: menuLabels.editRules, handler: onEditRulesFn, disabled: !option?.rules },
  { label: menuLabels.editProxies, handler: onEditProxiesFn, disabled: !option?.proxies },
  { label: menuLabels.editGroups, handler: onEditGroupsFn, disabled: !option?.groups },
  { label: menuLabels.extendConfig, handler: onEditMergeFn, disabled: !option?.merge },
  { label: menuLabels.extendScript, handler: onEditScriptFn, disabled: !option?.script },
  { label: menuLabels.openFile, handler: onOpenFile, disabled: false },
  { label: menuLabels.update, handler: () => onUpdate(0), disabled: false },
  { label: menuLabels.updateViaProxy, handler: () => onUpdate(2), disabled: false },
  { label: menuLabels.delete, handler: onDeleteClick, disabled: false },
])

const fileModeMenu = computed(() => [
  { label: menuLabels.select, handler: onForceSelect, disabled: false },
  { label: menuLabels.editInfo, handler: onEditInfo, disabled: false },
  { label: menuLabels.editFile, handler: onEditFileFn, disabled: false },
  { label: menuLabels.editRules, handler: onEditRulesFn, disabled: !option?.rules },
  { label: menuLabels.editProxies, handler: onEditProxiesFn, disabled: !option?.proxies },
  { label: menuLabels.editGroups, handler: onEditGroupsFn, disabled: !option?.groups },
  { label: menuLabels.extendConfig, handler: onEditMergeFn, disabled: !option?.merge },
  { label: menuLabels.extendScript, handler: onEditScriptFn, disabled: !option?.script },
  { label: menuLabels.openFile, handler: onOpenFile, disabled: false },
  { label: menuLabels.delete, handler: onDeleteClick, disabled: false },
])

const currentMenu = computed(() => (hasUrl.value ? urlModeMenu.value : fileModeMenu.value))

let saveLock = false
const handleSaveProfileDocument = async () => {
  if (saveLock) return
  saveLock = true
  try {
    const currentValue = profileDocument.value.value
    if (!(await saveProfileFile(uid, currentValue))) { await profileDocument.reload(); return }
    props.onSave?.(profileDocument.value.savedValue, currentValue)
    profileDocument.markSaved(currentValue)
  } finally { saveLock = false }
}

const handleSaveMergeDocument = async () => {
  if (saveLock) return
  saveLock = true
  try {
    const mergeUid = option?.merge ?? ''
    const currentValue = mergeDocument.value.value
    if (!(await saveProfileFile(mergeUid, currentValue))) { await mergeDocument.reload(); return }
    props.onSave?.(mergeDocument.value.savedValue, currentValue)
    mergeDocument.markSaved(currentValue)
  } finally { saveLock = false }
}

const handleSaveScriptDocument = async () => {
  if (saveLock) return
  saveLock = true
  try {
    const scriptUid = option?.script ?? ''
    const currentValue = scriptDocument.value.value
    if (!(await saveProfileFile(scriptUid, currentValue))) { await scriptDocument.reload(); return }
    props.onSave?.(scriptDocument.value.savedValue, currentValue)
    scriptDocument.markSaved(currentValue)
  } finally { saveLock = false }
}

function parseUrl(url?: string) {
  if (!url) return ''
  const regex = /https?:\/\/(.+?)\//
  const result = url.match(regex)
  return result ? result[1] : 'local file'
}

function parseExpire(expire?: number) {
  if (!expire) return '-'
  return dayjs(expire * 1000).format('YYYY-MM-DD')
}
</script>
