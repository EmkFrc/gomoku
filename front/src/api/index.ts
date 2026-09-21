/**
 * Seul point de contact entre l'interface et le moteur.
 *
 * Sous Tauri, chaque fonction appelle la commande Rust du même nom (snake_case).
 * Hors Tauri — `npm run dev` dans un navigateur — elle bascule sur le moteur
 * de secours TypeScript, pour que l'interface reste jouable.
 *
 * Aucun composant Vue n'appelle `invoke` directement : tout passe par ici.
 */

import { invoke } from "@tauri-apps/api/core";
import type { Coord, GameConfig, GameState, Suggestion } from "../types";
import {
  mockAiMove,
  mockGetState,
  mockNewGame,
  mockPlayMove,
  mockSuggest,
  mockUndo,
} from "./mock";

/** Tauri injecte cet objet dans la webview ; un navigateur nu ne l'a pas. */
export const hasBackend = "__TAURI_INTERNALS__" in window;

/** Vrai dès qu'une commande Rust a échoué : on ne retente plus pour rien. */
let backendBroken = false;

export function backendAvailable(): boolean {
  return hasBackend && !backendBroken;
}

async function call<T>(command: string, args: Record<string, unknown>, fallback: () => T | Promise<T>): Promise<T> {
  if (!backendAvailable()) return fallback();
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    backendBroken = true;
    console.warn(
      `[api] la commande « ${command} » a échoué, bascule sur le moteur de secours.`,
      error,
    );
    return fallback();
  }
}

export function newGame(config: GameConfig): Promise<GameState> {
  return call("new_game", { config }, () => mockNewGame(config));
}

export function getState(): Promise<GameState> {
  return call("get_state", {}, () => mockGetState());
}

export function playMove(coord: Coord): Promise<GameState> {
  return call("play_move", { coord }, () => mockPlayMove(coord));
}

export function requestAiMove(): Promise<GameState> {
  return call("request_ai_move", {}, () => mockAiMove());
}

export function undo(): Promise<GameState> {
  return call("undo", {}, () => mockUndo());
}

export function suggestMove(): Promise<Suggestion> {
  return call("suggest_move", {}, () => mockSuggest());
}
