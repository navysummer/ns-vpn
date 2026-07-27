<template>
  <EnhancedCard
    :title="t('home.components.tests.title')"
    iconColor="primary"
  >
    <template #icon>
      <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M15 9H9v2h6V9zm-2 4H9v2h4v-2zm5-10H6c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H6V5h12v14z"/></svg>
    </template>
    <template #action>
      <div :style="{ display: 'flex', gap: '8px' }">
        <n-tooltip :title="t('tests.page.actions.testAll')" :trigger="'hover'">
          <template #trigger>
            <n-button quaternary circle size="small" @click="handleTestAll">
              <template #icon>
                <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M15 9H9v2h6V9zm-2 4H9v2h4v-2zm5-10H6c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H6V5h12v14z"/></svg>
              </template>
            </n-button>
          </template>
        </n-tooltip>
        <n-tooltip :title="t('tests.modals.test.title.create')" :trigger="'hover'">
          <template #trigger>
            <n-button quaternary circle size="small" @click="handleCreateTest">
              <template #icon>
                <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/></svg>
              </template>
            </n-button>
          </template>
        </n-tooltip>
      </div>
    </template>
    <div :style="{ maxHeight: '180px', overflowY: 'auto', overflowX: 'hidden' }">
      <div :style="{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '8px' }">
        <div
          v-for="item in testList"
          :key="item.uid"
          :style="{ display: 'flex', flexDirection: 'column', alignItems: 'center', padding: '8px', borderRadius: '8px', backgroundColor: 'var(--action-hover-color)', cursor: 'pointer' }"
          @click="viewerRef?.edit(item)"
          @contextmenu.prevent="onDeleteTestListItem(item.uid)"
        >
          <div v-html="item.icon" :style="{ width: '24px', height: '24px', marginBottom: '4px' }" />
          <span :style="{ fontSize: '12px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', maxWidth: '100%' }">{{ item.name }}</span>
        </div>
      </div>
    </div>
    <TestViewer ref="viewerRef" :onChange="onTestListItemChange" />
  </EnhancedCard>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NTooltip } from 'naive-ui'
import { nanoid } from 'nanoid'
import { emit } from '@tauri-apps/api/event'

import EnhancedCard from './enhanced-card.vue'
import TestViewer from '@/components/test/test-viewer.vue'
import { useVerge } from '@/hooks/use-verge'

const { t } = useI18n()
const { verge, mutateVerge, patchVerge } = useVerge()
const viewerRef = ref<any>(null)

const DEFAULT_TEST_LIST = [
  { uid: nanoid(), name: 'Apple', url: 'https://www.apple.com', icon: '<svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor"><path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2.01.76-3.27.82-1.31.07-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.8-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.5-.61 1.29-1.07 2.08-1.09.07.86-.27 1.74-.76 2.37-.5.62-1.3 1.1-2.07 1.03-.07-.83.27-1.72.75-2.31z"/></svg>' },
  { uid: nanoid(), name: 'GitHub', url: 'https://www.github.com', icon: '<svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor"><path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/></svg>' },
  { uid: nanoid(), name: 'Google', url: 'https://www.google.com', icon: '<svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor"><path d="M12.48 10.92v3.28h7.84c-.24 1.84-.853 3.187-1.787 4.133-1.147 1.147-2.933 2.4-6.053 2.4-4.827 0-8.6-3.893-8.6-8.72s3.773-8.72 8.6-8.72c2.6 0 4.507 1.027 5.907 2.347l2.307-2.307C18.747 1.44 16.133 0 12.48 0 5.867 0 .16 5.653.16 12s5.707 12 12.32 12c3.573 0 6.267-1.173 8.373-3.36 2.16-2.16 2.84-5.213 2.84-7.667 0-.76-.053-1.467-.173-2.053H12.48z"/></svg>' },
  { uid: nanoid(), name: 'YouTube', url: 'https://www.youtube.com', icon: '<svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor"><path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/></svg>' },
]

const testList = computed(() => verge.value?.test_list ?? DEFAULT_TEST_LIST)

const onTestListItemChange = (uid: string, patch?: any) => {
  if (!patch) { mutateVerge(); return }
  const newList = testList.value.map((x: any) => x.uid === uid ? { ...x, ...patch } : x)
  mutateVerge({ ...verge.value, test_list: newList }, false)
}

const onDeleteTestListItem = (uid: string) => {
  const newList = testList.value.filter((x: any) => x.uid !== uid)
  patchVerge({ test_list: newList })
  mutateVerge({ ...verge.value, test_list: newList }, false)
}

const handleTestAll = () => emit('verge://test-all')
const handleCreateTest = () => viewerRef.value?.create()

if (verge.value && !verge.value.test_list) {
  patchVerge({ test_list: DEFAULT_TEST_LIST })
}
</script>
