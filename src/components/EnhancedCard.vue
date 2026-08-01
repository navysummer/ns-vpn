<script setup lang="ts">
import type { Component } from "vue";

interface Props {
  title?: string;
  icon?: Component;
  iconColor?: string;
  noPadding?: boolean;
}

withDefaults(defineProps<Props>(), {
  title: "",
  iconColor: "var(--accent)",
});
</script>

<template>
  <div class="enhanced-card">
    <div class="ec-header">
      <div class="ec-header-left">
        <div v-if="$slots.icon || icon" class="ec-icon" :style="{ backgroundColor: `color-mix(in srgb, ${iconColor} 12%, transparent)` }">
          <slot name="icon">
            <component v-if="icon" :is="icon" :size="18" :style="{ color: iconColor }" />
          </slot>
        </div>
        <div class="ec-title-wrap">
          <slot name="title">
            <span v-if="title" class="ec-title">{{ title }}</span>
          </slot>
        </div>
      </div>
      <div v-if="$slots.action" class="ec-header-right">
        <slot name="action" />
      </div>
    </div>
    <div class="ec-body" :class="{ 'ec-body-np': noPadding }">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.enhanced-card {
  display: flex;
  flex-direction: column;
  height: 100%;
  border-radius: 12px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
}

.ec-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
  min-height: 48px;
}

.ec-header-left {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
  flex: 1;
}

.ec-icon {
  width: 34px;
  height: 34px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.ec-title-wrap {
  min-width: 0;
  flex: 1;
}

.ec-title {
  font-size: 14px;
  font-weight: 600;
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.ec-header-right {
  margin-left: 12px;
  flex-shrink: 0;
}

.ec-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 16px;
}

.ec-body-np {
  padding: 0;
}
</style>