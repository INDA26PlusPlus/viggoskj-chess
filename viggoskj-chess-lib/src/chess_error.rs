#[derive(Debug, PartialEq)]
pub enum ChessError {
    InvalidSquare,
    InvalidMove { reason: InvalidMoveReason },
    InvalidMoveString,
    InvalidBoardString,
}

#[derive(Debug, PartialEq )]
pub enum InvalidMoveReason {
    Generic,
    NotAMoveOption,
    NoTargetPiece,
    WrongColor,
    InvalidPromotionPiece,
}
