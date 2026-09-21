<script setup lang="ts">
/**
 * Panneau d'instrumentation : ce que l'IA a exploré, ce qu'elle en pense,
 * et la bascule de la heatmap heuristique.
 */
import { computed } from "vue";
import type { AiReport } from "../types";

const props = defineProps<{
  report: AiReport | null;
  heatmap: boolean;
}>();

const emit = defineEmits<{ (event: "toggle-heatmap"): void }>();

const nf = new Intl.NumberFormat("fr-FR");

const nodes = computed(() => nf.format(props.report?.nodes ?? 0));
const nodesPerSecond = computed(() => nf.format(props.report?.nodesPerSecond ?? 0));

const evaluation = computed(() => props.report?.evaluation ?? 0);

const evaluationLabel = computed(() => {
  const value = evaluation.value;
  return `${value >= 0 ? "+" : "−"}${Math.abs(value).toFixed(2)}`;
});

/** La barre part du centre : à gauche pour noir, à droite pour l'IA. */
const bar = computed(() => {
  const width = Math.min(Math.abs(evaluation.value), 1) * 50;
  return evaluation.value >= 0
    ? { left: "50%", width: `${width}%` }
    : { left: `${50 - width}%`, width: `${width}%` };
});

const variation = computed(() => props.report?.principalVariation ?? []);
</script>

<template>
  <section class="instrumentation panel">
    <header class="head">
      <span class="label">Instrumentation IA</span>
      <span class="label">▾</span>
    </header>

    <div class="stats">
      <div class="stat">
        <span class="stat-label">Nœuds</span>
        <span class="stat-value tabular">{{ nodes }}</span>
      </div>
      <div class="stat">
        <span class="stat-label">Nœuds / s</span>
        <span class="stat-value tabular">{{ nodesPerSecond }}</span>
      </div>
    </div>

    <div class="evaluation">
      <div class="row">
        <span class="stat-label">Score d'évaluation</span>
        <span class="score tabular">{{ evaluationLabel }}</span>
      </div>
      <div class="gauge">
        <div class="fill" :style="bar" />
        <div class="middle" />
      </div>
      <div class="row ends">
        <span class="stat-label">Avantage noir</span>
        <span class="stat-label">Avantage IA</span>
      </div>
    </div>

    <div class="variation">
      <span class="stat-label">Variante principale</span>
      <div class="moves">
        <span v-for="(move, i) in variation" :key="move + i" class="move" :class="{ first: i === 0 }">
          {{ move }}
        </span>
        <span v-if="!variation.length" class="move empty">en attente</span>
      </div>
    </div>

    <button type="button" class="toggle" :aria-pressed="heatmap" @click="emit('toggle-heatmap')">
      <span class="toggle-text">
        <span class="toggle-title">Heatmap heuristique</span>
        <span class="stat-label">or → cinabre · sous les pierres</span>
      </span>
      <span class="switch" :class="{ on: heatmap }"><span class="knob" /></span>
    </button>
  </section>
</template>

<style scoped>
.instrumentation {
  padding: 18px;
  box-shadow: var(--shadow-cel);
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.head {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.stats {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}

.stat {
  padding: 8px 10px;
  border: var(--stroke-thin);
  border-radius: 12px;
  background: var(--paper-200);
}

.stat-label {
  display: block;
  font-family: var(--font-mono);
  font-size: 11px;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--ink-500);
}

.stat-value {
  font-family: var(--font-mono);
  font-size: 21px;
  font-weight: 700;
  color: var(--ink-900);
}

.evaluation {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.row {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
}

.score {
  font-family: var(--font-mono);
  font-size: 14px;
  font-weight: 700;
  color: var(--cinnabar-700);
}

.gauge {
  position: relative;
  height: 16px;
  border: var(--stroke-thin);
  border-radius: 999px;
  background: var(--paper-200);
  overflow: hidden;
}

.fill {
  position: absolute;
  top: 0;
  bottom: 0;
  background: var(--cinnabar-500);
  transition: left 0.3s ease, width 0.3s ease;
}

.middle {
  position: absolute;
  left: 50%;
  top: 0;
  bottom: 0;
  width: 2px;
  background: var(--ink-900);
}

.variation {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.moves {
  display: flex;
  flex-wrap: wrap;
  gap: 5px;
}

.move {
  padding: 3px 8px;
  border: 2px solid var(--paper-300);
  border-radius: 8px;
  background: var(--paper-200);
  font-family: var(--font-mono);
  font-size: 15px;
  color: var(--ink-700);
}

.move.first {
  border: var(--stroke-thin);
  background: var(--gold-500);
  font-weight: 700;
  color: var(--ink-900);
}

.move.empty {
  border-style: dashed;
  background: none;
  color: var(--ink-500);
}

.toggle {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  padding: 9px 11px;
  border: var(--stroke);
  border-radius: 14px;
  background: var(--paper-200);
  text-align: left;
}

.toggle-text {
  display: flex;
  flex-direction: column;
  line-height: 1.2;
}

.toggle-title {
  font-size: 19px;
  font-weight: 700;
  color: var(--ink-900);
}

.switch {
  position: relative;
  flex: none;
  width: 58px;
  height: 32px;
  border: var(--stroke);
  border-radius: 999px;
  background: var(--paper-300);
  transition: background 0.2s ease;
}

.switch.on {
  background: var(--jade-500);
}

.knob {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 24px;
  height: 24px;
  border-radius: 999px;
  background: var(--ink-900);
  transition: transform 0.2s ease;
}

.switch.on .knob {
  transform: translateX(26px);
}
</style>
