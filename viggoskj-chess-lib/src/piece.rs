use crate::bitboard::{Bitboard};
use crate::game::Color;

#[derive(Copy, Clone, PartialEq, Debug)]
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
    pub piece_color: Color,
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

impl Piece {
    pub fn to_char(&self) -> char {
        match self.piece_color {
          Color::Black => self.piece_type.to_char(),
          Color::White => self.piece_type.to_char().to_ascii_uppercase(),
        }
    }
}


pub(crate) fn if_piece_type(
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