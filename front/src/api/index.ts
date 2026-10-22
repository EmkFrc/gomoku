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

/**
 * Sous Tauri, le moteur Rust est la seule source de vérité : une erreur remonte
 * telle quelle (on ne mélange jamais une partie Rust et une partie de secours).
 * Le moteur de secours ne sert que dans un navigateur nu.
 */
async function call<T>(command: string, args: Record<string, unknown>, fallback: () => T | Promise<T>): Promise<T> {
  if (!hasBackend) return fallback();
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    console.warn(`[api] la commande « ${command} » a échoué :`, error);
    throw error;
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
