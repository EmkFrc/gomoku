<script setup lang="ts">
/**
 * Décor animé : nuages qui dérivent, brume basse, pétales qui tombent.
 * Lent et peu contrasté — il ne doit jamais accrocher l'œil pendant qu'on
 * réfléchit à un coup. Purement décoratif, donc invisible aux lecteurs d'écran.
 */
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    variant?: "menu" | "game";
    enabled?: boolean;
  }>(),
  { variant: "menu", enabled: true },
);

const TINTS = ["#F08A72", "#F7D77A", "#FDF8EC", "#D64933"];

/** 18 pétales aux trajectoires décalées, calculés une seule fois. */
const petals = Array.from({ length: 18 }, (_, i) => {
  const seed = (i * 100) / 18 + ((i * 37) % 5) - 2;
  return {
    key: i,
    left: `${Math.max(1, Math.min(96, seed))}%`,
    width: `${7 + (i % 3) * 3}px`,
    height: `${10 + (i % 4) * 3}px`,
    background: TINTS[i % 4],
    animation: `gk-fall ${11 + (i % 5) * 2.4}s linear ${((i * 0.83) % 12).toFixed(2)}s infinite`,
  };
});

const isMenu = computed(() => props.variant === "menu");
</script>

<template>
  <div v-if="enabled" class="ambient" :class="`is-${variant}`" aria-hidden="true">
    <template v-if="isMenu">
      <div class="cloud" style="left: 5%; top: 96px; width: 300px; height: 48px; opacity: 0.14; animation-duration: 17s" />
      <div class="cloud reverse" style="left: 70%; top: 62px; width: 210px; height: 40px; opacity: 0.12; animation-duration: 23s" />
      <div class="cloud" style="left: 58%; top: 392px; width: 340px; height: 52px; opacity: 0.1; animation-duration: 29s" />

      <svg class="mist" viewBox="0 0 1200 180" preserveAspectRatio="none">
        <path
          d="M0 170 C120 120 220 156 340 128 C460 100 540 146 660 122 C790 96 880 150 1010 126 C1100 110 1150 150 1200 138 L1200 180 L0 180 Z"
          fill="#F7D77A"
          opacity="0.09"
        />
      </svg>
    </template>

    <template v-else>
      <div class="cloud" style="left: 12%; top: 150px; width: 320px; height: 44px; opacity: 0.09; animation-duration: 27s" />
      <div class="cloud reverse" style="right: 10%; top: 300px; width: 240px; height: 36px; opacity: 0.08; animation-duration: 33s" />
    </template>

    <div class="petals">
      <span
        v-for="petal in petals"
        :key="petal.key"
        :style="{
          left: petal.left,
          width: petal.width,
          height: petal.height,
          background: petal.background,
          animation: petal.animation,
        }"
      />
    </div>
  </div>
</template>

<style scoped>
.ambient {
  position: absolute;
  inset: 0;
  pointer-events: none;
  overflow: hidden;
}

.ambient.is-game {
  opacity: 0.8;
}

.cloud {
  position: absolute;
  border-radius: 999px;
  background: var(--gold-300);
  animation-name: gk-drift;
  animation-timing-function: ease-in-out;
  animation-iteration-count: infinite;
  animation-direction: alternate;
  will-change: transform;
}

.cloud.reverse {
  animation-direction: alternate-reverse;
}

.mist {
  position: absolute;
  left: -6%;
  bottom: 40px;
  width: 112%;
  height: 190px;
  animation: gk-mist 31s ease-in-out infinite alternate;
  will-change: transform;
}

.petals {
  position: absolute;
  inset: 0;
  overflow: hidden;
}

.petals span {
  position: absolute;
  top: 0;
  border-radius: 70% 0 70% 0;
  opacity: 0.5;
  will-change: transform, opacity;
}
</style>
