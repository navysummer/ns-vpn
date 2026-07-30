<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";

interface MenuItem {
  label: string;
  icon?: any;
  danger?: boolean;
  divider?: boolean;
}

const props = defineProps<{
  items: MenuItem[];
  x: number;
  y: number;
}>();

const emit = defineEmits<{
  (e: "select", index: number): void;
  (e: "close"): void;
}>();

const menuRef = ref<HTMLDivElement | null>(null);

function onClickOutside(e: MouseEvent) {
  if (menuRef.value && !menuRef.value.contains(e.target as Node)) {
    emit("close");
  }
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") emit("close");
}

onMounted(() => {
  document.addEventListener("mousedown", onClickOutside);
  document.addEventListener("keydown", onKeydown);
  if (menuRef.value) {
    const rect = menuRef.value.getBoundingClientRect();
    const vw = window.innerWidth;
    const vh = window.innerHeight;
    if (props.x + rect.width > vw) {
      menuRef.value.style.left = `${props.x - rect.width}px`;
    }
    if (props.y + rect.height > vh) {
      menuRef.value.style.top = `${props.y - rect.height}px`;
    }
  }
});

onUnmounted(() => {
  document.removeEventListener("mousedown", onClickOutside);
  document.removeEventListener("keydown", onKeydown);
});
</script>

<template>
  <Teleport to="body">
    <div
      ref="menuRef"
      class="context-menu"
      :style="{ left: `${x}px`, top: `${y}px` }"
    >
      <template v-for="(item, i) in items" :key="i">
        <div v-if="item.divider" class="context-menu-divider" />
        <div
          v-else
          class="context-menu-item"
          :class="{ danger: item.danger }"
          @click="emit('select', i)"
        >
          <span>{{ item.label }}</span>
        </div>
      </template>
    </div>
  </Teleport>
</template>

<style scoped>
.context-menu {
  position: fixed;
  z-index: 9999;
  min-width: 160px;
  padding: 4px 0;
  border-radius: 8px;
  border: 1px solid var(--border);
  background-color: var(--bg-secondary);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  animation: context-menu-in 0.1s ease-out;
}

@keyframes context-menu-in {
  from { opacity: 0; transform: scale(0.95); }
  to { opacity: 1; transform: scale(1); }
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  font-size: 13px;
  color: var(--text-primary);
  cursor: pointer;
  transition: background-color 0.1s;
}

.context-menu-item:hover {
  background-color: var(--bg-hover);
}

.context-menu-item.danger {
  color: var(--red);
}

.context-menu-item.danger:hover {
  background-color: rgba(255, 69, 58, 0.08);
}

.context-menu-divider {
  height: 1px;
  margin: 4px 0;
  background-color: var(--border);
}
</style>
