use crate::{
    game::{
        Game, playing_board, possible_initialy_legal_oponent_turn,
        possible_legal_moves,
    },
};

/// what check state a board is in
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CheckState {
    Stalemate,
    Checkmate,
    Check,
}

/// checks if the current turns player is in check
pub fn is_check(game: &Game) -> bool {
    let attack_board: u64 = possible_initialy_legal_oponent_turn(game)
        .iter()
        .fold(0, |a, b| a | b.moves);

    let (playing, _) = playing_board(game);

    return playing.kings & attack_board > 0;
}

/// checks if the current turns player is in check, checkmate, stalemate or none
pub fn get_check_state(game: &Game) -> Option<CheckState> {
    let legal_move_count = possible_legal_moves(game).len();

    match (legal_move_count, is_check(game)) {
        (0, true) => Some(CheckState::Checkmate),
        (0, false) => Some(CheckState::Stalemate),
        (_, false) => None,
        (_, true) => Some(CheckState::Check),
    }
}
