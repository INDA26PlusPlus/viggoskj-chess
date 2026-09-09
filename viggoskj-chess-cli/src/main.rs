use viggoskj_chess_lib::{
    bit_board::{self, BitBoard, bitboard_string, displace},
    game,
    piece::{self, piece_basic_move_bit_board},
};

pub fn main() {
    let mut g = viggoskj_chess_lib::create_game();

    g = viggoskj_chess_lib::game::move_piece(&g, piece::parse_move("a2a3").unwrap()).unwrap();
    println!("{}", g.board.to_string());
    g = viggoskj_chess_lib::game::move_piece(&g, piece::parse_move("a3a4").unwrap()).unwrap();
    println!("{}", g.board.to_string());
    g = viggoskj_chess_lib::game::move_piece(&g, piece::parse_move("a1a3").unwrap()).unwrap();
    println!("{}", g.board.to_string());
    g = viggoskj_chess_lib::game::move_piece(&g, piece::parse_move("a3h3").unwrap()).unwrap();
    println!("{}", g.board.to_string());
    g = viggoskj_chess_lib::game::move_piece(&g, piece::parse_move("h3h7").unwrap()).unwrap();
    println!("{}", g.board.to_string());
    g = viggoskj_chess_lib::game::move_piece(&g, piece::parse_move("h7g7").unwrap()).unwrap();
    println!("{}", g.board.to_string());
}
