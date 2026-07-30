<script setup lang="ts">
import { useToastContainer } from "@/utils/toast"

const { toasts } = useToastContainer()
</script>

<template>
  <div class="toast-container">
    <div
      v-for="toast in toasts"
      :key="toast.id"
      class="toast"
      :class="{
        'toast-success': toast.type === 'success',
        'toast-error': toast.type === 'error',
        'toast-info': toast.type === 'info',
      }"
    >
      {{ toast.message }}
    </div>
  </div>
</template>