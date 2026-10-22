// Une partie : le plateau, le tour, l'historique et les compteurs de captures.

use crate::board::*;
use crate::rules::*;

pub struct Move {
    pub x: usize,
    pub y: usize,
    pub stone: Stone,
    pub captures: Vec<Pos>,
}

pub struct Game {
    pub board: Board,
    pub turn: Stone,
    pub history: Vec<Move>,
    pub captured_black: usize, // pierres blanches capturées par les noirs
    pub captured_white: usize, // pierres noires capturées par les blancs
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: empty_board(),
            turn: Stone::Black,
            history: Vec::new(),
            captured_black: 0,
            captured_white: 0,
        }
    }

    // pierres capturées PAR `stone`
    pub fn captured(&self, stone: Stone) -> usize {
        match stone {
            Stone::Black => self.captured_black,
            Stone::White => self.captured_white,
        }
    }

    fn captured_mut(&mut self, stone: Stone) -> &mut usize {
        match stone {
            Stone::Black => &mut self.captured_black,
            Stone::White => &mut self.captured_white,
        }
    }

    // dans le plateau et vide
    pub fn is_free(&self, x: usize, y: usize) -> bool {
        x < SIZE && y < SIZE && self.board[y][x] == Cell::Empty
    }

    pub fn is_valid_move(&self, x: usize, y: usize) -> bool {
        self.is_free(x, y) && !self.is_double_three(x, y)
    }

    // double-trois interdit, sauf si le coup capture une paire
    pub fn is_double_three(&self, x: usize, y: usize) -> bool {
        let me = self.turn.cell();
        let mut b = self.board;
        b[y][x] = me;
        if !find_captures(&b, x, y, me, self.turn.other().cell()).is_empty() {
            return false;
        }
        count_free_threes(&b, x, y, me) >= 2
    }

    pub fn is_full(&self) -> bool {
        self.board.iter().flatten().all(|&cell| cell != Cell::Empty)
    }

    // Joue un coup supposé valide. Renvoie le gagnant si la partie est finie.
    pub fn play(&mut self, x: usize, y: usize) -> Option<Stone> {
        let player = self.turn;
        let (me, opp) = (player.cell(), player.other().cell());

        // ligne laissée par l'adversaire au coup précédent (elle était cassable)
        let pending = match self.history.last() {
            Some(last) => find_lines(&self.board, last.x, last.y),
            None => Vec::new(),
        };

        self.board[y][x] = me;
        let captures = find_captures(&self.board, x, y, me, opp);
        for &(cx, cy) in &captures {
            self.board[cy][cx] = Cell::Empty;
        }
        *self.captured_mut(player) += captures.len();
        self.history.push(Move {
            x,
            y,
            stone: player,
            captures,
        });

        // 1. victoire par capture (passe avant tout le reste)
        if self.captured(player) >= CAPTURES_TO_WIN {
            return Some(player);
        }

        // 2. je n'ai pas cassé la ligne de l'adversaire : il gagne
        if !pending.is_empty() && lines_still_standing(&self.board, &pending, opp) {
            return Some(player.other());
        }

        // 3. mon alignement gagne tout de suite si personne ne peut répondre
        let lines = find_lines(&self.board, x, y);
        if !lines.is_empty() {
            let breakable = can_break_lines(&self.board, &lines, me, opp);
            let opp_wins_by_capture = self.captured(player.other()) + 2 >= CAPTURES_TO_WIN
                && can_capture_any(&self.board, opp, me);
            if !breakable && !opp_wins_by_capture {
                return Some(player);
            }
        }

        self.turn = player.other();
        None
    }

    // Annule le dernier coup. Renvoie false s'il n'y a rien à annuler.
    pub fn undo(&mut self) -> bool {
        let Some(mv) = self.history.pop() else {
            return false;
        };
        self.board[mv.y][mv.x] = Cell::Empty;
        let opp = mv.stone.other().cell();
        for &(cx, cy) in &mv.captures {
            self.board[cy][cx] = opp;
        }
        *self.captured_mut(mv.stone) -= mv.captures.len();
        self.turn = mv.stone;
        true
    }
}
