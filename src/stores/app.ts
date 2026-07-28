import { defineStore } from "pinia";
import { ref, computed } from "vue";

export const useAppStore = defineStore("app", () => {
  const theme = ref<"dark" | "light" | "auto">("dark");
  const sidebarCollapsed = ref(false);
  const proxyRunning = ref(false);

  const isDark = computed(() => {
    if (theme.value === "auto") {
      return window.matchMedia("(prefers-color-scheme: dark)").matches;
    }
    return theme.value === "dark";
  });

  function toggleTheme() {
    if (theme.value === "dark") theme.value = "light";
    else if (theme.value === "light") theme.value = "auto";
    else theme.value = "dark";
  }

  function setTheme(t: "dark" | "light" | "auto") {
    theme.value = t;
  }

  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value;
  }

  function setProxyRunning(running: boolean) {
    proxyRunning.value = running;
  }

  return {
    theme,
    sidebarCollapsed,
    proxyRunning,
    isDark,
    toggleTheme,
    setTheme,
    toggleSidebar,
    setProxyRunning,
  };
});