pub mod bitboard;
pub mod board;
pub mod chess_error;
pub mod game;
pub mod piece;
pub mod moves;
pub mod parsing;
pub mod check;
pub mod advanced_moves;
mod tests;
mod instantiation;


pub use bitboard::*;
pub use board::*;
pub use chess_error::*;
pub use game::*;
pub use piece::*;
pub use moves::*;
pub use parsing::*;
pub use check::*;
pub use advanced_moves::*;

/// creates the initial
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
        black_en_pessant: 0,
        white_en_pessant: 0
    };

    return game;
}
