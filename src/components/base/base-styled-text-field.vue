<template>
  <n-input
    v-bind="$attrs"
    size="small"
    :placeholder="placeholder ?? t('shared.placeholders.filter')"
    :class="{ 'light-bg': isLight }"
  />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useTranslation } from '@/composables/use-i18n'
const { t } = useTranslation()

interface Props {
  placeholder?: string
}

const props = defineProps<Props>()
const isLight = computed(() => document.documentElement.getAttribute('theme') === 'light')
</script>

<style scoped>
.light-bg {
  --n-color: #fff;
}
</style>
