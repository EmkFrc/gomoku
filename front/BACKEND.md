# Contrat front ↔ moteur

Ce que l'interface attend du moteur Rust. Les types exacts sont dans
[`src/types.ts`](src/types.ts) ; ce document en donne la contrepartie Rust.

L'interface n'appelle jamais `invoke` directement : tout passe par
[`src/api/index.ts`](src/api/index.ts). Si une commande est absente ou renvoie
une erreur, le front bascule sur un moteur de secours TypeScript
([`src/api/mock.ts`](src/api/mock.ts)) et affiche la mention
« moteur de secours » en bas à droite. L'interface est donc utilisable et
démontrable avant que le Rust ne soit prêt.

## Commandes attendues

| Commande | Arguments | Retour |
|----------|-----------|--------|
| `new_game` | `config: GameConfig` | `GameState` |
| `get_state` | — | `GameState` |
| `play_move` | `coord: Coord` | `GameState` |
| `request_ai_move` | — | `GameState` |
| `undo` | — | `GameState` |
| `suggest_move` | — | `Suggestion` |

Toutes sont appelées en `async`. Une erreur renvoyée par `invoke` fait basculer
le front sur le moteur de secours pour le reste de la session — autant renvoyer
un `GameState` inchangé qu'une `Err` pour un coup simplement illégal.

## Sérialisation

Le front est en **camelCase**. Côté Rust, le plus simple est :

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameConfig { /* … */ }
```

Les arguments de commande suivent la convention Tauri : le front envoie
`{ config }` et `{ coord }`, donc les paramètres Rust s'appellent `config` et
`coord`.

## Structures

```rust
#[serde(rename_all = "camelCase")]
pub struct Coord { pub x: usize, pub y: usize }   // 0..=18

pub enum Stone { Black, White }                    // "black" | "white"
pub enum Mode { VsAi, Hotseat }                    // "vs-ai" | "hotseat"
pub enum Opening { Std, Pro, Swap, Swap2 }         // "std" | "pro" | "swap" | "swap2"

#[serde(rename_all = "camelCase")]
pub struct GameConfig {
    pub mode: Mode,
    pub ai_color: Stone,
    pub opening: Opening,
    pub board_size: usize,      // 19
    pub ai_max_depth: u32,      // 10
    pub ai_time_limit_ms: u64,  // 8000
}

#[serde(rename_all = "camelCase")]
pub struct Move {
    pub index: u32,             // 1-indexé
    pub coord: Coord,
    pub stone: Stone,
    pub captures: Vec<Coord>,   // pierres retirées par ce coup
    pub notation: String,       // « K10 » — colonnes A..T sans I
}

#[serde(rename_all = "camelCase")]
pub struct PlayerState {
    pub name: String,
    pub stone: Stone,
    pub captured_pairs: u32,    // 0..=5
    pub is_ai: bool,
}

// Sérialisé en union discriminée sur « kind » :
//   { "kind": "playing" }
//   { "kind": "won", "winner": "black", "reason": "five", "line": [ … ] }
//   { "kind": "draw" }
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum GameStatus {
    Playing,
    Won { winner: Stone, reason: WinReason, line: Vec<Coord> },
    Draw,
}

#[serde(rename_all = "camelCase")]
pub struct AiReport {
    pub mood: String,                        // idle | thinking | confident | worried
    pub nodes: u64,
    pub nodes_per_second: u64,
    pub depth: u32,
    pub max_depth: u32,
    pub evaluation: f32,                     // -1.0 (noir gagne) → +1.0 (IA gagne)
    pub elapsed_ms: f64,
    pub principal_variation: Vec<String>,    // ["J10", "J11", …]
    pub heatmap: Vec<HeatCell>,
}

#[serde(rename_all = "camelCase")]
pub struct HeatCell { pub coord: Coord, pub weight: f32 }  // weight 0.0 → 1.0

#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub config: GameConfig,
    pub board: Vec<Vec<Option<Stone>>>,      // board[y][x]
    pub turn: Stone,
    pub moves: Vec<Move>,
    pub players: Players,                    // { black: PlayerState, white: PlayerState }
    pub status: GameStatus,
    pub last_move: Option<Coord>,
    pub forbidden: Vec<Coord>,               // double-trois pour le joueur courant
    pub ai: Option<AiReport>,
}

#[serde(rename_all = "camelCase")]
pub struct Suggestion { pub coord: Coord, pub report: AiReport }
```

## Ce que l'interface fait avec ces données

- **`forbidden`** — les intersections listées refusent le clic : la pierre
  fantôme rougit et tremble. Renvoyer une liste vide désactive simplement le
  retour visuel, rien ne casse.
- **`ai`** — alimente le chronomètre, la jauge de profondeur, le compteur de
  nœuds, la barre d'évaluation, la variante principale et la heatmap. `None`
  est accepté : les panneaux affichent des zéros.
- **`heatmap`** — dessinée **sous** les pierres, or en dessous de 0.6 et
  cinabre au-dessus. Une dizaine de cellules suffit.
- **`status.line`** — les cinq coordonnées gagnantes ; l'interface trace le
  trait rouge entre la première et la dernière.
- **`captures`** sur un `Move` — le compteur de paires de la carte joueur se
  remplit ; l'historique affiche `×1`, `×2`…
- **`undo`** — en mode vs-IA, le front appelle `undo` **deux fois** pour rendre
  la main au joueur. Une seule dépile un demi-coup.
- **`request_ai_move`** — appelée en boucle tant que `players[turn].is_ai` et
  que la partie n'est pas finie. Elle doit donc jouer **un seul** coup et
  rendre l'état.

## Ce que le front ne fait pas

Aucune règle du jeu ne vit côté TypeScript : ni légalité d'un coup, ni
détection d'alignement, ni captures, ni double-trois, ni protocole d'ouverture.
Le moteur de secours n'en implémente une partie que pour rester jouable ; il
n'est pas une référence.
