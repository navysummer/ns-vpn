<template>
  <div>
    <div :style="{ display: 'flex', height: '100%', gap: '16px' }">
      <div :style="{ flex: 1, position: 'relative' }">
        <div v-if="showRuleHeader" :style="{ borderBottom: '1px solid var(--border-color)' }">
          <div :style="{ padding: '12px 16px', borderBottom: '1px solid var(--border-color)', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }">
            <div :style="{ display: 'flex', alignItems: 'center', gap: '16px' }">
              <span :style="{ fontSize: '16px', fontWeight: 600 }">{{ t('proxies.page.rules.title') }}</span>
              <n-tag v-if="currentGroup" size="small" variant="outline" :style="{ fontSize: '12px', maxWidth: '200px' }">
                {{ currentGroup.name }} ({{ currentGroup.type }})
              </n-tag>
            </div>
            <n-button v-if="availableGroups.length > 0" quaternary size="small" @click="handleGroupMenuOpen">
              <span :style="{ fontSize: '12px', marginRight: '4px' }">{{ t('proxies.page.rules.select') }}</span>
              <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M16.59 8.59L12 13.17 7.41 8.59 6 10l6 6 6-6z"/></svg>
            </n-button>
          </div>
        </div>
        <div :style="{ height: showRuleHeader ? 'calc(100% - 80px)' : 'calc(100% - 14px)', overflow: 'auto' }" ref="parentRef">
          <div :style="{ height: totalSize + 'px', position: 'relative' }">
            <div
              v-for="virtualItem in virtualItems"
              :key="virtualItem.key"
              :data-index="virtualItem.index"
              :ref="measureElement"
              :style="getVirtualItemStyle(virtualItem.index)"
            >
              <ProxyRender
                :item="renderList[virtualItem.index]"
                :isChainMode="true"
                @onLocation="(group: any) => onLocation(group)"
                @onCheckAll="(groupName: string) => onCheckAll(groupName)"
                @onHeadState="(groupName: string, patch: any) => onHeadState(groupName, patch)"
                @onChangeProxy="handleChangeProxy"
              />
            </div>
            <div :style="{ height: '8px' }" />
          </div>
        </div>
        <ScrollTopButton v-if="showScrollTop" @click="onScrollToTop" />
      </div>
      <div :style="{ width: '400px', minWidth: '300px' }">
        <ProxyChain
          :proxyChain="currentProxyChain"
          :chainConfigData="chainConfigData"
          :mode="mode"
          :selectedGroup="activeSelectedGroup"
          @updateChain="(chain: any[]) => { setProxyChain(chain); persistChain(chain) }"
        />
      </div>
    </div>

    <n-modal v-model:show="duplicateWarning.open" :mask-closable="true" preset="dialog" type="warning" :title="t('proxies.page.chain.duplicateNode')" :content="duplicateWarning.message" @positive-click="duplicateWarning.open = false" @negative-click="duplicateWarning.open = false" />

    <n-dropdown
      placement="bottom-start"
      trigger="manual"
      :show="ruleMenuOpen"
      :x="ruleMenuAnchor?.left ?? 0"
      :y="ruleMenuAnchor?.top ?? 0"
      :options="groupMenuOptions"
      @clickoutside="handleGroupMenuClose"
      @select="handleGroupSelect"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, reactive } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NDropdown, NModal, NTag } from 'naive-ui'

import { useProxiesData } from '@/providers/app-data-context'
import { updateProxyChainConfigInRuntime } from '@/services/cmds'
import { isInteractableMember } from '@/types/proxy-view'

import ScrollTopButton from '@/components/layout/scroll-top-button.vue'
import ProxyChain from './proxy-chain.vue'
import { rebindProxyChainItems } from './proxy-chain-model'
import ProxyRender from './proxy-render.vue'

const props = defineProps<{
  mode: string
  chainConfigData?: string | null
  availableGroups: any[]
  activeSelectedGroup: string | null
  showScrollTop: boolean
  parentRef: any
  totalSize: number
  virtualItems: any[]
  renderList: any[]
  activeStickyIndex: number | null
  measureElement: any
  onCheckAll: (groupName: string) => void
  onHeadState: (groupName: string, patch: any) => void
  onLocation: (group: any) => void
  onGroupSelect: (groupName: string) => void
  onScrollToTop: () => void
}>()

const { t } = useI18n()
const { proxyView } = useProxiesData()

const proxyChain = ref<any[]>(() => {
  try {
    const saved = localStorage.getItem('proxy-chain-items')
    return saved ? JSON.parse(saved) : []
  } catch { return [] }
})

const candidateNodes = computed(() =>
  props.renderList.flatMap((item: any) => {
    const occurrences = item.memberCol ?? (item.member ? [item.member] : [])
    return occurrences.flatMap(({ member }: any) => member.kind === 'node' ? [member.node] : [])
  }),
)

const currentProxyChain = computed(() => {
  if (!proxyView.value) return proxyChain.value.map((item: any) => ({ ...item, recordId: undefined, delay: undefined }))
  return rebindProxyChainItems(proxyChain.value, candidateNodes.value, proxyView.value) || proxyChain.value
})

const persistChain = (chain: any[]) => {
  if (chain.length > 0) {
    const persisted = chain.map(({ id, name, type, delay }: any) => ({ id, name, type, delay }))
    localStorage.setItem('proxy-chain-items', JSON.stringify(persisted))
  } else {
    localStorage.removeItem('proxy-chain-items')
  }
}

watch(currentProxyChain, (chain) => persistChain(chain), { deep: true })

const ruleMenuOpen = ref(false)
const ruleMenuAnchor = ref<{ left: number; top: number } | null>(null)
const duplicateWarning = reactive({ open: false, message: '' })

const currentGroup = computed(() => {
  if (!props.activeSelectedGroup) return null
  return props.availableGroups.find((g: any) => g.name === props.activeSelectedGroup) ?? null
})

const groupMenuOptions = computed(() => {
  const groups = props.availableGroups.length > 0
    ? props.availableGroups.map((g: any) => ({
        key: g.name,
        label: g.name,
        type: 'group' as const,
        children: [{ key: g.name, label: `${g.name} (${g.type} · ${g.members.length})` }],
      }))
    : [{ key: 'empty', label: t('proxies.page.empty.noAvailableGroups'), disabled: true }]
  return groups
})

const handleGroupMenuOpen = (e: MouseEvent) => {
  const rect = (e.currentTarget as HTMLElement).getBoundingClientRect()
  ruleMenuAnchor.value = { left: rect.left, top: rect.bottom + 4 }
  ruleMenuOpen.value = true
}

const handleGroupMenuClose = () => { ruleMenuOpen.value = false }

const handleGroupSelect = (groupName: string) => {
  props.onGroupSelect(groupName)
  handleGroupMenuClose()
  if (props.mode === 'rule') {
    updateProxyChainConfigInRuntime(null)
    localStorage.removeItem('proxy-chain-group')
    localStorage.removeItem('proxy-chain-exit-node')
    localStorage.removeItem('proxy-chain-items')
    proxyChain.value = []
  }
}

const handleChangeProxy = (_group: any, member: any) => {
  if (!isInteractableMember(member) || member.kind !== 'node') return
  const { node } = member
  const current = proxyView.value ? rebindProxyChainItems(proxyChain.value, candidateNodes.value, proxyView.value) : proxyChain.value
  if (current.some((item: any) => item.recordId !== undefined && item.recordId === node.recordId)) {
    duplicateWarning.message = t('proxies.page.chain.duplicateNode')
    duplicateWarning.open = true
    return
  }
  const delay = node.history.length > 0 ? node.history[node.history.length - 1].delay : undefined
  proxyChain.value = [...current, {
    id: `${node.name}_${Date.now()}`,
    name: node.name,
    recordId: node.recordId,
    source: node.source,
    type: node.type,
    delay,
  }]
}

const getVirtualItemStyle = (index: number) => ({
  position: index === props.activeStickyIndex ? 'sticky' as const : 'absolute' as const,
  top: 0,
  left: 0,
  zIndex: index === props.activeStickyIndex ? 5 : undefined,
  display: index === props.activeStickyIndex ? 'flow-root' as const : undefined,
  backgroundColor: index === props.activeStickyIndex ? 'var(--bg-color)' : undefined,
  width: '100%',
  transform: index === props.activeStickyIndex ? undefined : `translateY(${props.virtualItems.find((v: any) => v.index === index)?.start ?? 0}px)`,
})

const showRuleHeader = computed(() => props.mode === 'rule' && props.availableGroups.length > 0)
const setProxyChain = (chain: any[]) => { proxyChain.value = chain }
</script>
