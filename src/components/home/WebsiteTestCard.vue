<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { Globe, Plus } from "lucide-vue-next";
import EnhancedCard from "@/components/EnhancedCard.vue";

const { t } = useI18n();

interface WebsiteTest {
  name: string;
  icon: string;
  color: string;
}

const websites = ref<WebsiteTest[]>([
  { name: "Apple", icon: "🍎", color: "#a2aaad" },
  { name: "GitHub", icon: "🐙", color: "#f0f6fc" },
  { name: "Google", icon: "G", color: "#4285f4" },
  { name: "YouTube", icon: "▶", color: "#ff0000" },
  { name: "Twitter", icon: "𝕏", color: "#1da1f2" },
  { name: "Netflix", icon: "N", color: "#e50914" },
]);
</script>

<template>
  <EnhancedCard :title="t('home.websiteTest.title')" :icon="Globe" icon-color="var(--accent)">
    <template #action>
      <button class="add-btn">
        <Globe :size="14" />
        <Plus :size="14" />
      </button>
    </template>
    <div class="website-grid">
      <div v-for="site in websites" :key="site.name" class="website-item">
        <div class="website-icon" :style="{ color: site.color }">
          {{ site.icon }}
        </div>
        <span class="website-name">{{ site.name }}</span>
      </div>
    </div>
  </EnhancedCard>
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
  gap: 8px;
  padding: 16px 8px;
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

@media (max-width: 768px) {
  .website-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}
</style>
