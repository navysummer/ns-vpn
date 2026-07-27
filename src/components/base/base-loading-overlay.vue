<template>
  <div v-if="isLoading" class="loading-overlay">
    <n-spin size="large" />
  </div>
</template>

<script setup lang="ts">
interface Props {
  isLoading: boolean
}

defineProps<Props>()
</script>

<style scoped>
.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: 1000;
}
</style>
