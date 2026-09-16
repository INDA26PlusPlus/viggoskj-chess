#[derive(Debug, PartialEq)]
pub enum ChessError {
    InvalidSquare,
    EmptySquare,
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
    ResultsInCheck,
}
