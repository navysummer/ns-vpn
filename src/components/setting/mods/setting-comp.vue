<template>
  <div v-if="isList" class="setting-list">
    <div class="setting-list-title">{{ title }}</div>
    <div class="setting-list-content"><slot /></div>
  </div>
  <div v-else class="setting-item">
    <div v-if="clickable" class="setting-item-row" @click="handleClick">
      <div class="setting-item-label">
        <span>{{ label }}</span>
        <span v-if="extra" class="setting-item-extra"><slot name="extra" /></span>
      </div>
      <div v-if="secondary" class="setting-item-secondary">{{ secondary }}</div>
      <n-spin v-if="isLoading" :size="16" />
      <n-icon v-else size="18"><ChevronForwardOutline /></n-icon>
    </div>
    <div v-else class="setting-item-row">
      <div class="setting-item-label">
        <span>{{ label }}</span>
      </div>
      <div v-if="secondary" class="setting-item-secondary">{{ secondary }}</div>
      <div class="setting-item-children"><slot /></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ChevronForwardOutline } from '@vicons/ionicons5'
import isAsyncFunction from '@/utils/is-async-function'

const props = defineProps<{
  title?: string
  label?: string
  extra?: boolean
  secondary?: string
  onClick?: () => void | Promise<any>
}>()

const isList = !!props.title
const clickable = !!props.onClick
const isLoading = ref(false)

const handleClick = async () => {
  if (props.onClick) {
    if (isAsyncFunction(props.onClick)) {
      isLoading.value = true
      try {
        await props.onClick()
      } finally {
        isLoading.value = false
      }
    } else {
      props.onClick()
    }
  }
}
</script>

<style scoped>
.setting-list {
  margin-bottom: 16px;
}
.setting-list-title {
  font-size: 16px;
  font-weight: 700;
  padding: 8px 0;
  color: var(--primary-text);
}
.setting-list-content {
  display: flex;
  flex-direction: column;
}
.setting-item {
  padding: 5px 0;
}
.setting-item-row {
  display: flex;
  align-items: center;
  min-height: 40px;
  cursor: pointer;
  gap: 8px;
}
.setting-item-label {
  display: flex;
  align-items: center;
  font-size: 14px;
  gap: 4px;
  flex: 1;
}
.setting-item-secondary {
  font-size: 12px;
  color: #888;
}
.setting-item-children {
  display: flex;
  align-items: center;
}
</style>
