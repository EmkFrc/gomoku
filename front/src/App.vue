<script setup lang="ts">
import MenuView from "./views/MenuView.vue";
import GameView from "./views/GameView.vue";
import { useGame } from "./stores/game";
import { hasBackend } from "./api";

const { screen } = useGame();
</script>

<template>
  <div class="app">
    <MenuView v-if="screen === 'menu'" key="menu" />
    <GameView v-else key="game" />

    <!-- Rappel discret quand on tourne sans le moteur Rust (navigateur). -->
    <span v-if="!hasBackend" class="mock-flag">moteur de secours</span>
  </div>
</template>

<style scoped>
.app {
  position: relative;
  width: 100%;
  height: 100%;
}

.mock-flag {
  position: fixed;
  right: 10px;
  bottom: 8px;
  z-index: 50;
  padding: 3px 9px;
  border-radius: 999px;
  background: rgba(20, 19, 26, 0.6);
  color: var(--paper-300);
  font-family: var(--font-mono);
  font-size: 10px;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  pointer-events: none;
}

</style>
