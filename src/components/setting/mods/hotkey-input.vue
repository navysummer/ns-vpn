<script setup lang="ts">
import { ref } from 'vue'
import i18n from 'i18next'

import { parseHotkey } from '@/utils/parse-hotkey'

const props = defineProps<{
  value: string[]
  onChange: (value: string[]) => void
}>()

const keys = ref<string[]>(props.value)
const changeRef = ref<string[]>([])

const handleKeyDown = (e: KeyboardEvent) => {
  e.preventDefault()
  e.stopPropagation()
  const key = parseHotkey(e)
  if (key === 'UNIDENTIFIED') return
  changeRef.value = [...new Set([...changeRef.value, key])]
  keys.value = [...changeRef.value]
}

const handleKeyUp = () => {
  const ret = changeRef.value.slice()
  if (ret.length) {
    props.onChange(ret)
    changeRef.value = []
  }
}
</script>

<template>
  <div style="display: flex; align-items: center;">
    <div class="hotkey-wrapper" style="position: relative; width: 230px; min-height: 36px;">
      <input
        style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; z-index: 1; opacity: 0;"
        @keydown="handleKeyDown"
        @keyup="handleKeyUp"
      />
      <div class="list" style="display: flex; align-items: center; flex-wrap: wrap; width: 100%; min-height: 36px; box-sizing: border-box; padding: 3px 4px; border: 1px solid rgba(0,0,0,0.15); border-radius: 4px;">
        <div v-for="(key, index) in keys" :key="key" style="display: flex;">
          <span class="delimiter" :hidden="index === 0" style="line-height: 25px; padding: 0 2px;">+</span>
          <div class="item" style="font-size: 14px; border: 1px solid rgba(0,0,0,0.2); border-radius: 2px; padding: 1px 5px; margin: 2px 0;">{{ key }}</div>
        </div>
      </div>
    </div>

    <button
      class="MuiIconButton-root MuiIconButton-sizeSmall"
      :title="i18n.t('shared.actions.delete')"
      color="inherit"
      @click="props.onChange([]); keys = []"
    >
      <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
    </button>
  </div>
</template>
