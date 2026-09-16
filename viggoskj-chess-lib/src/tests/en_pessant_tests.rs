#[cfg(test)]
mod tests {
    use crate::{
        chess_error::{ChessError, InvalidMoveReason::NotAMoveOption}, game::play_move, tests::tests::{basic_move, board_str_equal, game_from_board},
    };

    use super::*;

    #[test]
    fn white_en_pessant() {
        let mut game = game_from_board(
            "
                --------
                -----p--
                --------
                --------
                ----P---
                --------
                --------
                --------
            ",
        );

        game = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(4, 3, 4, 4),
            },
        )
        .unwrap();

        board_str_equal(
            game.board.to_string(),
            "
                --------
                -----p--
                --------
                ----P---
                --------
                --------
                --------
                --------
    "
            .to_string(),
        );

        game = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(5, 6, 5, 4),
            },
        )
        .unwrap();

        board_str_equal(
            game.board.to_string(),
            "
                --------
                --------
                --------
                ----Pp--
                --------
                --------
                --------
                --------
    "
            .to_string(),
        );

        game = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(4, 4, 5, 5),
            },
        )
        .unwrap();

        board_str_equal(
            game.board.to_string(),
            "
                --------
                --------
                -----P--
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
    fn black_en_pessant() {
        let mut game = game_from_board(
            "
                --------
                --------
                --------
                --------
                -----p--
                --------
                ----P---
                --------
            ",
        );

        game = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(4, 1, 4, 3),
            },
        )
        .unwrap();

        game = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(5, 3, 4, 2),
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
                ----p---
                --------
                --------
    "
            .to_string(),
        );
    }
    #[test]
    fn black_en_pessant_not_persisted() {
        let mut game = game_from_board(
            "
                --------
                --------
                --------
                --------
                -----pp-
                --------
                ---PP---
                --------
            ",
        );

        game = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(4, 1, 4, 3),
            },
        )
        .unwrap();

        game = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(6, 3, 6, 2),
            },
        )
        .unwrap();

        game = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(3, 1, 3, 3),
            },
        )
        .unwrap();

        let game_res = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(5, 3, 4, 2),
            },
        );

        assert_eq!(game_res, Err(ChessError::InvalidMove { reason: NotAMoveOption }));
    }
}
