<script setup lang="ts">
import { useI18n } from "vue-i18n";

const { t } = useI18n();

interface Props {
  show: boolean;
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  type?: "danger" | "warning" | "info";
}

const props = withDefaults(defineProps<Props>(), {
  confirmText: "",
  cancelText: "",
  type: "danger",
});

const emit = defineEmits<{
  (e: "confirm"): void;
  (e: "cancel"): void;
}>();

function confirmColor(type: string): string {
  switch (type) {
    case "danger": return "var(--red)";
    case "warning": return "var(--orange)";
    default: return "var(--accent)";
  }
}
</script>

<template>
  <Teleport to="body">
    <Transition name="page">
      <div
        v-if="show"
        class="fixed inset-0 flex items-center justify-center bg-black/50 z-50"
        @click="emit('cancel')"
      >
        <div
          class="card w-full max-w-sm mx-4 space-y-4"
          :style="{ backgroundColor: 'var(--bg-secondary)' }"
          @click.stop
        >
          <h3 class="text-base font-medium">{{ title }}</h3>
          <p class="text-sm" :style="{ color: 'var(--text-secondary)' }">{{ message }}</p>
          <div class="flex justify-end gap-2">
            <button class="btn-ghost text-xs" @click="emit('cancel')">
              {{ cancelText || t('common.cancel') }}
            </button>
            <button
              class="btn-primary text-xs"
              :style="{ backgroundColor: confirmColor(type) }"
              @click="emit('confirm')"
            >
              {{ confirmText || t('common.confirm') }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>
