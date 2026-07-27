import { ref, onMounted } from 'vue'
import { showNotice } from '@/services/notice-service'

interface UseEditorDocumentOptions {
  open: boolean
  load: () => Promise<string>
}

export const useEditorDocument = ({ open, load }: UseEditorDocumentOptions) => {
  const value = ref('')
  const savedValue = ref('')
  const loading = ref(true)

  const resetDocumentState = () => {
    value.value = ''
    savedValue.value = ''
    loading.value = true
  }

  const applyLoadedValue = (nextValue: string | null | undefined) => {
    const normalized = nextValue ?? ''
    value.value = normalized
    savedValue.value = normalized
    return normalized
  }

  const reload = async () => {
    loading.value = true
    try {
      return applyLoadedValue(await load())
    } catch (error) {
      showNotice.error(error)
      throw error
    } finally {
      loading.value = false
    }
  }

  onMounted(() => {
    resetDocumentState()

    if (!open) return

    load()
      .then((nextValue) => {
        applyLoadedValue(nextValue)
      })
      .catch((error) => {
        showNotice.error(error)
      })
      .finally(() => {
        loading.value = false
      })
  })

  const markSaved = (nextValue: string) => {
    savedValue.value = nextValue
  }

  const dirty = () => value.value !== savedValue.value

  const setValue = (v: string) => { value.value = v }

  return {
    value,
    setValue,
    savedValue,
    loading,
    dirty,
    markSaved,
    reload,
  }
}
