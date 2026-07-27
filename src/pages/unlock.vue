<template>
  <BasePage
    :title="t('tests.unlock.page.title')"
  >
    <template #header>
      <div style="display: flex; align-items: center; gap: 8px">
        <n-button
          size="small"
          type="primary"
          :disabled="isCheckingAll"
          @click="checkAllMedia"
        >
          <template #icon>
            <span v-if="isCheckingAll" class="loading-spinner">
              <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
                <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm0-14v8l5.25 3.15.75-1.23-4.5-2.67V6H12z"/>
              </svg>
            </span>
            <svg v-else viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
              <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
            </svg>
          </template>
          {{ isCheckingAll ? t('tests.unlock.page.actions.testing') : t('tests.page.actions.testAll') }}
        </n-button>
      </div>
    </template>

    <template v-if="unlockItems.length === 0">
      <div
        style="
          display: flex;
          justify-content: center;
          align-items: center;
          height: 50%;
        "
      >
        <BaseEmpty :text-key="'tests.unlock.page.empty'" />
      </div>
    </template>

    <div
      v-else
      class="unlock-grid"
    >
      <div
        v-for="item in unlockItems"
        :key="item.name"
        class="unlock-card"
        :style="{
          borderLeft: `4px solid ${getStatusBorderColor(item.status)}`,
        }"
      >
        <div style="padding: 10.4px; flex: 1;">
          <div
            style="
              display: flex;
              justify-content: space-between;
              align-items: center;
            "
          >
            <span
              style="
                font-weight: 600;
                font-size: 1rem;
                color: var(--text-primary);
              "
            >
              {{ item.name }}
            </span>
            <n-tooltip :title="t('tests.components.item.actions.test')" placement="top">
              <n-button
                size="small"
                :disabled="loadingItems.includes(item.name) || isCheckingAll"
                style="min-width: 32px; width: 32px; height: 32px; border-radius: 50%;"
                @click="checkSingleMedia(item.name)"
              >
                <template #icon>
                  <svg
                    viewBox="0 0 24 24"
                    width="16"
                    height="16"
                    fill="currentColor"
                    :class="{ spinning: loadingItems.includes(item.name) }"
                  >
                    <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
                  </svg>
                </template>
              </n-button>
            </n-tooltip>
          </div>

          <div
            style="
              display: flex;
              align-items: center;
              flex-wrap: wrap;
              gap: 8px;
              margin-top: 8px;
            "
          >
            <span
              class="status-chip"
              :class="'status-' + getStatusColor(item.status)"
            >
              <svg
                viewBox="0 0 24 24"
                width="14"
                height="14"
                fill="currentColor"
                style="margin-right: 4px; flex-shrink: 0;"
              >
                <path v-if="item.status === 'Pending'" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-5-9h2v2H7v-2zm4 0h2v2h-2v-2zm4 0h2v2h-2v-2z"/>
                <path v-else-if="item.status === 'Yes'" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
                <path v-else-if="item.status === 'No'" d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm3.59-13L12 10.59 8.41 7 7 8.41 10.59 12 7 15.59 8.41 17 12 13.41 15.59 17 17 15.59 13.41 12 17 8.41z"/>
                <path v-else-if="item.status === 'Soon'" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"/>
                <path v-else d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1-4h2v2h-2v-2zm1.61-9.96c-2.06-.3-3.88.97-4.43 2.79-.18.58.26 1.17.87 1.17h.2c.41 0 .74-.29.88-.67.32-.89 1.27-1.5 2.27-1.28.95.2 1.65 1.05 1.65 2.01 0 1.01-.71 1.38-1.55 1.89-.36.22-.67.58-.78 1.04-.13.52-.03 1.05.01 1.12.06.45.45.78.91.78h.18c.46 0 .84-.34.92-.79.18-.99.77-1.36 1.5-1.79.87-.51 1.88-1.37 1.88-3.01 0-1.79-1.32-3.31-3.09-3.75z"/>
              </svg>
              {{ t(STATUS_LABEL_KEYS[item.status] ?? item.status) }}
            </span>

            <span v-if="item.region" class="status-chip region-chip">
              {{ item.region }}
            </span>
          </div>
        </div>

        <div style="border-bottom: 1px dashed rgba(128,128,128,0.2); margin: 0 8px;" />

        <div style="padding: 1.6px 12px;">
          <span
            style="
              display: block;
              color: var(--secondary-text);
              font-size: 0.7rem;
              text-align: right;
            "
          >
            {{ item.check_time || '-- --' }}
          </span>
        </div>
      </div>
    </div>
  </BasePage>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useTranslation } from '@/composables/use-i18n'
import { invoke } from '@tauri-apps/api/core'
import { BaseEmpty, BasePage } from '@/components/base'
import { showNotice } from '@/services/notice-service'

interface UnlockItem {
  name: string
  status: string
  region?: string | null
  check_time?: string | null
}

const UNLOCK_RESULTS_STORAGE_KEY = 'clash_verge_unlock_results'
const UNLOCK_RESULTS_TIME_KEY = 'clash_verge_unlock_time'

const STATUS_LABEL_KEYS: Record<string, string> = {
  Pending: 'tests.statuses.test.pending',
  Yes: 'tests.statuses.test.yes',
  No: 'tests.statuses.test.no',
  Failed: 'tests.statuses.test.failed',
  Completed: 'tests.statuses.test.completed',
  'Disallowed ISP': 'tests.statuses.test.disallowedIsp',
  'Originals Only': 'tests.statuses.test.originalsOnly',
  'No (IP Banned By Disney+)': 'tests.statuses.test.noDisney',
  'Unsupported Country/Region': 'tests.statuses.test.unsupportedRegion',
  'Failed (Network Connection)': 'tests.statuses.test.failedNetwork',
}

const normalizeUnlockName = (name: string) => name.trim().toLowerCase()

const getStatusPriority = (status: string) => (status === 'Pending' ? 0 : 1)

const mergeOptionalFields = (preferred: UnlockItem, fallback: UnlockItem) => ({
  ...preferred,
  region: preferred.region ?? fallback.region,
  check_time: preferred.check_time ?? fallback.check_time,
})

const dedupeUnlockItems = (items: UnlockItem[]) => {
  const map = new Map<string, UnlockItem>()

  items.forEach((item) => {
    const key = normalizeUnlockName(item.name)
    const existing = map.get(key)

    if (!existing) {
      map.set(key, item)
      return
    }

    const existingPriority = getStatusPriority(existing.status)
    const itemPriority = getStatusPriority(item.status)

    if (itemPriority > existingPriority) {
      map.set(key, mergeOptionalFields(item, existing))
      return
    }

    if (itemPriority < existingPriority) {
      map.set(key, mergeOptionalFields(existing, item))
      return
    }

    map.set(key, mergeOptionalFields(item, existing))
  })

  return Array.from(map.values())
}

const { t } = useTranslation()

const unlockItems = ref<UnlockItem[]>([])
const isCheckingAll = ref(false)
const loadingItems = ref<string[]>([])
const checkAllLock = ref(false)
const singleCheckLocks = ref<Record<string, boolean>>({})

const sortItemsByName = (items: UnlockItem[]) => {
  return [...items].sort((a, b) => a.name.localeCompare(b.name))
}

const mergeUnlockItems = (
  defaults: UnlockItem[],
  existing?: UnlockItem[] | null,
) => {
  if (!existing || existing.length === 0) {
    return defaults
  }

  const normalizedExisting = dedupeUnlockItems(existing)
  const existingMap = new Map(
    normalizedExisting.map((item) => [
      normalizeUnlockName(item.name),
      item,
    ]),
  )
  const merged = defaults.map((item) => {
    const normalizedName = normalizeUnlockName(item.name)
    const matchedItem = existingMap.get(normalizedName)
    if (matchedItem) {
      return { ...matchedItem, name: item.name }
    }
    return item
  })

  const mergedNameSet = new Set(
    merged.map((item) => normalizeUnlockName(item.name)),
  )
  normalizedExisting.forEach((item) => {
    const normalizedName = normalizeUnlockName(item.name)
    if (!mergedNameSet.has(normalizedName)) {
      merged.push(item)
      mergedNameSet.add(normalizedName)
    }
  })

  return merged
}

const saveResultsToStorage = (items: UnlockItem[], time: string | null) => {
  try {
    localStorage.setItem(UNLOCK_RESULTS_STORAGE_KEY, JSON.stringify(items))
    if (time) {
      localStorage.setItem(UNLOCK_RESULTS_TIME_KEY, time)
    }
  } catch (err) {
    console.error('Failed to save results to storage:', err)
  }
}

const loadResultsFromStorage = (): {
  items: UnlockItem[] | null
  time: string | null
} => {
  try {
    const itemsJson = localStorage.getItem(UNLOCK_RESULTS_STORAGE_KEY)
    const time = localStorage.getItem(UNLOCK_RESULTS_TIME_KEY)

    if (itemsJson) {
      const parsedItems = JSON.parse(itemsJson) as UnlockItem[]
      return {
        items: dedupeUnlockItems(parsedItems),
        time,
      }
    }
  } catch (err) {
    console.error('Failed to load results from storage:', err)
  }

  return { items: null, time: null }
}

const getUnlockItems = async (
  existingItems: UnlockItem[] | null = null,
  existingTime: string | null = null,
) => {
  try {
    const defaultItems = await invoke<UnlockItem[]>('get_unlock_items')
    const mergedItems = mergeUnlockItems(defaultItems, existingItems)
    const sortedItems = sortItemsByName(mergedItems)

    unlockItems.value = sortedItems
    saveResultsToStorage(
      sortedItems,
      existingItems && existingItems.length > 0 ? existingTime : null,
    )
  } catch (err: any) {
    console.error('Failed to get unlock items:', err)
  }
}

onMounted(async () => {
  const { items: storedItems, time: storedTime } = loadResultsFromStorage()

  if (storedItems && storedItems.length > 0) {
    unlockItems.value = sortItemsByName(storedItems)
    await getUnlockItems(storedItems, storedTime)
  } else {
    await getUnlockItems()
  }
})

const invokeWithTimeout = async <T,>(
  cmd: string,
  args?: any,
  timeout = 15000,
): Promise<T> => {
  return Promise.race([
    invoke<T>(cmd, args),
    new Promise<T>((_, reject) =>
      setTimeout(
        () =>
          reject(new Error(t('tests.unlock.page.messages.detectionTimeout'))),
        timeout,
      ),
    ),
  ])
}

const checkAllMedia = async () => {
  if (checkAllLock.value) return
  checkAllLock.value = true
  try {
    isCheckingAll.value = true
    const result = await invokeWithTimeout<UnlockItem[]>('check_media_unlock')
    const sortedItems = sortItemsByName(dedupeUnlockItems(result))

    unlockItems.value = sortedItems
    const currentTime = new Date().toLocaleString()

    saveResultsToStorage(sortedItems, currentTime)

    isCheckingAll.value = false
  } catch (err: any) {
    isCheckingAll.value = false
    showNotice.error('tests.unlock.page.messages.detectionTimeout', err)
    console.error('Failed to check media unlock:', err)
  } finally {
    checkAllLock.value = false
  }
}

const checkSingleMedia = async (name: string) => {
  if (singleCheckLocks.value[name]) return
  singleCheckLocks.value[name] = true
  try {
    loadingItems.value = [...loadingItems.value, name]
    const result = await invokeWithTimeout<UnlockItem[]>('check_media_unlock')
    const dedupedResult = dedupeUnlockItems(result)

    const normalizedTargetName = normalizeUnlockName(name)
    const targetItem = dedupedResult.find(
      (item: UnlockItem) =>
        normalizeUnlockName(item.name) === normalizedTargetName,
    )

    if (targetItem) {
      const updatedItems = sortItemsByName(
        dedupeUnlockItems(
          unlockItems.value.map((item: UnlockItem) =>
            normalizeUnlockName(item.name) === normalizedTargetName
              ? targetItem
              : item,
          ),
        ),
      )

      unlockItems.value = updatedItems
      const currentTime = new Date().toLocaleString()

      saveResultsToStorage(updatedItems, currentTime)
    }

    loadingItems.value = loadingItems.value.filter((item) => item !== name)
  } catch (err: any) {
    loadingItems.value = loadingItems.value.filter((item) => item !== name)
    showNotice.error(
      'tests.unlock.page.messages.detectionFailedWithName',
      { name },
      err,
    )
    console.error(`Failed to check ${name}:`, err)
  } finally {
    singleCheckLocks.value[name] = false
  }
}

const getStatusColor = (status: string) => {
  if (status === 'Pending') return 'default'
  if (status === 'Yes') return 'success'
  if (status === 'No') return 'error'
  if (status === 'Soon') return 'warning'
  if (status.includes('Failed')) return 'error'
  if (status === 'Completed') return 'info'
  if (
    status === 'Disallowed ISP' ||
    status === 'Blocked' ||
    status === 'Unsupported Country/Region'
  ) {
    return 'error'
  }
  return 'default'
}

const getStatusBorderColor = (status: string) => {
  if (status === 'Yes') return 'var(--success-color)'
  if (status === 'No') return 'var(--error-color)'
  if (status === 'Soon') return 'var(--warning-color)'
  if (status.includes('Failed')) return 'var(--error-color)'
  if (status === 'Completed') return 'var(--primary-main)'
  return 'var(--border-color)'
}
</script>

<style scoped>
.unlock-grid {
  display: grid;
  grid-template-columns: repeat(1, 1fr);
  gap: 12px;
}

@media (min-width: 600px) {
  .unlock-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (min-width: 900px) {
  .unlock-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}

.unlock-card {
  height: 100%;
  border-radius: 8px;
  border: 1px solid var(--border-color);
  background-color: var(--bg-color);
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition: background-color 0.2s;
}

.unlock-card:hover {
  background-color: color-mix(in srgb, var(--primary-main) 8%, var(--bg-color));
}

.status-chip {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 0.8125rem;
  line-height: 1.5;
  height: 24px;
}

.status-chip.status-default {
  background-color: rgba(128, 128, 128, 0.15);
  color: var(--text-primary);
}

.status-chip.status-success {
  background-color: rgba(76, 175, 80, 0.15);
  color: var(--success-color);
  font-weight: 700;
}

.status-chip.status-error {
  background-color: rgba(244, 67, 54, 0.15);
  color: var(--error-color);
  font-weight: 700;
}

.status-chip.status-warning {
  background-color: rgba(255, 152, 0, 0.15);
  color: var(--warning-color);
  font-weight: 700;
}

.status-chip.status-info {
  background-color: rgba(33, 150, 243, 0.15);
  color: var(--primary-main);
  font-weight: 700;
}

.region-chip {
  background-color: transparent;
  border: 1px solid var(--border-color);
  color: var(--primary-main);
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.spinning {
  animation: spin 1s linear infinite;
}

.loading-spinner {
  display: inline-flex;
  animation: spin 1s linear infinite;
}
</style>
