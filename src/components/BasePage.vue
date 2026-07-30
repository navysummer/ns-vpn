<script setup lang="ts">
interface Props {
  title?: string;
  showHeader?: boolean;
}

withDefaults(defineProps<Props>(), {
  title: "",
  showHeader: true,
});
</script>

<template>
  <div class="base-page">
    <div v-if="showHeader && (title || $slots.header)" class="page-header">
      <div class="page-header-left">
        <h1 v-if="title" class="page-title">{{ title }}</h1>
        <slot name="header" />
      </div>
      <div v-if="$slots.actions" class="page-header-right">
        <slot name="actions" />
      </div>
    </div>
    <div class="page-content">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.base-page {
  display: flex;
  flex-direction: column;
  gap: 24px;
  height: 100%;
}

.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
}

.page-header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.page-header-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.page-title {
  font-size: 22px;
  font-weight: 600;
  line-height: 1.2;
  margin: 0;
}

.page-content {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
</style>