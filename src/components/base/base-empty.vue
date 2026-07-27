<template>
  <div class="base-empty">
    <n-icon size="4em" style="color: var(--secondary-text); opacity: 0.75">
      <MailOpen />
    </n-icon>
    <span class="base-empty-text">{{ resolvedText }}</span>
    <div v-if="$slots.extra">
      <slot name="extra" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { MailOpen } from '@vicons/ionicons5'
import { useTranslation } from '@/composables/use-i18n'
const { t } = useTranslation()
import type { TranslationKey } from '@/types/generated/i18n-keys'

interface Props {
  text?: string
  textKey?: TranslationKey
}

const props = withDefaults(defineProps<Props>(), {
  textKey: 'shared.statuses.empty' as TranslationKey,
})

const resolvedText = computed(() =>
  props.text !== undefined ? props.text : t(props.textKey),
)
</script>

<style scoped>
.base-empty {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--secondary-text);
  opacity: 0.75;
}

.base-empty-text {
  font-size: 1.25em;
  margin-top: 8px;
}
</style>
