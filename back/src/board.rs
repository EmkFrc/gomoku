// Le plateau et ce qu'on peut y lire : cases, couleurs, directions, déplacements.

pub const SIZE: usize = 19;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    pub fn other(self) -> Stone {
        match self {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        }
    }

    // la case occupée par une pierre de cette couleur
    pub fn cell(self) -> Cell {
        match self {
            Stone::Black => Cell::Black,
            Stone::White => Cell::White,
        }
    }
}

pub type Board = [[Cell; SIZE]; SIZE];
pub type Pos = (usize, usize);
pub type Line = Vec<Pos>;

// Les 4 axes. Chacun se parcourt dans les deux sens : (dx, dy) et (-dx, -dy).
// Un alignement compte les deux sens à la fois, une capture les teste un par un.
pub const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];

pub fn empty_board() -> Board {
    [[Cell::Empty; SIZE]; SIZE]
}

// Position à `k` pas de (x, y) dans la direction (dx, dy) ; None si on sort du plateau.
pub fn step(x: usize, y: usize, dx: i32, dy: i32, k: i32) -> Option<Pos> {
    let cx = x as i32 + dx * k;
    let cy = y as i32 + dy * k;
    let range = 0..SIZE as i32;
    if range.contains(&cx) && range.contains(&cy) {
        Some((cx as usize, cy as usize))
    } else {
        None
    }
}

// Contenu de la case à `k` pas ; None si on sort du plateau.
pub fn cell_at(board: &Board, x: usize, y: usize, dx: i32, dy: i32, k: i32) -> Option<Cell> {
    step(x, y, dx, dy, k).map(|(cx, cy)| board[cy][cx])
}

// Nombre de cases `cell` consécutives à partir de (x, y), sans compter (x, y).
pub fn count_dir(board: &Board, x: usize, y: usize, dx: i32, dy: i32, cell: Cell) -> usize {
    let mut k = 1;
    while cell_at(board, x, y, dx, dy, k) == Some(cell) {
        k += 1;
    }
    (k - 1) as usize
}
