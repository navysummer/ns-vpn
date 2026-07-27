<template>
  <fieldset
    :style="{
      position: 'relative',
      border: '1px solid #bbb',
      borderRadius: '5px',
      width: width ?? 'auto',
      padding: fieldsetPadding,
    }"
  >
    <legend
      :style="{
        position: 'absolute',
        top: '-10px',
        left: fieldsetPadding,
        backgroundColor: 'var(--background-color)',
        color: 'var(--primary-text)',
        fontSize: fontSize ?? '1em',
        padding: '0 4px',
      }"
    >
      {{ label }}
    </legend>
    <slot />
  </fieldset>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  label: string
  fontSize?: string
  width?: string
  padding?: string
}

const props = defineProps<Props>()
const fieldsetPadding = computed(() => props.padding ?? '15px')
</script>
