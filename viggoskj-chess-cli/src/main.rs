use viggoskj_chess_lib::{
    bit_board::{BitBoard, bitboard_string, displace},
    piece::piece_basic_move_bit_board,
};

pub fn main() {
    let mut g = viggoskj_chess_lib::create_game();
    
    g = viggoskj_chess_lib::game::move_piece(&g, 1, 0, 2, 0).unwrap();
    g = viggoskj_chess_lib::game::move_piece(&g, 2, 0, 3, 0).unwrap();
    g = viggoskj_chess_lib::game::move_piece(&g, 0, 0, 2, 0).unwrap();
    g = viggoskj_chess_lib::game::move_piece(&g, 2, 0, 2, 7).unwrap();

    println!("{}", g.board.to_string());
    println!("{}", g.board.to_string());
}
