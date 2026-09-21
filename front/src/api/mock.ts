/**
 * Moteur de secours, en TypeScript.
 *
 * Il n'existe que pour que l'interface soit jouable et vérifiable AVANT que le
 * moteur Rust ne soit branché : `npm run dev` dans un navigateur, sans Tauri.
 * Dès que les commandes Tauri répondent, c'est le Rust qui fait foi et ce
 * fichier n'est plus sollicité (voir `src/api/index.ts`).
 *
 * Il implémente volontairement une version simplifiée : captures par paires et
 * victoire à 5 alignés ou 5 paires, mais PAS l'interdiction du double-trois
 * (`forbidden` reste vide) ni les protocoles d'ouverture.
 */

import {
  BOARD_SIZE,
  DEFAULT_CONFIG,
  otherStone,
  toNotation,
  type AiReport,
  type Coord,
  type GameConfig,
  type GameState,
  type Move,
  type Stone,
  type Suggestion,
} from "../types";

type Board = (Stone | null)[][];

const DIRECTIONS: Coord[] = [
  { x: 1, y: 0 },
  { x: 0, y: 1 },
  { x: 1, y: 1 },
  { x: 1, y: -1 },
];

function emptyBoard(size: number): Board {
  return Array.from({ length: size }, () => Array<Stone | null>(size).fill(null));
}

function inside(board: Board, x: number, y: number): boolean {
  return y >= 0 && y < board.length && x >= 0 && x < board.length;
}

function at(board: Board, x: number, y: number): Stone | null {
  return inside(board, x, y) ? board[y][x] : null;
}

/** Paires adverses encadrées par le coup qui vient d'être posé. */
function capturesFor(board: Board, coord: Coord, stone: Stone): Coord[] {
  const foe = otherStone(stone);
  const taken: Coord[] = [];
  for (const d of DIRECTIONS) {
    for (const sign of [1, -1]) {
      const dx = d.x * sign;
      const dy = d.y * sign;
      const a = { x: coord.x + dx, y: coord.y + dy };
      const b = { x: coord.x + dx * 2, y: coord.y + dy * 2 };
      const c = { x: coord.x + dx * 3, y: coord.y + dy * 3 };
      if (
        at(board, a.x, a.y) === foe &&
        at(board, b.x, b.y) === foe &&
        at(board, c.x, c.y) === stone
      ) {
        taken.push(a, b);
      }
    }
  }
  return taken;
}

/** Les 5 pierres alignées passant par `coord`, ou null. */
function winningLine(board: Board, coord: Coord, stone: Stone): Coord[] | null {
  for (const d of DIRECTIONS) {
    const line: Coord[] = [coord];
    for (const sign of [1, -1]) {
      let step = 1;
      while (true) {
        const x = coord.x + d.x * sign * step;
        const y = coord.y + d.y * sign * step;
        if (at(board, x, y) !== stone) break;
        line.push({ x, y });
        step += 1;
      }
    }
    if (line.length >= 5) return line.slice(0, 5);
  }
  return null;
}

/** Valeur d'une intersection vide pour `stone`, sur les 4 directions. */
function cellScore(board: Board, x: number, y: number, stone: Stone): number {
  let total = 0;
  for (const d of DIRECTIONS) {
    let run = 1;
    let openEnds = 0;
    for (const sign of [1, -1]) {
      let step = 1;
      while (at(board, x + d.x * sign * step, y + d.y * sign * step) === stone) {
        run += 1;
        step += 1;
      }
      if (at(board, x + d.x * sign * step, y + d.y * sign * step) === null) openEnds += 1;
    }
    if (openEnds === 0 && run < 5) continue;
    if (run >= 5) total += 1_000_000;
    else if (run === 4) total += openEnds === 2 ? 100_000 : 10_000;
    else if (run === 3) total += openEnds === 2 ? 5_000 : 600;
    else if (run === 2) total += openEnds === 2 ? 300 : 60;
    else total += openEnds === 2 ? 20 : 5;
  }
  return total;
}

/** Intersections vides à portée d'une pierre déjà posée. */
function candidates(board: Board, radius = 2): Coord[] {
  const size = board.length;
  const out: Coord[] = [];
  let occupied = false;
  for (let y = 0; y < size; y += 1) {
    for (let x = 0; x < size; x += 1) {
      if (board[y][x]) {
        occupied = true;
        continue;
      }
      let near = false;
      for (let dy = -radius; dy <= radius && !near; dy += 1) {
        for (let dx = -radius; dx <= radius && !near; dx += 1) {
          if (at(board, x + dx, y + dy)) near = true;
        }
      }
      if (near) out.push({ x, y });
    }
  }
  if (!occupied) {
    const mid = Math.floor(size / 2);
    return [{ x: mid, y: mid }];
  }
  return out;
}

function rankMoves(board: Board, stone: Stone): { coord: Coord; score: number }[] {
  const foe = otherStone(stone);
  return candidates(board)
    .map((coord) => ({
      coord,
      // On attaque, mais on regarde aussi ce que l'adversaire jouerait ici.
      score:
        cellScore(board, coord.x, coord.y, stone) +
        cellScore(board, coord.x, coord.y, foe) * 0.9 +
        capturesFor(board, coord, stone).length * 4_000,
    }))
    .sort((a, b) => b.score - a.score);
}

function buildReport(
  board: Board,
  stone: Stone,
  config: GameConfig,
  elapsedMs: number,
): AiReport {
  const ranked = rankMoves(board, stone).slice(0, 12);
  const best = ranked[0]?.score ?? 0;
  const foeBest = rankMoves(board, otherStone(stone))[0]?.score ?? 0;
  const nodes = Math.round(180_000 + ranked.length * 42_000 + Math.random() * 90_000);
  const spread = Math.max(best, foeBest, 1);

  return {
    mood: best >= foeBest * 1.4 ? "confident" : best < foeBest * 0.7 ? "worried" : "idle",
    nodes,
    nodesPerSecond: Math.round(nodes / Math.max(elapsedMs, 1) * 1000),
    depth: config.aiMaxDepth,
    maxDepth: config.aiMaxDepth,
    evaluation: Math.max(-1, Math.min(1, (best - foeBest) / spread)),
    elapsedMs,
    principalVariation: ranked.slice(0, 6).map((entry) => toNotation(entry.coord, board.length)),
    heatmap: ranked.slice(0, 10).map((entry) => ({
      coord: entry.coord,
      weight: Math.max(0.12, Math.min(1, entry.score / spread)),
    })),
  };
}

function statusFor(board: Board, coord: Coord, stone: Stone, pairs: number) {
  const line = winningLine(board, coord, stone);
  if (line) return { kind: "won" as const, winner: stone, reason: "five" as const, line };
  if (pairs >= 5) {
    return { kind: "won" as const, winner: stone, reason: "captures" as const, line: [] };
  }
  const full = board.every((row) => row.every((cell) => cell !== null));
  return full ? { kind: "draw" as const } : { kind: "playing" as const };
}

/** Partie en cours, conservée en mémoire d'un appel à l'autre. */
let state: GameState = createGame(DEFAULT_CONFIG);
/** Pile des états précédents, pour l'annulation. */
let history: GameState[] = [];

function clone(value: GameState): GameState {
  return JSON.parse(JSON.stringify(value));
}

export function createGame(config: GameConfig): GameState {
  const size = config.boardSize || BOARD_SIZE;
  const aiPlaysBlack = config.mode === "vs-ai" && config.aiColor === "black";
  return {
    config: { ...config, boardSize: size },
    board: emptyBoard(size),
    turn: "black",
    moves: [],
    players: {
      black: {
        name: aiPlaysBlack ? "Gardien" : "Lin",
        stone: "black",
        capturedPairs: 0,
        isAi: aiPlaysBlack,
      },
      white: {
        name: config.mode === "vs-ai" && !aiPlaysBlack ? "Gardien" : "Kai",
        stone: "white",
        capturedPairs: 0,
        isAi: config.mode === "vs-ai" && !aiPlaysBlack,
      },
    },
    status: { kind: "playing" },
    lastMove: null,
    forbidden: [],
    ai: null,
  };
}

export function mockNewGame(config: GameConfig): GameState {
  history = [];
  state = createGame(config);
  return clone(state);
}

export function mockGetState(): GameState {
  return clone(state);
}

export function mockPlayMove(coord: Coord): GameState {
  if (state.status.kind !== "playing") return clone(state);
  if (state.board[coord.y][coord.x]) return clone(state);

  history.push(clone(state));
  const stone = state.turn;
  state.board[coord.y][coord.x] = stone;

  const captured = capturesFor(state.board, coord, stone);
  for (const cell of captured) state.board[cell.y][cell.x] = null;
  state.players[stone].capturedPairs += captured.length / 2;

  const move: Move = {
    index: state.moves.length + 1,
    coord,
    stone,
    captures: captured,
    notation: toNotation(coord, state.board.length),
  };
  state.moves.push(move);
  state.lastMove = coord;
  state.status = statusFor(state.board, coord, stone, state.players[stone].capturedPairs);
  if (state.status.kind === "playing") state.turn = otherStone(stone);
  return clone(state);
}

export async function mockAiMove(): Promise<GameState> {
  if (state.status.kind !== "playing") return clone(state);
  const stone = state.turn;
  const started = performance.now();
  // Un peu de latence : l'écran « Réflexion… » doit être visible au moins un instant.
  await new Promise((resolve) => setTimeout(resolve, 420 + Math.random() * 900));
  const best = rankMoves(state.board, stone)[0];
  if (!best) return clone(state);
  const next = mockPlayMove(best.coord);
  state.ai = buildReport(state.board, stone, state.config, performance.now() - started);
  next.ai = state.ai;
  return clone(state);
}

export function mockUndo(): GameState {
  const previous = history.pop();
  if (previous) state = previous;
  return clone(state);
}

export async function mockSuggest(): Promise<Suggestion> {
  const started = performance.now();
  await new Promise((resolve) => setTimeout(resolve, 260));
  const best = rankMoves(state.board, state.turn)[0];
  const coord = best?.coord ?? { x: 9, y: 9 };
  const report = buildReport(state.board, state.turn, state.config, performance.now() - started);
  return { coord, report };
}
