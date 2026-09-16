use crate::game::{Game, playing_board, possible_initialy_legal_oponent_turn};

pub fn is_check(game: &Game) -> bool {
    let attack_board = possible_initialy_legal_oponent_turn(game)
        .iter()
        .fold(0, |a, b| a | b.moves);

    let (playing, _) = playing_board(game);

    return playing.kings & attack_board > 0;
}
