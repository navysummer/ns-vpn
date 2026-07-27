<template>
  <div
    :style="{
      height: '100%',
      display: 'flex',
      flexDirection: 'column',
      borderRadius: '12px',
      backgroundColor: isDark ? '#282a36' : '#ffffff',
      ...(minHeight ? { minHeight: typeof minHeight === 'number' ? minHeight + 'px' : minHeight } : {}),
    }"
  >
    <div
      :style="{
        px: '16px',
        py: '8px',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        borderBottom: '1px solid',
        borderColor: 'var(--border-color)',
      }"
    >
      <div
        :style="{
          display: 'flex',
          alignItems: 'center',
          minWidth: 0,
          flex: 1,
          overflow: 'hidden',
        }"
      >
        <div
          :style="{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            borderRadius: '12px',
            width: '38px',
            height: '38px',
            marginRight: '12px',
            flexShrink: 0,
            backgroundColor: iconBgColor,
            color: iconColorValue,
          }"
        >
          <slot name="icon">
            <span v-if="icon" v-html="icon" />
          </slot>
        </div>
        <div :style="{ minWidth: 0, flex: 1 }">
          <span
            :style="{
              ...titleTruncateStyle,
              fontWeight: 500,
              fontSize: '18px',
            }"
            :title="typeof title === 'string' ? title : undefined"
          >
            {{ title }}
          </span>
        </div>
      </div>
      <div v-if="$slots.action" :style="{ marginLeft: '16px', flexShrink: 0 }">
        <slot name="action" />
      </div>
    </div>
    <div
      :style="{
        flex: 1,
        display: 'flex',
        flexDirection: 'column',
        padding: noContentPadding ? '0' : '16px',
      }"
    >
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useThemeMode } from '@/services/states'

const props = withDefaults(defineProps<{
  title?: string
  icon?: string
  iconColor?: 'primary' | 'secondary' | 'error' | 'warning' | 'info' | 'success'
  minHeight?: number | string
  noContentPadding?: boolean
}>(), {
  iconColor: 'primary',
  noContentPadding: false,
})

const mode = useThemeMode()
const isDark = computed(() => mode.value === 'dark')

const titleTruncateStyle = {
  minWidth: 0,
  maxWidth: '100%',
  overflow: 'hidden',
  textOverflow: 'ellipsis',
  whiteSpace: 'nowrap',
  display: 'block',
}

const iconColorMap: Record<string, string> = {
  primary: '#1890ff',
  secondary: '#722ed1',
  error: '#ff4d4f',
  warning: '#faad14',
  info: '#13c2c2',
  success: '#52c41a',
}

const iconColorValue = computed(() => iconColorMap[props.iconColor] || iconColorMap.primary)
const iconBgColor = computed(() => iconColorValue.value + '1F')
</script>
