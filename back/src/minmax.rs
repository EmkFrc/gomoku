use crate::board::*;
use crate::game::Game;
use crate::eval::evaluate;

pub const INF: i32 = 1_000_000_000;


fn detect_proxi_cell(x: usize, y: usize, game: &Game) -> bool {
    for dy in -2..=2 {
        for dx in -2..=2 {
            if let Some((nx, ny)) = step(x, y, dx, dy, 1)
                && game.board[ny][nx] != Cell::Empty
            {
                return true;
            }
        }
    }
    false
}

pub fn candidates(game: &Game) -> Vec<Pos> {
    let mut moves = Vec::new();

    for y in 0..SIZE {
        for x in 0..SIZE {
            if game.board[y][x] != Cell::Empty {
                continue;
            }
            if !detect_proxi_cell(x, y, game) {
                continue;
            }
            if !game.is_valid_move(x, y) {
                continue;
            }
            moves.push((x, y));
        }
    }

    if game.board.iter().flatten().all(|&cell| cell == Cell::Empty) {
        moves.push((SIZE / 2, SIZE / 2));
    }
    moves
}


// Tour de l'IA : elle essaie chaque coup et garde la note la plus HAUTE.
// `ai` = la couleur pour laquelle on cherche ; elle ne change pas en descendant.
pub fn maxi(game: &mut Game, depth: i32, ai: Stone) -> i32 {
    if depth == 0 {
        return evaluate(game, ai);
    }
    let mut max: i32 = -INF;

    for (x, y) in candidates(game) {
        game.play(x, y);
        let score: i32 = mini(game, depth - 1, ai);
        game.undo();
        if score > max {
            max = score;
        }
    }
    max
}

// Tour de l'adversaire : il essaie chaque coup et garde la note la plus BASSE.
// La note reste du point de vue de l'IA : pas de signe moins devant evaluate.
fn mini(game: &mut Game, depth: i32, ai: Stone) -> i32 {
    if depth == 0 {
        return evaluate(game, ai);
    }
    let mut min: i32 = INF;

    for (x, y) in candidates(game) {
        game.play(x, y);
        let score: i32 = maxi(game, depth - 1, ai);
        game.undo();
        if score < min {
            min = score;
        }
    }
    min
}
