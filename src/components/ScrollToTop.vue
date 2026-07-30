<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useI18n } from "vue-i18n";
import { ArrowUp } from "lucide-vue-next";

const { t } = useI18n();

interface Props {
  target?: string;
}

defineProps<Props>();

const visible = ref(false);

function checkScroll() {
  const main = document.querySelector("main");
  if (main) {
    visible.value = main.scrollTop > 200;
  }
}

function scrollToTop() {
  const main = document.querySelector("main");
  if (main) {
    main.scrollTo({ top: 0, behavior: "smooth" });
  }
}

onMounted(() => {
  const main = document.querySelector("main");
  if (main) {
    main.addEventListener("scroll", checkScroll);
  }
});

onUnmounted(() => {
  const main = document.querySelector("main");
  if (main) {
    main.removeEventListener("scroll", checkScroll);
  }
});
</script>

<template>
  <Transition name="page">
    <button
      v-if="visible"
      class="scroll-top-btn"
      @click="scrollToTop"
      :title="t('common.backToTop')"
    >
      <ArrowUp :size="16" />
    </button>
  </Transition>
</template>

<style scoped>
.scroll-top-btn {
  position: fixed;
  bottom: 24px;
  right: 24px;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background-color: var(--accent);
  color: #fff;
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 12px rgba(79, 142, 247, 0.4);
  transition: transform 150ms ease, opacity 150ms ease;
  z-index: 100;
}
.scroll-top-btn:hover {
  transform: scale(1.1);
}
</style>
