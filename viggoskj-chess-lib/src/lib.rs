mod piece;
mod board;
mod game;
mod bool_board;

pub fn create_game() -> game::Game {
    let game = game::Game {
        board: board::create_start_board(),
    };

    return game;
}
