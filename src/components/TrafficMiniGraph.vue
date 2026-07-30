<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";

interface Props {
  width?: number;
  height?: number;
  color?: string;
}

const props = withDefaults(defineProps<Props>(), {
  width: 120,
  height: 32,
  color: "var(--accent)",
});

const uploadSpeed = ref(0);
const downloadSpeed = ref(0);
const history = ref<{ up: number; down: number }[]>([]);
const maxPoints = 30;

let interval: ReturnType<typeof setInterval> | null = null;

function update() {
  const up = Math.random() * 50000;
  const down = Math.random() * 500000;
  uploadSpeed.value = up;
  downloadSpeed.value = down;
  history.value.push({ up, down });
  if (history.value.length > maxPoints) {
    history.value.shift();
  }
}

function formatSpeed(bytes: number): string {
  if (bytes < 1024) return `${bytes.toFixed(0)} B`;
  if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1048576).toFixed(1)} MB`;
}

const pathDown = () => {
  if (history.value.length < 2) return "";
  const w = props.width;
  const h = props.height;
  const maxVal = Math.max(...history.value.map((s) => Math.max(s.up, s.down)), 1);
  const points = history.value.map((s, i) => {
    const x = (i / (history.value.length - 1)) * w;
    const y = h - (s.down / maxVal) * h * 0.9;
    return `${x.toFixed(1)},${y.toFixed(1)}`;
  });
  return points.join(" ");
};

const pathUp = () => {
  if (history.value.length < 2) return "";
  const w = props.width;
  const h = props.height;
  const maxVal = Math.max(...history.value.map((s) => Math.max(s.up, s.down)), 1);
  const points = history.value.map((s, i) => {
    const x = (i / (history.value.length - 1)) * w;
    const y = h - (s.up / maxVal) * h * 0.9;
    return `${x.toFixed(1)},${y.toFixed(1)}`;
  });
  return points.join(" ");
};

onMounted(() => {
  update();
  interval = setInterval(update, 1000);
});

onUnmounted(() => {
  if (interval) clearInterval(interval);
});
</script>

<template>
  <div class="mini-graph">
    <div class="mini-graph-speeds">
      <span class="mini-speed-down">{{ formatSpeed(downloadSpeed) }}/s</span>
      <span class="mini-speed-up">{{ formatSpeed(uploadSpeed) }}/s</span>
    </div>
    <svg
      :width="width"
      :height="height"
      class="mini-graph-svg"
    >
      <polyline
        v-if="history.length > 1"
        :points="pathDown()"
        fill="none"
        stroke="var(--accent)"
        stroke-width="1"
        stroke-linecap="round"
        opacity="0.8"
      />
      <polyline
        v-if="history.length > 1"
        :points="pathUp()"
        fill="none"
        stroke="var(--orange)"
        stroke-width="1"
        stroke-linecap="round"
        opacity="0.6"
      />
    </svg>
  </div>
</template>

<style scoped>
.mini-graph {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.mini-graph-speeds {
  display: flex;
  justify-content: space-between;
  font-size: 10px;
  font-family: "SF Mono", "Fira Code", monospace;
}

.mini-speed-down {
  color: var(--accent);
}

.mini-speed-up {
  color: var(--orange);
}

.mini-graph-svg {
  display: block;
}
</style>