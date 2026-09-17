use std::vec;

use viggoskj_chess_lib::{bitboard::bitboard_string, parsing};

pub fn main() {
    let moves_ant_pessant = vec!["d2d3", "h7h6", "d3d4", "g7g6", "d4d5", "e7e5", "d5e6"];

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
                                viggoskj_chess_lib::game::legal_moves_bitboard(
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
                                    viggoskj_chess_lib::game::legal_moves_bitboard(
                                        &g,
                                        basic_move.piece_square
                                    )
                                    .unwrap()
                                )
                            );
                            println!("{}", piece_type.to_char());
                        }
                        viggoskj_chess_lib::moves::AdvancedMove::EnPessant { basic_move } => {
                            println!(
                                "{}",
                                bitboard_string(
                                    viggoskj_chess_lib::game::legal_moves_bitboard(
                                        &g,
                                        basic_move.piece_square
                                    )
                                    .unwrap()
                                )
                            );
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
