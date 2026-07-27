<template>
  <n-select
    v-bind="$attrs"
    size="small"
    :style="{
      width: '120px',
    }"
    :class="{ 'light-bg': isLight }"
  />
</template>

<script setup lang="ts">
import { computed } from 'vue'

const isLight = computed(() => document.documentElement.getAttribute('theme') === 'light')
</script>

<style scoped>
.light-bg {
  --n-color: #fff;
}
</style>
