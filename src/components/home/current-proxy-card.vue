<template>
  <EnhancedCard
    :title="t('home.components.currentProxy.title')"
    iconColor="primary"
  >
    <template #icon>
      <n-tooltip :trigger="'hover'" :style="{}">
        <template #trigger>
          <span :style="{ color: signalInfo.color }">
            <span v-html="signalInfo.icon" />
          </span>
        </template>
        <span>{{ currentProxy ? `${signalInfo.text}: ${delayManager.formatDelay(currentDelay)}` : t('home.components.currentProxy.status.noProxyNode') }}</span>
      </n-tooltip>
    </template>
    <template #action>
      <div :style="{ display: 'flex', alignItems: 'center', gap: '8px' }">
        <n-tooltip :title="t('home.components.currentProxy.actions.refreshDelay')" :trigger="'hover'">
          <template #trigger>
            <n-button quaternary circle size="small" :disabled="isDirectMode || unsortedProxyOptions.length === 0" @click="handleCheckDelay">
              <template #icon>
                <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M15 9H9v2h6V9zm-2 4H9v2h4v-2zm5-10H6c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H6V5h12v14z"/></svg>
              </template>
            </n-button>
          </template>
        </n-tooltip>
        <n-tooltip :title="getSortTooltip()" :trigger="'hover'">
          <template #trigger>
            <n-button quaternary circle size="small" @click="handleSortTypeChange">
              <template #icon>
                <span v-html="getSortIcon()" />
              </template>
            </n-button>
          </template>
        </n-tooltip>
        <n-button size="small" @click="goToProxies" :style="{ borderRadius: '12px' }">
          {{ t('layout.components.navigation.tabs.proxies') }}
          <template #suffix>
            <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/></svg>
          </template>
        </n-button>
      </div>
    </template>
    <template #default>
      <div v-if="isCoreDataPending" :style="{ padding: '32px 0', height: '24px' }" />
      <div v-else-if="currentProxy || (!isDirectMode && selectedGroup)">
        <div
          :style="{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            padding: '8px',
            marginBottom: '16px',
            borderRadius: '8px',
            backgroundColor: 'var(--primary-color-alpha)',
            border: '1px solid var(--primary-color-border)',
          }"
        >
          <div>
            <div :style="{ fontSize: '16px', fontWeight: 500 }">{{ currentProxy?.name ?? t('home.components.currentProxy.labels.noActiveNode') }}</div>
            <div :style="{ display: 'flex', alignItems: 'center', flexWrap: 'wrap' }">
              <span :style="{ fontSize: '12px', color: 'var(--text-secondary-color)', marginRight: '8px' }">{{ currentProxy?.type }}</span>
              <n-tag v-if="isGlobalMode" size="small" color="primary" :style="{ marginRight: '4px' }">{{ t('home.components.currentProxy.labels.globalMode') }}</n-tag>
              <n-tag v-if="isDirectMode" size="small" color="success" :style="{ marginRight: '4px' }">{{ t('home.components.currentProxy.labels.directMode') }}</n-tag>
              <n-tag v-if="currentProxy?.udp" size="small" :style="{ marginRight: '4px' }">UDP</n-tag>
              <n-tag v-if="currentProxy?.tfo" size="small" :style="{ marginRight: '4px' }">TFO</n-tag>
              <n-tag v-if="currentProxy?.xudp" size="small" :style="{ marginRight: '4px' }">XUDP</n-tag>
              <n-tag v-if="currentProxy?.mptcp" size="small" :style="{ marginRight: '4px' }">MPTCP</n-tag>
              <n-tag v-if="currentProxy?.smux" size="small" :style="{ marginRight: '4px' }">SMUX</n-tag>
            </div>
          </div>
          <n-tag v-if="currentProxy && !isDirectMode" size="small" :color="convertDelayColor(currentDelay)">{{ delayManager.formatDelay(currentDelay) }}</n-tag>
        </div>
        <div :style="{ marginBottom: '12px' }">
          <n-select
            v-model:value="selectedGroupName"
            :placeholder="t('home.components.currentProxy.labels.group')"
            :options="selectableGroupOptions"
            :disabled="isGlobalMode || isDirectMode"
            size="small"
            @update:value="handleGroupChange"
          />
        </div>
        <div>
          <n-select
            v-model:value="currentOptionValue"
            :placeholder="t('home.components.currentProxy.labels.proxy')"
            :options="proxySelectOptions"
            :disabled="isDirectMode"
            size="small"
            @update:value="handleProxyChange"
          />
        </div>
      </div>
      <div v-else :style="{ textAlign: 'center', padding: '32px 0' }">
        <span :style="{ fontSize: '16px', color: 'var(--text-secondary-color)' }">{{ t('home.components.currentProxy.labels.noActiveNode') }}</span>
      </div>
    </template>
  </EnhancedCard>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { NButton, NSelect, NTag, NTooltip } from 'naive-ui'
import { closeAllConnections } from 'tauri-plugin-mihomo-api'

import EnhancedCard from './enhanced-card.vue'
import { useGroupDelays } from '@/hooks/use-group-delays'
import { useProfiles } from '@/hooks/use-profiles'
import { useProxySelection } from '@/hooks/use-proxy-selection'
import { useVerge } from '@/hooks/use-verge'
import {
  useAppRefreshers,
  useClashConfigData,
  useCoreDataStatus,
  useProxiesData,
} from '@/providers/app-data-context'
import delayManager from '@/services/delay'
import { findCurrentGroupMember, getRecord, isInteractableMember, memberDetails, resolveMember } from '@/types/proxy-view'
import { debugLog } from '@/utils/debug'
import { compareByDelay, DEFAULT_DELAY_TIMEOUT } from '@/utils/delay'

const { t } = useI18n()
const router = useRouter()
const { proxyView } = useProxiesData()
const { clashConfig } = useClashConfigData()
const { refreshProxy } = useAppRefreshers()
const { isCoreDataPending } = useCoreDataStatus()
const { verge } = useVerge()
const { current: currentProfile } = useProfiles()

const STORAGE_KEY_GROUP = 'clash-verge-selected-proxy-group'
const STORAGE_KEY_SORT_TYPE = 'clash-verge-proxy-sort-type'
const AUTO_CHECK_DEFAULT_INTERVAL_MINUTES = 5
const AUTO_CHECK_INITIAL_DELAY_MS = 100

const autoDelayEnabled = computed(() => verge.value?.enable_auto_delay_detection ?? false)
const defaultLatencyTimeout = computed(() => verge.value?.default_latency_timeout)

const autoDelayIntervalMs = computed(() => {
  const rawInterval = verge.value?.auto_delay_detection_interval_minutes
  const intervalMinutes = typeof rawInterval === 'number' && rawInterval > 0 ? rawInterval : AUTO_CHECK_DEFAULT_INTERVAL_MINUTES
  return Math.max(1, Math.round(intervalMinutes)) * 60 * 1000
})

const currentProfileId = computed(() => currentProfile.value?.uid || null)

const getProfileStorageKey = (baseKey: string) => currentProfileId.value ? `${baseKey}:${currentProfileId.value}` : baseKey

const readProfileScopedItem = (baseKey: string) => {
  const profileKey = getProfileStorageKey(baseKey)
  const profileValue = localStorage.getItem(profileKey)
  if (profileValue != null) return profileValue
  if (profileKey !== baseKey) {
    const legacyValue = localStorage.getItem(baseKey)
    if (legacyValue != null) {
      localStorage.removeItem(baseKey)
      localStorage.setItem(profileKey, legacyValue)
      return legacyValue
    }
  }
  return null
}

const writeProfileScopedItem = (baseKey: string, value: string) => {
  const profileKey = getProfileStorageKey(baseKey)
  localStorage.setItem(profileKey, value)
  if (profileKey !== baseKey) localStorage.removeItem(baseKey)
}

const mode = computed(() => clashConfig.value?.mode?.toLowerCase() || 'rule')
const isGlobalMode = computed(() => mode.value === 'global')
const isDirectMode = computed(() => mode.value === 'direct')

const sortType = ref<number>(() => {
  const saved = localStorage.getItem(STORAGE_KEY_SORT_TYPE)
  return saved ? Number(saved) : 0
})

const selectedGroupName = ref('')
const delays = useGroupDelays(() => selectedGroupName.value)

const autoCheckInProgress = ref(false)
const latestTimeout = ref(verge.value?.default_latency_timeout || 10000)

watch(() => verge.value?.default_latency_timeout, (val) => {
  latestTimeout.value = val || 10000
})

const selectableGroups = computed(() => {
  if (!proxyView.value) return []
  return proxyView.value.groups.filter(
    (g: any) => !g.hidden && (g.type === 'Selector' || g.type === 'URLTest'),
  )
})

const selectableGroupOptions = computed(() =>
  selectableGroups.value.map((g: any) => ({ label: g.name, value: g.name })),
)

const selectedGroup = computed(() => {
  if (!proxyView.value || isDirectMode.value) return null
  if (isGlobalMode.value) return proxyView.value.global
  return selectableGroups.value.find((g: any) => g.name === selectedGroupName.value) ?? null
})

const optionsForGroup = (group: any) => {
  if (!proxyView.value || !group) return []
  return group.members.map((member: any, memberIndex: number) => ({
    memberIndex,
    member: resolveMember(proxyView.value, member),
  }))
}

const unsortedProxyOptions = computed(() => optionsForGroup(selectedGroup.value))

watch([proxyView, isDirectMode, isGlobalMode, selectableGroups], () => {
  if (!proxyView.value) return
  if (isDirectMode.value) { selectedGroupName.value = 'DIRECT'; return }
  if (isGlobalMode.value) { selectedGroupName.value = proxyView.value.global?.name ?? 'GLOBAL'; return }
  const savedGroup = readProfileScopedItem(STORAGE_KEY_GROUP)
  const primaryKeywords = ['auto', 'select', 'proxy', '节点选择', '自动选择']
  const primaryGroup = selectableGroups.value.find((g: any) =>
    primaryKeywords.some((kw: string) => g.name.toLowerCase().includes(kw.toLowerCase())),
  ) ?? selectableGroups.value[0]
  const nextGroup = selectableGroups.value.some((g: any) => g.name === selectedGroupName.value)
    ? selectedGroupName.value
    : selectableGroups.value.some((g: any) => g.name === savedGroup)
      ? savedGroup!
      : (primaryGroup?.name ?? '')
  if (nextGroup !== selectedGroupName.value) {
    selectedGroupName.value = nextGroup
    if (nextGroup) writeProfileScopedItem(STORAGE_KEY_GROUP, nextGroup)
  }
}, { immediate: true })

const currentOption = computed(() => {
  if (!proxyView.value) return undefined
  if (isDirectMode.value) {
    const node = proxyView.value.direct == null ? undefined : getRecord(proxyView.value, proxyView.value.direct)
    return node ? { memberIndex: 0, member: { kind: 'node', ref: { kind: 'node', name: node.name, recordId: node.recordId }, node } } : undefined
  }
  return selectedGroup.value ? findCurrentGroupMember(proxyView.value, selectedGroup.value) : undefined
})

const currentMember = computed(() => currentOption.value?.member)
const currentProxy = computed(() => currentMember.value ? memberDetails(currentMember.value) : undefined)
const selectedProxyName = computed(() => currentMember.value?.ref.name ?? '')

const currentDelay = computed(() =>
  currentMember.value && selectedGroupName.value ? delayManager.getDelayFix(currentMember.value, selectedGroupName.value) : -1,
)

const currentOptionValue = computed({
  get: () => currentOption.value ? optionValue(currentOption.value) : '',
  set: () => {},
})

const { handleSelectChange } = useProxySelection({
  onSuccess: () => refreshProxy(),
  onError: (error: any) => { console.error('代理切换失败', error); refreshProxy() },
})

const handleGroupChange = (value: string) => {
  if (isGlobalMode.value || isDirectMode.value) return
  selectedGroupName.value = value
  writeProfileScopedItem(STORAGE_KEY_GROUP, value)
}

const optionValue = (option: any) =>
  `${option.memberIndex}:${option.member.kind === 'node' ? option.member.node.recordId : option.member.ref.name}`

const handleProxyChange = (value: string) => {
  if (isDirectMode.value) return
  const option = unsortedProxyOptions.value.find((c: any) => optionValue(c) === value)
  if (!selectedGroup.value || !option || !isInteractableMember(option.member)) return
  const previousProxy = selectedGroup.value.now
  const nextName = option.member.ref.name
  handleSelectChange(selectedGroup.value.name, previousProxy, isGlobalMode.value)({ target: { value: nextName } })
}

const goToProxies = () => router.push('/proxies')

const getDelayColor = (delay: number) => delayManager.formatDelayColor(delay)

const convertDelayColor = (delay: number): string => {
  const c = getDelayColor(delay)
  if (!c) return 'default'
  const m = c.split('.')[0]
  return m
}

const signalInfo = computed(() => {
  if (!currentProxy.value || !selectedGroupName.value) return { icon: '', text: t('home.components.currentProxy.status.uninitialized'), color: 'var(--text-secondary-color)' }
  const delay = currentDelay.value
  if (delay === -2) return { icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M1 9l2 2c4.97-4.97 13.03-4.97 18 0l2-2C16.93 2.93 7.08 2.93 1 9zm8 8l3 3 3-3c-1.65-1.66-4.34-1.66-6 0zm-4-4l2 2c2.76-2.76 7.24-2.76 10 0l2-2C15.14 9.14 8.87 9.14 5 13z"/></svg>', text: t('home.components.currentProxy.status.testing'), color: 'var(--text-secondary-color)' }
  if (delay === -1) return { icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M1 9l2 2c4.97-4.97 13.03-4.97 18 0l2-2C16.93 2.93 7.08 2.93 1 9zm8 8l3 3 3-3c-1.65-1.66-4.34-1.66-6 0zm-4-4l2 2c2.76-2.76 7.24-2.76 10 0l2-2C15.14 9.14 8.87 9.14 5 13z"/></svg>', text: t('home.components.currentProxy.status.untested'), color: 'var(--text-secondary-color)' }
  if (delay > 1e5) return { icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>', text: t('home.components.currentProxy.status.error'), color: 'var(--error-color)' }
  if (delay === 0 || delay >= 10000) return { icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>', text: t('home.components.currentProxy.status.timeout'), color: 'var(--error-color)' }
  if (delay >= 500) return { icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>', text: t('home.components.currentProxy.status.latencyHigh'), color: 'var(--error-color)' }
  if (delay >= 300) return { icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>', text: t('home.components.currentProxy.status.latencyMedium'), color: 'var(--warning-color)' }
  if (delay >= 200) return { icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>', text: t('home.components.currentProxy.status.latencyGood'), color: 'var(--info-color)' }
  return { icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>', text: t('home.components.currentProxy.status.latencyExcellent'), color: 'var(--success-color)' }
})

const checkCurrentProxyDelay = async () => {
  if (autoCheckInProgress.value || isDirectMode.value) return
  const groupName = selectedGroupName.value
  const proxyName = selectedProxyName.value
  if (!groupName || !proxyName) return
  const proxyMember = currentMember.value
  if (!proxyMember || !isInteractableMember(proxyMember)) return
  autoCheckInProgress.value = true
  const timeout = latestTimeout.value || 10000
  try {
    await delayManager.checkDelay(proxyMember, groupName, timeout)
  } catch (error) {
    console.error(`[CurrentProxyCard] 自动检测失败`, error)
  } finally {
    autoCheckInProgress.value = false
    refreshProxy()
  }
}

let autoTimers: any[] = []
watch([autoDelayEnabled, selectedGroupName, selectedProxyName, isDirectMode], () => {
  autoTimers.forEach(clearTimeout)
  autoTimers = []
  if (isDirectMode.value || !autoDelayEnabled.value || !selectedGroupName.value || !selectedProxyName.value) return
  const runAndSchedule = async () => {
    await checkCurrentProxyDelay()
    autoTimers.push(setTimeout(runAndSchedule, autoDelayIntervalMs.value))
  }
  autoTimers.push(setTimeout(async () => {
    await checkCurrentProxyDelay()
    autoTimers.push(setTimeout(runAndSchedule, autoDelayIntervalMs.value))
  }, AUTO_CHECK_INITIAL_DELAY_MS))
}, { immediate: true })

onUnmounted(() => autoTimers.forEach(clearTimeout))

const proxyOptions = computed(() => {
  const sortWithLatency = (proxiesToSort: any[]) => {
    if (!proxiesToSort || sortType.value === 0) return proxiesToSort
    const list = [...proxiesToSort]
    if (sortType.value === 1) {
      const effectiveTimeout = typeof defaultLatencyTimeout.value === 'number' && defaultLatencyTimeout.value > 0
        ? defaultLatencyTimeout.value : DEFAULT_DELAY_TIMEOUT
      list.sort((a: any, b: any) => {
        const byDelay = compareByDelay(delays.of(a.member), delays.of(b.member), effectiveTimeout)
        if (byDelay !== 0) return byDelay
        return a.member.ref.name.localeCompare(b.member.ref.name)
      })
    } else {
      list.sort((a: any, b: any) => a.member.ref.name.localeCompare(b.member.ref.name))
    }
    return list
  }
  if (isDirectMode.value) return []
  return sortWithLatency(unsortedProxyOptions.value)
})

const proxySelectOptions = computed(() =>
  proxyOptions.value.map((option: any) => {
    const interactable = isInteractableMember(option.member)
    const delayValue = interactable ? delayManager.getDelayFix(option.member, selectedGroupName.value) : -1
    return {
      label: option.member.ref.name,
      value: optionValue(option),
      disabled: !interactable,
    }
  }),
)

const getSortIcon = () => {
  switch (sortType.value) {
    case 1: return '<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"/></svg>'
    case 2: return '<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M3 5h2v2H3V5zm4 2h14v2H7V7zm-4 6h2v-2H3v2zm4 2h14v-2H7v2zm-4 6h2v-2H3v2zm4 2h14v-2H7v2z"/></svg>'
    default: return '<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M3 18h6v-2H3v2zM3 6v2h18V6H3zm0 7h12v-2H3v2z"/></svg>'
  }
}

const getSortTooltip = () => {
  switch (sortType.value) {
    case 0: return t('proxies.page.tooltips.sortDefault')
    case 1: return t('proxies.page.tooltips.sortDelay')
    case 2: return t('proxies.page.tooltips.sortName')
    default: return ''
  }
}

const handleSortTypeChange = () => {
  sortType.value = (sortType.value + 1) % 3
  localStorage.setItem(STORAGE_KEY_SORT_TYPE, sortType.value.toString())
}

let checkingDelay = false
const handleCheckDelay = async () => {
  const groupName = selectedGroupName.value
  if (!groupName || isDirectMode.value || checkingDelay) return
  checkingDelay = true
  const timeout = verge.value?.default_latency_timeout || 10000
  const interactable = unsortedProxyOptions.value
    .map(({ member }: any) => member)
    .filter(isInteractableMember)
    .filter(({ ref }: any) => ref.name !== 'DIRECT' && ref.name !== 'REJECT')
  if (interactable.length > 0) {
    try { await delayManager.checkListDelay(interactable, groupName, timeout) }
    catch (error) { console.error(error) }
  }
  refreshProxy()
  checkingDelay = false
}
</script>
