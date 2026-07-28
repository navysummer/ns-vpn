<script setup lang="ts">
import { ref, computed } from "vue";
import { Search } from "lucide-vue-next";

interface RuleEntry {
  type: string;
  payload: string;
  proxy: string;
  behavior: string;
}

const rules = ref<RuleEntry[]>([
  { type: "DOMAIN-SUFFIX", payload: "google.com", proxy: "Proxy", behavior: "Domain" },
  { type: "DOMAIN-SUFFIX", payload: "youtube.com", proxy: "Proxy", behavior: "Domain" },
  { type: "DOMAIN-SUFFIX", payload: "twitter.com", proxy: "Proxy", behavior: "Domain" },
  { type: "DOMAIN-SUFFIX", payload: "github.com", proxy: "Direct", behavior: "Domain" },
  { type: "DOMAIN-SUFFIX", payload: "microsoft.com", proxy: "Direct", behavior: "Domain" },
  { type: "DOMAIN-SUFFIX", payload: "apple.com", proxy: "Direct", behavior: "Domain" },
  { type: "DOMAIN-SUFFIX", payload: "netflix.com", proxy: "Media", behavior: "Domain" },
  { type: "DOMAIN-SUFFIX", payload: "spotify.com", proxy: "Media", behavior: "Domain" },
  { type: "DOMAIN-SUFFIX", payload: "openai.com", proxy: "Ai", behavior: "Domain" },
  { type: "DOMAIN-KEYWORD", payload: "google", proxy: "Proxy", behavior: "Domain" },
  { type: "DOMAIN-KEYWORD", payload: "facebook", proxy: "Proxy", behavior: "Domain" },
  { type: "IP-CIDR", payload: "10.0.0.0/8", proxy: "Direct", behavior: "IPCIDR" },
  { type: "IP-CIDR", payload: "172.16.0.0/12", proxy: "Direct", behavior: "IPCIDR" },
  { type: "IP-CIDR", payload: "192.168.0.0/16", proxy: "Direct", behavior: "IPCIDR" },
  { type: "IP-CIDR", payload: "127.0.0.0/8", proxy: "Direct", behavior: "IPCIDR" },
  { type: "GEOIP", payload: "CN", proxy: "Direct", behavior: "IPCIDR" },
  { type: "MATCH", payload: "MATCH", proxy: "Proxy", behavior: "Domain" },
]);

const searchQuery = ref("");
const filterType = ref<"all" | "DOMAIN" | "IP-CIDR" | "GEOIP" | "MATCH">("all");

const filteredRules = computed(() => {
  return rules.value.filter((r) => {
    if (filterType.value !== "all") {
      if (filterType.value === "DOMAIN" && !r.type.startsWith("DOMAIN")) return false;
      if (filterType.value === "IP-CIDR" && !r.type.startsWith("IP-CIDR")) return false;
      if (filterType.value === "GEOIP" && r.type !== "GEOIP") return false;
      if (filterType.value === "MATCH" && r.type !== "MATCH") return false;
    }
    if (searchQuery.value) {
      const q = searchQuery.value.toLowerCase();
      return (
        r.payload.toLowerCase().includes(q) ||
        r.type.toLowerCase().includes(q) ||
        r.proxy.toLowerCase().includes(q)
      );
    }
    return true;
  });
});

function proxyColor(proxy: string): string {
  switch (proxy) {
    case "Proxy": return "var(--accent)";
    case "Direct": return "var(--green)";
    case "Reject": return "var(--red)";
    case "Media": return "var(--orange)";
    case "Ai": return "#bf5af2";
    default: return "var(--text-secondary)";
  }
}

function proxyBg(proxy: string): string {
  switch (proxy) {
    case "Proxy": return "rgba(79,142,247,0.12)";
    case "Direct": return "rgba(52,199,89,0.12)";
    case "Reject": return "rgba(255,69,58,0.12)";
    case "Media": return "rgba(255,159,10,0.12)";
    case "Ai": return "rgba(191,90,242,0.12)";
    default: return "rgba(152,152,158,0.12)";
  }
}

function typeColor(type: string): string {
  if (type.startsWith("DOMAIN")) return "#4f8ef7";
  if (type.startsWith("IP-CIDR") || type === "GEOIP") return "#34c759";
  if (type === "MATCH") return "#ff9f0a";
  return "var(--text-secondary)";
}

function typeBg(type: string): string {
  if (type.startsWith("DOMAIN")) return "rgba(79,142,247,0.1)";
  if (type.startsWith("IP-CIDR") || type === "GEOIP") return "rgba(52,199,89,0.1)";
  if (type === "MATCH") return "rgba(255,159,10,0.1)";
  return "rgba(152,152,158,0.1)";
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-semibold">规则</h1>
      <span class="text-sm" :style="{ color: 'var(--text-secondary)' }">
        共 {{ filteredRules.length }} / {{ rules.length }} 条规则
      </span>
    </div>

    <div class="flex items-center gap-3">
      <div
        class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm flex-1 max-w-xs"
        :style="{ backgroundColor: 'var(--bg-tertiary)' }"
      >
        <Search :size="14" :style="{ color: 'var(--text-secondary)' }" />
        <input
          v-model="searchQuery"
          placeholder="搜索规则..."
          class="bg-transparent outline-none flex-1 text-sm"
          :style="{ color: 'var(--text-primary)' }"
        />
      </div>
      <div class="flex gap-1 p-0.5 rounded-lg" :style="{ backgroundColor: 'var(--bg-tertiary)' }">
        <button
          v-for="opt in ([{ label: '全部', value: 'all' }, { label: '域名', value: 'DOMAIN' }, { label: 'IP', value: 'IP-CIDR' }, { label: 'GEOIP', value: 'GEOIP' }, { label: 'MATCH', value: 'MATCH' }] as const)"
          :key="opt.value"
          class="tab-btn"
          :class="filterType === opt.value ? 'tab-btn-active' : 'tab-btn-inactive'"
          @click="filterType = opt.value"
        >
          {{ opt.label }}
        </button>
      </div>
    </div>

    <div class="rounded-xl overflow-hidden border" :style="{ borderColor: 'var(--border)' }">
      <div
        class="grid grid-cols-4 gap-2 px-4 py-2.5 text-xs font-medium"
        :style="{
          backgroundColor: 'var(--bg-secondary)',
          color: 'var(--text-secondary)',
          borderBottom: '1px solid var(--border)',
        }"
      >
        <div>类型</div>
        <div>内容</div>
        <div>行为</div>
        <div>代理</div>
      </div>

      <div
        class="divide-y max-h-[calc(100vh-300px)] overflow-y-auto"
        :style="{ borderColor: 'var(--border)' }"
      >
        <div
          v-for="(rule, i) in filteredRules"
          :key="i"
          class="grid grid-cols-4 gap-2 px-4 py-2.5 text-sm items-center row-hover"
          :style="{ borderColor: 'var(--border)' }"
        >
          <div>
            <span
              class="tag"
              :style="{
                backgroundColor: typeBg(rule.type),
                color: typeColor(rule.type),
              }"
            >
              {{ rule.type }}
            </span>
          </div>
          <div class="font-mono text-xs truncate" :style="{ color: 'var(--text-primary)' }">
            {{ rule.payload }}
          </div>
          <div class="text-xs" :style="{ color: 'var(--text-secondary)' }">
            {{ rule.behavior }}
          </div>
          <div>
            <span
              class="tag"
              :style="{
                backgroundColor: proxyBg(rule.proxy),
                color: proxyColor(rule.proxy),
              }"
            >
              {{ rule.proxy }}
            </span>
          </div>
        </div>

        <div
          v-if="filteredRules.length === 0"
          class="px-4 py-12 text-center text-sm"
          :style="{ color: 'var(--text-secondary)' }"
        >
          暂无匹配规则
        </div>
      </div>
    </div>
  </div>
</template>