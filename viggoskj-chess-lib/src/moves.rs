use crate::board::Square;

#[derive(Copy, Clone)]
pub struct BasicMove {
    pub piece_square: Square,
    pub target_square: Square,
}

#[derive(Copy, Clone)]
pub enum AdvancedMove {
    KingSideCastle,
    QueenSideCastle,
}

#[derive(Copy, Clone)]
pub enum Move {
    Advanced { chess_move: AdvancedMove },
    Basic { chess_move: BasicMove },
}