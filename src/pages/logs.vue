<template>
  <BasePage
    full
    :content-style="{
      height: '100%',
      display: 'flex',
      flexDirection: 'column',
      overflow: 'auto',
    }"
  >
    <template #title>
      {{ t('logs.page.title') }}
    </template>
    <template #header>
      <div style="display: flex; align-items: center; gap: 16px">
        <n-button
          quaternary
          circle
          size="small"
          :title="t(enableLog ? 'shared.actions.pause' : 'shared.actions.resume')"
          :aria-label="t(enableLog ? 'shared.actions.pause' : 'shared.actions.resume')"
          @click="handleToggleLog"
        >
          <template #icon>
            <svg
              v-if="enableLog"
              viewBox="0 0 24 24"
              width="20"
              height="20"
              fill="currentColor"
            >
              <path
                d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 14h-2V8h2v8zm4 0h-2V8h2v8z"
              />
            </svg>
            <svg
              v-else
              viewBox="0 0 24 24"
              width="20"
              height="20"
              fill="currentColor"
            >
              <path
                d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 14.5v-9l6 4.5-6 4.5z"
              />
            </svg>
          </template>
        </n-button>
        <n-button
          quaternary
          circle
          size="small"
          :title="t(isDescending ? 'logs.actions.showAscending' : 'logs.actions.showDescending')"
          :aria-label="t(isDescending ? 'logs.actions.showAscending' : 'logs.actions.showDescending')"
          @click="handleToggleOrder"
        >
          <template #icon>
            <svg
              viewBox="0 0 24 24"
              width="20"
              height="20"
              fill="currentColor"
              :style="{ transform: isDescending ? 'scaleY(-1)' : 'none', transition: 'transform 0.2s ease' }"
            >
              <path
                d="M16 17.01V10h-2v7.01h-3L15 21l4-3.99h-3zM9 3L5 6.99h3V14h2V6.99h3L9 3z"
              />
            </svg>
          </template>
        </n-button>

        <n-button size="small" type="primary" @click="handleClear">
          {{ t('shared.actions.clear') }}
        </n-button>
      </div>
    </template>

    <div
      style="
        padding-top: 8px;
        margin-bottom: 4px;
        margin-left: 10px;
        margin-right: 10px;
        height: 39px;
        display: flex;
        align-items: center;
      "
    >
      <n-select
        :value="logState"
        :options="logLevelOptions"
        size="small"
        style="width: 120px"
        @update:value="handleLogLevelChange"
      />
      <BaseSearchBox
        :on-search="
          (matcher: any, state: any) => {
            matchFunc = matcher
            searchState = state
          }
        "
      />
    </div>

    <template v-if="filteredLogs.length > 0">
      <VirtualList
        ref="virtuosoRef"
        :count="filteredLogs.length"
        :estimate-size="50"
        :style="{ flex: 1 }"
        :on-scroll="handleScroll"
      >
        <template #item="{ index }">
          <LogItem :value="filteredLogs[index]" :search-state="searchState" />
        </template>
      </VirtualList>
    </template>
    <BaseEmpty v-else />
  </BasePage>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useTranslation } from '@/composables/use-i18n'

import {
  BaseEmpty,
  BasePage,
  BaseSearchBox,
  VirtualList,
} from '@/components/base'
import LogItem from '@/components/log/log-item.vue'
import { useClashLog } from '@/hooks/use-clash-log'
import { useLogData } from '@/hooks/use-log-data'

const { t } = useTranslation()
const [clashLog, setClashLog] = useClashLog()
const enableLog = clashLog.enable
const logState = clashLog.logFilter
const logOrder = clashLog.logOrder ?? 'asc'
const isDescending = logOrder === 'desc'

const matchFunc = ref((_: string) => true)
const searchState = ref<any>(undefined)
const {
  response: { data: logData },
  refreshGetClashLog,
} = useLogData()

const filterLogs = computed(() => {
  if (!logData || logData.length === 0) {
    return []
  }

  return logData.filter((data: any) => {
    const searchText =
      `${data.time || ''} ${data.type} ${data.payload}`.toLowerCase()

    const matchesSearch = matchFunc.value(searchText)

    return (
      (logState == 'all' ? true : data.type.includes(logState)) &&
      matchesSearch
    )
  })
})

const filteredLogs = computed(() =>
  isDescending ? [...filterLogs.value].reverse() : filterLogs.value,
)

const logLevelOptions = computed(() => [
  { label: t('shared.filters.logLevels.all'), value: 'all' },
  { label: t('shared.filters.logLevels.debug'), value: 'debug' },
  { label: t('shared.filters.logLevels.info'), value: 'info' },
  { label: t('shared.filters.logLevels.warn'), value: 'warn' },
  { label: t('shared.filters.logLevels.error'), value: 'err' },
])

const scrollRef = ref({ isNearBottom: true })
const virtuosoRef = ref<InstanceType<typeof VirtualList> | null>(null)

watch(
  () => filteredLogs.value.length,
  (newLen, oldLen) => {
    if (!isDescending && scrollRef.value.isNearBottom) {
      virtuosoRef.value?.scrollToIndex(newLen - 1, {
        behavior: 'smooth' as ScrollBehavior,
      })
    }
  },
)

const handleLogLevelChange = (newLevel: LogFilter) => {
  setClashLog((pre: any) => ({ ...pre, logFilter: newLevel }))
}

const handleToggleLog = async () => {
  setClashLog((pre: any) => ({ ...pre, enable: !enableLog }))
}

const handleToggleOrder = () => {
  setClashLog((pre: any) => ({
    ...pre,
    logOrder: pre.logOrder === 'desc' ? 'asc' : 'desc',
  }))
}

const handleClear = () => {
  refreshGetClashLog(true)
}

const handleScroll = (event: Event) => {
  const element = event.currentTarget as HTMLDivElement
  scrollRef.value.isNearBottom =
    element.scrollHeight - element.scrollTop - element.clientHeight <= 20
}
</script>
