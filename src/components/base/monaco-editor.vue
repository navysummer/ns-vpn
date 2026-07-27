<template>
  <div ref="containerRef" :style="{ width: '100%', height }"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, shallowRef } from 'vue'
import type { editor } from 'monaco-editor'
import { configureMonaco } from '@/services/monaco'
import type { MonacoEditorInstance, MonacoMarker } from '@/types/monaco'

const props = defineProps<{
  height?: string
  path?: string
  value?: string
  language?: string
  theme?: string
  loading?: any
  saveViewState?: boolean
  keepCurrentModel?: boolean
  options?: editor.IStandaloneEditorConstructionOptions
}>()

const emit = defineEmits<{
  mount: [editor: MonacoEditorInstance]
  change: [value: string | undefined]
  validate: [markers: MonacoMarker[]]
}>()

const containerRef = ref<HTMLDivElement>()
let editorInstance: MonacoEditorInstance | null = null
let model: editor.ITextModel | null = null

const createEditor = async () => {
  await configureMonaco()
  const monaco = await import('monaco-editor')

  if (!containerRef.value) return

  const uri = props.path ? monaco.Uri.parse(props.path) : undefined
  if (uri) {
    model = monaco.editor.getModel(uri) ?? null
  }

  if (!model) {
    model = monaco.editor.createModel(props.value ?? '', props.language, uri)
  }

  editorInstance = monaco.editor.create(containerRef.value, {
    model,
    language: props.language,
    theme: props.theme,
    automaticLayout: true,
    ...props.options,
  })

  emit('mount', editorInstance)

  editorInstance.onDidChangeModelContent(() => {
    const val = editorInstance?.getValue()
    emit('change', val)
  })

  const markers = monaco.editor.getModelMarkers({})
  const modelMarkers = markers.filter((m) => m.owner === model?.id)
  if (modelMarkers.length > 0) {
    emit('validate', modelMarkers as MonacoMarker[])
  }
}

watch(() => props.value, (newVal) => {
  if (editorInstance && model && newVal !== undefined && newVal !== model.getValue()) {
    model.setValue(newVal)
  }
})

watch(() => props.language, (lang) => {
  if (model && lang) {
    monaco.editor.setModelLanguage(model, lang)
  }
})

watch(() => props.theme, (theme) => {
  if (theme) {
    import('monaco-editor').then((monaco) => monaco.editor.setTheme(theme))
  }
})

onMounted(() => { createEditor() })

onUnmounted(() => {
  if (editorInstance) {
    editorInstance.dispose()
    editorInstance = null
  }
})
</script>
