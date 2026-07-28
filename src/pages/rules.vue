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

const filteredRules = computed(() => {
  if (!searchQuery.value) return rules.value;
  const q = searchQuery.value.toLowerCase();
  return rules.value.filter(
    (r) =>
      r.payload.toLowerCase().includes(q) ||
      r.type.toLowerCase().includes(q) ||
      r.proxy.toLowerCase().includes(q)
  );
});

function proxyColor(proxy: string): string {
  switch (proxy) {
    case "Proxy":
      return "var(--accent)";
    case "Direct":
      return "var(--green)";
    case "Reject":
      return "var(--red)";
    case "Media":
      return "var(--orange)";
    case "Ai":
      return "#bf5af2";
    default:
      return "var(--text-secondary)";
  }
}
</script>

<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-semibold">规则</h1>
      <span class="text-sm" :style="{ color: 'var(--text-secondary)' }">
        共 {{ rules.length }} 条规则
      </span>
    </div>

    <!-- Search -->
    <div
      class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm max-w-xs"
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

    <!-- Rules table -->
    <div class="rounded-xl overflow-hidden border" :style="{ borderColor: 'var(--border)' }">
      <!-- Header -->
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

      <!-- Body -->
      <div
        class="divide-y max-h-[calc(100vh-280px)] overflow-y-auto"
        :style="{ borderColor: 'var(--border)' }"
      >
        <div
          v-for="(rule, i) in filteredRules"
          :key="i"
          class="grid grid-cols-4 gap-2 px-4 py-2.5 text-sm items-center transition-colors"
          :style="{ borderColor: 'var(--border)' }"
          @mouseenter="(e) => (e.currentTarget as HTMLElement).style.backgroundColor = 'var(--bg-hover)'"
          @mouseleave="(e) => (e.currentTarget as HTMLElement).style.backgroundColor = 'transparent'"
        >
          <div>
            <span
              class="text-xs px-1.5 py-0.5 rounded"
              :style="{
                backgroundColor: 'rgba(79,142,247,0.1)',
                color: 'var(--accent)',
              }"
            >
              {{ rule.type }}
            </span>
          </div>
          <div class="font-mono text-xs" :style="{ color: 'var(--text-primary)' }">
            {{ rule.payload }}
          </div>
          <div class="text-xs" :style="{ color: 'var(--text-secondary)' }">
            {{ rule.behavior }}
          </div>
          <div>
            <span
              class="text-xs font-medium"
              :style="{ color: proxyColor(rule.proxy) }"
            >
              {{ rule.proxy }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>