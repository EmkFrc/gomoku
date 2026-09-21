<script setup lang="ts">
/**
 * Écran d'accueil : le décor porte l'ambiance, la carte de papier porte les
 * réglages. Un seul bouton rouge, et il ne fait qu'une chose — lancer.
 */
import { onMounted, onUnmounted } from "vue";
import AmbientLayer from "../components/AmbientLayer.vue";
import { useGame } from "../stores/game";
import type { Opening } from "../types";

import menuBg from "../assets/img/menu-bg.jpg";
import charLin from "../assets/img/char-lin.png";
import charGardien from "../assets/img/char-gardien.png";

const { draft, ambientMotion, busy, start, setMode, setAiColor, setOpening } = useGame();

const OPENINGS: { id: Opening; label: string }[] = [
  { id: "std", label: "std" },
  { id: "pro", label: "pro" },
  { id: "swap", label: "swap" },
  { id: "swap2", label: "swap2" },
];

function onKeydown(event: KeyboardEvent): void {
  if (event.key === "Enter" && !busy.value) void start();
}

onMounted(() => window.addEventListener("keydown", onKeydown));
onUnmounted(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
  <div class="menu" :style="{ backgroundImage: `url(${menuBg})` }">
    <div class="veil" aria-hidden="true" />

    <AmbientLayer variant="menu" :enabled="ambientMotion" />

    <img class="character left" :src="charLin" alt="Lin, la calligraphe" />
    <img class="character right" :src="charGardien" alt="Le Gardien" />

    <div class="badges">
      <span class="badge">19 × 19</span>
      <span class="badge">Profondeur {{ draft.aiMaxDepth }}</span>
    </div>

    <div class="content">
      <header class="title-block">
        <h1 class="title">五子連珠</h1>
        <p class="subtitle">
          <span class="rule" />
          <span>Gomoku · Ninuki</span>
          <span class="rule" />
        </p>
      </header>

      <section class="config panel">
        <div class="field">
          <span class="label">Mode de jeu</span>
          <div class="pair">
            <button
              type="button"
              class="cel-button"
              :class="{ 'is-active': draft.mode === 'vs-ai' }"
              :aria-pressed="draft.mode === 'vs-ai'"
              @click="setMode('vs-ai')"
            >
              <span class="dot" :class="draft.mode === 'vs-ai' ? 'on' : 'off'" />Contre l'IA
            </button>
            <button
              type="button"
              class="cel-button"
              :class="{ 'is-active': draft.mode === 'hotseat' }"
              :aria-pressed="draft.mode === 'hotseat'"
              @click="setMode('hotseat')"
            >
              <span class="dot" :class="draft.mode === 'hotseat' ? 'on' : 'off'" />Hotseat
            </button>
          </div>
        </div>

        <div class="columns">
          <div class="field">
            <span class="label">Couleur de l'IA</span>
            <div class="pair">
              <button
                type="button"
                class="cel-button small"
                :class="{ 'is-gold': draft.aiColor === 'black' }"
                :disabled="draft.mode === 'hotseat'"
                :aria-pressed="draft.aiColor === 'black'"
                @click="setAiColor('black')"
              >
                <span class="stone-dot black" />Noir
              </button>
              <button
                type="button"
                class="cel-button small"
                :class="{ 'is-gold': draft.aiColor === 'white' }"
                :disabled="draft.mode === 'hotseat'"
                :aria-pressed="draft.aiColor === 'white'"
                @click="setAiColor('white')"
              >
                <span class="stone-dot white" />Blanc
              </button>
            </div>
          </div>

          <div class="field">
            <span class="label">Ouverture</span>
            <div class="openings">
              <button
                v-for="item in OPENINGS"
                :key="item.id"
                type="button"
                class="cel-button small"
                :class="{ 'is-jade': draft.opening === item.id }"
                :aria-pressed="draft.opening === item.id"
                @click="setOpening(item.id)"
              >
                {{ item.label }}
              </button>
            </div>
          </div>
        </div>
      </section>

      <div class="launch">
        <button type="button" class="play" :disabled="busy" @click="start">
          <span class="play-dot" />
          {{ busy ? "…" : "JOUER" }}
        </button>
        <p class="hint">Entrée pour lancer · Échap pour quitter</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.menu {
  animation: gk-screen-in 0.4s cubic-bezier(0.3, 0.9, 0.3, 1);
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
  /* Le décor est un fond de conteneur et non une <img> : une image posée en
     élément se fait écraser par le calque composité de l'ambiance. */
  background-color: var(--indigo-900);
  background-size: cover;
  background-position: 50% 40%;
  background-repeat: no-repeat;
}

.veil {
  position: absolute;
  inset: 0;
  background: linear-gradient(
    180deg,
    rgba(43, 14, 16, 0.44) 0%,
    rgba(43, 14, 16, 0.14) 34%,
    rgba(43, 14, 16, 0.66) 100%
  );
}

.character {
  position: absolute;
  bottom: -1.7vh;
  width: auto;
  filter: drop-shadow(0 14px 18px rgba(20, 12, 11, 0.55));
  pointer-events: none;
}

.character.left {
  left: 1.8vh;
  height: 70vh;
}

.character.right {
  right: 1vh;
  height: 74vh;
}

.badges {
  position: absolute;
  left: 2vw;
  top: 3.3vh;
  display: flex;
  gap: 10px;
}

.badge {
  padding: 5px 10px;
  border: var(--stroke);
  border-radius: 999px;
  background: var(--paper-100);
  color: var(--ink-700);
  font-family: var(--font-mono);
  font-size: 11px;
  letter-spacing: 0.14em;
  text-transform: uppercase;
}

.content {
  position: absolute;
  inset: 0;
  top: 11vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 3.3vh;
  padding: 0 16px;
}

.title-block {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
}

.title {
  margin: 0;
  font-family: var(--font-display);
  font-size: clamp(56px, 8.5vh, 92px);
  line-height: 1.05;
  color: var(--gold-500);
  -webkit-text-stroke: 2.5px var(--ink-900);
  letter-spacing: 0.06em;
}

.subtitle {
  margin: 0;
  display: flex;
  align-items: center;
  gap: 14px;
  font-family: var(--font-mono);
  font-size: clamp(13px, 1.7vh, 18px);
  letter-spacing: 0.34em;
  color: var(--paper-200);
  text-transform: uppercase;
  text-shadow: 0 2px 8px rgba(20, 12, 11, 0.85);
}

.rule {
  width: 36px;
  height: 2.5px;
  background: var(--cinnabar-500);
}

.config {
  width: min(720px, 92vw);
  padding: 28px 30px 30px;
  display: flex;
  flex-direction: column;
  gap: 22px;
  box-shadow: var(--shadow-cel-lg), var(--shadow-panel);
}

.field {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.columns {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 18px;
}

.pair {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}

.openings {
  display: flex;
  gap: 6px;
}

.openings .cel-button {
  flex: 1;
  padding: 12px 0;
}

.cel-button.small {
  padding: 12px;
  font-size: 18px;
}

.cel-button.is-gold {
  background: var(--gold-500);
  box-shadow: var(--shadow-cel);
}

.cel-button.is-jade {
  background: var(--jade-500);
  color: var(--paper-50);
  box-shadow: var(--shadow-cel);
}

.cel-button .dot {
  width: 11px;
  height: 11px;
}

.dot.on { background: var(--paper-50); }
.dot.off { background: var(--ink-300); }

.stone-dot {
  width: 14px;
  height: 14px;
  border-width: 2px;
}

.launch {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.play {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: clamp(14px, 2vh, 22px) clamp(40px, 5vw, 68px);
  border: var(--stroke);
  border-radius: 999px;
  background: var(--cinnabar-500);
  color: var(--paper-50);
  font-family: var(--font-display);
  font-size: clamp(28px, 4vh, 44px);
  line-height: 1;
  box-shadow: var(--shadow-cel-lg);
  transition: transform 0.14s ease, background 0.14s ease;
}

.play:hover:not(:disabled) {
  transform: translateY(-2px) scale(1.02);
  background: var(--cinnabar-300);
  color: var(--ink-900);
}

.play:active:not(:disabled) {
  transform: translateY(4px);
  box-shadow: none;
}

.play:disabled {
  opacity: 0.7;
  cursor: progress;
}

.play-dot {
  width: 16px;
  height: 16px;
  border-radius: 999px;
  background: var(--gold-300);
  border: var(--stroke);
}

.hint {
  margin: 0;
  font-family: var(--font-mono);
  font-size: 12px;
  letter-spacing: 0.14em;
  color: var(--paper-300);
  text-transform: uppercase;
  text-shadow: 0 2px 8px rgba(20, 12, 11, 0.9);
}

/* Sous 1100 px les personnages mangent la carte : on les efface. */
@media (max-width: 1100px) {
  .character { display: none; }
}

@media (max-height: 760px) {
  .content { top: 7vh; gap: 2vh; }
}
</style>
