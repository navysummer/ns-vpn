<template>
  <n-button-group size="small">
    <n-button
      v-for="mode in modes"
      :key="mode"
      :type="mode === value ? 'primary' : 'default'"
      @click="$emit('change', mode)"
    >
      {{ t(`settings.sections.appearance.${mode}`) }}
    </n-button>
  </n-button-group>
</template>

<script setup lang="ts">
import { useTranslation } from '@/composables/use-i18n'

const { t } = useTranslation()
type ThemeValue = IVergeConfig['theme_mode']
const modes = ['light', 'dark', 'system'] as const

defineProps<{
  value?: ThemeValue
}>()

defineEmits<{
  change: [value: ThemeValue]
}>()
</script>
