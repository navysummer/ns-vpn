<script setup lang="ts">
import { ref } from "vue";
import { Zap, Check, X, Clock, RefreshCw } from "lucide-vue-next";
import BasePage from "@/components/BasePage.vue";
import { useToast } from "@/utils/toast";

const { show } = useToast();

interface TestResult {
  name: string;
  delay: number;
  status: "pending" | "testing" | "success" | "fail";
}

const testTargets = ref<TestResult[]>([
  { name: "Google", delay: 0, status: "pending" },
  { name: "YouTube", delay: 0, status: "pending" },
  { name: "GitHub", delay: 0, status: "pending" },
  { name: "Twitter", delay: 0, status: "pending" },
  { name: "Telegram", delay: 0, status: "pending" },
  { name: "Netflix", delay: 0, status: "pending" },
  { name: "OpenAI", delay: 0, status: "pending" },
  { name: "Cloudflare", delay: 0, status: "pending" },
]);

const testingAll = ref(false);

function testSingle(name: string) {
  const target = testTargets.value.find(t => t.name === name);
  if (!target || target.status === "testing") return;

  target.status = "testing";
  target.delay = 0;

  setTimeout(() => {
    const success = Math.random() > 0.1;
    target.status = success ? "success" : "fail";
    target.delay = success ? Math.floor(Math.random() * 300) + 20 : 0;
  }, Math.random() * 2000 + 500);
}

function testAll() {
  testingAll.value = true;
  testTargets.value.forEach(t => {
    t.status = "pending";
    t.delay = 0;
  });

  testTargets.value.forEach((t, i) => {
    setTimeout(() => testSingle(t.name), i * 200);
  });

  setTimeout(() => {
    testingAll.value = false;
    const successCount = testTargets.value.filter(t => t.status === "success").length;
    show(`测试完成: ${successCount}/${testTargets.value.length} 成功`, "info");
  }, 3000);
}

function delayColor(delay: number): string {
  if (delay === 0) return "var(--text-secondary)";
  if (delay < 100) return "var(--green)";
  if (delay < 300) return "var(--orange)";
  return "var(--red)";
}
</script>

<template>
  <BasePage title="测试">
    <template #actions>
      <button class="btn-primary text-xs" :disabled="testingAll" @click="testAll">
        <Zap :size="14" :class="{ spin: testingAll }" />
        {{ testingAll ? "测试中..." : "全面测试" }}
      </button>
    </template>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
      <div
        v-for="target in testTargets"
        :key="target.name"
        class="test-card"
        @click="testSingle(target.name)"
      >
        <div class="test-header">
          <span class="test-name">{{ target.name }}</span>
          <div class="test-status">
            <RefreshCw v-if="target.status === 'testing'" :size="14" class="spin" :style="{ color: 'var(--accent)' }" />
            <Check v-else-if="target.status === 'success'" :size="14" :style="{ color: 'var(--green)' }" />
            <X v-else-if="target.status === 'fail'" :size="14" :style="{ color: 'var(--red)' }" />
            <Clock v-else :size="14" :style="{ color: 'var(--text-secondary)' }" />
          </div>
        </div>
        <div class="test-delay">
          <span v-if="target.status === 'testing'" class="mono" :style="{ color: 'var(--accent)' }">测试中...</span>
          <span v-else-if="target.status === 'fail'" :style="{ color: 'var(--red)' }">超时</span>
          <span v-else-if="target.delay > 0" class="mono" :style="{ color: delayColor(target.delay) }">{{ target.delay }} ms</span>
          <span v-else :style="{ color: 'var(--text-secondary)' }">等待测试</span>
        </div>
      </div>
    </div>
  </BasePage>
</template>

<style scoped>
.test-card {
  padding: 16px;
  border-radius: 12px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
  cursor: pointer;
  transition: border-color 150ms ease, background-color 150ms ease;
}
.test-card:hover {
  border-color: var(--accent);
  background-color: var(--bg-hover);
}

.test-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.test-name {
  font-size: 14px;
  font-weight: 600;
}

.test-status {
  display: flex;
  align-items: center;
}

.test-delay {
  font-size: 13px;
}
</style>
