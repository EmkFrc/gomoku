/**
 * Contrat partagé entre l'interface (Vue/TS) et le moteur (Rust).
 * Toute structure échangée via les commandes Tauri est décrite ici.
 * Voir front/BACKEND.md pour la liste des commandes attendues.
 */

export const BOARD_SIZE = 19;

export type Stone = "black" | "white";
export type Mode = "vs-ai" | "hotseat";
export type Opening = "std" | "pro" | "swap" | "swap2";
export type AiMood = "idle" | "thinking" | "confident" | "worried";

export interface Coord {
  x: number; // 0 → 18, colonne (A → T sans I)
  y: number; // 0 → 18, ligne (19 en haut → 1 en bas)
}

export interface GameConfig {
  mode: Mode;
  /** Couleur jouée par l'IA. Ignoré en hotseat. */
  aiColor: Stone;
  opening: Opening;
  boardSize: number;
  /** Profondeur maximale de recherche demandée à l'IA. */
  aiMaxDepth: number;
  /** Budget de réflexion de l'IA, en millisecondes. */
  aiTimeLimitMs: number;
}

export interface Move {
  /** 1-indexé, tel qu'affiché dans l'historique. */
  index: number;
  coord: Coord;
  stone: Stone;
  /** Pierres retirées par ce coup (captures par paires). */
  captures: Coord[];
  /** Notation lisible, ex. « K10 ». */
  notation: string;
}

export interface PlayerState {
  name: string;
  stone: Stone;
  /** Nombre de paires capturées (0 → 5). */
  capturedPairs: number;
  isAi: boolean;
}

export type GameStatus =
  | { kind: "playing" }
  | { kind: "won"; winner: Stone; reason: "five" | "captures"; line: Coord[] }
  | { kind: "draw" };

export interface HeatCell {
  coord: Coord;
  /** 0 → 1. Sous 0.5 l'interface tire vers l'or, au-dessus vers le cinabre. */
  weight: number;
}

export interface AiReport {
  mood: AiMood;
  nodes: number;
  nodesPerSecond: number;
  depth: number;
  maxDepth: number;
  /** -1 (noir gagne) → +1 (l'IA gagne). */
  evaluation: number;
  elapsedMs: number;
  /** Variante principale en notation, ex. ["J10", "J11", "M9"]. */
  principalVariation: string[];
  heatmap: HeatCell[];
}

export interface GameState {
  config: GameConfig;
  /** board[y][x] — null si l'intersection est libre. */
  board: (Stone | null)[][];
  turn: Stone;
  moves: Move[];
  players: Record<Stone, PlayerState>;
  status: GameStatus;
  lastMove: Coord | null;
  /** Intersections interdites au joueur courant (double-trois). */
  forbidden: Coord[];
  /** Null tant que l'IA n'a pas encore réfléchi, ou en hotseat. */
  ai: AiReport | null;
}

export interface Suggestion {
  coord: Coord;
  report: AiReport;
}

export const DEFAULT_CONFIG: GameConfig = {
  mode: "vs-ai",
  aiColor: "white",
  opening: "pro",
  boardSize: BOARD_SIZE,
  aiMaxDepth: 10,
  aiTimeLimitMs: 8000,
};

/** Colonnes de goban : l'usage saute la lettre I. */
const COLUMNS = "ABCDEFGHJKLMNOPQRST";

export function toNotation(coord: Coord, size = BOARD_SIZE): string {
  return `${COLUMNS[coord.x]}${size - coord.y}`;
}

export function sameCoord(a: Coord | null, b: Coord | null): boolean {
  return !!a && !!b && a.x === b.x && a.y === b.y;
}

export function otherStone(stone: Stone): Stone {
  return stone === "black" ? "white" : "black";
}
