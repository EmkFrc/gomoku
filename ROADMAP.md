# Roadmap Gomoku

Ce qui reste à faire pour valider le sujet (`en.subject.pdf`), dans l'ordre conseillé.
Chaque phase dit **quoi faire**, **sur quoi se renseigner** et **comment savoir que c'est fini**.

## Où on en est

- [x] Règles complètes dans `back/` : captures, victoire à 10 pierres, ligne cassable, double-trois
- [x] 70 tests moteur (`cd back && cargo test`) et 5 tests du passe-plat (`cd front/src-tauri && cargo test`)
- [x] Interface branchée sur le moteur Rust en mode local (`cd front && pnpm tauri dev`)
- [x] Coups candidats : `candidates()` dans `back/src/minmax.rs`
- [x] `evaluate()` **provisoire** dans `back/src/eval.rs` (signature définitive, contenu à écrire en dernier)
- [ ] IA (obligatoire, y compris pour la suggestion de coup en mode local)

**Stratégie retenue : la machine d'abord, le cerveau ensuite.** On construit toute la recherche
(Min-Max, alpha-bêta, temps, duel IA contre IA, branchement Tauri) avec une heuristique provisoire.
La vraie fonction `evaluate` s'écrit **en dernier** : à ce moment-là, il n'y a plus qu'elle à faire,
et le duel permet de la régler.

## Ce que le sujet exige encore (et vérifiera en soutenance)

| Exigence | Où dans le sujet |
|----------|------------------|
| IA basée sur **Min-Max**, qui s'adapte aux coups du joueur | Mandatory part |
| **≥ 10 niveaux** de profondeur de recherche | Mandatory part |
| **< 0,5 s en moyenne** par coup de l'IA | General guidelines |
| **Timer** visible du temps de réflexion de l'IA | Mandatory part |
| **Suggestion de coup** en mode local (hotseat) | Mandatory part |
| Exécutable nommé **`Gomoku`**, **Makefile** (`$(NAME)`, `all`, `clean`, `fclean`, `re`, pas de relink) | General guidelines |
| **Ne jamais planter** ni quitter de façon inattendue (sinon note 0) | General guidelines |
| Outil de debug pour montrer le raisonnement de l'IA (fortement conseillé) | Mandatory part |
| Bonus (ouvertures Pro, Swap…) : évalués **seulement si l'obligatoire est parfait** | Bonus part |

---

## Phase 0 : petites finitions (½ journée)

- [ ] Aligner les versions de Tauri (message « mismatched Tauri packages ») :
      `pnpm add @tauri-apps/api@~2.12.0 @tauri-apps/plugin-opener@~2.7.0`
- [ ] Afficher les coordonnées (A–T sans I, 1–19) autour du plateau
- [ ] `cargo fmt` + `cargo clippy` dans `back/` et `front/src-tauri/`, puis commit

**Se renseigner :** `cargo clippy` (lire les explications des lints), semver et le `~` / `^` de npm.

## Phase 1 : préparer le moteur pour la recherche

L'IA va jouer et annuler des millions de coups : le moteur doit être rapide et ne proposer que des coups utiles.

- [x] **Coups candidats** : cases vides, à 2 cases max d'une pierre, et légales (`candidates`)
- [x] **`evaluate` provisoire** avec sa signature définitive : `fn evaluate(game: &Game, ai: Stone) -> i32`
- [ ] **Mesurer** : combien de `play` + `undo` par seconde ? combien de candidats en milieu de partie ?

**Se renseigner :**
- *make/unmake move* (chessprogramming.org)
- `std::time::Instant` pour chronométrer ; compiler en `--release` pour mesurer
- *Branching factor* : pourquoi réduire le nombre de candidats change tout

**Fini quand :** tu connais tes chiffres (coups/seconde, nombre de candidats) et les tests passent.

## Phase 2 : Min-Max, puis alpha-bêta (avec l'heuristique provisoire)

Une victoire, une défaite ou un nul ne passent pas par `evaluate` : c'est `play` qui les annonce.
La recherche se teste donc **sans vraie heuristique**.

- [ ] `maxi` / `mini` avec `game.play` → appel récursif → `game.undo`
- [ ] **Fins de partie** : si `play` renvoie un gagnant, on renvoie victoire/défaite sans descendre plus bas
- [ ] **Gagner vite, perdre lentement** : la note de victoire dépend de la profondeur
- [ ] **Aucun coup possible** (plateau plein) : nul, note 0
- [ ] **Meilleur coup à la racine** (pas seulement la note) et **compteur de nœuds**
- [ ] **Alpha-bêta** : même coup et même note que Min-Max, beaucoup moins de nœuds

**Se renseigner :**
- chessprogramming.org : *Minimax*, *Negamax*, *Alpha-Beta*, *Mate score*
- Convention de signe d'`evaluate` (point de vue de l'IA → **pas** de `-` dans `mini`)
- Piège Rust : pas de `i32::MIN` comme « moins l'infini » (`-i32::MIN` déborde) → `INF = 1_000_000_000`

**Fini quand :** à profondeur 2–3 et même avec `evaluate` vide, l'IA gagne en 1 coup, bloque un quatre,
gagne par la 5e paire, préfère la victoire la plus rapide ; alpha-bêta choisit le même coup que Min-Max.

## Phase 3 : un vrai moteur de recherche (10 niveaux en moins de 0,5 s)

- [ ] **Fonction d'entrée** `best_move(...)` qui renvoie : le coup, la note, la profondeur atteinte, les nœuds, le temps
- [ ] **Approfondissement itératif** + **limite de temps** : profondeur 1, 2, 3… et on s'arrête à temps
- [ ] **Tri des coups** avec une petite note **par case** (gagne ? capture ? bloque ?) : fonction **différente** d'`evaluate`
- [ ] **Table de transposition** avec **hachage de Zobrist**
- [ ] **Limiter la largeur** : ne garder que les N meilleurs candidats à chaque niveau
- [ ] *Killer moves* / *history heuristic* (optionnel)

**Se renseigner :**
- chessprogramming.org : *Iterative Deepening*, *Move Ordering*, *Transposition Table*, *Zobrist Hashing*, *Killer Heuristic*
- `std::collections::HashMap`, `std::time::Instant` / `Duration`
- (avancé) *Threat-space search* / *VCF*

**Fini quand :** ne dépasse jamais la limite de temps, et choisit le même coup qu'avant chaque optimisation.
⚠️ La vitesse mesurée ici est **optimiste** : la vraie `evaluate` sera plus lente → remesurer en phase 6.

## Phase 4 : le duel IA contre IA

L'outil qui servira à régler l'heuristique : faire jouer deux versions l'une contre l'autre et compter les victoires.

- [ ] `evaluate` **interchangeable** : barème dans une `struct` (`Weights`) ou fonction passée en paramètre
- [ ] Boucle de partie : les deux IA jouent tour à tour jusqu'au gagnant ou au plateau plein
- [ ] **Ouvertures variées** : 2–3 premières pierres au hasard (sinon, IA déterministes = même partie à chaque fois)
- [ ] **Couleurs alternées** : chaque IA joue autant avec les noirs qu'avec les blancs
- [ ] **Mêmes conditions** : même profondeur ou même temps pour les deux
- [ ] Statistiques : victoires A / victoires B / nuls, temps moyen par coup

**Se renseigner :** *Rust function pointer* (`fn(&Game, Stone) -> i32`), passer une `struct` en paramètre,
génération de nombres pseudo-aléatoires (crate `rand`, ou un générateur simple écrit à la main).

**Fini quand :** deux IA **identiques** font environ 50 / 50 (preuve que le duel est équitable).

## Phase 5 : brancher l'IA dans l'interface

L'interface est déjà prête : chronomètre, jauge de profondeur, nœuds, évaluation, variante principale, heatmap.

- [ ] `request_ai_move` : joue **un** coup de l'IA et renvoie l'état
- [ ] `suggest_move` : propose un coup sans le jouer (mode local)
- [ ] Remplir `AiReport` : `nodes`, `depth`, `elapsedMs`, `evaluation`, `principalVariation`, `heatmap`
- [ ] Remettre `is_ai` selon la config dans `dto.rs` (aujourd'hui forcé à `false`)
- [ ] Ne pas **figer la fenêtre** pendant la réflexion (commande asynchrone ou thread)

**Se renseigner :**
- Tauri v2 : *Calling Rust from the frontend*, section *Async commands*
- Le Rust Book, chapitre 16 (*Fearless Concurrency*)
- `front/BACKEND.md` : la forme exacte de `AiReport`

**Fini quand :** on joue une partie complète contre l'IA dans la fenêtre, avec le timer.

## Phase 6 : l'heuristique `evaluate` (en dernier)

Le sujet : *« The heuristic is actually the hardest part. »* Tout le reste est prêt : il n'y a plus qu'elle.

- [ ] Reconnaître les **segments** sur les 4 directions : longueur + bouts libres (`count_dir`, `cell_at`)
- [ ] Ne compter un segment **qu'à partir de sa première pierre**
- [ ] Barème : cinq > quatre ouvert > quatre fermé > trois libre > trois fermé > deux (≈ ×10 par niveau)
- [ ] Les **trous** (`X X . X`) et les **captures** (paires capturées, paires menacées)
- [ ] Tests : symétrie `evaluate(g, Noir) == -evaluate(g, Blanc)`, bon signe sur des positions simples
- [ ] **Régler le barème avec le duel** (phase 4) et noter chaque essai pour la soutenance
- [ ] **Remesurer** profondeur et temps avec la vraie heuristique

**Se renseigner :**
- *Gomoku threat types*, *straight four*, *broken three*
- L.V. Allis, *Go-Moku and Threat-Space Search* (1994)
- *Pente AI evaluation* : la variante la plus proche (Gomoku + captures de paires)
- chessprogramming.org : *Evaluation* ; le championnat **Gomocup** (moteurs Rapfi, Yixin, Embryo)

**Fini quand :** la nouvelle heuristique bat nettement l'ancienne en duel, et le temps moyen reste < 0,5 s.

## Phase 7 : exigences du rendu

- [ ] **Makefile** à la racine : `$(NAME)` (= `Gomoku`), `all`, `clean`, `fclean`, `re`, sans relink
- [ ] L'exécutable final s'appelle **`Gomoku`** (`pnpm tauri build`, puis copie/renommage par le Makefile)
- [ ] **Zéro plantage** : chercher tous les `unwrap()` / `expect()` / index hors limites qui pourraient paniquer
- [ ] **Mode debug** de l'IA (heatmap, variante principale) prêt à montrer
- [ ] Préparer la soutenance : savoir expliquer Min-Max, alpha-bêta, l'heuristique, et les choix de `back/RULES.md`

**Se renseigner :**
- Makefile : règles, dépendances, `.PHONY`, ce que veut dire « relink »
- `pnpm tauri build` et `productName` / `mainBinaryName` dans `tauri.conf.json`
- Rust : `panic`, `Result`, l'opérateur `?`, et pourquoi `unwrap` est risqué

**Fini quand :** `make re` produit `Gomoku` depuis zéro, et une longue partie ne plante jamais.

## Phase 8 : bonus (seulement si tout le reste est parfait)

- [ ] Choix des règles au lancement (le menu a déjà un sélecteur d'ouverture)
- [ ] Ouvertures **Pro**, **Swap**, **Swap2**
- [ ] Jusqu'à 5 bonus comptés : choisir ceux qui sont utiles et démontrables

**Se renseigner :** les règles d'ouverture du Gomoku (*Pro*, *Long Pro*, *Swap*, *Swap2*).

---

## Comment je t'aide pour les tests

Tu codes, je t'aide à vérifier. À chaque phase, tu peux me demander :

- **Positions-pièges** : des plateaux où l'IA doit gagner en 1 coup, bloquer un quatre, ou repérer une capture décisive
- **Mesures** : nœuds visités, profondeur atteinte, temps moyen par coup sur une partie complète
- **Non-régression** : relancer tous les tests (`back` + `src-tauri`) après chaque optimisation
- **IA contre IA** : faire jouer deux versions l'une contre l'autre pour savoir si un changement l'a vraiment améliorée
- **Relecture** de ton code et explication des erreurs du compilateur
