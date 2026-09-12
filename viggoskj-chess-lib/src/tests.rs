#[cfg(test)]

mod tests {
    use crate::{
        board::Square,
        chess_error::ChessError,
        create_game,
        game::{Color, Game, play_move},
        moves::BasicMove,
        parsing::{parse_board, parse_move},
    };

    use super::*;

    fn basic_move(col1: u32, row1: u32, col2: u32, row2: u32) -> BasicMove {
        return BasicMove {
            piece_square: Square {
                row: row1,
                col: col1,
            },
            target_square: Square {
                row: row2,
                col: col2,
            },
        };
    }

    fn game_from_board(board_str: &str) -> Game {
        Game {
            white_rook_left_moved: false,
            white_rook_right_moved: false,
            white_king_moved: false,
            black_rook_left_moved: false,
            black_rook_right_moved: false,
            black_king_moved: false,
            white_en_pessant: 0,
            black_en_pessant: 0,
            board: parse_board(
                board_str
                    .replace(' ', "")
                    .replace('\t', "")
                    .replace('\n', ""),
            )
            .unwrap(),
            turn: Color::White,
        }
    }

    fn board_str_equal(b1: String, b2: String) {
        assert_eq!(
            b1.replace(' ', "").replace('\t', "").replace('\n', ""),
            b2.replace(' ', "").replace('\t', "").replace('\n', "")
        )
    }

    #[test]
    fn parse_board_correct() {
        let string = "RNBQKBNR
PPPPPPPP
--------
----q---
k-------
-----P--
pppppppp
rnbqkbnr
";
        assert_eq!(parse_board(string.to_string()).unwrap().to_string(), string);
    }

    #[test]
    fn game_default_board_correct() {
        let game = create_game();

        assert_eq!(
            game.board.to_string(),
            "RNBQKBNR
PPPPPPPP
--------
--------
--------
--------
pppppppp
rnbqkbnr
"
        );
    }

    #[test]
    fn white_first() {
        let game = create_game();

        assert_eq!(game.turn, Color::White);
    }

    #[test]
    fn basic_move_works() {
        let mut game = create_game();

        game = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(1, 0, 2, 2),
            },
        )
        .unwrap();

        board_str_equal(
            game.board.to_string(),
            "
            RNBQKBNR
            PPPPPPPP
            --------
            --------
            --------
            --n-----
            pppppppp
            r-bqkbnr
    "
            .to_string(),
        );
    }

    #[test]
    fn white_cant_play_black_basic() {
        let game = create_game();

        let game_res = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(1, 7, 2, 5),
            },
        );

        assert_eq!(
            game_res,
            Err(ChessError::InvalidMove {
                reason: crate::chess_error::InvalidMoveReason::WrongColor
            })
        );
    }

    #[test]
    fn white_castling() {
        let mut game = game_from_board(
            "
                --------
                --------
                --------
                --------
                --------
                --------
                --------
                r---k--r
            ",
        );

        game = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(4, 0, 2, 0),
            },
        )
        .unwrap();

        board_str_equal(
            game.board.to_string(),
            "
                --------
                --------
                --------
                --------
                --------
                --------
                --------
                --kr---r
    "
            .to_string(),
        );
    }

    #[test]
    fn white_promote() {
        let mut game = game_from_board(
            "
                --------
                ----p---
                --------
                --------
                --------
                --------
                --------
                --------
            ",
        );

        game = play_move(
            &game,
            crate::moves::Move::Advanced {
                chess_move: crate::moves::AdvancedMove::Promotion {
                    piece_type: crate::piece::PieceType::Knight,
                    basic_move: basic_move(4, 6, 4, 7),
                },
            },
        )
        .unwrap();

        board_str_equal(
            game.board.to_string(),
            "
                ----n---
                --------
                --------
                --------
                --------
                --------
                --------
                --------
    "
            .to_string(),
        );
    }

    #[test]
    fn cant_promote_to_king() {
        let game = game_from_board(
            "
                --------
                ----p---
                --------
                --------
                --------
                --------
                --------
                --------
            ",
        );

        let game_res = play_move(
            &game,
            crate::moves::Move::Advanced {
                chess_move: crate::moves::AdvancedMove::Promotion {
                    piece_type: crate::piece::PieceType::King,
                    basic_move: basic_move(4, 6, 4, 7),
                },
            },
        );

        assert_eq!(
            game_res,
            Err(ChessError::InvalidMove {
                reason: crate::chess_error::InvalidMoveReason::InvalidPromotionPiece
            })
        );
    }

    #[test]
    fn cant_promote_to_pawn() {
        let game = game_from_board(
            "
                --------
                ----p---
                --------
                --------
                --------
                --------
                --------
                --------
            ",
        );

        let game_res = play_move(
            &game,
            crate::moves::Move::Advanced {
                chess_move: crate::moves::AdvancedMove::Promotion {
                    piece_type: crate::piece::PieceType::Pawn,
                    basic_move: basic_move(4, 6, 4, 7),
                },
            },
        );

        assert_eq!(
            game_res,
            Err(ChessError::InvalidMove {
                reason: crate::chess_error::InvalidMoveReason::InvalidPromotionPiece
            })
        );
    }
}
