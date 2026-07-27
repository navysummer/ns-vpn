<template>
  <template v-if="listState.kind === 'direct'">
    <BaseEmpty textKey="proxies.page.messages.directMode" />
  </template>
  <template v-else-if="listState.kind === 'loading'">
    <BaseLoading />
  </template>
  <template v-else-if="listState.kind === 'empty'">
    <ProxyEmptyState :reason="listState.reason" />
  </template>
  <template v-else>
    <ChainProxyGroups v-if="props.isChainMode" :mode="props.mode" :chainConfigData="props.chainConfigData" />
    <NormalProxyGroups v-else :mode="props.mode" />
  </template>
</template>

<script setup lang="ts">
import { computed, ref, defineAsyncComponent } from 'vue'
import { useLockFn } from '@/hooks/use-lock-fn'
import { throttle } from 'lodash-es'

import { BaseEmpty, BaseLoading, StickyVirtualList } from '@/components/base'
import { useProfiles } from '@/hooks/use-profiles'
import { useProxySelection } from '@/hooks/use-proxy-selection'
import { useVerge } from '@/hooks/use-verge'
import { useProxiesData, useSystemData } from '@/providers/app-data-context'
import delayManager from '@/services/delay'
import { isInteractableMember, resolveMember } from '@/types/proxy-view'
import { debugLog } from '@/utils/debug'

import ProxyEmptyState from './proxy-empty-state.vue'
import { resolveEmptyListReason, resolveProxyListState } from './proxy-empty-state-model'
import { DEFAULT_HOVER_DELAY } from './proxy-group-navigator.vue'
import ProxyGroupNavigator from './proxy-group-navigator.vue'
import ProxyRender from './proxy-render.vue'
import { hasRenderableItems, useRenderList, type IRenderItem } from './use-render-list'

const ChainProxyGroups = defineAsyncComponent(() => import('./proxy-groups-chain.vue'))
const NormalProxyGroups = defineAsyncComponent(() => import('./proxy-groups-normal.vue'))

const props = withDefaults(defineProps<{
  mode: string
  isChainMode?: boolean
  chainConfigData?: string | null
}>(), {
  isChainMode: false,
  chainConfigData: null,
})

const { profiles, isLoading: isProfilesLoading } = useProfiles()
const { isProxyViewPending, isProxyViewError } = useProxiesData()
const { isRunningModePending, runningMode } = useSystemData()

const listState = computed(() => resolveProxyListState({
  mode: props.mode,
  profiles: profiles.value,
  isProfilesPending: !profiles.value && isProfilesLoading.value,
  isProxyViewPending: isProxyViewPending.value,
  isRunningModePending: isRunningModePending.value,
}))
</script>
