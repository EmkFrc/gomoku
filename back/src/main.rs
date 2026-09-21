#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Black,
    White,
}

fn main() 
{
    let mut board = [[Cell::Empty; 19]; 19];

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
