mod dto;

use std::sync::Mutex;

use dto::{Coord, GameConfig, GameState, GameStatus};
use gomoku::game::Game;
use tauri::State;

// Ce qu'on garde dans le frigo : la partie du moteur, plus ce que l'interface
// doit retrouver entre deux commandes (la configuration choisie, le statut).
struct Session {
    game: Game,
    config: GameConfig,
    status: GameStatus,
}

impl Session {
    fn new(config: GameConfig) -> Session {
        Session {
            game: Game::new(),
            config,
            status: GameStatus::Playing,
        }
    }

    fn state(&self) -> GameState {
        dto::snapshot(&self.game, &self.config, &self.status)
    }
}

// Le frigo : UNE session partagée par toutes les commandes.
// Mutex = un seul cuisinier à la fois peut l'ouvrir.
type SharedSession = Mutex<Session>;

// Nouvelle partie avec les réglages du menu.
#[tauri::command]
fn new_game(state: State<SharedSession>, config: GameConfig) -> GameState {
    let mut session = state.lock().unwrap();
    *session = Session::new(config);
    session.state()
}

#[tauri::command]
fn get_state(state: State<SharedSession>) -> GameState {
    state.lock().unwrap().state()
}

// Un coup illégal ne renvoie pas d'erreur : l'état revient simplement inchangé.
#[tauri::command]
fn play_move(state: State<SharedSession>, coord: Coord) -> GameState {
    let mut session = state.lock().unwrap();
    let playing = matches!(session.status, GameStatus::Playing);
    if playing && session.game.is_valid_move(coord.x, coord.y) {
        let winner = session.game.play(coord.x, coord.y);
        session.status = dto::status_of(&session.game, winner);
    }
    session.state()
}

#[tauri::command]
fn undo(state: State<SharedSession>) ->  GameState {
    let mut session = state.lock().unwrap();
    session.game.undo();
    session.status = GameStatus::Playing;
    session.state()
}


// L'IA n'existe pas encore : ces deux commandes répondent par une erreur claire.
#[tauri::command]
fn request_ai_move() -> Result<GameState, String> {
    Err("IA pas encore implémentée".to_string())
}

#[tauri::command]
fn suggest_move() -> Result<GameState, String> {
    Err("Suggestion pas encore implémentée (elle viendra avec l'IA)".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(Session::new(GameConfig::default())))
        .invoke_handler(tauri::generate_handler![
            new_game,
            get_state,
            play_move,
            undo,
            request_ai_move,
            suggest_move
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
