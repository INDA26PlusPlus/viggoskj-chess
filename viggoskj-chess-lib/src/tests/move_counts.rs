#[cfg(test)]
mod tests {
    use crate::{
        chess_error::{ChessError, InvalidMoveReason::NotAMoveOption},
        create_game,
        game::{play_move, possible_moves},
        moves::BasicMove,
        tests::tests::{basic_move, board_str_equal, game_from_board},
    };

    use super::*;

    #[test]
    fn initial_move_counts() {
        let mut game = create_game();

        assert_eq!(possible_moves(&game).len(), 20);

        game = play_move(
            &game,
            crate::moves::Move::Basic {
                chess_move: basic_move(0, 1, 0, 2),
            },
        )
        .unwrap();

        assert_eq!(possible_moves(&game).len(), 20);
    }
}
