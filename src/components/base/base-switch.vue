<template>
  <n-switch v-bind="$attrs" :style="switchStyle" />
</template>

<script setup lang="ts">
import { computed } from 'vue'

const switchStyle = computed(() => ({
  '--n-width': '42px',
  '--n-height': '26px',
  '--n-border-radius': '13px',
  '--n-button-border-radius': '50%',
  'marginRight': '1px',
}))
</script>
