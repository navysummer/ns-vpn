<template>
  <div style="margin-top: 16px; margin-bottom: 8px; display: flex; align-items: center;">
    <button
      class="MuiButton-root MuiButton-outlined MuiButton-outlinedPrimary MuiButton-sizeSmall MuiButton-outlinedSizeSmall MuiButtonBase-root"
      style="flex: none;"
      @click="inputRef?.click()"
    >
      {{ t('profiles.components.fileInput.chooseFile') }}
    </button>
    <input
      ref="inputRef"
      type="file"
      accept=".yaml,.yml"
      style="display: none;"
      @change="onFileInput"
    />
    <span class="MuiTypography-root MuiTypography-noWrap" style="margin-left: 8px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
      {{ loading ? t('shared.statuses.loading') : fileName }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const props = defineProps<{
  onChange: (file: File, value: string) => void
}>()

const { t } = useI18n()
const inputRef = ref<HTMLInputElement | null>(null)
const loading = ref(false)
const fileName = ref('')

let locked = false
const onFileInput = async (e: Event) => {
  if (locked) return
  locked = true
  const target = e.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) { locked = false; return }
  fileName.value = file.name
  loading.value = true
  try {
    const text = await file.text()
    props.onChange(file, text)
  } finally {
    loading.value = false
    locked = false
  }
}
</script>
