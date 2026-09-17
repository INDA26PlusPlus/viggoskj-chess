#[cfg(test)]
mod tests {
    use crate::{
        bitboard::bitboard_string,
        board::{Square, to_square},
        check::{CheckState, get_check_state},
        chess_error::{ChessError, InvalidMoveReason::NotAMoveOption},
        create_game,
        game::{Color, legal_moves_bitboard, play_move, possible_legal_moves},
        moves::{BasicMove, Move, is_same_movement},
        parsing,
        tests::tests::{basic_move, board_str_equal, game_from_board, game_from_board_advanced},
    };

    use super::*;

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
                        chess_move: BasicMove {
                            piece_square: to_square(actual.piece.board_position).unwrap(),
                            target_square: actual.taget_square,
                        },
                    },
                    game.turn,
                )
            });

            if !found {
                missing.push(expected);
            }
        }

        for actual in &actual_moves {
            let actual_move = Move::Basic {
                chess_move: BasicMove {
                    piece_square: to_square(actual.piece.board_position).unwrap(),
                    target_square: actual.taget_square,
                },
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
    fn check_mate_1() {
        let game = game_from_board_advanced(
            "
--------
--------
--------
--r----r
---K-r--
----r--r
--------    
--------",
            Color::White,
        );

        assert_eq!(get_check_state(&game), Some(CheckState::Checkmate));
    }
}
