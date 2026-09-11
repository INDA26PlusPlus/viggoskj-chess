pub mod bitboard;
pub mod board;
pub mod chess_error;
pub mod game;
pub mod piece;
pub mod moves;
pub mod parsing;
mod advanced_moves;
mod instantiation;

pub fn create_game() -> game::Game {
    let game: game::Game = game::Game {
        board: board::create_start_board(),
        turn: game::Color::White,
        black_rook_left_moved: false,
        black_rook_right_moved: false,
        white_king_moved: false,
        white_rook_left_moved: false,
        white_rook_right_moved: false,
        black_king_moved: false,
    };

    return game;
}
