<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { MapPin, RefreshCw, Eye, EyeOff } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import EnhancedCard from "@/components/EnhancedCard.vue";
import { fetchIpInfo } from "@/utils/tauri";

const app = useAppStore();
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
const fetchFailed = ref(false);

async function fetchIp() {
  loading.value = true;
  fetchFailed.value = false;
  try {
    const info = await fetchIpInfo();
    ip.value = info.ip;
    country.value = info.country;
    asn.value = info.asn;
    isp.value = info.isp;
    org.value = info.org;
    city.value = info.city;
    timezone.value = info.timezone;
  } catch {
    fetchFailed.value = true;
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
        <div class="ip-row">
          <span class="ip-label">{{ t('home.ipInfo.isp') }}:</span>
          <span class="ip-value">{{ isp || '-' }}</span>
        </div>
        <div class="ip-row">
          <span class="ip-label">{{ t('home.ipInfo.org') }}:</span>
          <span class="ip-value">{{ org || '-' }}</span>
        </div>
        <div class="ip-row">
          <span class="ip-label">{{ t('home.ipInfo.location') }}:</span>
          <span class="ip-value">{{ city || '-' }}</span>
        </div>
        <div class="ip-row">
          <span class="ip-label">{{ t('home.ipInfo.timezone') }}:</span>
          <span class="ip-value">{{ timezone || '-' }}</span>
        </div>
      </div>
    </div>
  </EnhancedCard>
</template>

<style scoped>
.ec-action-btn { display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; border-radius: 6px; background: transparent; border: none; cursor: pointer; color: var(--text-secondary); transition: background-color 150ms ease; }
.ec-action-btn:hover { background-color: var(--bg-hover); color: var(--text-primary); }
.ec-action-btn:disabled { opacity: 0.5; cursor: not-allowed; }
.ip-content { display: flex; flex-direction: column; gap: 6px; overflow: hidden; }
.ip-left { display: flex; flex-direction: column; gap: 6px; }
.ip-country { display: flex; align-items: center; gap: 8px; }
.country-flag { font-size: 24px; flex-shrink: 0; }
.country-name { font-size: 16px; font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.ip-row { display: flex; align-items: flex-start; gap: 6px; font-size: 13px; }
.ip-label { color: var(--text-secondary); white-space: nowrap; flex-shrink: 0; }
.ip-value { font-weight: 500; word-break: break-all; overflow-wrap: break-word; min-width: 0; }
.ip-toggle { background: transparent; border: none; cursor: pointer; color: var(--text-secondary); padding: 2px; display: flex; align-items: center; flex-shrink: 0; }
.ip-toggle:hover { color: var(--text-primary); }
.spin { animation: spin 1s linear infinite; }
@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }
</style>