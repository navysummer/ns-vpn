<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { RefreshCw } from "lucide-vue-next";

const { t } = useI18n();

interface Props {
  name: string;
  type: "proxy" | "rule";
  count?: number;
  loading?: boolean;
}

defineProps<Props>();

defineEmits<{
  (e: "refresh"): void;
}>();
</script>

<template>
  <button class="provider-btn" @click="$emit('refresh')">
    <div class="provider-info">
      <span class="provider-type" :class="type">{{ type === "proxy" ? t('provider.proxy') : t('provider.rule') }}{{ t('provider.provider') }}</span>
      <span class="provider-name">{{ name }}</span>
      <span v-if="count !== undefined" class="provider-count">{{ count }} {{ t('provider.entries') }}</span>
    </div>
    <RefreshCw :size="12" :class="{ spin: loading }" class="provider-refresh" />
  </button>
</template>

<style scoped>
.provider-btn {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 8px 12px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
  cursor: pointer;
  transition: border-color 150ms ease;
}
.provider-btn:hover {
  border-color: var(--accent);
}

.provider-info {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.provider-type {
  font-size: 10px;
  font-weight: 600;
  padding: 1px 6px;
  border-radius: 4px;
}
.provider-type.proxy {
  color: var(--accent);
  background-color: rgba(79,142,247,0.12);
}
.provider-type.rule {
  color: var(--orange);
  background-color: rgba(255,159,10,0.12);
}

.provider-name {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
}

.provider-count {
  font-size: 11px;
  color: var(--text-secondary);
}

.provider-refresh {
  color: var(--text-secondary);
  flex-shrink: 0;
}
</style>
