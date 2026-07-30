<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import QRCode from "qrcode";
import { X, Download } from "lucide-vue-next";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const props = defineProps<{
  open: boolean;
  value: string;
  title?: string;
}>();

const emit = defineEmits<{
  (e: "close"): void;
}>();

const canvasRef = ref<HTMLCanvasElement | null>(null);

async function generateQR() {
  if (!canvasRef.value || !props.value) return;
  try {
    await QRCode.toCanvas(canvasRef.value, props.value, {
      width: 240,
      margin: 2,
      color: {
        dark: "#000000",
        light: "#ffffff",
      },
    });
  } catch (err) {
    console.error("QR code generation failed:", err);
  }
}

function downloadQR() {
  if (!canvasRef.value) return;
  const link = document.createElement("a");
  link.download = `qr-${props.title || "code"}.png`;
  link.href = canvasRef.value.toDataURL("image/png");
  link.click();
}

watch(() => props.open, (val) => {
  if (val) {
    setTimeout(generateQR, 50);
  }
});

onMounted(() => {
  if (props.open) {
    setTimeout(generateQR, 50);
  }
});
</script>

<template>
  <Teleport to="body">
    <Transition name="page">
      <div v-if="open" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="emit('close')">
        <div class="qr-dialog" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
          <div class="qr-header">
            <h3 class="qr-title">{{ title || t('subscriptions.ctxShareQR') }}</h3>
            <button class="close-btn" @click="emit('close')">
              <X :size="16" />
            </button>
          </div>
          <div class="qr-body">
            <canvas ref="canvasRef" class="qr-canvas" />
            <p class="qr-value">{{ value }}</p>
          </div>
          <div class="qr-footer">
            <button class="btn-ghost text-xs" @click="downloadQR">
              <Download :size="14" />
              {{ t('subscriptions.download') }}
            </button>
            <button class="btn-primary text-xs" @click="emit('close')">
              {{ t('subscriptions.close') }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.qr-dialog {
  width: 380px;
  border-radius: 12px;
  border: 1px solid var(--border);
  box-shadow: 0 16px 48px rgba(0, 0, 0, 0.3);
  animation: dialog-in 0.2s ease-out;
}

@keyframes dialog-in {
  from { opacity: 0; transform: scale(0.95); }
  to { opacity: 1; transform: scale(1); }
}

.qr-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px 12px;
}

.qr-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}

.close-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.qr-body {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 0 20px 16px;
}

.qr-canvas {
  border-radius: 8px;
  border: 1px solid var(--border);
}

.qr-value {
  margin-top: 12px;
  font-size: 11px;
  color: var(--text-secondary);
  word-break: break-all;
  text-align: center;
  max-width: 100%;
  line-height: 1.4;
}

.qr-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 12px 20px;
  border-top: 1px solid var(--border);
}
</style>
