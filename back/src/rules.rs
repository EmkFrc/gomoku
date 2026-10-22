// Les règles, sous forme de fonctions sur un plateau : elles ne modifient rien.

use crate::board::*;

pub const CAPTURES_TO_WIN: usize = 10;

// Alignements de 5 pierres ou plus passant par (x, y), chacun d'un bout à l'autre.
// Une liste vide = pas d'alignement.
pub fn find_lines(board: &Board, x: usize, y: usize) -> Vec<Line> {
    let cell = board[y][x];
    let mut lines = Vec::new();
    if cell == Cell::Empty {
        return lines;
    }
    for (dx, dy) in DIRECTIONS {
        let before = count_dir(board, x, y, -dx, -dy, cell) as i32;
        let after = count_dir(board, x, y, dx, dy, cell) as i32;
        if before + 1 + after >= 5 {
            lines.push(
                (-before..=after)
                    .filter_map(|k| step(x, y, dx, dy, k))
                    .collect(),
            );
        }
    }
    lines
}

// Paires de `opp` capturées par la pierre `me` posée en (x, y) : motif  me opp opp me.
pub fn find_captures(board: &Board, x: usize, y: usize, me: Cell, opp: Cell) -> Vec<Pos> {
    let mut found = Vec::new();
    for (dx, dy) in DIRECTIONS {
        for (dx, dy) in [(dx, dy), (-dx, -dy)] {
            if cell_at(board, x, y, dx, dy, 1) == Some(opp)
                && cell_at(board, x, y, dx, dy, 2) == Some(opp)
                && cell_at(board, x, y, dx, dy, 3) == Some(me)
            {
                found.extend(step(x, y, dx, dy, 1));
                found.extend(step(x, y, dx, dy, 2));
            }
        }
    }
    found
}

// `attacker` peut-il capturer au moins une paire de `victim`, n'importe où ?
pub fn can_capture_any(board: &Board, attacker: Cell, victim: Cell) -> bool {
    for y in 0..SIZE {
        for x in 0..SIZE {
            if board[y][x] == Cell::Empty
                && !find_captures(board, x, y, attacker, victim).is_empty()
            {
                return true;
            }
        }
    }
    false
}

// Reste-t-il un alignement de 5 de `owner` passant par une pierre de ces lignes ?
pub fn lines_still_standing(board: &Board, lines: &[Line], owner: Cell) -> bool {
    lines
        .iter()
        .flatten()
        .any(|&(x, y)| board[y][x] == owner && !find_lines(board, x, y).is_empty())
}

// `attacker` peut-il, en un coup, capturer une paire qui casse TOUTES les lignes de `owner` ?
// Une capture qui laisse encore 5 pierres alignées (bout d'une ligne de 6) ne casse rien.
pub fn can_break_lines(board: &Board, lines: &[Line], owner: Cell, attacker: Cell) -> bool {
    for y in 0..SIZE {
        for x in 0..SIZE {
            if board[y][x] != Cell::Empty {
                continue;
            }
            let mut b = *board;
            b[y][x] = attacker;
            let caps = find_captures(&b, x, y, attacker, owner);
            if caps.is_empty() {
                continue;
            }
            for &(cx, cy) in &caps {
                b[cy][cx] = Cell::Empty;
            }
            if !lines_still_standing(&b, lines, owner) {
                return true;
            }
        }
    }
    false
}

// Nombre de directions où la pierre `me` en (x, y) forme un trois libre : en ajoutant
// une seule pierre on obtient exactement 4 alignés avec les deux bouts vides.
// `O . X X X . O` n'en est donc pas un : chaque quatre possible a un bout bloqué.
pub fn count_free_threes(board: &Board, x: usize, y: usize, me: Cell) -> usize {
    let mut count = 0;
    for (dx, dy) in DIRECTIONS {
        for k in -3..=3 {
            let Some((ex, ey)) = step(x, y, dx, dy, k) else {
                continue;
            };
            if k == 0 || board[ey][ex] != Cell::Empty {
                continue;
            }
            let mut b = *board;
            b[ey][ex] = me;
            let before = count_dir(&b, x, y, -dx, -dy, me) as i32;
            let after = count_dir(&b, x, y, dx, dy, me) as i32;
            if before + 1 + after == 4
                && (-before..=after).contains(&k)
                && cell_at(&b, x, y, dx, dy, -before - 1) == Some(Cell::Empty)
                && cell_at(&b, x, y, dx, dy, after + 1) == Some(Cell::Empty)
            {
                count += 1;
                break;
            }
        }
    }
    count
}
