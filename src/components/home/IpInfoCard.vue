<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useI18n } from "vue-i18n";
import { MapPin, RefreshCw, Eye, EyeOff } from "lucide-vue-next";
import { useToast } from "@/utils/toast";
import { useAppStore } from "@/stores/app";
import EnhancedCard from "@/components/EnhancedCard.vue";

const app = useAppStore();
const { show } = useToast();
const { t } = useI18n();

const ip = ref("");
const country = ref("");
const asn = ref("");
const isp = ref("");
const org = ref("");
const city = ref("");
const timezone = ref("");
const showIp = ref(false);
const loading = ref(false);

async function fetchIp() {
  loading.value = true;
  try {
    const resp = await fetch("https://api.ipify.org?format=json");
    const data = await resp.json();
    ip.value = data.ip;
    // Try to get more info from ip-api
    try {
      const locResp = await fetch(`http://ip-api.com/json/${data.ip}`);
      const locData = await locResp.json();
      country.value = locData.country || "";
      asn.value = locData.as || "";
      isp.value = locData.isp || "";
      org.value = locData.org || "";
      city.value = [locData.city, locData.regionName].filter(Boolean).join(", ");
      timezone.value = locData.timezone || "";
    } catch {}
  } catch {
    ip.value = t("home.ipInfo.fetchFailed");
  }
  loading.value = false;
}

onMounted(fetchIp);
</script>

<template>
  <EnhancedCard :title="t('home.ipInfo.title')" :icon="MapPin" icon-color="#bf5af2">
    <template #action>
      <button class="ec-action-btn" :disabled="loading" @click="fetchIp" :title="t('common.refresh')">
        <RefreshCw :size="14" :class="{ spin: loading }" />
      </button>
    </template>
    <div class="ip-content">
      <div class="ip-left">
        <div class="ip-country">
          <span class="country-flag">🌍</span>
          <span class="country-name">{{ country || '-' }}</span>
        </div>
        <div class="ip-row">
          <span class="ip-label">IP:</span>
          <span class="ip-value mono">{{ showIp ? ip : '••••••••••' }}</span>
          <button class="ip-toggle" @click="showIp = !showIp">
            <Eye v-if="!showIp" :size="12" />
            <EyeOff v-else :size="12" />
          </button>
        </div>
        <div class="ip-row">
          <span class="ip-label">{{ t('home.ipInfo.asn') }}:</span>
          <span class="ip-value mono">{{ asn || '-' }}</span>
        </div>
      </div>
      <div class="ip-right">
        <div class="info-row">
          <span class="info-label">{{ t('home.ipInfo.isp') }}:</span>
          <span class="info-value">{{ isp || '-' }}</span>
        </div>
        <div class="info-row">
          <span class="info-label">{{ t('home.ipInfo.org') }}:</span>
          <span class="info-value">{{ org || '-' }}</span>
        </div>
        <div class="info-row">
          <span class="info-label">{{ t('home.ipInfo.location') }}:</span>
          <span class="info-value">{{ city || '-' }}</span>
        </div>
        <div class="info-row">
          <span class="info-label">{{ t('home.ipInfo.timezone') }}:</span>
          <span class="info-value">{{ timezone || '-' }}</span>
        </div>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.ec-action-btn { display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; border-radius: 6px; background: transparent; border: none; cursor: pointer; color: var(--text-secondary); transition: background-color 150ms ease; }
.ec-action-btn:hover { background-color: var(--bg-hover); color: var(--text-primary); }
.ec-action-btn:disabled { opacity: 0.5; cursor: not-allowed; }
.ip-content { display: flex; gap: 20px; }
.ip-left { flex: 0 0 auto; display: flex; flex-direction: column; gap: 8px; }
.ip-country { display: flex; align-items: center; gap: 8px; margin-bottom: 4px; }
.country-flag { font-size: 24px; }
.country-name { font-size: 16px; font-weight: 600; }
.ip-row { display: flex; align-items: center; gap: 6px; font-size: 13px; }
.ip-label { color: var(--text-secondary); }
.ip-value { font-weight: 500; }
.ip-toggle { background: transparent; border: none; cursor: pointer; color: var(--text-secondary); padding: 2px; display: flex; align-items: center; }
.ip-toggle:hover { color: var(--text-primary); }
.ip-right { flex: 1; display: flex; flex-direction: column; gap: 6px; }
.info-row { display: flex; align-items: flex-start; gap: 8px; font-size: 13px; }
.info-label { color: var(--text-secondary); white-space: nowrap; }
.info-value { color: var(--text-primary); word-break: break-all; }
</style>
