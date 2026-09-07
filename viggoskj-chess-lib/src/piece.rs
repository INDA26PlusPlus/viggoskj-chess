use crate::game;

pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Queen,
    King,
    Knight,
}

pub struct Piece
{
    piece_color: game::Color,
    pice_type: PieceType,
}