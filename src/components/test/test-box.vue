<template>
  <div
    class="test-box"
    :class="{ selected: ariaSelected === 'true' }"
    :aria-selected="ariaSelected"
    @contextmenu.prevent="onContextmenu"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
defineProps<{
  ariaSelected?: string
  onContextmenu?: (e: MouseEvent) => void
}>()
</script>

<style scoped>
.test-box {
  position: relative;
  width: 100%;
  display: block;
  cursor: pointer;
  text-align: left;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12);
  padding: 8px 16px;
  box-sizing: border-box;
  transition: background-color 0.3s, box-shadow 0.3s;
  background-color: color-mix(in srgb, var(--primary-main) 8%, var(--bg-color));
  color: var(--secondary-text);
}

.test-box h2 {
  color: var(--text-primary);
}

.test-box.selected {
  background-color: color-mix(in srgb, var(--primary-main) 15%, var(--bg-color));
}

.test-box.selected h2 {
  color: var(--primary-main);
}

.test-box:hover {
  background-color: color-mix(in srgb, var(--primary-main) 15%, var(--bg-color));
  box-shadow: 0 3px 6px rgba(0, 0, 0, 0.16);
}
</style>
