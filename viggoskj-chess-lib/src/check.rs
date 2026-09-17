use std::arch::x86_64::_MM_CMPINT_TRUE;

use crate::{
    game::{
        Color, Game, if_other_turn, play_move, playing_board, possible_initialy_legal_oponent_turn,
        possible_legal_moves,
    },
    moves::possible_move_to_move,
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CheckState {
    Stalemate,
    Checkmate,
    Check,
}

pub fn is_check(game: &Game) -> bool {
    let attack_board: u64 = possible_initialy_legal_oponent_turn(game)
        .iter()
        .fold(0, |a, b| a | b.moves);

    let (playing, _) = playing_board(game);

    return playing.kings & attack_board > 0;
}

pub fn get_check_state(game: &Game) -> Option<CheckState> {
    let legal_move_count = possible_legal_moves(game).len();

    match (legal_move_count, is_check(game)) {
        (0, true) => Some(CheckState::Checkmate),
        (0, false) => Some(CheckState::Stalemate),
        (_, false) => None,
        (_, true) => Some(CheckState::Check),
    }
}
