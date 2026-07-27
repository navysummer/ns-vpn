<template>
  <div :style="{ marginBottom: '8px' }">
    <div
      :style="{
        ...itemButtonStyle,
        height: '40px',
        borderRadius: '8px',
        cursor: unresolved ? 'default' : 'pointer',
        opacity: unresolved ? 0.5 : 1,
      }"
      @click="unresolved ? undefined : onClick?.(member)"
    >
      <div :style="{ flex: 1, overflow: 'hidden' }">
        <div :style="{ display: 'flex', alignItems: 'center', overflow: 'hidden' }">
          <span :style="{ fontSize: '14px', color: 'var(--text-color)', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', marginRight: '8px' }">{{ name }}</span>
          <span v-if="showType && now" :style="{ fontSize: '14px', color: 'var(--text-secondary-color)', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }"> - {{ now }}</span>
        </div>
        <div v-if="showType" :style="{ display: 'flex', alignItems: 'center', marginTop: '2px', overflow: 'hidden' }">
          <span v-if="provider" :style="typeBoxStyle">{{ provider }}</span>
          <span :style="typeBoxStyle">{{ type }}</span>
          <span v-if="!unresolved && showType && details?.udp" :style="typeBoxStyle">UDP</span>
          <span v-if="!unresolved && showType && details?.xudp" :style="typeBoxStyle">XUDP</span>
          <span v-if="!unresolved && showType && details?.tfo" :style="typeBoxStyle">TFO</span>
          <span v-if="!unresolved && showType && details?.mptcp" :style="typeBoxStyle">MPTCP</span>
          <span v-if="!unresolved && showType && details?.smux" :style="typeBoxStyle">SMUX</span>
        </div>
      </div>
      <div :style="{ display: 'flex', alignItems: 'center', marginLeft: '8px', color: 'var(--primary-color)' }">
        <BaseLoading v-if="!unresolved && delayValue === -2" />
        <span
          v-if="!unresolved && delayValue !== -2"
          :style="{
            display: delayValue > 0 ? 'none' : 'block',
            padding: '3px 6px',
            fontSize: '14px',
            borderRadius: '4px',
          }"
          @click.stop="onDelay()"
        >{{ t('shared.actions.check') }}</span>
        <span
          v-if="!unresolved && delayValue > 0"
          :style="{
            display: delayValue > 0 ? 'block' : 'none',
            padding: '3px 6px',
            fontSize: '14px',
            borderRadius: '4px',
            color: delayColor,
          }"
          @click.stop="onDelay()"
        >{{ delayManager.formatDelay(delayValue, timeout) }}</span>
        <svg
          v-if="!unresolved && delayValue !== -2 && delayValue <= 0 && selected"
          viewBox="0 0 24 24" width="16" height="16" fill="currentColor"
          :style="{ fontSize: '16px' }"
        >
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
        </svg>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

import { BaseLoading } from '@/components/base'
import { useProxyDelayState } from '@/hooks/use-proxy-delay-state'
import delayManager from '@/services/delay'
import { memberDetails, providerNameOf } from '@/types/proxy-view'

const props = defineProps<{
  group: any
  member: any
  selected: boolean
  showType?: boolean
  sx?: any
  onClick?: (member: any) => void
}>()

const { t } = useI18n()

const details = computed(() => memberDetails(props.member))
const unresolved = computed(() => props.member.kind === 'unresolved')
const name = computed(() => props.member.ref.name)
const type = computed(() => unresolved.value ? props.member.ref.reason : (details.value?.type ?? ''))
const provider = computed(() => props.member.kind === 'node' ? providerNameOf(props.member.node) : undefined)
const now = computed(() => props.member.kind === 'group' ? props.member.group.now : undefined)

const { delayValue, isPreset, timeout, onDelay } = useProxyDelayState(props.member, props.group.name)
const delayColor = computed(() => delayManager.formatDelayColor(delayValue.value, timeout.value))

const itemButtonStyle = {
  display: 'flex',
  alignItems: 'center',
  padding: '0 16px',
  borderRadius: '8px',
  backgroundColor: 'var(--bg-color)',
}
const typeBoxStyle = {
  display: 'inline-block',
  border: '1px solid var(--text-secondary-color)',
  color: 'var(--text-secondary-color)',
  borderRadius: '4px',
  fontSize: '10px',
  marginRight: '4px',
  padding: '0 2px',
  lineHeight: '1.25',
}
</script>
