use std::{panic::panic_any, vec};

use viggoskj_chess_lib::{
    bitboard::{self, Bitboard, bitboard_string, displace},
    game, parsing,
    piece::{self},
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
        "a2a4", "h7h6", "a4a5", "h6h5", "a5a6", "h5h4", "a6b7", "h4h3", "b7a8n",
    ];

    let moves_ant_pessant = vec!["d2d3", "h7h6", "d3d4", "g7g6", "d4d5", "e7e5", "d5e6"];

    play_game(moves_with_queen_castle);
    play_game(moves_with_promotion);
    play_game(moves_ant_pessant);
}

fn play_game(moves: std::vec::Vec<&str>) {
    let mut g = viggoskj_chess_lib::create_game();

    for ms in moves {
        let m = parsing::parse_move(ms).unwrap();

        let r = viggoskj_chess_lib::game::play_move(&g, m);
        match r {
            Ok(g2) => {
                g = g2;
                println!("{}", g.board.to_string());
            }
            Err(v) => {
                println!("{}", ms);
                match m {
                    viggoskj_chess_lib::moves::Move::Basic { chess_move } => {
                        println!(
                            "{}",
                            bitboard_string(
                                viggoskj_chess_lib::game::pice_moves_bitboard(
                                    &g,
                                    chess_move.piece_square
                                )
                                .unwrap()
                            )
                        );
                    }
                    viggoskj_chess_lib::moves::Move::Advanced { chess_move } => match chess_move {
                        viggoskj_chess_lib::moves::AdvancedMove::KingSideCastle => {
                            println!("kingside castle")
                        }
                        viggoskj_chess_lib::moves::AdvancedMove::Promotion {
                            piece_type,
                            basic_move,
                        } => {
                            println!(
                                "{}",
                                bitboard_string(
                                    viggoskj_chess_lib::game::pice_moves_bitboard(
                                        &g,
                                        basic_move.piece_square
                                    )
                                    .unwrap()
                                )
                            );
                            println!("{}", piece_type.to_char());
                        }
                        viggoskj_chess_lib::moves::AdvancedMove::QueenSideCastle => {
                            println!("queenside castle")
                        }
                    },
                }
                Err(v).unwrap()
            }
        };
    }
}
