<script setup lang="ts">
/**
 * Écran de partie. Trois colonnes : le joueur noir à gauche, le goban au
 * centre, l'IA et son instrumentation à droite. Le plateau est le héros —
 * tout le reste est périphérique.
 */
import { computed, onMounted, onUnmounted } from "vue";
import AmbientLayer from "../components/AmbientLayer.vue";
import GomokuBoard from "../components/GomokuBoard.vue";
import PlayerCard from "../components/PlayerCard.vue";
import MoveHistory from "../components/MoveHistory.vue";
import AiTimer from "../components/AiTimer.vue";
import AiInstrumentation from "../components/AiInstrumentation.vue";
import VictoryOverlay from "../components/VictoryOverlay.vue";
import { useGame } from "../stores/game";
import type { Coord, Stone } from "../types";

import gameBg from "../assets/img/game-bg.jpg";
import dragon from "../assets/img/dragon.png";
import charLin from "../assets/img/char-lin.png";
import charGardien from "../assets/img/char-gardien.png";

const {
  state,
  ambientMotion,
  aiThinking,
  suggestion,
  suggesting,
  showHeatmap,
  busy,
  finished,
  humanTurn,
  winningLine,
  victoryDismissed,
  place,
  undo,
  askSuggestion,
  replay,
  backToMenu,
  switchMode,
} = useGame();

const config = computed(() => state.value.config);
const hotseat = computed(() => config.value.mode === "hotseat");
const players = computed(() => state.value.players);

const portraits: Record<Stone, string> = { black: charLin, white: charGardien };

const turnLabel = computed(() => (state.value.turn === "black" ? "Noir" : "Blanc"));
const turnOwner = computed(() => players.value[state.value.turn].name);

function statusFor(stone: Stone): string {
  if (finished.value) return "Partie terminée";
  if (state.value.turn !== stone) return "En attente";
  if (players.value[stone].isAi) return aiThinking.value ? "Réflexion…" : "À son tour";
  return "À son tour";
}

const rules = computed(() => [
  { label: "Captures par paires", hot: false },
  { label: "Double-trois interdit", hot: true },
  { label: "5 paires = victoire", hot: false },
  { label: `Ouverture ${config.value.opening}`, hot: false },
]);

const suggestHint = computed(() =>
  hotseat.value ? "Requis en hotseat" : "Analyse à la demande",
);

const showVictory = computed(() => finished.value && !victoryDismissed.value);

const outcomeLabel = computed(() => {
  const status = state.value.status;
  if (status.kind !== "won") return "Égalité";
  return `${players.value[status.winner].name} l'emporte`;
});

const names = computed<Record<Stone, string>>(() => ({
  black: players.value.black.name,
  white: players.value.white.name,
}));

/** Annonce du dernier coup pour les lecteurs d'écran. */
const announcement = computed(() => {
  const last = state.value.moves[state.value.moves.length - 1];
  if (!last) return "Partie lancée. À Noir de jouer.";
  const who = last.stone === "black" ? "Noir" : "Blanc";
  const captures = last.captures.length ? `, ${last.captures.length / 2} paire capturée` : "";
  return `${who} joue ${last.notation}${captures}. À ${turnLabel.value} de jouer.`;
});

function onPlay(coord: Coord): void {
  void place(coord);
}

function onKeydown(event: KeyboardEvent): void {
  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "z") {
    event.preventDefault();
    void undo();
    return;
  }
  if (event.key === "Escape") backToMenu();
}

onMounted(() => window.addEventListener("keydown", onKeydown));
onUnmounted(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
  <div class="game" :style="{ backgroundImage: `url(${gameBg})` }">
    <div class="veil" aria-hidden="true" />
    <AmbientLayer variant="game" :enabled="ambientMotion" />

    <img class="dragon" :src="dragon" alt="" aria-hidden="true" />

    <header class="topbar">
      <div class="brand">
        <span class="mark">五子連珠</span>
        <span class="tag">Gomoku · Ninuki</span>
      </div>

      <div class="turn" :class="{ over: finished }">
        <span class="stone-dot" :class="state.turn" />
        <span class="turn-text">{{ finished ? "Partie terminée" : `Tour : ${turnLabel}` }}</span>
        <span class="turn-owner">{{ finished ? outcomeLabel : turnOwner }}</span>
      </div>

      <div class="tools">
        <button type="button" class="icon" title="Annuler le dernier coup (Ctrl+Z)" :disabled="!state.moves.length || busy" @click="undo">⟲</button>
        <button type="button" class="icon" title="Retour au menu (Échap)" @click="backToMenu">⏸</button>
      </div>
    </header>

    <div class="layout">
      <!-- Colonne gauche : joueur noir -->
      <aside class="column">
        <PlayerCard
          :player="players.black"
          :portrait="portraits.black"
          :active="state.turn === 'black' && !finished"
          :thinking="players.black.isAi && aiThinking"
          :status="statusFor('black')"
        />

        <button
          type="button"
          class="suggest"
          :disabled="suggesting || finished || !humanTurn"
          @click="askSuggestion"
        >
          <span class="suggest-mark">?</span>
          <span class="suggest-text">
            <span class="suggest-title">{{ suggesting ? "Analyse en cours…" : "Suggestion de coup" }}</span>
            <span class="suggest-hint">{{ suggestHint }}</span>
          </span>
        </button>

        <section class="rules panel">
          <span class="label">Règles actives</span>
          <div class="chips">
            <span v-for="rule in rules" :key="rule.label" class="chip" :class="{ hot: rule.hot }">
              {{ rule.label }}
            </span>
          </div>
        </section>

        <section class="mode">
          <span class="label label-light">Mode de partie</span>
          <div class="mode-pair">
            <button
              type="button"
              class="cel-button small"
              :class="{ 'is-active': !hotseat }"
              @click="switchMode('vs-ai')"
            >
              Contre l'IA
            </button>
            <button
              type="button"
              class="cel-button small"
              :class="{ 'is-active': hotseat }"
              @click="switchMode('hotseat')"
            >
              Hotseat
            </button>
          </div>
        </section>
      </aside>

      <!-- Colonne centrale : le goban -->
      <main class="board-column">
        <GomokuBoard
          :board="state.board"
          :turn="state.turn"
          :last-move="state.lastMove"
          :winning-line="winningLine"
          :suggestion="suggestion"
          :heatmap="state.ai?.heatmap ?? []"
          :show-heatmap="showHeatmap"
          :forbidden="state.forbidden"
          :interactive="humanTurn && !busy"
          @play="onPlay"
        />

        <MoveHistory :moves="state.moves" />
      </main>

      <!-- Colonne droite : l'IA -->
      <aside class="column">
        <PlayerCard
          :player="players.white"
          :portrait="portraits.white"
          :active="state.turn === 'white' && !finished"
          :thinking="players.white.isAi && aiThinking"
          :status="statusFor('white')"
        />

        <AiTimer :report="state.ai" :config="config" :thinking="aiThinking" :hotseat="hotseat" />

        <AiInstrumentation
          :report="state.ai"
          :heatmap="showHeatmap"
          @toggle-heatmap="showHeatmap = !showHeatmap"
        />
      </aside>
    </div>

    <VictoryOverlay
      v-if="showVictory"
      :status="state.status"
      :names="names"
      :move-count="state.moves.length"
      @replay="replay"
      @menu="backToMenu"
      @dismiss="victoryDismissed = true"
    />

    <p class="sr-only" role="status" aria-live="polite">{{ announcement }}</p>
  </div>
</template>

<style scoped>
.game {
  animation: gk-screen-in 0.4s cubic-bezier(0.3, 0.9, 0.3, 1);
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
  background-color: var(--indigo-900);
  background-size: cover;
  background-position: 50% 38%;
  background-repeat: no-repeat;
}

.veil {
  position: absolute;
  inset: 0;
  background: linear-gradient(
    180deg,
    rgba(43, 14, 16, 0.62) 0%,
    rgba(43, 14, 16, 0.5) 40%,
    rgba(43, 14, 16, 0.7) 100%
  );
}

/* Barre du haut ------------------------------------------------------ */

.topbar {
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 30px;
  background: var(--indigo-700);
  border-bottom: var(--stroke);
  overflow: hidden;
  z-index: 3;
}

.brand {
  display: flex;
  align-items: center;
  gap: 12px;
}

.mark {
  font-family: var(--font-display);
  font-size: 34px;
  color: var(--gold-500);
  -webkit-text-stroke: 1.5px var(--ink-900);
}

.tag {
  font-family: var(--font-mono);
  font-size: 11px;
  letter-spacing: 0.2em;
  color: var(--paper-300);
  text-transform: uppercase;
}

.turn {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 6px 14px 6px 8px;
  border: var(--stroke);
  border-radius: 999px;
  background: var(--paper-100);
  box-shadow: var(--shadow-cel);
}

.turn .stone-dot {
  width: 26px;
  height: 26px;
}

.turn-text {
  font-size: 19px;
  font-weight: 700;
  color: var(--ink-900);
}

.turn-owner {
  font-family: var(--font-mono);
  font-size: 11px;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--cinnabar-700);
}

.turn.over {
  background: var(--gold-500);
}

.tools {
  display: flex;
  align-items: center;
  gap: 10px;
  position: relative;
  z-index: 5;
}

.dragon {
  position: absolute;
  top: 10px;
  right: 18px;
  width: clamp(140px, 16vw, 300px);
  height: auto;
  opacity: 0.92;
  animation: gk-dragon 21s ease-in-out infinite alternate;
  will-change: transform;
  filter: drop-shadow(0 3px 6px rgba(20, 12, 11, 0.5));
  pointer-events: none;
  z-index: 4;
}

.icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  border: var(--stroke);
  border-radius: 12px;
  background: var(--paper-100);
  color: var(--ink-900);
  font-size: 21px;
  font-weight: 700;
  transition: transform 0.12s ease;
}

.icon:hover:not(:disabled) { transform: translateY(-2px); }
.icon:disabled { opacity: 0.5; cursor: not-allowed; }

/* Grille principale --------------------------------------------------- */

.layout {
  position: absolute;
  left: 28px;
  right: 28px;
  top: 100px;
  bottom: 28px;
  display: grid;
  grid-template-columns: clamp(300px, 20vw, 380px) minmax(0, 1fr) clamp(320px, 21vw, 400px);
  gap: 28px;
  z-index: 2;
}

.column {
  display: flex;
  flex-direction: column;
  gap: 14px;
  min-height: 0;
  overflow-y: auto;
  scrollbar-width: none;
}

.column::-webkit-scrollbar { display: none; }

.board-column {
  display: flex;
  flex-direction: column;
  gap: 12px;
  min-width: 0;
  min-height: 0;
}

.board-column :deep(.board-frame) {
  max-height: calc(100% - 80px);
  max-width: min(840px, calc(100vh - 230px));
}

.board-column :deep(.history) {
  margin-top: auto;
}

/* Colonne gauche ------------------------------------------------------ */

.suggest {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 17px 20px;
  border: var(--stroke);
  border-radius: 16px;
  background: var(--jade-500);
  color: var(--paper-50);
  box-shadow: var(--shadow-cel-lg);
  text-align: left;
  transition: transform 0.14s ease;
}

.suggest:hover:not(:disabled) { transform: translateY(-2px); }
.suggest:active:not(:disabled) { transform: translateY(4px); box-shadow: none; }
.suggest:disabled { opacity: 0.55; cursor: not-allowed; }

.suggest-mark {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 999px;
  border: 2.5px solid var(--paper-50);
  font-size: 19px;
  font-weight: 800;
  flex: none;
}

.suggest-text {
  display: flex;
  flex-direction: column;
  line-height: 1.15;
}

.suggest-title {
  font-size: 21px;
  font-weight: 700;
}

.suggest-hint {
  font-family: var(--font-mono);
  font-size: 10px;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  opacity: 0.85;
}

.rules {
  padding: 18px;
  box-shadow: var(--shadow-cel);
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.chips {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.chip.hot {
  background: var(--cinnabar-300);
}

.mode {
  padding: 16px 18px;
  border: var(--stroke);
  border-radius: 16px;
  background: var(--indigo-700);
  display: flex;
  flex-direction: column;
  gap: 9px;
}

.mode-pair {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 7px;
}

.cel-button.small {
  padding: 9px 0;
  font-size: 17px;
}

/* Écrans étroits : le plateau d'abord, les panneaux en dessous. */
@media (max-width: 1280px) {
  .layout {
    display: flex;
    flex-direction: column;
    gap: 16px;
    left: 16px;
    right: 16px;
    bottom: 16px;
    overflow-y: auto;
    scrollbar-width: none;
  }

  .layout::-webkit-scrollbar { display: none; }

  /* Rien ne rétrécit : la colonne défile, elle ne comprime pas le goban. */
  .layout > * { flex: none; }

  .board-column { order: -1; }

  /* Sans flex: none, la colonne écrase le goban et le cadre le rogne. */
  .board-column :deep(.board-frame) {
    flex: none;
    max-width: min(840px, 92vw);
    max-height: none;
  }

  .column {
    flex-direction: row;
    flex-wrap: wrap;
    align-items: flex-start;
    overflow: visible;
  }

  .column > * { flex: 1 1 280px; }
}
</style>
