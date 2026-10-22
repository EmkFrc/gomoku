// Les tickets : ce que Rust échange avec Vue, traduit en JSON par serde.
// Chaque type ici a son jumeau dans front/src/types.ts, avec les mêmes noms de champs.
// Le moteur (back/) n'en sait rien : on traduit ses types vers ceux-ci.

use gomoku::board::{Cell, Pos, Stone, SIZE};
use gomoku::game::Game;
use gomoku::rules::{find_lines, CAPTURES_TO_WIN};
use serde::{Deserialize, Serialize};

// ---------- les petits morceaux ----------

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")] // Black -> "black"
pub enum Color {
    Black,
    White,
}

impl From<Stone> for Color {
    fn from(stone: Stone) -> Color {
        match stone {
            Stone::Black => Color::Black,
            Stone::White => Color::White,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

fn coord((x, y): Pos) -> Coord {
    Coord { x, y }
}

// « K10 » : colonnes A..T sans I, lignes 19 en haut -> 1 en bas
fn notation(x: usize, y: usize) -> String {
    const COLUMNS: &[u8] = b"ABCDEFGHJKLMNOPQRST";
    format!("{}{}", COLUMNS[x] as char, SIZE - y)
}

// ---------- la configuration envoyée par le menu ----------

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "kebab-case")] // VsAi -> "vs-ai"
pub enum Mode {
    VsAi,
    Hotseat,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Opening {
    Std,
    Pro,
    Swap,
    Swap2,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")] // ai_color -> "aiColor"
pub struct GameConfig {
    pub mode: Mode,
    pub ai_color: Color,
    pub opening: Opening,
    pub board_size: usize,
    pub ai_max_depth: u32,
    pub ai_time_limit_ms: u64,
}

impl Default for GameConfig {
    fn default() -> GameConfig {
        GameConfig {
            mode: Mode::Hotseat,
            ai_color: Color::White,
            opening: Opening::Std,
            board_size: SIZE,
            ai_max_depth: 10,
            ai_time_limit_ms: 500,
        }
    }
}

// ---------- l'état complet renvoyé après chaque commande ----------

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Move {
    index: usize,
    coord: Coord,
    stone: Color,
    captures: Vec<Coord>,
    notation: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerState {
    name: String,
    stone: Color,
    captured_pairs: usize,
    is_ai: bool,
}

#[derive(Serialize)]
pub struct Players {
    black: PlayerState,
    white: PlayerState,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WinReason {
    Five,
    Captures,
}

// { "kind": "playing" } | { "kind": "won", "winner": ..., "reason": ..., "line": [...] } | { "kind": "draw" }
#[derive(Serialize, Clone)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum GameStatus {
    Playing,
    Won {
        winner: Color,
        reason: WinReason,
        line: Vec<Coord>,
    },
    Draw,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    config: GameConfig,
    board: Vec<Vec<Option<Color>>>, // board[y][x], null = case libre
    turn: Color,
    moves: Vec<Move>,
    players: Players,
    status: GameStatus,
    last_move: Option<Coord>,
    forbidden: Vec<Coord>,
    ai: Option<serde_json::Value>, // null tant qu'il n'y a pas d'IA
}

// ---------- traductions moteur -> tickets ----------

// Statut de la partie après un coup qui a renvoyé `winner`.
pub fn status_of(game: &Game, winner: Option<Stone>) -> GameStatus {
    match winner {
        Some(stone) if game.captured(stone) >= CAPTURES_TO_WIN => GameStatus::Won {
            winner: stone.into(),
            reason: WinReason::Captures,
            line: Vec::new(),
        },
        Some(stone) => GameStatus::Won {
            winner: stone.into(),
            reason: WinReason::Five,
            line: win_line(game, stone),
        },
        None if game.is_full() => GameStatus::Draw,
        None => GameStatus::Playing,
    }
}

// La ligne gagnante passe par le dernier coup du gagnant. Ce n'est pas toujours le
// dernier coup joué : si l'adversaire n'a pas cassé la ligne, c'est l'avant-dernier.
fn win_line(game: &Game, winner: Stone) -> Vec<Coord> {
    let Some(mv) = game.history.iter().rev().find(|mv| mv.stone == winner) else {
        return Vec::new();
    };
    match find_lines(&game.board, mv.x, mv.y).into_iter().next() {
        Some(line) => line.into_iter().map(coord).collect(),
        None => Vec::new(),
    }
}

// Tant que l'IA n'existe pas, les deux joueurs sont humains, même en mode « vs-ai » :
// sinon l'interface attendrait un coup de l'IA qui ne viendrait jamais.
fn player(color: Color, name: &str, captured: usize) -> PlayerState {
    PlayerState {
        name: name.to_string(),
        stone: color,
        captured_pairs: captured / 2,
        is_ai: false,
    }
}

// Photo complète de la partie, prête à partir vers Vue.
pub fn snapshot(game: &Game, config: &GameConfig, status: &GameStatus) -> GameState {
    let board = game
        .board
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| match cell {
                    Cell::Empty => None,
                    Cell::Black => Some(Color::Black),
                    Cell::White => Some(Color::White),
                })
                .collect()
        })
        .collect();

    let moves = game
        .history
        .iter()
        .enumerate()
        .map(|(i, mv)| Move {
            index: i + 1,
            coord: Coord { x: mv.x, y: mv.y },
            stone: mv.stone.into(),
            captures: mv.captures.iter().copied().map(coord).collect(),
            notation: notation(mv.x, mv.y),
        })
        .collect();

    // cases interdites (double-trois) pour le joueur qui doit jouer
    let mut forbidden = Vec::new();
    if matches!(status, GameStatus::Playing) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                if game.is_free(x, y) && game.is_double_three(x, y) {
                    forbidden.push(Coord { x, y });
                }
            }
        }
    }

    GameState {
        config: config.clone(),
        board,
        turn: game.turn.into(),
        moves,
        players: Players {
            black: player(Color::Black, "Lin", game.captured_black),
            white: player(Color::White, "Kai", game.captured_white),
        },
        status: status.clone(),
        last_move: game.history.last().map(|mv| Coord { x: mv.x, y: mv.y }),
        forbidden,
        ai: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn la_config_du_menu_se_lit() {
        // exactement ce qu'envoie front/src/types.ts (DEFAULT_CONFIG)
        let sent = r#"{"mode":"vs-ai","aiColor":"white","opening":"pro","boardSize":19,"aiMaxDepth":10,"aiTimeLimitMs":8000}"#;
        let config: GameConfig = serde_json::from_str(sent).unwrap();
        assert!(config.mode == Mode::VsAi && config.ai_color == Color::White);
        let hotseat = sent.replace("vs-ai", "hotseat");
        assert!(serde_json::from_str::<GameConfig>(&hotseat).unwrap().mode == Mode::Hotseat);
    }

    // « K10 » -> (9, 9), comme toNotation côté TypeScript
    fn at(n: &str) -> (usize, usize) {
        let x = "ABCDEFGHJKLMNOPQRST".find(&n[..1]).unwrap();
        (x, SIZE - n[1..].parse::<usize>().unwrap())
    }

    fn after(moves: &[&str]) -> serde_json::Value {
        let mut game = Game::new();
        for m in moves {
            let (x, y) = at(m);
            assert!(game.is_valid_move(x, y), "coup {} refusé", m);
            game.play(x, y);
        }
        serde_json::to_value(snapshot(&game, &GameConfig::default(), &GameStatus::Playing)).unwrap()
    }

    #[test]
    fn test_1_k10_est_interdit() {
        let v = after(&["G13", "A1", "H12", "C1", "L10", "E1", "M10", "G1"]);
        assert_eq!(v["turn"], json!("black"));
        assert_eq!(v["forbidden"], json!([{"x":9,"y":9}]), "K10 doit être la seule case interdite");
    }

    #[test]
    fn test_2_pierre_en_j10_k10_autorise() {
        let v = after(&["G13", "A1", "H12", "C1", "L10", "E1", "M10", "J10"]);
        assert_eq!(v["forbidden"], json!([]));
    }

    #[test]
    fn test_3_double_trois_avec_capture_autorise() {
        let v = after(&["G13", "K9", "H12", "K8", "L10", "A1", "M10", "C1", "K7", "E1"]);
        assert_eq!(v["forbidden"], json!([]));
    }

    #[test]
    fn la_photo_a_la_forme_attendue() {
        let mut game = Game::new();
        let mut winner = None;
        for (x, y) in [(0, 0), (0, 5), (1, 0), (0, 6), (2, 0), (0, 7), (3, 0), (0, 8), (4, 0)] {
            winner = game.play(x, y);
        }
        let status = status_of(&game, winner);
        let v = serde_json::to_value(snapshot(&game, &GameConfig::default(), &status)).unwrap();

        assert_eq!(v["config"]["mode"], json!("hotseat"));
        assert_eq!(v["board"][0][0], json!("black"));
        assert_eq!(v["board"][9][9], json!(null));
        assert_eq!(v["turn"], json!("black"));
        assert_eq!(v["moves"].as_array().unwrap().len(), 9);
        assert_eq!(v["moves"][0], json!({"index":1,"coord":{"x":0,"y":0},"stone":"black","captures":[],"notation":"A19"}));
        assert_eq!(v["players"]["black"], json!({"name":"Lin","stone":"black","capturedPairs":0,"isAi":false}));
        assert_eq!(v["status"]["kind"], json!("won"));
        assert_eq!(v["status"]["winner"], json!("black"));
        assert_eq!(v["status"]["reason"], json!("five"));
        assert_eq!(v["status"]["line"].as_array().unwrap().len(), 5);
        assert_eq!(v["lastMove"], json!({"x":4,"y":0}));
        assert_eq!(v["forbidden"], json!([]));
        assert_eq!(v["ai"], json!(null));
    }
}
