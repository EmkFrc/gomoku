<script setup lang="ts">
/**
 * Fin de partie. Ce n'est pas un écran : le plateau reste visible derrière,
 * avec sa ligne gagnante surlignée. Le bandeau glisse depuis la gauche.
 */
import { computed } from "vue";
import type { GameStatus, Stone } from "../types";

const props = defineProps<{
  status: GameStatus;
  names: Record<Stone, string>;
  moveCount: number;
}>();

const emit = defineEmits<{
  (event: "replay"): void;
  (event: "menu"): void;
  (event: "dismiss"): void;
}>();

const won = computed(() => (props.status.kind === "won" ? props.status : null));

const title = computed(() => (won.value ? "VICTOIRE" : "ÉGALITÉ"));

const subtitle = computed(() => {
  if (!won.value) return "Le goban est plein — personne ne l'emporte.";
  const name = props.names[won.value.winner];
  const color = won.value.winner === "black" ? "Noir" : "Blanc";
  return won.value.reason === "five"
    ? `${name} (${color}) aligne cinq pierres.`
    : `${name} (${color}) capture cinq paires.`;
});

/** Confettis or et pétales cinabre, deux secondes puis ils retombent. */
const confetti = Array.from({ length: 26 }, (_, i) => ({
  key: i,
  left: `${(i * 97) % 100}%`,
  background: i % 3 === 0 ? "var(--cinnabar-500)" : "var(--gold-500)",
  animation: `gk-fall ${2.2 + (i % 5) * 0.5}s linear ${(i % 7) * 0.12}s 1`,
  width: `${6 + (i % 3) * 3}px`,
  height: `${9 + (i % 4) * 3}px`,
}));
</script>

<template>
  <div class="overlay" role="dialog" aria-modal="true" :aria-label="title" @keydown.esc="emit('dismiss')">
    <div class="confetti" aria-hidden="true">
      <span v-for="piece in confetti" :key="piece.key" :style="piece" />
    </div>

    <div class="banner">
      <span class="banner-text">{{ title }}</span>
    </div>

    <div class="card panel">
      <p class="subtitle">{{ subtitle }}</p>
      <p class="count label">{{ moveCount }} coups joués</p>

      <div class="actions">
        <button type="button" class="cel-button is-active" @click="emit('replay')">Rejouer</button>
        <button type="button" class="cel-button" @click="emit('menu')">Menu</button>
      </div>

      <button type="button" class="ghost-link" @click="emit('dismiss')">
        revoir le plateau
      </button>
    </div>
  </div>
</template>

<style scoped>
.overlay {
  position: absolute;
  inset: 0;
  z-index: 20;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 26px;
  background: rgba(43, 14, 16, 0.58);
  backdrop-filter: blur(2px);
}

.confetti {
  position: absolute;
  inset: 0;
  overflow: hidden;
  pointer-events: none;
}

.confetti span {
  position: absolute;
  top: 0;
  border-radius: 70% 0 70% 0;
}

.banner {
  padding: 16px 84px;
  border: var(--stroke);
  border-radius: 14px;
  background: var(--cinnabar-500);
  box-shadow: var(--shadow-cel-lg), var(--shadow-panel);
  animation: gk-banner 0.45s cubic-bezier(0.2, 0.9, 0.3, 1.2) both;
}

.banner-text {
  font-family: var(--font-display);
  font-size: clamp(42px, 5vw, 76px);
  line-height: 1;
  color: var(--gold-300);
  -webkit-text-stroke: 2.5px var(--ink-900);
  letter-spacing: 0.08em;
}

.card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 26px 34px;
  animation: gk-rise 0.35s ease-out 0.18s both;
}

.subtitle {
  margin: 0;
  font-size: 22px;
  font-weight: 700;
  color: var(--ink-900);
}

.count {
  margin: 0;
}

.actions {
  display: flex;
  gap: 10px;
  margin-top: 6px;
}

.actions .cel-button {
  min-width: 160px;
  box-shadow: var(--shadow-cel);
}

.ghost-link {
  font-family: var(--font-mono);
  font-size: 11px;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--ink-500);
  text-decoration: underline;
}
</style>
