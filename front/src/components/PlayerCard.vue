<script setup lang="ts">
/**
 * Fiche d'un joueur : portrait, nom, état du tour, paires capturées.
 * Quand l'IA réfléchit, un halo cinabre pulse derrière la carte et trois
 * points s'animent — c'est le seul endroit de l'écran qui s'agite vraiment.
 */
import { computed } from "vue";
import type { PlayerState } from "../types";

const props = withDefaults(
  defineProps<{
    player: PlayerState;
    portrait: string;
    active?: boolean;
    thinking?: boolean;
    /** Libellé d'état : « Réflexion… », « À son tour », « En attente ». */
    status?: string;
  }>(),
  { active: false, thinking: false, status: "" },
);

const CAPTURE_GOAL = 5;

const slots = computed(() =>
  Array.from({ length: CAPTURE_GOAL }, (_, i) => i < props.player.capturedPairs),
);

const statusLabel = computed(
  () => props.status || (props.active ? "À son tour" : "En attente"),
);

/** « Lin — Noir » pour un humain, « Gardien — IA » pour la machine. */
const title = computed(() => {
  const suffix = props.player.isAi ? "IA" : props.player.stone === "black" ? "Noir" : "Blanc";
  return `${props.player.name} — ${suffix}`;
});
</script>

<template>
  <section class="player panel" :class="{ active, dim: !active }">
    <div v-if="thinking" class="aura" aria-hidden="true" />

    <header class="head">
      <div class="portrait">
        <img :src="portrait" alt="" />
      </div>

      <div class="identity">
        <div class="name-row">
          <span class="stone-dot" :class="player.stone" />
          <span class="name">{{ title }}</span>
        </div>
        <span class="status" :class="{ live: active }">
          <span class="dot" />{{ statusLabel }}
        </span>
      </div>

      <div v-if="thinking" class="dots" aria-hidden="true">
        <span /><span /><span />
      </div>
    </header>

    <div class="captures">
      <div class="captures-head">
        <span class="label">Paires capturées</span>
        <span class="label count tabular">{{ player.capturedPairs }} / {{ CAPTURE_GOAL }}</span>
      </div>
      <div class="slots">
        <span
          v-for="(filled, i) in slots"
          :key="i"
          class="slot"
          :class="{ filled }"
          :style="filled ? { background: player.stone === 'black' ? 'var(--stone-white-fill)' : 'var(--stone-black-fill)' } : undefined"
        />
      </div>
    </div>
  </section>
</template>

<style scoped>
.player {
  position: relative;
  padding: 18px;
  overflow: hidden;
  transition: opacity 0.25s ease, transform 0.25s ease;
}

.player.dim {
  opacity: 0.62;
  transform: translateY(4px);
}

.aura {
  position: absolute;
  left: -40px;
  top: -40px;
  width: 200px;
  height: 200px;
  border-radius: 999px;
  background: var(--cinnabar-300);
  animation: gk-aura 1.6s ease-in-out infinite;
  will-change: transform, opacity;
  pointer-events: none;
}

.head {
  position: relative;
  display: flex;
  align-items: center;
  gap: 12px;
}

.portrait {
  flex: none;
  width: 88px;
  height: 88px;
  border: var(--stroke);
  border-radius: 20px;
  background: var(--paper-200);
  box-shadow: var(--shadow-cel);
  overflow: hidden;
}

.portrait img {
  width: 100%;
  height: 170%;
  object-fit: cover;
  object-position: 50% 3%;
  display: block;
}

.identity {
  display: flex;
  flex-direction: column;
  gap: 5px;
  min-width: 0;
}

.name-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.name-row .stone-dot {
  width: 24px;
  height: 24px;
}

.name {
  font-size: 23px;
  font-weight: 700;
  color: var(--ink-900);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.status {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  align-self: flex-start;
  padding: 3px 9px;
  border: var(--stroke-thin);
  border-radius: 999px;
  background: var(--paper-200);
  font-family: var(--font-mono);
  font-size: 12px;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--ink-700);
}

.status .dot {
  width: 7px;
  height: 7px;
  background: var(--ink-300);
}

.status.live {
  background: var(--cinnabar-500);
  color: var(--paper-50);
}

.status.live .dot {
  background: var(--gold-300);
}

.dots {
  margin-left: auto;
  display: flex;
  gap: 4px;
  align-items: flex-end;
  padding-bottom: 6px;
}

.dots span {
  width: 7px;
  height: 7px;
  border-radius: 999px;
  background: var(--ink-700);
  animation: gk-dots 1.2s ease-in-out infinite;
}

.dots span:nth-child(2) { animation-delay: 0.2s; }
.dots span:nth-child(3) { animation-delay: 0.4s; }

.captures {
  position: relative;
  margin-top: 14px;
  padding-top: 12px;
  border-top: 2.5px dashed var(--paper-300);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.captures-head {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
}

.count {
  font-size: 15px;
  letter-spacing: 0;
  color: var(--ink-700);
}

.slots {
  display: flex;
  gap: 7px;
}

.slot {
  width: 34px;
  height: 34px;
  border-radius: 999px;
  background: var(--paper-200);
  border: 2.5px dashed var(--paper-300);
  transition: transform 0.18s ease;
}

.slot.filled {
  border: var(--stroke);
  box-shadow: var(--shadow-cel);
  animation: gk-drop 0.2s ease-out;
}
</style>
