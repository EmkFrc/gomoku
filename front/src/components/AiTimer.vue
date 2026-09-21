<script setup lang="ts">
/**
 * Chronomètre de réflexion de l'IA, sur fond d'encre.
 * Pendant le calcul, des traits de vitesse défilent derrière les chiffres.
 */
import { computed, onUnmounted, ref, watch } from "vue";
import type { AiReport, GameConfig } from "../types";

const props = defineProps<{
  report: AiReport | null;
  config: GameConfig;
  thinking: boolean;
  /** Hotseat : le panneau affiche la durée de la dernière analyse. */
  hotseat: boolean;
}>();

/** Compteur qui monte pendant que l'IA cherche, figé sur le résultat ensuite. */
const live = ref(0);
let started = 0;
let frame = 0;

function tick(): void {
  live.value = (performance.now() - started) / 1000;
  frame = requestAnimationFrame(tick);
}

watch(
  () => props.thinking,
  (value) => {
    cancelAnimationFrame(frame);
    if (value) {
      started = performance.now();
      live.value = 0;
      frame = requestAnimationFrame(tick);
    }
  },
);

onUnmounted(() => cancelAnimationFrame(frame));

const seconds = computed(() => {
  if (props.thinking) return live.value;
  return (props.report?.elapsedMs ?? 0) / 1000;
});

const display = computed(() => seconds.value.toFixed(3));

const depth = computed(() => props.report?.depth ?? 0);
const maxDepth = computed(() => props.report?.maxDepth ?? props.config.aiMaxDepth);

const bars = computed(() =>
  Array.from({ length: maxDepth.value }, (_, i) => {
    if (i < depth.value - 1) return "var(--gold-500)";
    if (i === depth.value - 1) return "var(--gold-700)";
    return "var(--ink-700)";
  }),
);

const title = computed(() =>
  props.hotseat ? "Dernière analyse" : "Minuteur de réflexion IA",
);
</script>

<template>
  <section class="timer">
    <div v-if="thinking" class="speed" aria-hidden="true">
      <span /><span /><span />
    </div>

    <div class="body">
      <div class="row">
        <span class="title">{{ title }}</span>
        <span class="limit">limite {{ Math.round(config.aiTimeLimitMs / 1000) }} s</span>
      </div>

      <div class="value">
        <span class="number tabular">{{ display }}</span>
        <span class="unit">s</span>
      </div>

      <div class="depth">
        <span class="limit">Profondeur</span>
        <div class="bars">
          <span v-for="(color, i) in bars" :key="i" :style="{ background: color }" />
        </div>
        <span class="ratio tabular">{{ depth }}/{{ maxDepth }}</span>
      </div>
    </div>
  </section>
</template>

<style scoped>
.timer {
  position: relative;
  padding: 18px 20px;
  border: var(--stroke);
  border-radius: 20px;
  background: var(--ink-900);
  box-shadow: var(--shadow-cel-lg);
  overflow: hidden;
}

.speed {
  position: absolute;
  inset: 0;
  pointer-events: none;
  opacity: 0.22;
}

.speed span {
  position: absolute;
  left: 0;
  width: 140%;
  height: 2px;
  background: var(--gold-300);
  animation: gk-speed 1.1s linear infinite;
  will-change: transform;
}

.speed span:nth-child(1) { top: 26px; }
.speed span:nth-child(2) { top: 58px; animation-duration: 1.4s; }
.speed span:nth-child(3) { bottom: 22px; animation-duration: 1.25s; }

.body {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.row {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
}

.title {
  font-family: var(--font-mono);
  font-size: 10px;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  color: var(--gold-300);
}

.limit {
  font-family: var(--font-mono);
  font-size: 11px;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--paper-300);
}

.value {
  display: flex;
  align-items: flex-end;
  gap: 8px;
}

.number {
  font-family: var(--font-mono);
  font-size: clamp(38px, 3.4vw, 62px);
  font-weight: 700;
  line-height: 1;
  color: var(--paper-50);
}

.unit {
  font-family: var(--font-mono);
  font-size: 16px;
  color: var(--paper-300);
  padding-bottom: 4px;
}

.depth {
  display: flex;
  align-items: center;
  gap: 8px;
}

.bars {
  display: flex;
  gap: 3px;
  flex: 1;
}

.bars span {
  flex: 1;
  height: 16px;
  border-radius: 2px;
  transition: background 0.2s ease;
}

.ratio {
  font-family: var(--font-mono);
  font-size: 16px;
  font-weight: 700;
  color: var(--paper-50);
}
</style>
