use crate::{
    board::Square,
    game::{Color, Game},
    moves::BasicMove,
    parsing,
};

mod basic_tests;
mod en_pessant_tests;
mod move_counts;
#[cfg(test)]

pub mod tests {
    use crate::{board::Square, game::{Color, Game}, moves::BasicMove, parsing};


    pub fn basic_move(col1: u32, row1: u32, col2: u32, row2: u32) -> BasicMove {
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

    pub fn game_from_board(board_str: &str) -> Game {
        Game {
            white_rook_left_moved: false,
            white_rook_right_moved: false,
            white_king_moved: false,
            black_rook_left_moved: false,
            black_rook_right_moved: false,
            black_king_moved: false,
            white_en_pessant: 0,
            black_en_pessant: 0,
            board: parsing::parse_board(
                board_str
                    .replace(' ', "")
                    .replace('\t', "")
                    .replace('\n', ""),
            )
            .unwrap(),
            turn: Color::White,
        }
    }

    pub fn board_str_equal(b1: String, b2: String) {
        assert_eq!(
            b1.replace(' ', "").replace('\t', "").replace('\n', ""),
            b2.replace(' ', "").replace('\t', "").replace('\n', "")
        )
    }
}
