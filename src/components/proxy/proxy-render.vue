<template>
  <div v-if="type === 0" :style="{ padding: '4px 8px' }">
    <div
      :style="{
        backgroundColor: itembackgroundcolor,
        height: '100%',
        borderRadius: '8px',
        padding: '4px 12px',
        cursor: 'pointer',
        boxShadow: stickyed && headState?.open ? '0 4px 8px rgba(0,0,0,0.2)' : undefined,
      }"
      @click="handleGroupClick"
    >
      <div :style="{ display: 'flex', alignItems: 'center', width: '100%' }">
        <img
          v-if="enable_group_icon && group.icon?.trim().startsWith('http')"
          :src="iconCachePath === '' ? group.icon : iconCachePath"
          alt="group icon"
          width="32"
          :style="{ marginRight: '12px', borderRadius: '6px' }"
        />
        <img
          v-if="enable_group_icon && group.icon?.trim().startsWith('data')"
          :src="group.icon"
          alt="group icon"
          width="32"
          :style="{ marginRight: '12px', borderRadius: '6px' }"
        />
        <img
          v-if="enable_group_icon && group.icon?.trim().startsWith('<svg')"
          :src="`data:image/svg+xml;base64,${btoa(group.icon)}`"
          alt="group icon"
          width="32"
        />
        <div :style="{ flex: '0 1 auto', minWidth: 0 }">
          <span :style="{ fontSize: '16px', fontWeight: 700, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', display: 'block' }">{{ group.name }}</span>
          <div :style="{ display: 'flex', alignItems: 'center', paddingTop: '2px', overflow: 'hidden', whiteSpace: 'nowrap' }">
            <span :style="typeBoxStyle">{{ group.type }}</span>
            <span :style="{ fontSize: '13px', color: 'var(--text-secondary-color)', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }">{{ group.now }}</span>
          </div>
        </div>
        <div :style="{ display: 'flex', alignItems: 'center', justifyContent: 'flex-end', flex: '1 1 auto', minWidth: 0 }">
          <ProxyGroupTools
            :url="group.testUrl"
            :groupName="group.name"
            :headState="headState || {}"
            @onLocation="() => onLocation(group)"
            @onCheckDelay="() => onCheckAll(group.name)"
            @onHeadState="(p: any) => onHeadState(group.name, p)"
          />
          <n-tooltip :title="t('proxies.page.labels.proxyCount')" :trigger="'hover'">
            <template #trigger>
              <div :style="{ minWidth: '50px', display: 'flex', justifyContent: 'flex-end', alignItems: 'center' }">
                <n-tag size="small" :style="{ backgroundColor: 'var(--primary-color-alpha)', color: 'var(--primary-color)', marginRight: '8px' }">
                  {{ group.members.length }}
                </n-tag>
              </div>
            </template>
          </n-tooltip>
          <svg v-if="headState?.open" viewBox="0 0 24 24" width="24" height="24" fill="currentColor"><path d="M12 8l-6 6 1.41 1.41L12 10.83l4.59 4.58L18 14z"/></svg>
          <svg v-else viewBox="0 0 24 24" width="24" height="24" fill="currentColor"><path d="M16.59 8.59L12 13.17 7.41 8.59 6 10l6 6 6-6z"/></svg>
        </div>
      </div>
    </div>
  </div>
  <div v-else-if="type === 1">
    <ProxyHead
      :sx="{ paddingLeft: '16px', paddingRight: '24px', marginTop: '4px', marginBottom: '8px' }"
      :url="group.testUrl"
      :groupName="group.name"
      :headState="headState || {}"
      @onLocation="() => onLocation(group)"
      @onCheckDelay="() => onCheckAll(group.name)"
      @onHeadState="(p: any) => onHeadState(group.name, p)"
    />
  </div>
  <ProxyItem
    v-else-if="type === 2"
    :group="group"
    :member="member?.member"
    :selected="group.now === member?.member.ref.name"
    :showType="headState?.showType"
    :sx="{ padding: '0 16px' }"
    @onClick="(nextMember: any) => onChangeProxy(group, nextMember)"
  />
  <div v-else-if="type === 3" :style="{ padding: '16px 0', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center' }">
    <svg viewBox="0 0 24 24" width="40" height="40" fill="currentColor"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z"/></svg>
    <span :style="{ color: 'inherit' }">{{ t('proxies.page.empty.noProxies') }}</span>
  </div>
  <div v-else-if="type === 4" :style="{ height: '56px', display: 'grid', margin: '4px 0', gap: '8px', padding: '0 16px', gridTemplateColumns: `repeat(${item.col || 2}, 1fr)` }">
    <ProxyItemMini
      v-for="occurrence in memberColItems"
      :key="`${item.key}-${occurrence.memberIndex}`"
      :group="group"
      :member="occurrence.member"
      :selected="group.now === occurrence.member.ref.name"
      :showType="headState?.showType"
      @onClick="(nextMember: any) => onChangeProxy(group, nextMember)"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NTooltip, NTag } from 'naive-ui'

import { useIconCache } from '@/hooks/use-icon-cache'
import { useVerge } from '@/hooks/use-verge'
import { useThemeMode } from '@/services/states'

import ProxyGroupTools from './proxy-group-tools.vue'
import ProxyHead from './proxy-head.vue'
import ProxyItem from './proxy-item.vue'
import ProxyItemMini from './proxy-item-mini.vue'

const props = defineProps<{
  item: any
  stickyed?: boolean
  isChainMode?: boolean
  onLocation?: (group: any) => void
  onCheckAll?: (groupName: string) => void
  onHeadState?: (groupName: string, patch: any) => void
  onChangeProxy?: (group: any, member: any) => void
  onGroupToggle?: (group: any) => void
}>()

const { t } = useI18n()
const { type, group, headState, member, memberCol, key } = computed(() => props.item)
const { verge } = useVerge()
const enable_group_icon = computed(() => verge.value?.enable_group_icon ?? true)
const mode = useThemeMode()
const isDark = computed(() => mode.value === 'dark')
const itembackgroundcolor = computed(() => isDark.value ? '#282A36' : '#ffffff')

const iconCachePath = useIconCache({
  icon: computed(() => group.value.icon),
  cacheKey: computed(() => group.value.name.replaceAll(' ', '')),
  enabled: enable_group_icon,
})

const showType = computed(() => headState.value?.showType)

const memberColItems = computed(() => {
  if (type.value !== 4 || !memberCol.value) return []
  return memberCol.value.map((occurrence: any) => ({
    memberIndex: occurrence.memberIndex,
    member: occurrence.member,
  }))
})

const handleGroupClick = () => {
  if (headState.value?.open) props.onGroupToggle?.(group.value)
  props.onHeadState?.(group.value.name, { open: !headState.value?.open })
}

const typeBoxStyle = {
  display: 'inline-block',
  border: '1px solid var(--primary-color)',
  color: 'var(--primary-color)',
  borderRadius: '4px',
  fontSize: '10px',
  padding: '0 4px',
  lineHeight: '1.5',
  marginRight: '8px',
}
</script>
