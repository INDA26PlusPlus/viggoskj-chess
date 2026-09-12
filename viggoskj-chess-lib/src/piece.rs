use crate::bitboard::{Bitboard, displace, point};
use crate::board::{Square, validate_square};
use crate::chess_error::ChessError;
use crate::game::Color;
use crate::instantiation::{black_default_pawn_board, white_default_pawn_board};
use crate::moves::BasicMove;
use crate::{bitboard, game};

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Queen,
    King,
    Knight,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_color: game::Color,
    pub piece_type: PieceType,
    pub board_position: Bitboard,
}

impl PieceType {
    pub fn to_char(&self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Rook => 'r',
            PieceType::Bishop => 'b',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
            PieceType::Knight => 'n',
        }
    }
}


pub fn if_piece_type(
    piece_type: PieceType,
    required_piece_type: PieceType,
    board: Bitboard,
) -> Bitboard {
    if piece_type == required_piece_type {
        board
    } else {
        0
    }
}
