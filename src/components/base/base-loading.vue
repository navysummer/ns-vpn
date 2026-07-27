<template>
  <div class="loading">
    <div class="loading-item" />
    <div class="loading-item" />
    <div class="loading-item" />
  </div>
</template>

<style scoped>
.loading {
  position: relative;
  display: flex;
  height: 100%;
  min-height: 18px;
  box-sizing: border-box;
  align-items: center;
}

.loading-item {
  box-sizing: border-box;
  width: 6px;
  height: 6px;
  margin: 2px;
  border-radius: 100%;
  animation: loading 0.7s -0.15s infinite linear;
  background: var(--secondary-text);
}

.loading-item:nth-child(2n-1) {
  animation-delay: -0.5s;
}

@keyframes loading {
  50% {
    opacity: 0.2;
    transform: scale(0.75);
  }
  100% {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
