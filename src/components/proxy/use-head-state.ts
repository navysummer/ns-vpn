import { ref, watch, onMounted, onUnmounted } from 'vue'

import { useProfiles } from '@/hooks/use-profiles'

import { ProxySortType } from './use-filter-sort'

export interface HeadState {
  open?: boolean
  showType: boolean
  sortType: ProxySortType
  filterText: string
  filterMatchCase?: boolean
  filterMatchWholeWord?: boolean
  filterUseRegularExpression?: boolean
  textState: 'url' | 'filter' | null
  testUrl: string
}

type HeadStateStorage = Record<string, Record<string, HeadState>>

const HEAD_STATE_KEY = 'proxy-head-state'
export const DEFAULT_STATE: HeadState = {
  open: false,
  showType: true,
  sortType: 0,
  filterText: '',
  filterMatchCase: false,
  filterMatchWholeWord: false,
  filterUseRegularExpression: false,
  textState: null,
  testUrl: '',
}

export function useHeadStateNew() {
  const { profiles } = useProfiles()
  const current = profiles?.current || ''
  const state = ref<Record<string, HeadState>>({})

  const loadFromStorage = () => {
    try {
      const data = JSON.parse(
        localStorage.getItem(HEAD_STATE_KEY)!,
      ) as HeadStateStorage
      const value = data[current] || {}
      if (value && typeof value === 'object') {
        state.value = value
      } else {
        state.value = {}
      }
    } catch {
      state.value = {}
    }
  }

  watch(() => current, () => {
    loadFromStorage()
  })

  onMounted(() => {
    loadFromStorage()
  })

  const saveTimer = ref<ReturnType<typeof setTimeout> | null>(null)

  watch(state, () => {
    if (saveTimer.value) clearTimeout(saveTimer.value)
    saveTimer.value = setTimeout(() => {
      try {
        const item = localStorage.getItem(HEAD_STATE_KEY)
        let data = (item ? JSON.parse(item) : {}) as HeadStateStorage
        if (!data || typeof data !== 'object') data = {}
        data[current] = state.value
        localStorage.setItem(HEAD_STATE_KEY, JSON.stringify(data))
      } catch {}
    })
  }, { deep: true })

  onUnmounted(() => {
    if (saveTimer.value) clearTimeout(saveTimer.value)
  })

  const setHeadState = (groupName: string, obj: Partial<HeadState>) => {
    const prev = state.value[groupName] || DEFAULT_STATE
    state.value = { ...state.value, [groupName]: { ...prev, ...obj } }
  }

  return [state, setHeadState] as const
}
