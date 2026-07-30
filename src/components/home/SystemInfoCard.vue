<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { Settings } from "lucide-vue-next";
import { useRouter } from "vue-router";
import { useAppStore } from "@/stores/app";
import { getVersion, getSystemInfo } from "@/utils/tauri";
import EnhancedCard from "@/components/EnhancedCard.vue";

const router = useRouter();
const app = useAppStore();
const { t } = useI18n();
const osInfo = ref("");
const arch = ref("");
const appVersion = ref("");
const lastCheckUpdate = ref(new Date().toLocaleString("zh-CN", { hour12: false }));

onMounted(async () => {
  try {
    const info = await getSystemInfo();
    osInfo.value = info.os;
    arch.value = info.arch;
    appVersion.value = info.version;
  } catch {
    osInfo.value = navigator.userAgent.includes("Mac") ? "macOS" : navigator.userAgent.includes("Win") ? "Windows" : "Linux";
    arch.value = navigator.userAgent.includes("ARM") ? "arm64" : "x64";
    appVersion.value = "1.0.0";
  }
});

function goToSettings() {
  router.push("/settings");
}
</script>

<template>
  <EnhancedCard :title="t('home.systemInfo.title')" :icon="Settings" icon-color="var(--red)">
    <template #action>
      <button class="ec-action-btn" @click="goToSettings" :title="t('home.systemInfo.openSettings')">
        <Settings :size="14" />
      </button>
    </template>
    <div class="info-list">
      <div class="info-row">
        <span class="info-label">{{ t('home.systemInfo.osInfo') }}</span>
        <span class="info-value">{{ osInfo }} {{ arch }}</span>
      </div>
      <div class="info-row">
        <span class="info-label">{{ t('home.systemInfo.autoStart') }}</span>
        <span class="info-value">{{ app.startAtBoot ? t('common.yes') : t('common.no') }}</span>
      </div>
      <div class="info-row">
        <span class="info-label">{{ t('home.systemInfo.runMode') }}</span>
        <span class="info-value">{{ app.proxyRunning ? 'Running' : 'Stopped' }}</span>
      </div>
      <div class="info-row">
        <span class="info-label">{{ t('home.systemInfo.lastCheckUpdate') }}</span>
        <span class="info-value">{{ lastCheckUpdate }}</span>
      </div>
      <div class="info-row">
        <span class="info-label">{{ t('home.systemInfo.appVersion') }}</span>
        <span class="info-value mono">v{{ appVersion }}</span>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.ec-action-btn { display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; border-radius: 6px; background: transparent; border: none; cursor: pointer; color: var(--text-secondary); transition: background-color 150ms ease; }
.ec-action-btn:hover { background-color: var(--bg-hover); color: var(--text-primary); }
.info-list { display: flex; flex-direction: column; }
.info-row { display: flex; align-items: center; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid var(--border); }
.info-row:last-child { border-bottom: none; }
.info-label { font-size: 12px; color: var(--text-secondary); }
.info-value { font-size: 12px; font-weight: 500; color: var(--text-primary); }
</style>
