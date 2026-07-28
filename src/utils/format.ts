export function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  const val = bytes / Math.pow(1024, i);
  return `${val.toFixed(i === 0 ? 0 : 1)} ${sizes[i]}`;
}

export function formatSpeed(bytesPerSec: number): string {
  return `${formatBytes(bytesPerSec)}/s`;
}

export function formatDelay(ms: number): string {
  if (ms === 0) return "N/A";
  if (ms < 100) return `${ms}ms`;
  if (ms < 500) return `${ms}ms`;
  return `${ms}ms`;
}

export function delayQuality(ms: number): "good" | "medium" | "bad" | "none" {
  if (ms === 0) return "none";
  if (ms < 150) return "good";
  if (ms < 500) return "medium";
  return "bad";
}

export function formatTime(ts: number): string {
  const d = new Date(ts);
  return d.toLocaleTimeString("zh-CN", { hour12: false });
}

export function formatDate(ts: number): string {
  const d = new Date(ts);
  return d.toLocaleString("zh-CN", { hour12: false });
}