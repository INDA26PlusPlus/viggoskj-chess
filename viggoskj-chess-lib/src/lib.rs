pub mod board;
pub mod game;
pub mod bit_board;
pub mod chess_error;
pub mod piece;

pub fn create_game() -> game::Game {
    let game: game::Game = game::Game {
        board: board::create_start_board(),
        turn: game::Color::White
    };

    return game;
}