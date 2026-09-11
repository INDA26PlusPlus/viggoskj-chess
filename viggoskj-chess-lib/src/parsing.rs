use crate::board;
use crate::chess_error::ChessError;
use crate::moves;
use crate::piece::PieceType;

pub fn parse_move(move_str: &str) -> Result<moves::Move, ChessError> {
    match move_str.len() {
        4 => Ok(moves::Move::Basic {
            chess_move: parse_basic_move(move_str)?,
        }),
        5 => Ok(moves::Move::Advanced {
            chess_move: parse_promotion_move(move_str)?,
        }),
        _ => Err(ChessError::InvalidMoveString),
    }
}

pub fn parse_square(move_str: &str) -> Result<board::Square, ChessError> {
    if move_str.len() != 2 {
        return Err(ChessError::InvalidMoveString);
    } else {
        return Ok(board::Square {
            col: parse_col(move_str.chars().nth(0).unwrap())?,
            row: parse_row(move_str.chars().nth(1).unwrap())?,
        });
    }
}
fn parse_basic_move(move_str: &str) -> Result<moves::BasicMove, ChessError> {
    let (piece_str, target_str) = move_str.split_at(2);
    return Ok(moves::BasicMove {
        target_square: parse_square(target_str)?,
        piece_square: parse_square(piece_str)?,
    });
}

fn parse_promotion_move(move_str: &str) -> Result<moves::AdvancedMove, ChessError> {
    let (basic_str, promotion_type) = move_str.split_at(4);
    return Ok(moves::AdvancedMove::Promotion {
        piece_type: char_to_piece_type(promotion_type.chars().nth(0).unwrap())?,
        basic_move: parse_basic_move(basic_str)?,
    });
}

fn char_to_piece_type(c: char) -> Result<PieceType, ChessError> {
    match c {
        'p' => Ok(PieceType::Pawn),
        'r' => Ok(PieceType::Rook),
        'b' => Ok(PieceType::Bishop),
        'q' => Ok(PieceType::Queen),
        'k' => Ok(PieceType::King),
        'n' => Ok(PieceType::Knight),
        _ => Err(ChessError::InvalidMoveString),
    }
}

fn parse_row(c: char) -> Result<u32, ChessError> {
    match c {
        '1' => Ok(0),
        '2' => Ok(1),
        '3' => Ok(2),
        '4' => Ok(3),
        '5' => Ok(4),
        '6' => Ok(5),
        '7' => Ok(6),
        '8' => Ok(7),
        _ => Err(ChessError::InvalidMoveString),
    }
}

fn parse_col(c: char) -> Result<u32, ChessError> {
    match c {
        'a' => Ok(0),
        'b' => Ok(1),
        'c' => Ok(2),
        'd' => Ok(3),
        'e' => Ok(4),
        'f' => Ok(5),
        'g' => Ok(6),
        'h' => Ok(7),
        _ => Err(ChessError::InvalidMoveString),
    }
}
