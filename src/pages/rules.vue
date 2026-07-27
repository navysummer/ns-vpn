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
      {{ t('rules.page.title') }}
    </template>
    <template #header>
      <div style="display: flex; align-items: center; gap: 8px">
        <ProviderButton />
      </div>
    </template>

    <div
      style="
        padding-top: 8px;
        margin-bottom: 4px;
        margin-left: 10px;
        margin-right: 10px;
        height: 36px;
        display: flex;
        align-items: center;
      "
    >
      <BaseSearchBox :on-search="handleSearch" />
    </div>

    <template v-if="filteredRules.length > 0">
      <VirtualList
        ref="virtuosoRef"
        :count="filteredRules.length"
        :estimate-size="40"
        :style="{ flex: 1 }"
        :on-scroll="handleScroll"
      >
        <template #item="{ index }">
          <RuleItem :value="filteredRules[index]" />
        </template>
      </VirtualList>
      <ScrollTopButton :on-click="scrollToTop" :show="showScrollTop" />
    </template>
    <BaseEmpty v-else />
  </BasePage>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useTranslation } from '@/composables/use-i18n'

import { BaseEmpty, BasePage, BaseSearchBox, VirtualList } from '@/components/base'
import ScrollTopButton from '@/components/layout/scroll-top-button.vue'
import ProviderButton from '@/components/rule/provider-button.vue'
import RuleItem from '@/components/rule/rule-item.vue'
import { useVisibility } from '@/hooks/use-visibility'
import { useAppRefreshers, useRulesData } from '@/providers/app-data-context'

const { t } = useTranslation()
const { rules = [] } = useRulesData()
const { refreshRules, refreshRuleProviders } = useAppRefreshers()
const matchFunc = ref((_: string) => true)
const virtuosoRef = ref<InstanceType<typeof VirtualList> | null>(null)
const showScrollTop = ref(false)
const pageVisible = useVisibility()

onMounted(() => {
  refreshRules()
  refreshRuleProviders()

  if (pageVisible) {
    refreshRules()
    refreshRuleProviders()
  }
})

const filteredRules = computed(() => {
  const rulesWithLineNo = rules.map((item: any, index: number) => ({
    ...item,
    lineNo: index + 1,
  }))

  return rulesWithLineNo.filter((item: any) => matchFunc.value(item.payload ?? ''))
})

const handleSearch = (match: (text: string) => boolean) => {
  matchFunc.value = match
}

const handleScroll = (e: Event) => {
  showScrollTop.value = (e.target as HTMLElement).scrollTop > 100
}

const scrollToTop = () => {
  virtuosoRef.value?.scrollTo({ top: 0, behavior: 'smooth' })
}
</script>
