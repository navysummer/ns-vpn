<script setup lang="ts">
import { watch } from "vue";
import { useAppStore } from "@/stores/app";
import Sidebar from "@/components/Sidebar.vue";
import ToastContainer from "@/components/ToastContainer.vue";

const app = useAppStore();

watch(
  () => app.isDark,
  (dark) => {
    if (dark) {
      document.documentElement.classList.remove("light");
    } else {
      document.documentElement.classList.add("light");
    }
  },
  { immediate: true }
);
</script>

<template>
  <div
    class="flex h-screen overflow-hidden"
    :style="{
      backgroundColor: 'var(--bg-primary)',
      color: 'var(--text-primary)',
    }"
  >
    <Sidebar />
    <main class="flex-1 overflow-y-auto p-6 lg:p-8">
      <router-view v-slot="{ Component }">
        <transition name="page" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
    <ToastContainer />
  </div>
</template>