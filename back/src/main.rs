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

fn ask_moove(board: &Board) -> (usize, usize) {
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

fn play_turn(board: &mut Board, turn: &mut Stone)
{
    let (x, y) = ask_moove(board);

    board[y][x] = match *turn { // on choisie la caractère soit X ou O
        Stone::Black => Cell::Black,
        Stone::White => Cell::White,
    };

    *turn = match *turn { // on inverse la couleur
        Stone::Black => Stone::White,
        Stone::White => Stone::Black,
    };
}

fn gomoku()
{
    let mut board: Board = [[Cell::Empty; 19]; 19];
    let mut finish = false;
    let mut turn = Stone::Black;
    while finish != true
    {
        board_frame(&board);
        play_turn(&mut board, &mut turn);
    }
}

fn main()
{
    gomoku();
}
