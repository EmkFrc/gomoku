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

fn read_move() -> Option<(usize, usize)> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
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

    Some((x, y))
}

fn ask_move(board: &Board) -> (usize, usize) {
    loop {
        println!("Coordonnées (x y) : ");

        match read_move() {
            Some((x, y)) => {
                if x < 19 && y < 19 && board[y][x] == Cell::Empty {
                    return (x, y);   // valide, on sort de la boucle avec la valeur
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


fn play_turn(board: &mut Board, turn: &mut Stone) -> bool
{
    let (x, y) = ask_move(board);

    board[y][x] = match *turn { // on choisie le caractère soit X ou O
        Stone::Black => Cell::Black,
        Stone::White => Cell::White,
    };

    if check_win(board, x, y) {
        return true; // on ne change pas de tour : *turn reste le gagnant
    }

    *turn = match *turn { // on inverse la couleur
        Stone::Black => Stone::White,
        Stone::White => Stone::Black,
    };
    false
}

fn is_full(board: &Board) -> bool
{
    board.iter().all(|row| row.iter().all(|cell| *cell != Cell::Empty))
}

fn gomoku()
{
    let mut board: Board = [[Cell::Empty; 19]; 19];
    let mut finish = false;
    let mut win = false;
    let mut turn = Stone::Black;
    while !finish
    {
        board_frame(&board);
        win = play_turn(&mut board, &mut turn);
        finish = win || is_full(&board);
    }

    board_frame(&board);
    if win {
        match turn {
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

fn check_win(board: &Board, x: usize, y: usize) -> bool {
    let cell = board[y][x];
    if cell == Cell::Empty {
        return false;
    }

    for (dx, dy) in DIRECTIONS {
        let total = 1
            + count_dir(board, x, y, dx, dy, cell)
            + count_dir(board, x, y, -dx, -dy, cell);
        if total >= 5 {
            return true;
        }
    }
    false
}

fn main()
{
    gomoku();
}
