// Partie dans le terminal : affichage et saisie. Aucune règle ici, tout passe par Game.

use std::io;

use crate::board::*;
use crate::game::Game;

#[derive(Debug, PartialEq)]
pub enum Input {
    Move(usize, usize),
    Undo,
    Quit,
}

fn board_frame(board: &Board) {
    for row in board {
        let line: String = row
            .iter()
            .map(|cell| match cell {
                Cell::Empty => '.',
                Cell::Black => 'X',
                Cell::White => 'O',
            })
            .collect();
        println!("{}", line);
    }
}

// Analyse une ligne tapée : « x y » ou « return ». None si elle ne veut rien dire.
pub fn parse_input(line: &str) -> Option<Input> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts[..] {
        ["return"] => Some(Input::Undo),
        [x, y] => Some(Input::Move(x.parse().ok()?, y.parse().ok()?)),
        _ => None,
    }
}

fn read_move() -> Option<Input> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) | Err(_) => Some(Input::Quit), // entrée fermée (Ctrl+D) ou erreur
        Ok(_) => parse_input(&input),
    }
}

fn ask_move(game: &Game) -> Input {
    loop {
        println!("Coordonnées (x y) ou « return » pour annuler : ");
        match read_move() {
            Some(Input::Move(x, y)) => {
                if game.is_valid_move(x, y) {
                    return Input::Move(x, y);
                } else if game.is_free(x, y) {
                    println!("Coup interdit : double-trois, réessaie");
                } else {
                    println!("Case invalide ou occupée, réessaie");
                }
            }
            Some(other) => return other,
            None => println!("Entrée invalide, réessaie"),
        }
    }
}

pub fn gomoku() {
    let mut game = Game::new();
    loop {
        board_frame(&game.board);
        match ask_move(&game) {
            Input::Move(x, y) => {
                if let Some(winner) = game.play(x, y) {
                    board_frame(&game.board);
                    match winner {
                        Stone::Black => println!("Les noirs (X) gagnent !"),
                        Stone::White => println!("Les blancs (O) gagnent !"),
                    }
                    return;
                }
                if game.is_full() {
                    board_frame(&game.board);
                    println!("Match nul !");
                    return;
                }
            }
            Input::Undo => {
                if !game.undo() {
                    println!("Impossible de retourner en arrière");
                }
            }
            Input::Quit => {
                println!("Entrée fermée, fin de la partie");
                return;
            }
        }
    }
}
