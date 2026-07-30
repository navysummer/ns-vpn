<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const show = ref(false);
const progress = ref(0);
const status = ref("");
const message = ref("");

let unlisten: (() => void) | null = null;

onMounted(async () => {
  try {
    const { listen } = await import("@tauri-apps/api/event");
    unlisten = await listen<{ status: string; progress: number; message: string }>(
      "core-install-status",
      (e) => {
        progress.value = e.payload.progress;
        status.value = e.payload.status;
        message.value = e.payload.message;

        if (e.payload.status === "checking" || e.payload.status === "downloading" || e.payload.status === "installing") {
          show.value = true;
        }
        if (e.payload.status === "done") {
          setTimeout(() => {
            show.value = false;
          }, 500);
        }
      }
    );
  } catch {
    // not in Tauri env
  }
});

onUnmounted(() => {
  unlisten?.();
});

function statusLabel(s: string): string {
  const map: Record<string, string> = {
    checking: t("coreInstall.checking"),
    downloading: t("coreInstall.downloading"),
    installing: t("coreInstall.installing"),
    done: t("coreInstall.done"),
  };
  return map[s] || s;
}
</script>

<template>
  <Teleport to="body">
    <div v-if="show" class="core-install-overlay">
      <div class="core-install-card">
        <div class="core-install-icon">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="4 17 10 11 4 5" />
            <line x1="12" y1="19" x2="20" y2="19" />
          </svg>
        </div>
        <h3 class="core-install-title">NS VPN</h3>
        <p class="core-install-status">{{ statusLabel(status) }}</p>
        <p class="core-install-message">{{ message }}</p>
        <div class="core-install-bar-track">
          <div class="core-install-bar-fill" :style="{ width: `${progress * 100}%` }"></div>
        </div>
        <p class="core-install-percent">{{ Math.round(progress * 100) }}%</p>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.core-install-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-primary);
}

.core-install-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  text-align: center;
}

.core-install-icon {
  color: var(--accent);
  margin-bottom: 8px;
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.core-install-title {
  font-size: 20px;
  font-weight: 700;
  margin: 0;
  color: var(--text-primary);
}

.core-install-status {
  font-size: 14px;
  font-weight: 500;
  margin: 0;
  color: var(--accent);
}

.core-install-message {
  font-size: 13px;
  margin: 0;
  color: var(--text-secondary);
}

.core-install-bar-track {
  width: 240px;
  height: 4px;
  border-radius: 2px;
  background: var(--bg-tertiary);
  margin-top: 12px;
  overflow: hidden;
}

.core-install-bar-fill {
  height: 100%;
  border-radius: 2px;
  background: var(--accent);
  transition: width 300ms ease;
}

.core-install-percent {
  font-size: 12px;
  color: var(--text-tertiary);
  margin: 0;
}
</style>