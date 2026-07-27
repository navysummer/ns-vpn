<template>
  <span>{{ displayText }}</span>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'

dayjs.extend(relativeTime)

const props = defineProps<{
  start: string
}>()

const currentTime = ref(Date.now())
let timerId: number | null = null

const displayText = ref('')

const updateTime = () => {
  currentTime.value = Date.now()
  displayText.value = dayjs(props.start).from(currentTime.value)
}

onMounted(() => {
  updateTime()
  timerId = window.setInterval(updateTime, 5000)
})

onUnmounted(() => {
  if (timerId !== null) {
    clearInterval(timerId)
  }
})
</script>
