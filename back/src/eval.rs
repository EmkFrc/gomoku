// L'heuristique : la note d'une position, du point de vue de `ai`.
// Positif = bon pour `ai`, négatif = bon pour son adversaire.
//
// PROVISOIRE : elle ne regarde que les captures, juste pour que la recherche compile
// et tourne. La vraie heuristique (segments, bouts libres, trous, paires menacées)
// s'écrit en dernier : voir la phase 6 de ROADMAP.md. Garder la même signature.

use crate::board::Stone;
use crate::game::Game;

pub fn evaluate(game: &Game, ai: Stone) -> i32 {
    let mine = game.captured(ai) as i32;
    let theirs = game.captured(ai.other()) as i32;
    (mine - theirs) * 100
}
