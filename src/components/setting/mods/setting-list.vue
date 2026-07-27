<template>
  <ul :style="{ listStyle: 'none', padding: '8px 0' }">
    <li :style="{ background: 'transparent', fontSize: '16px', fontWeight: 700, padding: '8px 16px', listStyle: 'none' }">
      <slot name="title">{{ title }}</slot>
    </li>
    <slot />
  </ul>
</template>

<script setup lang="ts">
defineProps<{
  title: string
}>()
</script>
