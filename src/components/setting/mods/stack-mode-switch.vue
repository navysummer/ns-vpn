<template>
  <n-button-group size="small">
    <n-button
      :type="value?.toLowerCase() === 'system' ? 'primary' : 'default'"
      @click="emit('change', 'system')"
    >
      System
    </n-button>
    <n-button
      :type="value?.toLowerCase() === 'gvisor' ? 'primary' : 'default'"
      @click="emit('change', 'gvisor')"
    >
      gVisor
    </n-button>
    <n-button
      :type="value?.toLowerCase() === 'mixed' ? 'primary' : 'default'"
      @click="emit('change', 'mixed')"
    >
      Mixed
    </n-button>
  </n-button-group>
</template>

<script setup lang="ts">
defineProps<{
  value?: string
}>()
const emit = defineEmits<{
  change: [value: string]
}>()
</script>
