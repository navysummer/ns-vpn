<template>
  <BasePage
    full
    :title="t('profiles.page.title')"
    :content-style="{ height: '100%' }"
  >
    <template #header>
      <div style="display: flex; align-items: center; gap: 8px;">
        <template v-if="!batchMode">
          <button
            class="MuiIconButton-root MuiIconButton-sizeSmall"
            :title="t('profiles.page.batch.title')"
            @click="toggleBatchMode"
          >
            <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M19 5v14H5V5h14m0-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z"/></svg>
          </button>
          <button
            class="MuiIconButton-root MuiIconButton-sizeSmall"
            :title="t('profiles.page.actions.updateAll')"
            @click="onUpdateAll"
          >
            <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
          </button>
          <button
            class="MuiIconButton-root MuiIconButton-sizeSmall"
            :title="t('profiles.page.actions.viewRuntimeConfig')"
            @click="configRef?.open()"
          >
            <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H5.17L4 17.17V4h16v12z"/></svg>
          </button>
          <button
            class="MuiIconButton-root MuiIconButton-sizeSmall MuiIconButton-colorPrimary"
            :title="t('profiles.page.actions.reactivate')"
            @click="onEnhance(true)"
          >
            <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M19.48 12.35c-1.57 4.08-7.16 4.3-5.81 10.65h-1.56c-.2-3.92-4.49-6.03-4.49-10.65 0-3.09 2.01-5.74 4.75-6.79.45 1.95 1.95 3.52 3.88 4.04 2.04.55 4.01.89 4.01 2.91 0 .86-.27 1.68-.78 1.84z"/></svg>
          </button>
          <button
            v-if="error || isStale"
            class="MuiIconButton-root MuiIconButton-sizeSmall MuiIconButton-colorWarning"
            :title="t('profiles.page.feedback.tooltips.forceRefreshStaleData')"
            @click="onEmergencyRefresh"
            style="animation: pulse 2s infinite;"
          >
            <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
          </button>
        </template>
        <template v-else>
          <div style="display: flex; align-items: center; gap: 8px;">
            <button
              class="MuiIconButton-root MuiIconButton-sizeSmall"
              :title="isAllSelected ? t('profiles.page.batch.actions.deselectAll') : t('profiles.page.batch.actions.selectAll')"
              @click="isAllSelected ? clearAllSelections() : selectAllProfiles()"
            >
              <svg v-if="selectionState === 'all'" viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM17.99 9l-1.41-1.42-6.59 6.59-2.58-2.57-1.42 1.41 4 3.99z"/></svg>
              <svg v-else-if="selectionState === 'partial'" viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM7 11h10v2H7z"/></svg>
              <svg v-else viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M19 5v14H5V5h14m0-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z"/></svg>
            </button>
            <button
              class="MuiIconButton-root MuiIconButton-sizeSmall MuiIconButton-colorError"
              :title="t('profiles.page.batch.actions.delete')"
              :disabled="selectedProfiles.size === 0"
              @click="deleteSelectedProfiles"
            >
              <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
            </button>
            <button class="MuiButton-root MuiButton-outlined MuiButton-sizeSmall" @click="toggleBatchMode">
              {{ t('profiles.page.batch.actions.done') }}
            </button>
            <span style="flex: 1; text-align: right; color: var(--text-secondary);">
              {{ t('profiles.page.batch.summary.selected') }} {{ selectedProfiles.size }} {{ t('profiles.page.batch.summary.items') }}
            </span>
          </div>
        </template>
      </div>
    </template>

    <div style="display: flex; gap: 8px; padding-top: 8px; margin-bottom: 4px; margin-left: 10px; margin-right: 10px; height: 36px; align-items: center;">
      <input
        v-model="url"
        type="text"
        :placeholder="t('profiles.page.importForm.placeholder')"
        style="flex: 1; height: 100%; padding: 0 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color); box-sizing: border-box; font-size: 14px;"
        @keydown="onImportKeydown"
      />
      <button
        class="MuiIconButton-root MuiIconButton-sizeSmall"
        style="margin-left: -32px; z-index: 1;"
        v-if="!url"
        :title="t('profiles.page.importForm.actions.paste')"
        @click="onCopyLink"
      >
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M19 2h-4.18C14.4.84 13.3 0 12 0S9.6.84 9.18 2H5c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm7 18H5V4h2v3h10V4h2v16z"/></svg>
      </button>
      <button
        class="MuiIconButton-root MuiIconButton-sizeSmall"
        style="margin-left: -32px; z-index: 1;"
        v-else
        :title="t('shared.actions.clear')"
        @click="url = ''"
      >
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
      </button>
      <button
        class="MuiButton-root MuiButton-contained MuiButton-sizeSmall"
        :disabled="!url || disabled"
        style="border-radius: 6px; min-width: 80px;"
        @click="onImport"
      >
        {{ loading ? t('shared.statuses.loading') : t('profiles.page.actions.import') }}
      </button>
      <button
        class="MuiButton-root MuiButton-contained MuiButton-sizeSmall"
        style="border-radius: 6px;"
        @click="viewerRef?.create()"
      >
        {{ t('shared.actions.new') }}
      </button>
    </div>

    <div style="padding-left: 10px; padding-right: 10px; height: calc(100% - 48px); overflow-y: auto;">
      <div style="margin-bottom: 12px;">
        <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 8px;">
          <div v-for="item in profileItems" :key="item.file">
            <SortableProfileItem
              :id="item.uid"
              :selected="(switchTarget ?? profiles.current) === item.uid"
              :activating="activatings.includes(item.uid) || visibleSwitchingProfile === item.uid"
              :item-data="item"
              :timer-update-revision="timerUpdateRevisions.get(item.uid) ?? 0"
              :completed-update-revision="completedUpdateRevisions.get(item.uid) ?? 0"
              :mutate-profiles="mutateProfiles"
              :on-select="(f: boolean) => onSelect(item.uid, f)"
              :on-edit="() => viewerRef?.edit(item)"
              :on-save="async (prev: string | undefined, curr: string | undefined) => {
                if (prev !== curr && profiles.current === item.uid) { await onEnhance(false) }
              }"
              :on-delete="() => batchMode ? toggleProfileSelection(item.uid) : onDelete(item.uid)"
              :batch-mode="batchMode"
              :is-selected="selectedProfiles.has(item.uid)"
              :on-selection-change="() => toggleProfileSelection(item.uid)"
            />
          </div>
        </div>
      </div>
      <hr style="margin: 8px 32px; border: none; border-bottom: 1px solid; border-color: dividercolor;" />
      <div style="margin-top: 12px; margin-bottom: 10px;">
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px;">
          <ProfileMore
            id="Merge"
            @save="async (prev: string | undefined, curr: string | undefined) => {
              if (prev !== curr) { await onEnhance(false) }
            }"
          />
          <ProfileMore
            id="Script"
            :log-info="chainLogs['Script']"
            @save="async (prev: string | undefined, curr: string | undefined) => {
              if (prev !== curr) { await onEnhance(false) }
            }"
          />
        </div>
      </div>
    </div>

    <ProfileViewer
      ref="viewerRef"
      @change="async (isActivating: boolean) => {
        mutateProfiles()
        if (isActivating) { await onEnhance(false) }
      }"
    />
    <ConfigViewer ref="configRef" />
  </BasePage>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { TauriEvent } from '@tauri-apps/api/event'
import { readText } from '@tauri-apps/plugin-clipboard-manager'
import { readTextFile } from '@tauri-apps/plugin-fs'
import { throttle } from 'lodash-es'
import { useRoute } from 'vue-router'
import { closeAllConnections } from 'tauri-plugin-mihomo-api'

import { BasePage, BaseStyledTextField, type DialogRef } from '@/components/base'
import ProfileMore from '@/components/profile/profile-more.vue'
import ProfileViewer from '@/components/profile/profile-viewer.vue'
import type { ProfileViewerRef } from '@/components/profile/profile-viewer.vue'
import SortableProfileItem from '@/components/profile/sortable-profile-item.vue'
import ConfigViewer from '@/components/setting/mods/config-viewer.vue'
import { useListen } from '@/hooks/use-listen'
import { useProfiles } from '@/hooks/use-profiles'
import {
  createProfile, deleteProfile, enhanceProfiles, getProfiles,
  getRuntimeLogs, importProfile, reorderProfile, updateProfile,
} from '@/services/cmds'
import { subscribeVergeEvents } from '@/services/events'
import { showNotice } from '@/services/notice-service'
import { fetchCacheData, revalidateQueries, useQuery } from '@/services/query-client'
import { useLoadingCache, useSetLoadingCache, useThemeMode } from '@/services/states'
import { debugLog } from '@/utils/debug'

const PROFILE_UPDATE_WORKER_LIMIT = 8
const PROFILE_SWITCH_LOADING_DELAY = 400

const { t } = useI18n()
const route = useRoute()
const { addListener } = useListen()
const url = ref('')
const disabled = ref(false)
const activatings = ref<string[]>([])
const switchTarget = ref<string | null>(null)
const visibleSwitchingProfile = ref<string | null>(null)
const loading = ref(false)
const timerUpdateRevisions = ref<Map<string, number>>(new Map())
const completedUpdateRevisions = ref<Map<string, number>>(new Map())

const batchMode = ref(false)
const selectedProfiles = ref<Set<string>>(new Set())

const currentProfileRef = ref<string | undefined>(undefined)
const profilePageMountedRef = ref(true)
const latestSwitchTargetRef = ref<string | null>(null)
const queuedSwitchRef = ref<ProfileSwitchRequest | null>(null)
const switchRunnerRef = ref<Promise<void> | null>(null)
const switchLoadingTimerRef = ref<ReturnType<typeof setTimeout> | null>(null)

const {
  profiles = {},
  patchProfiles,
  mutateProfiles,
  error,
  isStale,
} = useProfiles()

interface ProfileSwitchRequest {
  profile: string
  notifySuccess: boolean
  force: boolean
}

const debugProfileSwitch = (action: string, profile: string, extra?: any) => {
  const timestamp = new Date().toISOString().substring(11, 23)
  debugLog(`[Profile-Debug][${timestamp}] ${action}: ${profile}`, extra || '')
}

watch(profiles, (val) => { currentProfileRef.value = val.current }, { deep: true })

const { data: chainLogs = {}, refetch: refetchLogs } = useQuery({
  queryKey: ['getRuntimeLogs'],
  queryFn: getRuntimeLogs,
})

const mutateLogs = () => refetchLogs()

const viewerRef = ref<InstanceType<typeof ProfileViewer> | null>(null)
const configRef = ref<DialogRef | null>(null)

const profileItems = computed(() => {
  const items = profiles.items || []
  return items.filter((i: any) => i && ['local', 'remote'].includes(i.type!))
})

onMounted(async () => {
  const unlisten = await addListener(TauriEvent.DRAG_DROP, async (event: any) => {
    const paths = event.payload.paths
    for (const file of paths) {
      if (!file.endsWith('.yaml') && !file.endsWith('.yml')) {
        showNotice.error('profiles.page.feedback.errors.onlyYaml')
        continue
      }
      const item: IProfileItem = {
        type: 'local',
        name: file.split(/\/|\\/).pop() ?? 'New Profile',
        desc: '',
        url: '',
        option: { with_proxy: false, self_proxy: false },
      }
      const data = await readTextFile(file)
      await createProfile(item, data)
      await mutateProfiles()
    }
    await enhanceProfiles()
  })
  onUnmounted(() => unlisten())
})

const onEmergencyRefresh = async () => {
  debugLog('[紧急刷新] 开始强制刷新所有数据')
  try {
    await revalidateQueries([['getProfiles'], ['getRuntimeLogs']])
    await mutateProfiles()
    await new Promise(resolve => setTimeout(resolve, 500))
    await onEnhance(false)
    showNotice.success('profiles.page.feedback.notices.forceRefreshCompleted', 2000)
  } catch (err) {
    console.error('[紧急刷新] 失败:', err)
    showNotice.error('profiles.page.feedback.notices.emergencyRefreshFailed', { message: String(err) }, 4000)
  }
}

const currentActivatings = () => [...new Set([profiles.current ?? ''])].filter(Boolean)

let importLock = false
const onImport = async () => {
  if (importLock || !url.value) return
  if (!/^https?:\/\//i.test(url.value)) {
    showNotice.error('profiles.page.feedback.errors.invalidUrl')
    return
  }
  importLock = true
  loading.value = true
  try {
    await importProfile(url.value)
    showNotice.success('shared.feedback.notifications.importSuccess')
    url.value = ''
    await performRobustRefresh()
  } catch (initialErr) {
    if (String(initialErr).toLowerCase().includes('legacy tls')) {
      showNotice.error(String(initialErr))
      return
    }
    showNotice.info('profiles.page.feedback.notifications.importRetry')
    try {
      await importProfile(url.value, { with_proxy: false, self_proxy: true })
      showNotice.success('shared.feedback.notifications.importWithClashProxy')
      url.value = ''
      await performRobustRefresh()
    } catch (retryErr) {
      showNotice.error('profiles.page.feedback.notifications.importFail', String(retryErr))
    }
  } finally {
    disabled.value = false
    loading.value = false
    importLock = false
  }
}

const onImportKeydown = (event: KeyboardEvent) => {
  if (event.key !== 'Enter' || event.isComposing) return
  if (!url.value || disabled.value || loading.value) return
  event.preventDefault()
  onImport()
}

const performRobustRefresh = async () => {
  let retryCount = 0
  const maxRetries = 1
  const baseDelay = 200
  while (retryCount < maxRetries) {
    try {
      await mutateProfiles()
      await new Promise(resolve => setTimeout(resolve, baseDelay * (retryCount + 1)))
      await onEnhance(false)
      return
    } catch (err) {
      console.error(`[导入刷新] 第${retryCount + 1}次刷新失败:`, err)
      retryCount++
      await new Promise(resolve => setTimeout(resolve, baseDelay * retryCount))
    }
  }
  try {
    await fetchCacheData(['getProfiles'], getProfiles)
    await onEnhance(false)
    showNotice.error('profiles.page.feedback.notifications.importNeedsRefresh', 3000)
  } catch (finalError) {
    console.error('[导入刷新] 最终刷新尝试失败:', finalError)
    showNotice.error('profiles.page.feedback.notifications.importSuccess', 5000)
  }
}

let enhanceLock = false
const onEnhance = async (notifySuccess: boolean) => {
  if (enhanceLock || switchRunnerRef.value) return
  enhanceLock = true
  const current = currentActivatings()
  activatings.value = [...new Set([...activatings.value, ...current])]
  try {
    if (!(await enhanceProfiles())) return
    mutateLogs()
    if (notifySuccess) showNotice.success('profiles.page.feedback.notifications.profileReactivated', 1000)
  } catch (err) {
    showNotice.error(err, 3000)
  } finally {
    activatings.value = []
    enhanceLock = false
  }
}

let deleteLock = false
const onDelete = async (uid: string) => {
  if (deleteLock) return
  deleteLock = true
  const isCurrent = profiles.current === uid
  try {
    activatings.value = [...(isCurrent ? currentActivatings() : []), uid]
    await deleteProfile(uid)
    await mutateProfiles()
    mutateLogs()
    if (isCurrent) await onEnhance(false)
  } catch (err) {
    showNotice.error(err)
  } finally {
    activatings.value = []
    deleteLock = false
  }
}

const loadingCache = useLoadingCache()
const setLoadingCache = useSetLoadingCache()
const setLoadingProfiles = (uids: string[], loadingFlag: boolean) => {
  setLoadingCache((cache: Set<string>) => {
    const next = new Set(cache)
    for (const uid of uids) {
      if (loadingFlag) next.add(uid); else next.delete(uid)
    }
    return next
  })
}

onMounted(() => subscribeVergeEvents({
  'profile-update-started': ({ uid }: any) => { if (uid) setLoadingProfiles([uid], true) },
  'profile-update-completed': ({ uid }: any) => {
    if (!uid) return
    setLoadingProfiles([uid], false)
    completedUpdateRevisions.value = new Map(completedUpdateRevisions.value).set(uid, (completedUpdateRevisions.value.get(uid) ?? 0) + 1)
    void mutateProfiles()
  },
  'verge://timer-updated': (uid: string) => {
    timerUpdateRevisions.value = new Map(timerUpdateRevisions.value).set(uid, (timerUpdateRevisions.value.get(uid) ?? 0) + 1)
  },
}))

const runProfileUpdates = async (uids: string[]) => {
  if (uids.length === 0) return
  const throttleMutate = throttle(mutateProfiles, 2000, { trailing: true })
  let cursor = 0
  const updateOne = async (uid: string) => {
    try { await updateProfile(uid); throttleMutate() }
    catch (err) { console.error(`更新订阅 ${uid} 失败:`, err) }
  }
  const worker = async () => {
    while (cursor < uids.length) { const uid = uids[cursor++]; await updateOne(uid) }
  }
  try {
    const active = Math.min(PROFILE_UPDATE_WORKER_LIMIT, uids.length)
    await Promise.allSettled(Array.from({ length: active }, worker))
  } finally {
    setLoadingProfiles(uids, false)
    void mutateProfiles()
  }
}

let updateAllLock = false
const onUpdateAll = async () => {
  if (updateAllLock) return
  updateAllLock = true
  try {
    const items = profileItems.value.filter((e: any) => e.type === 'remote')
    const target = items.map((item: any) => item.uid).filter((uid: string) => !loadingCache.value.has(uid))
    setLoadingProfiles(target, true)
    await runProfileUpdates(target)
  } finally { updateAllLock = false }
}

const onCopyLink = async () => {
  const text = await readText()
  if (text) url.value = text
}

const toggleBatchMode = () => {
  batchMode.value = !batchMode.value
  if (!batchMode.value) selectedProfiles.value = new Set()
}

const toggleProfileSelection = (uid: string) => {
  const newSet = new Set(selectedProfiles.value)
  if (newSet.has(uid)) newSet.delete(uid)
  else newSet.add(uid)
  selectedProfiles.value = newSet
}

const selectAllProfiles = () => {
  selectedProfiles.value = new Set(profileItems.value.map((item: any) => item.uid))
}

const clearAllSelections = () => { selectedProfiles.value = new Set() }

const isAllSelected = computed(() => profileItems.value.length > 0 && profileItems.value.length === selectedProfiles.value.size)

const selectionState = computed(() => {
  if (selectedProfiles.value.size === 0) return 'none'
  if (selectedProfiles.value.size === profileItems.value.length) return 'all'
  return 'partial'
})

let batchDeleteLock = false
const deleteSelectedProfiles = async () => {
  if (batchDeleteLock || selectedProfiles.value.size === 0) return
  batchDeleteLock = true
  try {
    const currentActivating = profiles.current && selectedProfiles.value.has(profiles.current) ? [profiles.current] : []
    activatings.value = [...new Set([...activatings.value, ...currentActivating])]
    for (const uid of selectedProfiles.value) { await deleteProfile(uid) }
    await mutateProfiles()
    await mutateLogs()
    if (currentActivating.length > 0) await onEnhance(false)
    selectedProfiles.value = new Set()
    batchMode.value = false
    showNotice.success('profiles.page.feedback.notifications.batchDeleted')
  } catch (err) { showNotice.error(err) }
  finally { activatings.value = []; batchDeleteLock = false }
}

const mode = useThemeMode()
const isLight = computed(() => mode.value === 'light')
const dividercolor = computed(() => isLight.value ? 'rgba(0, 0, 0, 0.06)' : 'rgba(255, 255, 255, 0.06)')

const runProfileSwitchQueue = async () => {
  while (profilePageMountedRef.value && queuedSwitchRef.value) {
    const request = queuedSwitchRef.value
    queuedSwitchRef.value = null
    await executeProfileSwitch(request)
  }
}

const executeProfileSwitch = async ({ profile, notifySuccess, force }: ProfileSwitchRequest) => {
  if (!force && currentProfileRef.value === profile) { debugProfileSwitch('ALREADY_CURRENT_IGNORED', profile); return }
  debugProfileSwitch('SWITCH_START', profile)
  try {
    const outcome = await patchProfiles({ current: profile })
    if (outcome.status === 'busy') {
      debugProfileSwitch('SWITCH_BUSY', profile)
      showNotice.info('profiles.page.feedback.notifications.switchBusy', 2000)
      return
    }
    if (outcome.status === 'valid') {
      currentProfileRef.value = profile
      void mutateLogs().catch(() => {})
      void closeAllConnections().catch(() => {})
      if (notifySuccess && latestSwitchTargetRef.value === profile && queuedSwitchRef.value === null) {
        showNotice.success('profiles.page.feedback.notifications.profileSwitched', 1000)
      }
      debugProfileSwitch('SWITCH_SUCCESS', profile)
    } else { debugProfileSwitch('SWITCH_REJECTED', profile, outcome) }
  } catch (err) {
    console.error('[Profile] 切换失败:', err)
    showNotice.error(err, 4000)
  } finally { debugProfileSwitch('SWITCH_END', profile) }
}

const activateProfile = (profile: string, notifySuccess: boolean, force = false) => {
  if (!profilePageMountedRef.value) return Promise.resolve()
  if (!force && currentProfileRef.value === profile && switchRunnerRef.value === null) {
    debugProfileSwitch('ALREADY_CURRENT_IGNORED', profile)
    return Promise.resolve()
  }
  if (latestSwitchTargetRef.value === profile && switchRunnerRef.value) {
    debugProfileSwitch('DUPLICATE_SWITCH_IGNORED', profile)
    return switchRunnerRef.value
  }
  latestSwitchTargetRef.value = profile
  queuedSwitchRef.value = { profile, notifySuccess, force }
  switchTarget.value = profile
  visibleSwitchingProfile.value = null
  if (switchLoadingTimerRef.value) { clearTimeout(switchLoadingTimerRef.value) }
  switchLoadingTimerRef.value = setTimeout(() => {
    if (profilePageMountedRef.value && latestSwitchTargetRef.value === profile) {
      visibleSwitchingProfile.value = profile
    }
  }, PROFILE_SWITCH_LOADING_DELAY)
  if (switchRunnerRef.value) { debugProfileSwitch('SWITCH_QUEUED', profile); return switchRunnerRef.value }
  const runner = runProfileSwitchQueue().finally(() => {
    if (switchRunnerRef.value === runner) {
      switchRunnerRef.value = null
      latestSwitchTargetRef.value = null
      if (switchLoadingTimerRef.value) { clearTimeout(switchLoadingTimerRef.value); switchLoadingTimerRef.value = null }
      if (profilePageMountedRef.value) { switchTarget.value = null; visibleSwitchingProfile.value = null }
    }
  })
  switchRunnerRef.value = runner
  return runner
}

const onSelect = async (profile: string, force: boolean) => { await activateProfile(profile, true, force) }

onMounted(() => {
  const current = (route.state as any)?.current
  if (current) {
    mutateProfiles().then(() => activateProfile(current, false))
  }
  profilePageMountedRef.value = true
})

onUnmounted(() => {
  profilePageMountedRef.value = false
  queuedSwitchRef.value = null
  latestSwitchTargetRef.value = null
  if (switchLoadingTimerRef.value) { clearTimeout(switchLoadingTimerRef.value); switchLoadingTimerRef.value = null }
})
</script>
