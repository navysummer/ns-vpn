<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { Globe, Plus, Zap, X } from "lucide-vue-next";
import { useAppStore } from "@/stores/app";
import EnhancedCard from "@/components/EnhancedCard.vue";
import { useToast } from "@/utils/toast";

const { t } = useI18n();
const app = useAppStore();
const { show } = useToast();

interface WebsiteTest {
  name: string;
  icon: string;
  color: string;
  url: string;
  delay?: number;
  testing?: boolean;
}

const websites = ref<WebsiteTest[]>([
  { name: "Apple", icon: "🍎", color: "#a2aaad", url: "https://www.apple.com" },
  { name: "GitHub", icon: "🐙", color: "#f0f6fc", url: "https://www.github.com" },
  { name: "Google", icon: "G", color: "#4285f4", url: "https://www.google.com" },
  { name: "YouTube", icon: "▶", color: "#ff0000", url: "https://www.youtube.com" },
  { name: "Twitter", icon: "𝕏", color: "#1da1f2", url: "https://www.twitter.com" },
  { name: "Netflix", icon: "N", color: "#e50914", url: "https://www.netflix.com" },
]);

const testingAll = ref(false);
const showAddDialog = ref(false);
const newName = ref("");
const newUrl = ref("");

function openAddDialog() {
  newName.value = "";
  newUrl.value = "";
  showAddDialog.value = true;
}

function addWebsite() {
  if (!newName.value.trim() || !newUrl.value.trim()) {
    show(t("home.websiteTest.invalid"), "error");
    return;
  }
  websites.value.push({
    name: newName.value.trim(),
    icon: "🌐",
    color: "var(--accent)",
    url: newUrl.value.trim(),
  });
  showAddDialog.value = false;
}

async function testWebsite(site: WebsiteTest) {
  site.testing = true;
  site.delay = undefined;
  try {
    const start = performance.now();
    await fetch(site.url, { mode: "no-cors" });
    const end = performance.now();
    site.delay = Math.round(end - start);
  } catch {
    site.delay = -1;
  }
  site.testing = false;
}

async function testAllWebsites() {
  testingAll.value = true;
  await Promise.all(websites.value.map(testWebsite));
  testingAll.value = false;
}

function delayColor(delay: number | undefined): string {
  if (delay === undefined) return "var(--text-secondary)";
  if (delay === -1) return "var(--red)";
  if (delay < 500) return "var(--green)";
  if (delay < 1500) return "var(--orange)";
  return "var(--red)";
}
</script>

<template>
  <EnhancedCard :title="t('home.websiteTest.title')" :icon="Globe" icon-color="var(--accent)">
    <template #action>
      <button class="add-btn" :disabled="testingAll" @click="testAllWebsites">
        <Zap :size="14" />
      </button>
      <button class="add-btn" @click="openAddDialog">
        <Plus :size="14" />
      </button>
    </template>
    <div class="website-grid">
      <div v-for="site in websites" :key="site.name" class="website-item" @click="testWebsite(site)">
        <div class="website-icon" :style="{ color: site.color }">
          {{ site.icon }}
        </div>
        <span class="website-name">{{ site.name }}</span>
        <span v-if="site.testing" class="website-testing">测试中</span>
        <span v-else-if="site.delay === -1" class="website-delay" style="color: var(--red)">失败</span>
        <span v-else-if="site.delay !== undefined" class="website-delay" :style="{ color: delayColor(site.delay) }">{{ site.delay }}ms</span>
        <span v-else class="website-delay hint">点击测试</span>
      </div>
    </div>
  </EnhancedCard>

  <Teleport to="body">
    <Transition name="page">
      <div v-if="showAddDialog" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showAddDialog = false">
        <div class="add-dialog" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
          <div class="dialog-header">
            <h3 class="dialog-title">{{ t('home.websiteTest.add') }}</h3>
            <button class="dialog-close" @click="showAddDialog = false"><X :size="18" /></button>
          </div>
          <div class="dialog-body">
            <div class="field">
              <label class="field-label">{{ t('home.websiteTest.name') }}</label>
              <input v-model="newName" class="field-input" :placeholder="t('home.websiteTest.namePlaceholder')" @keydown.enter="addWebsite" />
            </div>
            <div class="field">
              <label class="field-label">{{ t('home.websiteTest.url') }}</label>
              <input v-model="newUrl" class="field-input" placeholder="https://example.com" @keydown.enter="addWebsite" />
            </div>
          </div>
          <div class="dialog-footer">
            <button class="btn-ghost text-xs" @click="showAddDialog = false">{{ t('common.cancel') }}</button>
            <button class="btn-primary text-xs" @click="addWebsite">{{ t('common.confirm') }}</button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.add-btn {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 12px;
  border: 1px solid var(--border);
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}
.add-btn:hover {
  border-color: var(--accent);
  color: var(--accent);
}

.website-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
}

.website-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 14px 8px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  cursor: pointer;
  transition: all 150ms ease;
}
.website-item:hover {
  border-color: var(--accent);
  background-color: var(--bg-hover);
}

.website-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: 700;
  background-color: var(--card-bg);
}

.website-name {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
}

.website-delay {
  font-size: 11px;
  font-weight: 600;
  font-family: "SF Mono", "Fira Code", monospace;
}
.website-delay.hint { color: var(--text-secondary); font-weight: 400; }
.website-testing { font-size: 11px; color: var(--accent); font-weight: 500; }

.add-dialog {
  border-radius: 14px;
  border: 1px solid var(--border);
  width: 360px;
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0,0,0,0.3);
}
.dialog-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 16px 20px; border-bottom: 1px solid var(--border);
}
.dialog-title { font-size: 15px; font-weight: 600; margin: 0; }
.dialog-close { background: none; border: none; color: var(--text-secondary); cursor: pointer; padding: 4px; border-radius: 6px; display: flex; }
.dialog-close:hover { background-color: var(--bg-hover); color: var(--text-primary); }
.dialog-body { padding: 20px; display: flex; flex-direction: column; gap: 16px; }
.dialog-footer { display: flex; justify-content: flex-end; gap: 8px; padding: 12px 20px; border-top: 1px solid var(--border); }
.field { display: flex; flex-direction: column; gap: 6px; }
.field-label { font-size: 12px; font-weight: 500; color: var(--text-secondary); }
.field-input { border-radius: 8px; padding: 8px 12px; font-size: 13px; outline: none; border: 1px solid var(--border); background-color: var(--bg-tertiary); color: var(--text-primary); }

@media (max-width: 768px) {
  .website-grid { grid-template-columns: repeat(3, 1fr); }
}
</style>