#[derive(Debug)]
pub enum ChessError {
    InvalidSquare,
    InvalidMove {reason: InvalidMoveReason},
    InvalidMoveString,
}

#[derive(Debug)]
pub enum InvalidMoveReason
{
    Generic,
    NotAMoveOption,
    NoTargetPiece,
}