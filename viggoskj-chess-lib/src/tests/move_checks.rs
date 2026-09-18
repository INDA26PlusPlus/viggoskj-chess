#[cfg(test)]
mod tests {
    use crate::{
        board::to_square,
        create_game,
        game::{Color, play_move, possible_legal_moves},
        moves::{BasicMove, Move},
        parsing,
        tests::tests::{basic_move, game_from_board_advanced, is_same_movement},
    };

    fn moves_from_string(moves: String) -> Vec<Move> {
        if moves.len() == 0 {
            return vec![];
        }
        Vec::from_iter(moves.split(" ").map(|x| parsing::parse_move(x).unwrap()))
    }

    fn assert_same_moves(board_str: &str, moves_str: &str, color: Color) {
        let expected_moves = moves_from_string(moves_str.to_string());
        let game = game_from_board_advanced(board_str, color);
        let actual_moves = possible_legal_moves(&game);
        let mut missing = Vec::new();
        let mut overflow = Vec::new();

        // Expected but not generated
        for expected in &expected_moves {
            let found = actual_moves.iter().any(|actual| {
                is_same_movement(
                    *expected,
                    Move::Basic {
                        chess_move: *actual,
                    },
                    game.turn,
                )
            });

            if !found {
                missing.push(expected);
            }
        }

        for actual in actual_moves {
            let actual_move = Move::Basic {
                chess_move: actual
            };

            let found = expected_moves
                .iter()
                .any(|expected| is_same_movement(*expected, actual_move, game.turn));

            if !found {
                overflow.push(actual_move);
            }
        }

        if !missing.is_empty() || !overflow.is_empty() {
            println!("Missing moves:");
            for m in &missing {
                println!("  {:?}", m);
            }

            println!("Overflow moves:");
            for m in &overflow {
                println!("  {:?}", m);
            }

            panic!(
                "Move sets differ: {} missing, {} overflow",
                missing.len(),
                overflow.len()
            );
        }
    }

    #[test]
    fn initial_move_counts_1() {
        let mut game = create_game();

        assert_eq!(possible_legal_moves(&game).len(), 20);

        game = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(0, 1, 0, 2),
            },
        )
        .unwrap();

        assert_eq!(possible_legal_moves(&game).len(), 20);
    }

    #[test]
    fn correct_moves_1() {
        assert_same_moves(
            "--------
-----p-p
-------k
----pqR-
-------P
-----PK-
------P-
--------",
            "f5c8 f5d7 f5g6 f5f6 f5e6 f5g5 f5g4 f5f4 f5e4 f5h3 f5f3 f5d3 f5c2 f5b1 f7f6 e5e4",
            Color::Black,
        );
    }

    #[test]
    fn correct_moves_2() {
        assert_same_moves(
            "
r--q-rk-
-pp--pp-
p-n-bn-p
--bppN--
-PBPP---
--P--N--
P----PPP
R-BQK--R",
            "g8h8 g8h7 f8e8 d8e8 d8c8 d8b8 d8e7 d8d7 d8d6 a8c8 a8b8 a8a7 f6e8 f6h7 f6d7 f6h5 f6g4 f6e4 e6c8 e6d7 e6f5 c6b8 c6e7 c6a7 c6a5 c6d4 c6b4 c5e7 c5a7 c5d6 c5b6 c5d4 c5b4 e5d4 d5e4 d5c4 g7g6 b7b6 h6h5 a6a5 g7g5 b7b5",
            Color::Black,
        );
    }

    #[test]
    fn correct_moves_3() {
        assert_same_moves(
            "
rnq--rk-
pb--bppp
-p-ppn--
--------
PPPNP---
------P-
---N-PBP
R-BQ-RK-",
            "d4e6 d4c6 d4f5 d4b5 d4f3 d4b3 d4e2 d4c2 g2h3 g2f3 g2h1 d2f3 d2b3 d2b1 g1h1 f1e1 d1h5 d1g4 d1f3 d1b3 d1e2 d1c2 d1e1 c1a3 c1b2 a1a3 a1a2 a1b1 e4e5 c4c5 b4b5 a4a5 g3g4 h2h3 f2f3 h2h4 f2f4",
            Color::White,
        );
    }

    #[test]
    fn stalemate_white() {
        assert_same_moves(
            "
--------
--------
--------
--r----r
---K----
----r--r
--------
--------
",
            "",
            Color::White,
        );
    }
}
