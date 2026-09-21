<script setup lang="ts">
/**
 * Le goban. Tout est en SVG : net à n'importe quelle taille, et chaque
 * élément (pierre, halo, heatmap) reste un nœud qu'on peut animer en CSS.
 *
 * Le composant est « bête » : il dessine ce qu'on lui passe et signale les
 * intentions du joueur. Aucune règle du jeu ici.
 */
import { computed, ref, watch } from "vue";
import { sameCoord, toNotation, type Coord, type HeatCell, type Stone } from "../types";

const props = withDefaults(
  defineProps<{
    board: (Stone | null)[][];
    turn: Stone;
    lastMove?: Coord | null;
    winningLine?: Coord[];
    suggestion?: Coord | null;
    heatmap?: HeatCell[];
    showHeatmap?: boolean;
    forbidden?: Coord[];
    /** Faux quand c'est à l'IA de jouer, ou la partie terminée. */
    interactive?: boolean;
  }>(),
  {
    lastMove: null,
    winningLine: () => [],
    suggestion: null,
    heatmap: () => [],
    showHeatmap: false,
    forbidden: () => [],
    interactive: true,
  },
);

const emit = defineEmits<{ (event: "play", coord: Coord): void }>();

/* Géométrie reprise de la maquette : marge 28, pas de 34, pierre r 15.5. */
const PAD = 28;
const CELL = 34;
const STONE_R = 15.5;

const size = computed(() => props.board.length);
const view = computed(() => PAD * 2 + (size.value - 1) * CELL);
const axis = computed(() => Array.from({ length: size.value }, (_, i) => i));

function pos(index: number): number {
  return PAD + index * CELL;
}

/** Points étoiles : 4e, 10e et 16e lignes sur un 19×19. */
const hoshi = computed<Coord[]>(() => {
  const marks = size.value === 19 ? [3, 9, 15] : [3, Math.floor(size.value / 2), size.value - 4];
  const out: Coord[] = [];
  for (const y of marks) for (const x of marks) out.push({ x, y });
  return out;
});

const stones = computed(() => {
  const out: { coord: Coord; stone: Stone; key: string }[] = [];
  props.board.forEach((row, y) =>
    row.forEach((stone, x) => {
      if (stone) out.push({ coord: { x, y }, stone, key: `${x}-${y}-${stone}` });
    }),
  );
  return out;
});

const hovered = ref<Coord | null>(null);
/** Curseur clavier : n'apparaît qu'après une première touche fléchée. */
const cursor = ref<Coord | null>(null);
/** Coordonnée qui vient d'être refusée — déclenche la secousse. */
const rejected = ref<Coord | null>(null);

const focusCell = computed(() => cursor.value ?? hovered.value);

function isEmpty(coord: Coord): boolean {
  return !props.board[coord.y]?.[coord.x];
}

function isForbidden(coord: Coord): boolean {
  return props.forbidden.some((cell) => sameCoord(cell, coord));
}

function canPlay(coord: Coord): boolean {
  return props.interactive && isEmpty(coord) && !isForbidden(coord);
}

function attempt(coord: Coord): void {
  if (canPlay(coord)) {
    emit("play", coord);
    return;
  }
  if (!props.interactive) return;
  // Coup illégal : le fantôme tremble et rougit, pas de boîte de dialogue.
  rejected.value = coord;
  window.setTimeout(() => {
    if (sameCoord(rejected.value, coord)) rejected.value = null;
  }, 260);
}

function onEnter(coord: Coord): void {
  hovered.value = coord;
  cursor.value = null;
}

function onLeave(): void {
  hovered.value = null;
}

function moveCursor(dx: number, dy: number): void {
  const base = cursor.value ?? props.lastMove ?? { x: Math.floor(size.value / 2), y: Math.floor(size.value / 2) };
  cursor.value = {
    x: Math.max(0, Math.min(size.value - 1, base.x + dx)),
    y: Math.max(0, Math.min(size.value - 1, base.y + dy)),
  };
  hovered.value = null;
}

function onKeydown(event: KeyboardEvent): void {
  switch (event.key) {
    case "ArrowLeft": moveCursor(-1, 0); break;
    case "ArrowRight": moveCursor(1, 0); break;
    case "ArrowUp": moveCursor(0, -1); break;
    case "ArrowDown": moveCursor(0, 1); break;
    case "Enter":
    case " ":
      if (cursor.value) attempt(cursor.value);
      break;
    default:
      return;
  }
  event.preventDefault();
}

const ghost = computed(() => {
  const cell = focusCell.value;
  if (!cell || !props.interactive || !isEmpty(cell)) return null;
  return { coord: cell, illegal: isForbidden(cell) };
});

const heatCells = computed(() =>
  props.showHeatmap
    ? props.heatmap.map((cell) => ({
        ...cell,
        // Or pour une menace tiède, cinabre pour une menace brûlante.
        fill: cell.weight > 0.6 ? "#D64933" : "#E8B04B",
        opacity: 0.22 + cell.weight * 0.45,
      }))
    : [],
);

const winSegment = computed(() => {
  const line = props.winningLine;
  if (line.length < 2) return null;
  const sorted = [...line].sort((a, b) => a.y - b.y || a.x - b.x);
  const from = sorted[0];
  const to = sorted[sorted.length - 1];
  return { x1: pos(from.x), y1: pos(from.y), x2: pos(to.x), y2: pos(to.y) };
});

/** L'onde de pose est rejouée à chaque nouveau dernier coup. */
const rippleKey = ref(0);
watch(
  () => props.lastMove,
  (value) => {
    if (value) rippleKey.value += 1;
  },
);

function stoneLabel(coord: Coord, stone: Stone): string {
  return `${stone === "black" ? "Noir" : "Blanc"} en ${toNotation(coord, size.value)}`;
}
</script>

<template>
  <div class="board-frame">
    <svg
      :viewBox="`0 0 ${view} ${view}`"
      class="board"
      tabindex="0"
      role="application"
      :aria-label="`Goban ${size} par ${size}. Flèches pour déplacer le curseur, Entrée pour poser une pierre.`"
      @keydown="onKeydown"
      @mouseleave="onLeave"
    >
      <!-- Bois et cadre -->
      <rect :width="view" :height="view" fill="var(--board-wood)" />
      <rect
        x="14"
        y="14"
        :width="view - 28"
        :height="view - 28"
        fill="none"
        stroke="var(--board-wood-deep)"
        stroke-width="6"
      />

      <!-- Quadrillage -->
      <g stroke="var(--board-line)" stroke-width="1" opacity="0.7">
        <line v-for="i in axis" :key="`h${i}`" :x1="PAD" :y1="pos(i)" :x2="view - PAD" :y2="pos(i)" />
        <line v-for="i in axis" :key="`v${i}`" :x1="pos(i)" :y1="PAD" :x2="pos(i)" :y2="view - PAD" />
      </g>
      <rect
        :x="PAD"
        :y="PAD"
        :width="(size - 1) * CELL"
        :height="(size - 1) * CELL"
        fill="none"
        stroke="var(--board-edge)"
        stroke-width="2.5"
      />

      <!-- Points étoiles -->
      <g fill="var(--board-line)">
        <circle v-for="(mark, i) in hoshi" :key="`hoshi${i}`" :cx="pos(mark.x)" :cy="pos(mark.y)" r="4" />
      </g>

      <!-- Heatmap heuristique, sous les pierres -->
      <g v-if="heatCells.length" class="heat">
        <rect
          v-for="(cell, i) in heatCells"
          :key="`heat${i}`"
          :x="pos(cell.coord.x) - CELL / 2"
          :y="pos(cell.coord.y) - CELL / 2"
          :width="CELL"
          :height="CELL"
          :fill="cell.fill"
          :opacity="cell.opacity"
        />
      </g>

      <!-- Pierres -->
      <g>
        <g v-for="item in stones" :key="item.key" class="stone">
          <circle
            :cx="pos(item.coord.x)"
            :cy="pos(item.coord.y)"
            :r="STONE_R"
            :fill="item.stone === 'black' ? 'var(--stone-black-fill)' : 'var(--stone-white-fill)'"
            stroke="var(--ink-900)"
            stroke-width="2.5"
          />
          <!-- Ombre interne des pierres blanches, en bas à droite -->
          <path
            v-if="item.stone === 'white'"
            :d="`M${pos(item.coord.x) + 9} ${pos(item.coord.y) + 8} A11 11 0 0 1 ${pos(item.coord.x) + 3} ${pos(item.coord.y) + 14}`"
            fill="none"
            stroke="var(--stone-white-shade)"
            stroke-width="3.5"
            stroke-linecap="round"
          />
          <!-- Point de lumière, en haut à gauche -->
          <path
            :d="`M${pos(item.coord.x) - 9} ${pos(item.coord.y) - 4} A10 10 0 0 1 ${pos(item.coord.x) - 3} ${pos(item.coord.y) - 10}`"
            fill="none"
            :stroke="item.stone === 'black' ? 'var(--stone-black-spec)' : '#FFFFFF'"
            stroke-width="3.5"
            stroke-linecap="round"
          />
          <title>{{ stoneLabel(item.coord, item.stone) }}</title>
        </g>
      </g>

      <!-- Onde de pose -->
      <circle
        v-if="lastMove"
        :key="rippleKey"
        class="ripple"
        :cx="pos(lastMove.x)"
        :cy="pos(lastMove.y)"
        r="16"
        fill="none"
        stroke="var(--cinnabar-500)"
        stroke-width="3"
      />

      <!-- Dernier coup -->
      <circle
        v-if="lastMove"
        class="marker"
        :cx="pos(lastMove.x)"
        :cy="pos(lastMove.y)"
        r="20"
        fill="none"
        stroke="var(--cinnabar-500)"
        stroke-width="2.5"
      />

      <!-- Suggestion de coup -->
      <g v-if="suggestion" class="suggestion">
        <circle
          :cx="pos(suggestion.x)"
          :cy="pos(suggestion.y)"
          r="17"
          fill="none"
          stroke="var(--jade-500)"
          stroke-width="2.5"
          stroke-dasharray="6 5"
        />
        <circle :cx="pos(suggestion.x)" :cy="pos(suggestion.y)" r="5" fill="var(--jade-500)" />
      </g>

      <!-- Ligne gagnante -->
      <line
        v-if="winSegment"
        class="win-line"
        v-bind="winSegment"
        stroke="var(--cinnabar-500)"
        stroke-width="7"
        stroke-linecap="round"
        opacity="0.85"
      />

      <!-- Pierre fantôme + repère de l'intersection visée -->
      <g v-if="ghost" :class="['ghost', { illegal: sameCoord(rejected, ghost.coord) || ghost.illegal }]">
        <rect
          :x="pos(ghost.coord.x) - CELL / 2"
          :y="pos(ghost.coord.y) - CELL / 2"
          :width="CELL"
          :height="CELL"
          rx="5"
          fill="none"
          stroke="var(--gold-300)"
          stroke-width="2.5"
        />
        <circle
          :cx="pos(ghost.coord.x)"
          :cy="pos(ghost.coord.y)"
          :r="STONE_R"
          :fill="turn === 'black' ? 'var(--stone-black-fill)' : 'var(--stone-white-fill)'"
          stroke="var(--ink-900)"
          stroke-width="2.5"
        />
      </g>

      <!-- Zones sensibles : une case pleine par intersection -->
      <g v-if="interactive">
        <template v-for="y in axis" :key="`hit-row-${y}`">
          <rect
            v-for="x in axis"
            :key="`hit-${x}-${y}`"
            class="hit"
            :x="pos(x) - CELL / 2"
            :y="pos(y) - CELL / 2"
            :width="CELL"
            :height="CELL"
            fill="transparent"
            @mouseenter="onEnter({ x, y })"
            @click="attempt({ x, y })"
          />
        </template>
      </g>
    </svg>
  </div>
</template>

<style scoped>
.board-frame {
  width: 100%;
  max-width: 840px;
  margin: 0 auto;
  border: var(--stroke);
  border-radius: 14px;
  background: var(--board-wood);
  box-shadow: var(--shadow-cel-lg);
  overflow: hidden;
}

.board {
  width: 100%;
  height: auto;
  display: block;
}

.hit {
  cursor: pointer;
}

.stone {
  animation: gk-drop 0.16s ease-out;
  transform-box: fill-box;
  transform-origin: center;
}

.ripple {
  animation: gk-ripple-scale 0.36s ease-out forwards;
  transform-box: fill-box;
  transform-origin: center;
  pointer-events: none;
}

@keyframes gk-ripple-scale {
  from { transform: scale(0.5); opacity: 0.75; }
  to { transform: scale(2.1); opacity: 0; }
}

.marker {
  animation: gk-marker 2.4s ease-in-out infinite;
  pointer-events: none;
}

.suggestion {
  animation: gk-marker 1.1s ease-in-out infinite;
  pointer-events: none;
}

.win-line {
  pointer-events: none;
  stroke-dasharray: 600;
  stroke-dashoffset: 600;
  animation: gk-draw 0.45s ease-out forwards, gk-marker 1.6s ease-in-out 0.45s infinite;
}

@keyframes gk-draw {
  to { stroke-dashoffset: 0; }
}

.ghost {
  opacity: 0.38;
  pointer-events: none;
}

.ghost.illegal circle {
  fill: var(--cinnabar-500);
}

.ghost.illegal {
  opacity: 0.6;
  animation: gk-shake 0.22s ease-in-out 2;
}
</style>
