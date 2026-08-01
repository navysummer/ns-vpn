<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { Monitor, Wifi } from "lucide-vue-next";
import { useToast } from "@/utils/toast";
import { useAppStore } from "@/stores/app";
import EnhancedCard from "@/components/EnhancedCard.vue";

const app = useAppStore();
const { show } = useToast();
const { t } = useI18n();

const mode = ref<"system" | "tun">("system");

async function toggleSystemProxy() {
  const newVal = !app.systemProxy;
  app.systemProxy = newVal;
  await app.setSystemProxyMode(newVal);
  await app.pushToBackend();
  show(newVal ? t("home.networkSettings.systemProxyOn") : t("home.networkSettings.systemProxyOff"), "info");
}

async function toggleTun() {
  const newVal = !app.tunMode;
  app.tunMode = newVal;
  await app.setTunModeEnabled(newVal);
  await app.pushToBackend();
  show(newVal ? t("home.networkSettings.tunOn") : t("home.networkSettings.tunOff"), "info");
}
</script>

<template>
  <EnhancedCard :title="t('home.networkSettings.title')" :icon="Monitor" icon-color="var(--accent)">
    <div class="ns-body">
      <div class="mode-tabs">
        <button class="mode-tab" :class="{ 'mode-tab-active': mode === 'system' }" @click="mode = 'system'">
          <Monitor :size="14" />
          {{ t('home.networkSettings.systemProxy') }}
        </button>
        <button class="mode-tab" :class="{ 'mode-tab-active': mode === 'tun' }" @click="mode = 'tun'">
          <Wifi :size="14" />
          {{ t('home.networkSettings.tunMode') }}
        </button>
      </div>
      <div class="ns-hint">
        <span v-if="mode === 'system'">{{ t('home.networkSettings.systemProxyHint', { status: app.systemProxy ? t('home.networkSettings.enabled') : t('home.networkSettings.disabled') }) }}</span>
        <span v-else>{{ t('home.networkSettings.tunHint') }}</span>
      </div>
      <div v-if="mode === 'system'" class="proxy-toggle">
        <div class="toggle-label">
          <span class="toggle-icon">⏸</span>
          <span>{{ t('home.networkSettings.systemProxy') }}</span>
        </div>
        <div class="toggle" :class="{ active: app.systemProxy }" @click="toggleSystemProxy">
          <div class="toggle-knob"></div>
        </div>
      </div>
      <div v-else class="proxy-toggle">
        <div class="toggle-label">
          <span class="toggle-icon">⚡</span>
          <span>{{ t('home.networkSettings.tunMode') }}</span>
        </div>
        <div class="toggle" :class="{ active: app.tunMode }" @click="toggleTun">
          <div class="toggle-knob"></div>
        </div>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.ns-body { display: flex; flex-direction: column; gap: 12px; }
.mode-tabs { display: flex; gap: 0; border-radius: 8px; overflow: hidden; border: 1px solid var(--border); }
.mode-tab { flex: 1; display: flex; align-items: center; justify-content: center; gap: 6px; padding: 10px 16px; font-size: 13px; font-weight: 500; border: none; cursor: pointer; transition: all 150ms ease; background: transparent; color: var(--text-secondary); }
.mode-tab:first-child { border-right: 1px solid var(--border); }
.mode-tab:hover { background-color: var(--bg-hover); }
.mode-tab-active { background-color: var(--accent) !important; color: #fff !important; }
.ns-hint { font-size: 12px; color: var(--text-secondary); text-align: center; padding: 8px 12px; border-radius: 6px; border: 1px dashed var(--border); }
.proxy-toggle { display: flex; align-items: center; justify-content: space-between; padding: 10px 12px; border-radius: 8px; border: 1px solid var(--border); background-color: var(--bg-tertiary); }
.toggle-label { display: flex; align-items: center; gap: 8px; font-size: 13px; font-weight: 500; }
.toggle-icon { font-size: 14px; }
</style>
