<script setup lang="ts">
import { useI18n } from "vue-i18n";
import type { Component } from "vue";

const { t } = useI18n();

interface Props {
  icon?: Component;
  title?: string;
  description?: string;
}

withDefaults(defineProps<Props>(), {
  title: "",
  description: "",
});
</script>

<template>
  <div class="empty-state">
    <component v-if="icon" :is="icon" :size="48" class="empty-icon" />
    <div class="empty-title">{{ title || t('common.noData') }}</div>
    <div v-if="description" class="empty-desc">{{ description }}</div>
    <slot />
  </div>
</template>

<style scoped>
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 48px 24px;
  text-align: center;
}

.empty-icon {
  color: var(--text-secondary);
  opacity: 0.3;
  margin-bottom: 12px;
}

.empty-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.empty-desc {
  font-size: 12px;
  color: var(--text-secondary);
  opacity: 0.6;
}
</style>
