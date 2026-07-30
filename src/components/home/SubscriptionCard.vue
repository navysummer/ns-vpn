<script setup lang="ts">
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { FileText, RefreshCw, List } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import EnhancedCard from "@/components/EnhancedCard.vue";
import { useToast } from "@/utils/toast";

const app = useAppStore();
const { show } = useToast();
const router = useRouter();
const { t } = useI18n();

function goToSubscriptions() {
  router.push("/subscriptions");
}
</script>

<template>
  <EnhancedCard title="" :no-padding="true">
    <template #icon>
      <FileText :size="18" style="color: var(--accent)" />
    </template>
    <template #title>
      <span class="sub-title">{{ app.proxyGroups.length }} {{ t('nav.proxies') }}</span>
    </template>
    <template #action>
      <div class="flex items-center gap-2">
        <button class="action-btn" @click="goToSubscriptions">
          <List :size="12" />
        </button>
      </div>
    </template>
    <div class="sub-body">
      <div class="sub-row">
        <FileText :size="14" :style="{ color: 'var(--text-secondary)' }" />
        <span class="sub-label">{{ t('home.subscription.source') }}:</span>
        <span class="sub-value mono">mihomo</span>
      </div>
      <div class="sub-row">
        <RefreshCw :size="14" :style="{ color: 'var(--text-secondary)' }" />
        <span class="sub-label">{{ t('home.subscription.updateTime') }}:</span>
        <span class="sub-value">{{ new Date().toLocaleString("zh-CN", { hour12: false }) }}</span>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.sub-title { font-size: 16px; font-weight: 700; }
.sub-body { padding: 12px 16px; display: flex; flex-direction: column; gap: 10px; }
.sub-row { display: flex; align-items: center; gap: 8px; font-size: 13px; }
.sub-label { color: var(--text-secondary); }
.sub-value { color: var(--text-primary); }
.action-btn { display: inline-flex; align-items: center; gap: 4px; padding: 4px 10px; border-radius: 6px; font-size: 12px; font-weight: 500; border: 1px solid var(--border); background: transparent; color: var(--text-primary); cursor: pointer; transition: all 150ms ease; }
.action-btn:hover { border-color: var(--accent); color: var(--accent); }
</style>
