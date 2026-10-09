use std::io;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq)]
enum Stone {
    Black,
    White,
}

type Board = [[Cell; 19]; 19];

enum Input {
    Move(usize, usize),
    Undo,               
}

struct Move {
    x: usize,
    y: usize,
    stone: Stone,
    captures: Vec<(usize, usize)>,
}
struct Game {
    board: Board,
    turn: Stone,
    history: Vec<Move>,
    captured_black: usize, 
    captured_white: usize, 
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[Cell::Empty; 19]; 19],
            turn: Stone::Black,
            history: Vec::new(),
            captured_black: 0,
            captured_white: 0,
        }
    }
    fn is_valid_move(&self, x: usize, y: usize) -> bool {
        x < 19 && y < 19 && self.board[y][x] == Cell::Empty
    }
    fn play(&mut self, x: usize, y: usize) -> bool {

        let (me, opp) = match self.turn {
            Stone::Black => (Cell::Black, Cell::White),
            Stone::White => (Cell::White, Cell::Black),
        };
        self.board[y][x] = me;

        let captures = find_captures(&self.board, x, y, me, opp);
        for &(cx, cy) in &captures {
            self.board[cy][cx] = Cell::Empty;
        }
        match self.turn {
            Stone::Black => self.captured_black += captures.len(),
            Stone::White => self.captured_white += captures.len(),
        }

        self.history.push(Move { x, y, stone: self.turn, captures });

        let captured = match self.turn {
            Stone::Black => self.captured_black,
            Stone::White => self.captured_white,
        };
        if captured >= 10 {
            return true;
        }

        if !find_lines(&self.board, x, y).is_empty() {
            return true; 
        }


        self.turn = match self.turn { 
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        };
        
        false
    }

    fn is_full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|cell| *cell != Cell::Empty))
    }

    fn undo(&mut self) -> bool {
        match self.history.pop() {
            Some(mv) => {
                self.board[mv.y][mv.x] = Cell::Empty;

                let opp = match mv.stone {
                    Stone::Black => Cell::White,
                    Stone::White => Cell::Black,
                };
                for &(cx, cy) in &mv.captures {
                    self.board[cy][cx] = opp;
                }
                match mv.stone {
                    Stone::Black => self.captured_black -= mv.captures.len(),
                    Stone::White => self.captured_white -= mv.captures.len(),
                }

                self.turn = mv.stone;
                true
            },
            None => {
                println!("Impossible de retourner en arrière");
                false
            },
        }
    }
}

fn board_frame(board: &Board)
{
    for row in board.iter() {
        for cell in row.iter() {
            let symbol = match cell {
                Cell::Empty => ".",
                Cell::Black => "X",
                Cell::White => "O",
            };
            print!("{}", symbol);
        }
        println!();
    }
}

fn read_move() -> Option<Input> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() == 1 && parts[0] == "return" {
        return Some(Input::Undo);
    }
    if parts.len() != 2 {
        return None;
    }

    let x: usize = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => return None,
    };
    let y: usize = match parts[1].parse() {
        Ok(n) => n,
        Err(_) => return None,
    };

    Some(Input::Move(x, y))
}


fn ask_move(game: &Game) -> Input {
    loop {
        println!("Coordonnées (x y) ou « return » pour annuler : ");

        match read_move() {
            Some(Input::Undo) => return Input::Undo,
            Some(Input::Move(x, y)) => {
                if game.is_valid_move(x, y) {
                    return Input::Move(x, y);   
                } else {
                    println!("Case invalide ou occupée, réessaie");
                }
            }
            None => {
                println!("Entrée invalide, réessaie");
            }
        }
    }
}

fn gomoku()
{
    let mut game = Game::new();
    let mut finish = false;
    let mut win = false;

    while !finish
    {
        board_frame(&game.board);
        match ask_move(&game) {
            Input::Move(x, y) => {
                win = game.play(x, y);
                finish = win || game.is_full();
            }
            Input::Undo => {
                game.undo();
            }
        }
    }

    board_frame(&game.board);
    if win {
        match game.turn {
            Stone::Black => println!("Les noirs (X) gagnent !"),
            Stone::White => println!("Les blancs (O) gagnent !"),
        }
    } else {
        println!("Match nul !");
    }
}
const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];


fn count_dir(board: &Board, x: usize, y: usize, dx: i32, dy: i32, cell: Cell) -> usize {
    let mut count = 0;
    let mut cx = x as i32 + dx;
    let mut cy = y as i32 + dy;

    while cx >= 0 && cx < 19 && cy >= 0 && cy < 19
        && board[cy as usize][cx as usize] == cell
    {
        count += 1;
        cx += dx;
        cy += dy;
    }
    count
}

// Alignements de 5 pierres ou plus passant par (x, y). Une liste vide = pas d'alignement.
// Chaque ligne donne toutes ses coordonnées, d'un bout à l'autre : il en faut la liste
// pour savoir si l'adversaire peut la casser par une capture.
fn find_lines(board: &Board, x: usize, y: usize) -> Vec<Vec<(usize, usize)>> {
    let mut lines = Vec::new();
    let cell = board[y][x];
    if cell == Cell::Empty {
        return lines;
    }

    for (dx, dy) in DIRECTIONS {
        let before = count_dir(board, x, y, -dx, -dy, cell);
        let after = count_dir(board, x, y, dx, dy, cell);
        let total = before + 1 + after;
        if total >= 5 {
            // première pierre de la ligne : `before` pas en arrière
            let start_x = x as i32 - dx * before as i32;
            let start_y = y as i32 - dy * before as i32;
            let mut line = Vec::new();
            for k in 0..total as i32 {
                line.push(((start_x + dx * k) as usize, (start_y + dy * k) as usize));
            }
            lines.push(line);
        }
    }
    lines
}

const DIRECTIONS_8: [(i32, i32); 8] = [
    (1, 0), (-1, 0), (0, 1), (0, -1),
    (1, 1), (-1, -1), (1, -1), (-1, 1),
];

fn cell_at(board: &Board, x: usize, y: usize, dx: i32, dy: i32, k: i32) -> Option<Cell> {
    let cx = x as i32 + dx * k;
    let cy = y as i32 + dy * k;
    if cx < 0 || cx >= 19 || cy < 0 || cy >= 19 {
        return None;
    }
    Some(board[cy as usize][cx as usize])
}

fn find_captures(board: &Board, x: usize, y: usize, me: Cell, opp: Cell) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    for (dx, dy) in DIRECTIONS_8 {
        if cell_at(board, x, y, dx, dy, 1) == Some(opp)
            && cell_at(board, x, y, dx, dy, 2) == Some(opp)
            && cell_at(board, x, y, dx, dy, 3) == Some(me)
        {
            found.push(((x as i32 + dx) as usize, (y as i32 + dy) as usize));
            found.push(((x as i32 + dx * 2) as usize, (y as i32 + dy * 2) as usize));
        }
    }
    return found
}

fn main()
{
    gomoku();
}
