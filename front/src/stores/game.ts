/**
 * État de l'application, partagé par tous les composants.
 * Un seul exemplaire pour toute l'app : les `ref` sont créés au niveau du
 * module, pas dans la fonction.
 */

import { computed, reactive, ref } from "vue";
import * as api from "../api";
import {
  DEFAULT_CONFIG,
  sameCoord,
  type Coord,
  type GameConfig,
  type GameState,
  type Stone,
} from "../types";
import { createGame } from "../api/mock";

export type Screen = "menu" | "game";

const screen = ref<Screen>("menu");

/** Réglages en cours d'édition dans le menu, appliqués au lancement. */
const draft = reactive<GameConfig>({ ...DEFAULT_CONFIG });

const state = ref<GameState>(createGame({ ...DEFAULT_CONFIG }));

const busy = ref(false);
const aiThinking = ref(false);
const suggestion = ref<Coord | null>(null);
const suggesting = ref(false);
const hovered = ref<Coord | null>(null);
const showHeatmap = ref(false);
const ambientMotion = ref(true);
const victoryDismissed = ref(false);

const finished = computed(() => state.value.status.kind !== "playing");

const currentPlayer = computed(() => state.value.players[state.value.turn]);

const humanTurn = computed(
  () => !finished.value && !currentPlayer.value.isAi && !aiThinking.value,
);

const winningLine = computed<Coord[]>(() =>
  state.value.status.kind === "won" ? state.value.status.line : [],
);

const forbidden = computed(() => state.value.forbidden ?? []);

function isForbidden(coord: Coord): boolean {
  return forbidden.value.some((cell) => sameCoord(cell, coord));
}

function isPlayable(coord: Coord): boolean {
  if (!humanTurn.value || busy.value) return false;
  if (state.value.board[coord.y][coord.x]) return false;
  return !isForbidden(coord);
}

function adopt(next: GameState): void {
  state.value = next;
  suggestion.value = null;
}

/** Fait jouer l'IA tant que c'est à son tour. */
async function runAiTurns(): Promise<void> {
  while (
    state.value.status.kind === "playing" &&
    state.value.players[state.value.turn].isAi
  ) {
    aiThinking.value = true;
    try {
      adopt(await api.requestAiMove());
    } finally {
      aiThinking.value = false;
    }
  }
}

async function start(): Promise<void> {
  busy.value = true;
  try {
    victoryDismissed.value = false;
    suggestion.value = null;
    hovered.value = null;
    adopt(await api.newGame({ ...draft }));
    screen.value = "game";
    await runAiTurns();
  } finally {
    busy.value = false;
  }
}

async function place(coord: Coord): Promise<void> {
  if (!isPlayable(coord)) return;
  busy.value = true;
  try {
    adopt(await api.playMove(coord));
    await runAiTurns();
  } finally {
    busy.value = false;
  }
}

async function undo(): Promise<void> {
  if (busy.value || aiThinking.value) return;
  busy.value = true;
  try {
    // En vs-IA on remonte de deux demi-coups pour rendre la main au joueur.
    adopt(await api.undo());
    if (state.value.config.mode === "vs-ai" && state.value.players[state.value.turn].isAi) {
      adopt(await api.undo());
    }
    victoryDismissed.value = false;
  } finally {
    busy.value = false;
  }
}

async function askSuggestion(): Promise<void> {
  if (suggesting.value || finished.value) return;
  suggesting.value = true;
  try {
    const result = await api.suggestMove();
    suggestion.value = result.coord;
    state.value.ai = result.report;
  } finally {
    suggesting.value = false;
  }
}

async function replay(): Promise<void> {
  await start();
}

function backToMenu(): void {
  screen.value = "menu";
  victoryDismissed.value = false;
}

function setMode(mode: GameConfig["mode"]): void {
  draft.mode = mode;
}

function setAiColor(stone: Stone): void {
  draft.aiColor = stone;
}

function setOpening(opening: GameConfig["opening"]): void {
  draft.opening = opening;
}

/** Change de mode sans quitter la partie : relance immédiatement. */
async function switchMode(mode: GameConfig["mode"]): Promise<void> {
  if (draft.mode === mode && state.value.config.mode === mode) return;
  draft.mode = mode;
  await start();
}

export function useGame() {
  return {
    // état
    screen,
    draft,
    state,
    busy,
    aiThinking,
    suggestion,
    suggesting,
    hovered,
    showHeatmap,
    ambientMotion,
    victoryDismissed,
    // dérivé
    finished,
    humanTurn,
    currentPlayer,
    winningLine,
    forbidden,
    // actions
    isPlayable,
    isForbidden,
    start,
    place,
    undo,
    askSuggestion,
    replay,
    backToMenu,
    setMode,
    setAiColor,
    setOpening,
    switchMode,
  };
}
