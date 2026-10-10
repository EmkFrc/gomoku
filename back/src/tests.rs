// Tests des règles, rangés par chapitre (un module par règle).
// Lancer : cargo test            (tout)
//          cargo test captures   (un seul chapitre)

use super::*;

// Plateau avec des pierres noires et blanches posées à la main.
fn board(black: &[(usize, usize)], white: &[(usize, usize)]) -> Board {
    let mut b: Board = [[Cell::Empty; 19]; 19];
    for &(x, y) in black {
        b[y][x] = Cell::Black;
    }
    for &(x, y) in white {
        b[y][x] = Cell::White;
    }
    b
}

// Partie prête à jouer, au tour des noirs.
fn game(black: &[(usize, usize)], white: &[(usize, usize)]) -> Game {
    let mut g = Game::new();
    g.board = board(black, white);
    g
}

fn sorted(mut v: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    v.sort();
    v
}

fn row(xs: std::ops::Range<usize>, y: usize) -> Vec<(usize, usize)> {
    xs.map(|x| (x, y)).collect()
}

mod chapitre_1_alignement {
    use super::*;

    fn lines(b: &Board, x: usize, y: usize) -> Vec<Vec<(usize, usize)>> {
        find_lines(b, x, y).into_iter().map(sorted).collect()
    }

    #[test]
    fn cinq_horizontaux_pose_au_milieu() {
        let h = row(3..8, 5);
        assert_eq!(lines(&board(&h, &[]), 5, 5), vec![h]);
    }

    #[test]
    fn cinq_horizontaux_pose_aux_deux_bouts() {
        let h = row(3..8, 5);
        assert_eq!(lines(&board(&h, &[]), 3, 5), vec![h.clone()]);
        assert_eq!(lines(&board(&h, &[]), 7, 5), vec![h]);
    }

    #[test]
    fn cinq_verticaux() {
        let v: Vec<_> = (2..7).map(|y| (9, y)).collect();
        assert_eq!(lines(&board(&v, &[]), 9, 4), vec![v]);
    }

    #[test]
    fn cinq_en_diagonale() {
        let d: Vec<_> = (0..5).map(|k| (4 + k, 4 + k)).collect();
        assert_eq!(lines(&board(&d, &[]), 6, 6), vec![d]);
    }

    #[test]
    fn cinq_en_anti_diagonale() {
        let a: Vec<_> = (0..5).map(|k| (10 - k, 2 + k)).collect();
        assert_eq!(lines(&board(&a, &[]), 8, 4), vec![sorted(a)]);
    }

    #[test]
    fn six_alignes_gagnent_aussi() {
        let six = row(2..8, 9);
        assert_eq!(lines(&board(&six, &[]), 4, 9), vec![six]);
    }

    #[test]
    fn quatre_ne_suffisent_pas() {
        assert!(find_lines(&board(&row(3..7, 5), &[]), 5, 5).is_empty());
    }

    #[test]
    fn un_trou_coupe_la_ligne() {
        let mut b = row(3..5, 5);
        b.extend(row(6..9, 5));
        assert!(find_lines(&board(&b, &[]), 4, 5).is_empty());
    }

    #[test]
    fn contre_le_bord_du_plateau() {
        let e = row(14..19, 0);
        assert_eq!(lines(&board(&e, &[]), 18, 0), vec![e]);
    }

    #[test]
    fn croix_donne_deux_lignes() {
        let h = row(3..8, 5);
        let v: Vec<_> = (3..8).map(|y| (5, y)).collect();
        let mut stones = h.clone();
        stones.extend(v.iter().filter(|&&p| p != (5, 5)));
        assert_eq!(lines(&board(&stones, &[]), 5, 5), vec![h, v]);
    }

    #[test]
    fn case_vide_aucune_ligne() {
        assert!(find_lines(&board(&[], &[]), 5, 5).is_empty());
    }
}

mod chapitre_2_captures {
    use super::*;

    fn caps(b: &Board, x: usize, y: usize) -> Vec<(usize, usize)> {
        sorted(find_captures(b, x, y, Cell::Black, Cell::White))
    }

    #[test]
    fn paire_horizontale_des_deux_cotes() {
        let b = board(&[(3, 3), (6, 3)], &[(4, 3), (5, 3)]);
        assert_eq!(caps(&b, 3, 3), vec![(4, 3), (5, 3)]);
        assert_eq!(caps(&b, 6, 3), vec![(4, 3), (5, 3)]);
    }

    #[test]
    fn paire_verticale() {
        let b = board(&[(5, 5), (5, 2)], &[(5, 4), (5, 3)]);
        assert_eq!(caps(&b, 5, 5), vec![(5, 3), (5, 4)]);
    }

    #[test]
    fn paire_en_diagonale_et_anti_diagonale() {
        let d = board(&[(2, 2), (5, 5)], &[(3, 3), (4, 4)]);
        assert_eq!(caps(&d, 2, 2), vec![(3, 3), (4, 4)]);
        let a = board(&[(6, 2), (3, 5)], &[(5, 3), (4, 4)]);
        assert_eq!(caps(&a, 6, 2), vec![(4, 4), (5, 3)]);
    }

    #[test]
    fn double_capture_en_un_coup() {
        let b = board(&[(3, 3), (6, 3), (3, 6)], &[(4, 3), (5, 3), (3, 4), (3, 5)]);
        assert_eq!(caps(&b, 3, 3), vec![(3, 4), (3, 5), (4, 3), (5, 3)]);
    }

    #[test]
    fn trois_pierres_ne_se_capturent_pas() {
        let b = board(&[(3, 3), (7, 3)], &[(4, 3), (5, 3), (6, 3)]);
        assert!(caps(&b, 3, 3).is_empty());
    }

    #[test]
    fn une_seule_pierre_ne_se_capture_pas() {
        let b = board(&[(3, 3), (5, 3)], &[(4, 3)]);
        assert!(caps(&b, 3, 3).is_empty());
    }

    #[test]
    fn il_faut_ma_pierre_au_bout() {
        let b = board(&[(3, 3)], &[(4, 3), (5, 3)]);
        assert!(caps(&b, 3, 3).is_empty());
    }

    #[test]
    fn bord_du_plateau_sans_plantage() {
        assert!(caps(&board(&[(16, 0)], &[(17, 0), (18, 0)]), 16, 0).is_empty());
        assert!(caps(&board(&[(0, 0)], &[]), 0, 0).is_empty());
    }

    #[test]
    fn capture_depuis_la_colonne_zero() {
        let b = board(&[(0, 0), (3, 0)], &[(1, 0), (2, 0)]);
        assert_eq!(caps(&b, 0, 0), vec![(1, 0), (2, 0)]);
    }

    #[test]
    fn se_placer_entre_deux_adverses_est_sans_danger() {
        let b = board(&[(4, 3)], &[(3, 3), (5, 3), (6, 3)]);
        assert!(caps(&b, 4, 3).is_empty());
    }
}

mod chapitre_3_partie {
    use super::*;

    #[test]
    fn le_tour_alterne() {
        let mut g = Game::new();
        assert!(g.turn == Stone::Black);
        g.play(3, 3);
        assert!(g.turn == Stone::White);
        g.play(4, 4);
        assert!(g.turn == Stone::Black);
    }

    #[test]
    fn play_retire_les_pierres_et_compte() {
        let mut g = Game::new();
        for &(x, y) in &[(3, 3), (4, 3), (10, 10), (5, 3), (6, 3)] {
            g.play(x, y);
        }
        assert!(g.board[3][4] == Cell::Empty && g.board[3][5] == Cell::Empty);
        assert_eq!(g.captured_black, 2);
        assert_eq!(g.captured_white, 0);
        assert_eq!(sorted(g.history.last().unwrap().captures.clone()), vec![(4, 3), (5, 3)]);
    }

    #[test]
    fn historique_contient_le_coup_gagnant() {
        let mut g = Game::new();
        let mut win = false;
        for &(x, y) in &[(0, 0), (0, 5), (1, 0), (0, 6), (2, 0), (0, 7), (3, 0), (0, 8), (4, 0)] {
            win = g.play(x, y);
        }
        assert!(win);
        assert!(g.turn == Stone::Black);
        assert_eq!(g.history.len(), 9);
        let last = g.history.last().unwrap();
        assert_eq!((last.x, last.y), (4, 0));
    }

    #[test]
    fn plateau_plein() {
        let mut g = Game::new();
        g.board = [[Cell::Black; 19]; 19];
        assert!(g.is_full());
        g.board[9][9] = Cell::Empty;
        assert!(!g.is_full());
    }
}

mod chapitre_4_annulation {
    use super::*;

    #[test]
    fn chaque_undo_retrouve_l_etat_exact() {
        let moves = [
            (3, 3), (4, 3), (10, 10), (5, 3), (6, 3),
            (7, 7), (7, 8), (0, 0), (7, 9), (7, 10),
            (6, 12), (4, 12), (3, 15), (5, 12), (15, 0), (3, 13), (15, 2), (3, 14),
            (3, 12),
        ];
        let mut g = Game::new();
        let mut snaps = Vec::new();
        for &(x, y) in moves.iter() {
            snaps.push((g.board, g.turn == Stone::Black, g.captured_black, g.captured_white));
            g.play(x, y);
        }
        assert_eq!((g.captured_black, g.captured_white), (6, 2));
        for i in (0..moves.len()).rev() {
            assert!(g.undo());
            let (b, black, cb, cw) = snaps[i];
            assert!(g.board == b, "plateau différent après l'annulation du coup {}", i);
            assert_eq!(g.turn == Stone::Black, black);
            assert_eq!((g.captured_black, g.captured_white), (cb, cw));
        }
    }

    #[test]
    fn undo_sans_historique_renvoie_false() {
        let mut g = Game::new();
        assert!(!g.undo());
    }

    #[test]
    fn undo_rend_le_tour_a_celui_qui_avait_joue() {
        let mut g = Game::new();
        g.play(3, 3);
        g.play(4, 4);
        g.undo();
        assert!(g.turn == Stone::White);
        assert!(g.board[4][4] == Cell::Empty);
    }
}

mod chapitre_5_victoire_par_capture {
    use super::*;

    // 5 captures noires séparées, sans alignement possible
    fn cinq_captures(g: &mut Game) -> Vec<bool> {
        let mut wins = Vec::new();
        for r in 0..5usize {
            let seq = [(2 * r, r), (2 * r + 1, r), (18, 2 * r), (2 * r + 2, r), (2 * r + 3, r), (16, 3 * r)];
            for (k, &(x, y)) in seq.iter().enumerate() {
                if r == 4 && k == 5 {
                    break;
                }
                let w = g.play(x, y);
                if k == 4 {
                    wins.push(w);
                }
            }
        }
        wins
    }

    #[test]
    fn dix_pierres_capturees_gagnent() {
        let mut g = Game::new();
        assert_eq!(cinq_captures(&mut g), vec![false, false, false, false, true]);
        assert_eq!(g.captured_black, 10);
        assert!(g.turn == Stone::Black);
    }

    #[test]
    fn undo_de_la_victoire_puis_rejouer() {
        let mut g = Game::new();
        cinq_captures(&mut g);
        g.undo();
        assert_eq!(g.captured_black, 8);
        assert!(g.board[4][9] == Cell::White && g.board[4][10] == Cell::White);
        assert!(g.play(11, 4));
    }
}

mod chapitre_6_ligne_cassable {
    use super::*;

    // 4 noirs en (3..7, 5) : le 5e se joue en (7, 5)
    fn four() -> Vec<(usize, usize)> {
        row(3..7, 5)
    }

    #[test]
    fn ligne_impossible_a_casser_gagne_tout_de_suite() {
        assert!(game(&four(), &[]).play(7, 5));
    }

    #[test]
    fn paire_de_la_ligne_capturable_la_partie_continue() {
        let mut b = four();
        b.push((4, 6));
        assert!(!game(&b, &[(4, 7)]).play(7, 5));
    }

    #[test]
    fn paire_capturable_hors_de_la_ligne_ne_sauve_pas() {
        let mut b = four();
        b.extend([(12, 12), (12, 13)]);
        assert!(game(&b, &[(12, 14)]).play(7, 5));
    }

    #[test]
    fn ligne_de_six_capturer_le_bout_laisse_cinq() {
        let mut b = row(3..8, 5);
        b.push((3, 6));
        assert!(game(&b, &[(3, 7)]).play(8, 5));
    }

    #[test]
    fn adversaire_a_huit_pierres_et_peut_capturer() {
        let mut b = four();
        b.extend([(12, 12), (12, 13)]);
        let mut g = game(&b, &[(12, 14)]);
        g.captured_white = 8;
        assert!(!g.play(7, 5));
    }

    #[test]
    fn adversaire_a_six_pierres_seulement() {
        let mut b = four();
        b.extend([(12, 12), (12, 13)]);
        let mut g = game(&b, &[(12, 14)]);
        g.captured_white = 6;
        assert!(g.play(7, 5));
    }

    #[test]
    fn adversaire_a_huit_mais_aucune_capture_possible() {
        let mut g = game(&four(), &[]);
        g.captured_white = 8;
        assert!(g.play(7, 5));
    }

    #[test]
    fn croix_une_seule_ligne_cassee_gagne() {
        let b = [(3, 5), (4, 5), (6, 5), (7, 5), (5, 3), (5, 4), (5, 6), (5, 7), (3, 6)];
        assert!(game(&b, &[(3, 7)]).play(5, 5));
    }

    #[test]
    fn croix_une_capture_casse_les_deux_lignes() {
        let b = [(3, 5), (4, 5), (6, 5), (7, 5), (5, 3), (5, 4), (5, 6), (5, 7), (4, 6)];
        assert!(!game(&b, &[(4, 7)]).play(5, 5));
    }
}

mod chapitre_7_ligne_en_attente {
    use super::*;

    fn breakable() -> Game {
        let mut b = row(3..7, 5);
        b.push((4, 6));
        game(&b, &[(4, 7)])
    }

    #[test]
    fn adversaire_ne_casse_pas_l_aligneur_gagne() {
        let mut g = breakable();
        assert!(!g.play(7, 5));
        assert!(g.play(15, 15));
        assert!(g.turn == Stone::Black);
    }

    #[test]
    fn adversaire_casse_la_partie_continue() {
        let mut g = breakable();
        g.play(7, 5);
        assert!(!g.play(4, 4));
        assert!(g.board[5][4] == Cell::Empty);
        assert!(!g.play(0, 18));
    }

    #[test]
    fn undo_puis_casser() {
        let mut g = breakable();
        g.play(7, 5);
        g.play(15, 15);
        g.undo();
        assert!(g.turn == Stone::White);
        assert!(!g.play(4, 4));
    }

    #[test]
    fn adversaire_a_huit_capture_et_gagne() {
        let mut b = row(3..7, 5);
        b.extend([(12, 12), (12, 13)]);
        let mut g = game(&b, &[(12, 14)]);
        g.captured_white = 8;
        g.play(7, 5);
        assert!(g.play(12, 11));
        assert!(g.turn == Stone::White);
        assert_eq!(g.captured_white, 10);
    }

    #[test]
    fn adversaire_a_huit_ne_capture_pas_l_aligneur_gagne() {
        let mut b = row(3..7, 5);
        b.extend([(12, 12), (12, 13)]);
        let mut g = game(&b, &[(12, 14)]);
        g.captured_white = 8;
        g.play(7, 5);
        assert!(g.play(15, 15));
        assert!(g.turn == Stone::Black);
    }

    #[test]
    fn la_premiere_ligne_gagne_si_l_adversaire_aligne_au_lieu_de_casser() {
        let mut b = row(3..7, 5);
        b.push((4, 6));
        let mut g = game(&b, &[(4, 7), (10, 0), (11, 0), (12, 0), (13, 0)]);
        g.play(7, 5);
        assert!(g.play(14, 0));
        assert!(g.turn == Stone::Black);
    }
}

mod chapitre_8_trois_libre {
    use super::*;

    fn free_threes(black: &[(usize, usize)], white: &[(usize, usize)], x: usize, y: usize) -> usize {
        let mut b = board(black, white);
        b[y][x] = Cell::Black;
        count_free_threes(&b, x, y, Cell::Black)
    }

    #[test]
    fn trois_colles() {
        assert_eq!(free_threes(&[(5, 9), (6, 9)], &[], 7, 9), 1);
    }

    #[test]
    fn trois_avec_trou() {
        assert_eq!(free_threes(&[(5, 9), (7, 9)], &[], 8, 9), 1);
        assert_eq!(free_threes(&[(5, 9), (6, 9)], &[], 8, 9), 1);
    }

    #[test]
    fn trois_en_diagonale() {
        assert_eq!(free_threes(&[(5, 5), (6, 6)], &[], 7, 7), 1);
    }

    #[test]
    fn bloque_aux_deux_bouts_eloignes_pas_libre() {
        // O . X X X . O
        assert_eq!(free_threes(&[(5, 9), (6, 9)], &[(3, 9), (9, 9)], 7, 9), 0);
    }

    #[test]
    fn bloque_d_un_cote_pas_libre() {
        assert_eq!(free_threes(&[(5, 9), (6, 9)], &[(4, 9)], 7, 9), 0);
    }

    #[test]
    fn contre_le_bord_pas_libre() {
        assert_eq!(free_threes(&[(0, 9), (1, 9)], &[], 2, 9), 0);
    }

    #[test]
    fn quatre_n_est_pas_un_trois() {
        assert_eq!(free_threes(&[(5, 9), (6, 9), (7, 9)], &[], 8, 9), 0);
    }

    #[test]
    fn deux_pierres_ou_trou_trop_grand() {
        assert_eq!(free_threes(&[(5, 9)], &[], 6, 9), 0);
        assert_eq!(free_threes(&[(4, 9), (7, 9)], &[], 8, 9), 0);
    }
}

mod chapitre_9_double_trois {
    use super::*;

    // exemple du sujet (annexe VI.2), décalé de +5 : coup a = (9, 9), b = (8, 9)
    const SUJET: [(usize, usize); 4] = [(6, 6), (7, 7), (10, 9), (11, 9)];

    #[test]
    fn exemple_du_sujet_interdit() {
        let g = game(&SUJET, &[]);
        assert!(g.is_double_three(9, 9));
        assert!(!g.is_valid_move(9, 9));
    }

    #[test]
    fn exemple_du_sujet_avec_pierre_en_b_legal() {
        assert!(game(&SUJET, &[(8, 9)]).is_valid_move(9, 9));
    }

    #[test]
    fn double_trois_qui_capture_est_autorise() {
        let mut black = SUJET.to_vec();
        black.push((9, 12));
        assert!(game(&black, &[(9, 10), (9, 11)]).is_valid_move(9, 9));
    }

    #[test]
    fn croix_de_deux_trois_interdite() {
        assert!(!game(&[(5, 9), (6, 9), (7, 7), (7, 8)], &[]).is_valid_move(7, 9));
    }

    #[test]
    fn un_seul_trois_libre_legal() {
        assert!(game(&[(5, 9), (6, 9)], &[]).is_valid_move(7, 9));
        assert!(game(&[(5, 9), (6, 9), (5, 2), (6, 2), (7, 2)], &[]).is_valid_move(7, 9));
    }

    #[test]
    fn double_quatre_et_quatre_trois_autorises() {
        assert!(game(&[(4, 9), (5, 9), (6, 9), (7, 6), (7, 7), (7, 8)], &[]).is_valid_move(7, 9));
        assert!(game(&[(4, 9), (5, 9), (6, 9), (7, 7), (7, 8)], &[]).is_valid_move(7, 9));
        assert!(game(&[(5, 9), (6, 9), (8, 9), (9, 9), (7, 6), (7, 7), (7, 8)], &[]).is_valid_move(7, 9));
    }

    #[test]
    fn la_regle_ne_vise_que_le_joueur_qui_joue() {
        let mut g = game(&SUJET, &[]);
        g.turn = Stone::White;
        assert!(g.is_valid_move(9, 9));
    }

    #[test]
    fn case_occupee_ou_hors_plateau() {
        assert!(!game(&[(5, 5)], &[]).is_valid_move(5, 5));
        assert!(!game(&[], &[]).is_valid_move(19, 3));
    }
}
