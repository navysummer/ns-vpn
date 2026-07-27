import { ref, onMounted, onUnmounted } from 'vue'

export const useWindowWidth = () => {
  const width = ref(document.body.clientWidth)

  onMounted(() => {
    const handleResize = () => { width.value = document.body.clientWidth }
    window.addEventListener('resize', handleResize)
    onUnmounted(() => {
      window.removeEventListener('resize', handleResize)
    })
  })

  return { width }
}
