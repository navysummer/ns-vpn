<script setup lang="ts">
import { ref } from "vue";
import { Plus, Download, Upload, MoreHorizontal, Check, FileJson, RefreshCw } from "lucide-vue-next";

interface Profile {
  name: string;
  type: "local" | "remote" | "merge" | "script";
  url?: string;
  active: boolean;
  updated: string;
  nodes: number;
}

const profiles = ref<Profile[]>([
  {
    name: "config.yaml",
    type: "local",
    active: true,
    updated: "2026-01-15 14:30",
    nodes: 45,
  },
  {
    name: "机场订阅",
    type: "remote",
    url: "https://example.com/sub",
    active: false,
    updated: "2026-01-14 10:00",
    nodes: 32,
  },
  {
    name: "规则合并",
    type: "merge",
    active: false,
    updated: "2026-01-13 09:15",
    nodes: 0,
  },
  {
    name: "自定义脚本",
    type: "script",
    active: false,
    updated: "2026-01-12 16:45",
    nodes: 0,
  },
]);

const showAddDialog = ref(false);
const newProfileName = ref("");
const newProfileUrl = ref("");

function activateProfile(name: string) {
  profiles.value.forEach((p) => (p.active = p.name === name));
}

function updateProfile(name: string) {
  // Would call Tauri command to update subscription
  console.log("Updating:", name);
}

function importProfile() {
  // Would use Tauri dialog
}

function typeIcon(type: string) {
  switch (type) {
    case "local":
      return "📄";
    case "remote":
      return "🌐";
    case "merge":
      return "🔀";
    case "script":
      return "📜";
    default:
      return "📄";
  }
}

function typeLabel(type: string) {
  switch (type) {
    case "local":
      return "本地";
    case "remote":
      return "远程";
    case "merge":
      return "合并";
    case "script":
      return "脚本";
    default:
      return type;
  }
}
</script>

<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-semibold">配置</h1>
      <div class="flex items-center gap-2">
        <button class="btn-ghost text-xs" @click="importProfile">
          <Download :size="14" />
          导入
        </button>
        <button class="btn-primary text-xs" @click="showAddDialog = true">
          <Plus :size="14" />
          添加
        </button>
      </div>
    </div>

    <!-- Profile list -->
    <div class="space-y-2">
      <div
        v-for="profile in profiles"
        :key="profile.name"
        class="card flex items-center gap-4 cursor-pointer transition-colors duration-150"
        :style="{
          borderColor: profile.active ? 'var(--accent)' : 'var(--border)',
        }"
        @click="activateProfile(profile.name)"
        @mouseenter="(e) => {
          if (!profile.active)
            (e.currentTarget as HTMLElement).style.borderColor = 'var(--accent)';
        }"
        @mouseleave="(e) => {
          if (!profile.active)
            (e.currentTarget as HTMLElement).style.borderColor = 'var(--border)';
        }"
      >
        <!-- Active indicator -->
        <div class="w-5 shrink-0">
          <Check v-if="profile.active" :size="16" style="color: var(--green)" />
        </div>

        <!-- Icon -->
        <div
          class="w-10 h-10 rounded-lg flex items-center justify-center text-lg shrink-0"
          :style="{
            backgroundColor: profile.active ? 'var(--accent)' : 'var(--bg-tertiary)',
          }"
        >
          <FileJson v-if="!profile.active" :size="18" :style="{ color: 'var(--text-secondary)' }" />
          <FileJson v-else :size="18" color="#fff" />
        </div>

        <!-- Info -->
        <div class="flex-1 min-w-0">
          <div class="text-sm font-medium truncate">{{ profile.name }}</div>
          <div class="flex items-center gap-2 text-xs mt-0.5" :style="{ color: 'var(--text-secondary)' }">
            <span>{{ typeLabel(profile.type) }}</span>
            <span>·</span>
            <span>{{ profile.updated }}</span>
            <span v-if="profile.nodes > 0">·</span>
            <span v-if="profile.nodes > 0">{{ profile.nodes }} 节点</span>
          </div>
        </div>

        <!-- Actions -->
        <div class="flex items-center gap-1 shrink-0">
          <button
            v-if="profile.type === 'remote'"
            class="btn-ghost text-xs"
            @click.stop="updateProfile(profile.name)"
          >
            <RefreshCw :size="12" />
            更新
          </button>
          <button class="btn-ghost p-1" @click.stop>
            <MoreHorizontal :size="14" />
          </button>
        </div>
      </div>
    </div>

    <!-- Add dialog -->
    <div
      v-if="showAddDialog"
      class="fixed inset-0 flex items-center justify-center bg-black/50 z-50"
      @click="showAddDialog = false"
    >
      <div
        class="card w-full max-w-md mx-4 space-y-4"
        :style="{ backgroundColor: 'var(--bg-secondary)' }"
        @click.stop
      >
        <h3 class="text-base font-medium">添加配置</h3>

        <div class="space-y-3">
          <div>
            <label class="text-xs font-medium mb-1 block" :style="{ color: 'var(--text-secondary)' }">
              名称
            </label>
            <input
              v-model="newProfileName"
              placeholder="配置名称"
              class="w-full rounded-lg px-3 py-2 text-sm outline-none border"
              :style="{
                backgroundColor: 'var(--bg-tertiary)',
                color: 'var(--text-primary)',
                borderColor: 'var(--border)',
              }"
            />
          </div>
          <div>
            <label class="text-xs font-medium mb-1 block" :style="{ color: 'var(--text-secondary)' }">
              订阅 URL（可选）
            </label>
            <input
              v-model="newProfileUrl"
              placeholder="https://example.com/sub"
              class="w-full rounded-lg px-3 py-2 text-sm outline-none border"
              :style="{
                backgroundColor: 'var(--bg-tertiary)',
                color: 'var(--text-primary)',
                borderColor: 'var(--border)',
              }"
            />
          </div>
        </div>

        <div class="flex justify-end gap-2">
          <button class="btn-ghost text-xs" @click="showAddDialog = false">取消</button>
          <button class="btn-primary text-xs" @click="showAddDialog = false">添加</button>
        </div>
      </div>
    </div>
  </div>
</template>