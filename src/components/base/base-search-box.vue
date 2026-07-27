<template>
  <n-tooltip :disabled="!effectiveErrorMessage" placement="bottom-start">
    <template #trigger>
      <n-input
        :value="text"
        size="small"
        clearable
        :placeholder="placeholder ?? t('shared.placeholders.filter')"
        :status="effectiveErrorMessage ? 'error' : undefined"
        :style="{ '--n-padding': '4px 10px' }"
        @input="handleChangeText"
        @click="onClick"
        @clear="handleClearInput"
      >
        <template #suffix>
          <div style="display: flex; align-items: center; gap: 2px">
            <n-tooltip v-if="text" :title="t('shared.placeholders.resetInput')" placement="bottom">
              <template #trigger>
                <n-button quaternary circle size="tiny" @click="handleClearInput" style="height: 24px; width: 24px">
                  <template #icon>
                    <Close />
                  </template>
                </n-button>
              </template>
            </n-tooltip>
            <n-tooltip :title="t('shared.placeholders.matchCase')" placement="bottom">
              <template #trigger>
                <span
                  :aria-label="matchCase ? 'active' : 'inactive'"
                  style="cursor: pointer; height: 24px; width: 24px; display: flex; align-items: center; justify-content: center"
                  :class="{ active: matchCase }"
                  @click="handleToggleMatchCase"
                >
                  <MatchCaseIcon v-if="matchCaseIconStr" />
                </span>
              </template>
            </n-tooltip>
            <n-tooltip :title="t('shared.placeholders.matchWholeWord')" placement="bottom">
              <template #trigger>
                <span
                  :aria-label="matchWholeWord ? 'active' : 'inactive'"
                  style="cursor: pointer; height: 24px; width: 24px; display: flex; align-items: center; justify-content: center"
                  :class="{ active: matchWholeWord }"
                  @click="handleToggleMatchWholeWord"
                >
                  <MatchWholeWordIcon v-if="matchWholeWordIconStr" />
                </span>
              </template>
            </n-tooltip>
            <n-tooltip :title="t('shared.placeholders.useRegex')" placement="bottom">
              <template #trigger>
                <span
                  :aria-label="useRegularExpression ? 'active' : 'inactive'"
                  style="cursor: pointer; height: 24px; width: 24px; display: flex; align-items: center; justify-content: center"
                  :class="{ active: useRegularExpression }"
                  @click="handleToggleUseRegularExpression"
                >
                  <UseRegexIcon v-if="useRegexIconStr" />
                </span>
              </template>
            </n-tooltip>
          </div>
        </template>
      </n-input>
    </template>
    {{ effectiveErrorMessage }}
  </n-tooltip>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { Close } from '@vicons/ionicons5'
import { useTranslation } from '@/composables/use-i18n'
const { t } = useTranslation()
import { buildRegex, compileStringMatcher } from '@/utils/search-matcher'

export type SearchState = {
  text: string
  matchCase: boolean
  matchWholeWord: boolean
  useRegularExpression: boolean
}

type SearchOptionState = Omit<SearchState, 'text'>

interface SearchProps {
  value?: string
  defaultValue?: string
  autoFocus?: boolean
  placeholder?: string
  matchCase?: boolean
  matchWholeWord?: boolean
  useRegularExpression?: boolean
  searchState?: Partial<SearchOptionState>
  onSearch: (match: (content: string) => boolean, state: SearchState) => void
  onClick?: (e: MouseEvent) => void
}

const props = defineProps<SearchProps>()

const text = ref(props.value ?? props.defaultValue ?? '')
const matchCase = ref(props.searchState?.matchCase ?? props.matchCase ?? false)
const matchWholeWord = ref(props.searchState?.matchWholeWord ?? props.matchWholeWord ?? false)
const useRegularExpression = ref(props.searchState?.useRegularExpression ?? props.useRegularExpression ?? false)

const matchCaseIconStr = ref('')
const matchWholeWordIconStr = ref('')
const useRegexIconStr = ref('')

onMounted(async () => {
  try {
    const [mc, mww, ur] = await Promise.all([
      import('@/assets/image/component/match_case.svg?raw').then(m => m.default || m),
      import('@/assets/image/component/match_whole_word.svg?raw').then(m => m.default || m),
      import('@/assets/image/component/use_regular_expression.svg?raw').then(m => m.default || m),
    ])
    matchCaseIconStr.value = mc
    matchWholeWordIconStr.value = mww
    useRegexIconStr.value = ur
  } catch {
    // icons not critical
  }
})

const lastSearchState = ref<SearchState | null>({
  text: props.value ?? props.defaultValue ?? '',
  matchCase: props.searchState?.matchCase ?? props.matchCase ?? false,
  matchWholeWord: props.searchState?.matchWholeWord ?? props.matchWholeWord ?? false,
  useRegularExpression: props.searchState?.useRegularExpression ?? props.useRegularExpression ?? false,
})

const effectiveErrorMessage = computed(() => {
  if (!useRegularExpression.value || !text.value) return ''
  const flags = matchCase.value ? '' : 'i'
  return buildRegex(text.value, flags) ? '' : t('shared.validation.invalidRegex')
})

const emitSearch = (nextState: SearchState) => {
  const prevState = lastSearchState.value
  const isSameState =
    !!prevState &&
    prevState.text === nextState.text &&
    prevState.matchCase === nextState.matchCase &&
    prevState.matchWholeWord === nextState.matchWholeWord &&
    prevState.useRegularExpression === nextState.useRegularExpression
  if (isSameState) return

  const compiled = compileStringMatcher(nextState.text, nextState)
  props.onSearch(compiled.matcher, nextState)
  lastSearchState.value = nextState
}

const handleChangeText = (val: string) => {
  text.value = val
  emitSearch({
    text: val,
    matchCase: matchCase.value,
    matchWholeWord: matchWholeWord.value,
    useRegularExpression: useRegularExpression.value,
  })
}

const handleClearInput = () => {
  text.value = ''
  emitSearch({ text: '', matchCase: matchCase.value, matchWholeWord: matchWholeWord.value, useRegularExpression: useRegularExpression.value })
}

const handleToggleMatchCase = () => {
  matchCase.value = !matchCase.value
  emitSearch({ text: text.value, matchCase: matchCase.value, matchWholeWord: matchWholeWord.value, useRegularExpression: useRegularExpression.value })
}

const handleToggleMatchWholeWord = () => {
  matchWholeWord.value = !matchWholeWord.value
  emitSearch({ text: text.value, matchCase: matchCase.value, matchWholeWord: matchWholeWord.value, useRegularExpression: useRegularExpression.value })
}

const handleToggleUseRegularExpression = () => {
  useRegularExpression.value = !useRegularExpression.value
  emitSearch({ text: text.value, matchCase: matchCase.value, matchWholeWord: matchWholeWord.value, useRegularExpression: useRegularExpression.value })
}

watch(() => props.value, (newVal) => {
  if (newVal !== undefined) text.value = newVal
})
</script>

<style scoped>
.active svg path {
  fill: var(--primary-main) !important;
}

svg path {
  fill: #A7A7A7;
}
</style>
