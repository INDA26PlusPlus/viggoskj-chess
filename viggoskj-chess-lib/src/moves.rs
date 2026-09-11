use crate::board::Square;
use crate::piece::PieceType;

#[derive(Copy, Clone)]
pub struct BasicMove {
    pub piece_square: Square,
    pub target_square: Square,
}

#[derive(Copy, Clone)]
pub enum AdvancedMove {
    KingSideCastle,
    QueenSideCastle,
    Promotion {
        piece_type: PieceType,
        basic_move: BasicMove,
    },
}

#[derive(Copy, Clone)]
pub enum Move {
    Advanced { chess_move: AdvancedMove },
    Basic { chess_move: BasicMove },
}
