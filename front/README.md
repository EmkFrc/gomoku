# Front — Gomoku · Ninuki

Interface de l'application, d'après la maquette `design/Gomoku - Menu et Partie.html`.
Vue 3 + TypeScript + Vite, embarqué dans Tauri.

## Lancer

```bash
pnpm install
pnpm dev          # navigateur, avec le moteur de secours TypeScript
pnpm tauri dev    # application desktop, branchée sur le moteur Rust
pnpm build        # vérification des types + build de production
```

Dans le navigateur, une mention « moteur de secours » s'affiche en bas à
droite : le moteur Rust n'est pas là, c'est `src/api/mock.ts` qui joue.

## Organisation

```
src/
  types.ts            contrat partagé avec le Rust (voir BACKEND.md)
  api/index.ts        seul point d'appel des commandes Tauri
  api/mock.ts         moteur de secours TypeScript, pour développer sans Rust
  stores/game.ts      état de l'application, partagé par tous les composants
  style.css           jetons de design, motifs réutilisés, animations
  views/MenuView.vue  écran d'accueil
  views/GameView.vue  écran de partie
  components/         goban, cartes joueurs, panneaux IA, décor, fin de partie
  assets/             images et polices extraites de la maquette
```

## Repères

- **Le contrat avec le moteur est dans [BACKEND.md](BACKEND.md).** Aucune règle
  du jeu n'est implémentée côté TypeScript.
- Le goban est en **SVG** : marge 28, pas de 34, pierre de rayon 15.5 — la
  géométrie de la maquette. Il est jouable entièrement au clavier (flèches,
  Entrée, Ctrl+Z).
- Les couleurs, ombres et polices sont des variables CSS définies une fois dans
  `style.css`. Ne pas écrire de couleur en dur dans un composant.
- Le décor animé est volontairement lent et peu contrasté. Tout se coupe avec
  `prefers-reduced-motion`.
