<template>
  <n-tooltip :title="title" placement="top">
    <template #trigger>
      <n-button quaternary circle size="small" v-bind="$attrs">
        <template #icon>
          <n-icon size="inherit" style="cursor: pointer; opacity: 0.75">
            <component :is="iconComponent" />
          </n-icon>
        </template>
      </n-button>
    </template>
  </n-tooltip>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { InformationCircle } from '@vicons/ionicons5'

interface Props {
  title?: string
  icon?: any
}

const props = withDefaults(defineProps<Props>(), {
  title: '',
})

const iconComponent = computed(() => props.icon || InformationCircle)
</script>
