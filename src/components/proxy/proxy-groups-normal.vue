<template>
  <div :style="{ position: 'relative', height: '100%' }">
    <StickyVirtualList
      ref="stickyListRef"
      :items="renderList"
      :isGroupItem="(item: any) => item.type === 0"
      :getItemKey="(item: any) => item.key"
      :estimateGroupItemHeight="76"
      :estimateItemHeight="64"
      :renderGroupItem="renderGroupItem"
      :renderItem="renderProxyItem"
    />
    <ProxyGroupNavigator
      v-if="mode === 'rule'"
      :proxyGroupNames="proxyGroupNames"
      :onGroupLocation="handleGroupLocationByName"
      :enableHoverJump="verge?.enable_hover_jump_navigator ?? true"
      :hoverDelay="verge?.hover_jump_navigator_delay ?? DEFAULT_HOVER_DELAY"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, h, useTemplateRef, watch, onMounted, onUnmounted } from 'vue'
import { throttle } from 'lodash-es'

import { StickyVirtualList } from '@/components/base'
import { useProfiles } from '@/hooks/use-profiles'
import { useProxySelection } from '@/hooks/use-proxy-selection'
import { useVerge } from '@/hooks/use-verge'
import { useProxiesData, useSystemData } from '@/providers/app-data-context'
import delayManager from '@/services/delay'
import { isInteractableMember, resolveMember } from '@/types/proxy-view'
import { debugLog } from '@/utils/debug'

import ProxyEmptyState from './proxy-empty-state.vue'
import { resolveEmptyListReason } from './proxy-empty-state-model'
import { DEFAULT_HOVER_DELAY } from './proxy-group-navigator.vue'
import ProxyGroupNavigator from './proxy-group-navigator.vue'
import ProxyRender from './proxy-render.vue'
import { hasRenderableItems, useRenderList, type IRenderItem } from './use-render-list'

const props = defineProps<{
  mode: string
}>()

const { verge } = useVerge()
const { proxyView } = useProxiesData()
const { runningMode, isProxyViewError } = useSystemData()
const stickyListRef = ref<any>(null)

const { renderList, onProxies, onHeadState } = useRenderList(props.mode, false, null)
const scrollPositionKey = computed(() => `${props.mode}:normal`)

const handleCheckAll = useLockFn(async (groupName: string) => {
  const timeout = verge.value?.default_latency_timeout || 10000
  const group = proxyView.value?.groups.find(({ name }: any) => name === groupName) ??
    (proxyView.value?.global?.name === groupName ? proxyView.value.global : undefined)
  const occurrences = proxyView.value && group
    ? group.members.map((member: any, memberIndex: number) => ({ memberIndex, member: resolveMember(proxyView.value, member) }))
    : []
  const interactable = occurrences.map(({ member }: any) => member).filter(isInteractableMember)
  try { await delayManager.checkListDelay(interactable, groupName, timeout) }
  catch (error) { console.error(error) }
  finally { onProxies() }
})

const saveScrollPosition = (scrollTop: number) => {
  const scrollPositions = JSON.parse(localStorage.getItem('proxy-scroll-positions') || '{}')
  scrollPositions[scrollPositionKey.value] = scrollTop
  localStorage.setItem('proxy-scroll-positions', JSON.stringify(scrollPositions))
}

const saveScrollPositionThrottled = throttle(saveScrollPosition, 500)

const renderFirst = ref(true)
const isRestoring = ref(false)

const getScrollPosition = () => {
  try {
    const saved = JSON.parse(localStorage.getItem('proxy-scroll-positions') || '{}')
    return saved[scrollPositionKey.value] ?? 0
  } catch { return 0 }
}

watch(() => renderList.value?.length, (len) => {
  if (!len || !renderFirst.value) return
  const node = stickyListRef.value?.getScrollElement()
  if (!node) return
  const savedPos = getScrollPosition()
  if (!savedPos) { renderFirst.value = false; return }
  isRestoring.value = true
  let rafId = 0; let attempts = 0; const maxAttempts = 30
  const step = () => {
    const el = stickyListRef.value?.getScrollElement()
    if (!el) { isRestoring.value = false; return }
    el.scrollTop = savedPos; attempts++
    if (Math.abs(el.scrollTop - savedPos) <= 1 || attempts >= maxAttempts) {
      renderFirst.value = false; isRestoring.value = false; return
    }
    rafId = requestAnimationFrame(step)
  }
  rafId = requestAnimationFrame(step)
})

onMounted(() => {
  const node = stickyListRef.value?.getScrollElement()
  if (!node) return
  const handleScroll = (e: Event) => {
    if (isRestoring.value) return
    const target = e.target as HTMLElement
    saveScrollPositionThrottled(target?.scrollTop ?? 0)
  }
  node.addEventListener('scroll', handleScroll, { passive: true })
  onUnmounted(() => {
    node.removeEventListener('scroll', handleScroll)
    const finalScroll = node.scrollTop
    if (finalScroll != null) saveScrollPosition(finalScroll)
  })
})

const { handleProxyGroupChange } = useProxySelection({
  onSuccess: () => onProxies(),
  onError: (error: any) => { console.error(error); onProxies() },
})

const handleChangeProxy = (group: any, member: any) => {
  if (!['Selector', 'URLTest', 'Fallback'].includes(group.type)) return
  if (!isInteractableMember(member)) return
  handleProxyGroupChange(group, { name: member.ref.name })
}

const handleLocation = (group: any) => {
  if (!group) return
  const { name, now } = group
  const index = renderList.value.findIndex(
    (e: any) => e.group?.name === name &&
      ((e.type === 2 && e.member?.member.ref.name === now) ||
       (e.type === 4 && e.memberCol?.some(({ member }: any) => member.ref.name === now))),
  )
  if (index >= 0) stickyListRef.value?.scrollToIndex(index, { align: 'center', behavior: 'smooth' })
}

const handleGroupLocationByName = (groupName: string) => {
  const index = renderList.value.findIndex((item: any) => item.type === 0 && item.group?.name === groupName)
  if (index >= 0) stickyListRef.value?.scrollToIndex(index, { align: 'start', behavior: 'smooth' })
}

const proxyGroupNames = computed(() => {
  const names = renderList.value.filter((item: any) => item.type === 0 && item.group?.name).map((item: any) => item.group!.name)
  return Array.from(new Set(names))
})

const handleGroupToggle = async (group: any) => {
  const index = renderList.value.findIndex((item: any) => item.type === 0 && item.group.name === group.name)
  if (index < 0) return
  if (!stickyListRef.value?.isItemScrolledPastStart(index, 1)) return
  stickyListRef.value.scrollToIndex(index, { align: 'start', behavior: 'auto' })
  await new Promise(resolve => requestAnimationFrame(resolve))
}

const renderGroupItem = (item: any, _index: number, stickyed: boolean) =>
  h(ProxyRender, {
    item,
    stickyed,
    onLocation: handleLocation,
    onCheckAll: handleCheckAll,
    onHeadState: (groupName: string, patch: any) => {
      if (stickyed && patch.filterText !== undefined) { handleGroupLocationByName(groupName); stickyListRef.value?.waitForScrollEnd() }
      onHeadState(groupName, patch)
    },
    onChangeProxy: handleChangeProxy,
  })

const renderProxyItem = (item: any) =>
  h(ProxyRender, {
    item,
    onLocation: handleLocation,
    onCheckAll: handleCheckAll,
    onHeadState,
    onChangeProxy: handleChangeProxy,
  })

const emptyList = computed(() => {
  if (hasRenderableItems(renderList.value)) return null
  return h(ProxyEmptyState, {
    reason: resolveEmptyListReason({ runningMode: runningMode.value, isProxyViewError: isProxyViewError.value }),
  })
})
</script>
