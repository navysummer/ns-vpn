<script setup lang="ts">
import { watch, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useAppStore } from "@/stores/app";
import Sidebar from "@/components/Sidebar.vue";
import ToastContainer from "@/components/ToastContainer.vue";
import ScrollToTop from "@/components/ScrollToTop.vue";
import CoreInstallOverlay from "@/components/CoreInstallOverlay.vue";
import { checkCoreInstalled, installCoreWithProgress, autoStartCore, writeConfigOnly, fetchSubscriptionUrl, convertContent } from "@/utils/tauri";
import { isEnabled as autostartIsEnabled } from "@tauri-apps/plugin-autostart";
import { getCurrentWindow } from "@tauri-apps/api/window";

const app = useAppStore();
const router = useRouter();

const bgThemes: Record<string, { dark: Record<string, string>; light: Record<string, string> }> = {
  default: {
    dark: { "--bg-primary": "#0f0f11", "--bg-secondary": "#1a1a1e", "--bg-tertiary": "#252529", "--bg-hover": "#2a2a2e", "--border": "#38383a", "--card-bg": "#1a1a1e" },
    light: { "--bg-primary": "#f5f5f7", "--bg-secondary": "#ffffff", "--bg-tertiary": "#e8e8ed", "--bg-hover": "#dcdcdf", "--border": "#d2d2d7", "--card-bg": "#ffffff" },
  },
  navy: {
    dark: { "--bg-primary": "#0d1117", "--bg-secondary": "#161b22", "--bg-tertiary": "#21262d", "--bg-hover": "#262c35", "--border": "#30363d", "--card-bg": "#161b22" },
    light: { "--bg-primary": "#f0f2f5", "--bg-secondary": "#ffffff", "--bg-tertiary": "#e4e7eb", "--bg-hover": "#d9dce2", "--border": "#c9ccd1", "--card-bg": "#ffffff" },
  },
  midnight: {
    dark: { "--bg-primary": "#0a0a1a", "--bg-secondary": "#13132b", "--bg-tertiary": "#1c1c3a", "--bg-hover": "#222245", "--border": "#2d2d50", "--card-bg": "#13132b" },
    light: { "--bg-primary": "#eeeef5", "--bg-secondary": "#ffffff", "--bg-tertiary": "#ddddee", "--bg-hover": "#ccccdd", "--border": "#bbbbcc", "--card-bg": "#ffffff" },
  },
  forest: {
    dark: { "--bg-primary": "#0a140a", "--bg-secondary": "#122012", "--bg-tertiary": "#1a2e1a", "--bg-hover": "#1f351f", "--border": "#2a402a", "--card-bg": "#122012" },
    light: { "--bg-primary": "#eef5ee", "--bg-secondary": "#ffffff", "--bg-tertiary": "#ddeedd", "--bg-hover": "#ccddcc", "--border": "#bbccbb", "--card-bg": "#ffffff" },
  },
  warm: {
    dark: { "--bg-primary": "#14100c", "--bg-secondary": "#1e1a15", "--bg-tertiary": "#2a2520", "--bg-hover": "#302b25", "--border": "#3a3530", "--card-bg": "#1e1a15" },
    light: { "--bg-primary": "#f5f3ee", "--bg-secondary": "#ffffff", "--bg-tertiary": "#eee8dd", "--bg-hover": "#ddd8cc", "--border": "#ccc8bb", "--card-bg": "#ffffff" },
  },
  dracula: {
    dark: { "--bg-primary": "#1e1e2e", "--bg-secondary": "#282840", "--bg-tertiary": "#313150", "--bg-hover": "#363658", "--border": "#414160", "--card-bg": "#282840" },
    light: { "--bg-primary": "#f0f0f5", "--bg-secondary": "#ffffff", "--bg-tertiary": "#e0e0ee", "--bg-hover": "#d0d0dd", "--border": "#c0c0cc", "--card-bg": "#ffffff" },
  },
};

function applyBgTheme(name: string, isDark: boolean) {
  const theme = bgThemes[name] || bgThemes.default;
  const vars = isDark ? theme.dark : theme.light;
  for (const [key, val] of Object.entries(vars)) {
    document.documentElement.style.setProperty(key, val);
  }
}

watch(
  () => app.isDark,
  (dark) => {
    if (dark) {
      document.documentElement.classList.remove("light");
    } else {
      document.documentElement.classList.add("light");
    }
    applyBgTheme(app.bgColor, dark);
  },
  { immediate: true }
);

watch(
  () => app.accentColor,
  (color) => {
    document.documentElement.style.setProperty("--accent", color);
  },
  { immediate: true }
);

watch(
  () => app.bgColor,
  (name) => {
    applyBgTheme(name, app.isDark);
  },
  { immediate: true }
);

onMounted(async () => {
  app.syncFromBackend();

  // Sync autostart state from OS
  try {
    const osEnabled = await autostartIsEnabled();
    app.startAtBoot = osEnabled;
  } catch {
    // not available on this platform
  }

  try {
    const info = await checkCoreInstalled();
    if (!info.hasCore) {
      await installCoreWithProgress();
    }
  } catch {
    // not in Tauri or error
  }

  // Auto-apply saved subscription
  try {
    const ACTIVE_KEY = "ns-vpn-active-sub";
    const STORAGE_KEY = "ns-vpn-subscriptions";
    const savedId = localStorage.getItem(ACTIVE_KEY);
    if (savedId) {
      const saved = localStorage.getItem(STORAGE_KEY);
      if (saved) {
        const subs = JSON.parse(saved);
        const sub = subs.find((s: any) => s.id === savedId);
        if (sub) {
          app.activeSubId = sub.id;
          app.activeSubName = sub.name;
          app.activeSubUrl = sub.url || "";
          app.activeSubUpdateTime = sub.lastUpdate || "";
          let content = sub.fileContent || sub.pasteContent || "";
          let needConvert = sub.format !== "clash" && !sub.fileContent;
          if (!content && sub.type === "remote" && sub.url) {
            content = await fetchSubscriptionUrl(sub.url);
            needConvert = true;
          }
          if (needConvert && content) {
            try {
              content = await convertContent(content, sub.format);
            } catch {
              content = "";
            }
          }
          if (content) {
            await writeConfigOnly(content);
          }
        }
      }
    }
  } catch {
    // auto-apply failed, ignore
  }

  // Auto-start core if it was running when app was closed (like Clash Verge Rev)
  try {
    const started = await autoStartCore();
    if (started) {
      app.proxyRunning = true;
      await app.waitForCore();
      await app.fetchProxies();
      await app.fetchRules();
      await app.fetchConnections();
    }
  } catch {
    // auto-start failed, ignore
  }

  // SilentStart: hide window when silent start is enabled
  try {
    if (app.silentStart) {
      await getCurrentWindow().hide();
    }
  } catch {
    // not in Tauri or error
  }

  // StartupPage: navigate to saved page
  const pageMap: Record<string, string> = {
    dashboard: "/dashboard",
    proxies: "/proxies",
    settings: "/settings",
  };
  const target = pageMap[app.startupPage] || "/dashboard";
  if (router.currentRoute.value.path === "/") {
    router.replace(target);
  }
});
</script>

<template>
  <CoreInstallOverlay />
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
    <ScrollToTop />
  </div>
</template>
