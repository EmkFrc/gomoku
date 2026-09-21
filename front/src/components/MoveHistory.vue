<script setup lang="ts">
/**
 * Bandeau d'historique sous le plateau. Le dernier coup est mis en avant en
 * cinabre ; la bande défile toute seule pour le garder visible.
 */
import { nextTick, ref, watch } from "vue";
import type { Move } from "../types";

const props = defineProps<{ moves: Move[] }>();

const track = ref<HTMLElement | null>(null);

watch(
  () => props.moves.length,
  async () => {
    await nextTick();
    const element = track.value;
    if (element) element.scrollLeft = element.scrollWidth;
  },
);
</script>

<template>
  <div class="history">
    <span class="label">Historique</span>

    <div ref="track" class="track">
      <span
        v-for="(move, index) in moves"
        :key="move.index"
        class="entry"
        :class="{ last: index === moves.length - 1 }"
      >
        <span class="stone-dot" :class="move.stone" />
        {{ move.index }} {{ move.notation }}
        <span v-if="move.captures.length" class="taken">×{{ move.captures.length / 2 }}</span>
      </span>

      <span v-if="!moves.length" class="entry empty">aucun coup</span>
      <span v-else class="entry empty">{{ moves.length + 1 }} …</span>
    </div>
  </div>
</template>

<style scoped>
.history {
  padding: 14px 18px;
  border: var(--stroke);
  border-radius: 16px;
  background: var(--paper-100);
  box-shadow: var(--shadow-cel);
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.label {
  flex: none;
}

.track {
  display: flex;
  gap: 6px;
  overflow-x: auto;
  scrollbar-width: none;
  scroll-behavior: smooth;
}

.track::-webkit-scrollbar {
  display: none;
}

.entry {
  display: flex;
  align-items: center;
  gap: 6px;
  flex: none;
  padding: 5px 9px;
  border: var(--stroke-thin);
  border-radius: 999px;
  background: var(--paper-200);
  font-family: var(--font-mono);
  font-size: 15px;
  color: var(--ink-900);
}

.entry .stone-dot {
  width: 10px;
  height: 10px;
  border-width: 1.5px;
}

.entry.last {
  border: var(--stroke);
  background: var(--cinnabar-300);
  font-weight: 700;
}

.entry.empty {
  border: 2px dashed var(--paper-300);
  background: none;
  color: var(--ink-500);
}

.taken {
  font-weight: 700;
  color: var(--cinnabar-700);
}
</style>
