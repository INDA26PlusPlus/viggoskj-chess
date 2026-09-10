use std::{panic::panic_any, vec};

use viggoskj_chess_lib::{
    bit_board::{self, BitBoard, bitboard_string, displace},
    game,
    piece::{self, piece_basic_move_bit_board},
};

pub fn main() {
    let moves_with_queen_castle = vec![
        "e2e4", "e7e5", "g1f3", "g8f6", "f3e5", "f6e4", "d1e2", "d8e7", "e2e4", "d7d6", "d2d4",
        "d6e5", "d4e5", "b8c6", "b1c3", "e7e5", "e4e5", "c6e5", "c3b5", "f8d6", "b5d6", "c7d6",
        "c1e3", "c8g4", "f2f3", "g4f5", "e1c1", "e8e7", "h2h4", "a8c8", "c2c3", "a7a6", "f1e2",
        "b7b5", "g2g4", "f5e6", "h1g1", "b5b4", "f3f4", "e5c4", "e2c4", "e6c4", "g1e1", "b4c3",
        "b2c3", "e7d7", "e3d4", "f7f6", "g4g5", "c4a2", "g5f6", "g7f6", "d4f6", "h8e8", "e1e8",
        "d7e8", "d1d6", "a2c4", "f6d4", "e8f8", "f4f5", "c8e8", "f5f6", "e8e6", "d6d8", "f8f7",
        "d8d7", "f7g6", "d7g7", "g6f5", "f6f7", "e6e1", "c1c2",
    ];

    let moves_with_promotion = vec![
        "a2a4", "h7h5", "a4a5", "h5h4", "a5a6", "h4h3", "a6b7", "h3g2", "b7a8q",
    ];

    let moves_with_black_king_castle = vec![
        "e2e4", "e7e6", "g1f3", "g8f6", "d2d3", "f8e7", "b1c3", "e8g8",
    ];

    play_game(moves_with_queen_castle);
    play_game(moves_with_black_king_castle);
}

fn play_game(moves: std::vec::Vec<&str>) {
    let mut g = viggoskj_chess_lib::create_game();

    for ms in moves {
        let m = piece::parse_move(ms).unwrap();

        let r = viggoskj_chess_lib::game::move_piece(&g, m);
        match r {
            Ok(g2) => {
                g = g2;
                println!("{}", g.board.to_string());
            }
            Err(v) => {
                println!("{}", ms);
                println!(
                    "{}",
                    bitboard_string(
                        viggoskj_chess_lib::game::pice_moves_bitboard(&g, m.piece_square).unwrap()
                    )
                );
                Err(v).unwrap()
            }
        };
    }
}
