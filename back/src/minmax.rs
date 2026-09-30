
fn minimax(board: &mut Board, depth: u32, maximizing: bool) -> i32 {
    if depth == 0 {
        return evaluate(board);       // feuille : on note la position
    }

    let mut best = if maximizing { i32::MIN } else { i32::MAX };

    for (x, y) in candidate_moves(board) {
        board[y][x] = /* la pierre du joueur courant */;   // joue
        let score = minimax(board, depth - 1, !maximizing); // descend
        board[y][x] = Cell::Empty;                          // annule

        best = if maximizing { best.max(score) } else { best.min(score) };
    }
    best
}


fn evalutation_function(sommet: &i32)
{
    
}