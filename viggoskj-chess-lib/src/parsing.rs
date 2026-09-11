use crate::chess_error::ChessError;
use crate::moves;
use crate::board;

pub fn parse_move(move_str: &str) -> Result<moves::BasicMove, ChessError> {
    if move_str.len() != 4 {
        return Err(ChessError::InvalidMoveString);
    } else {
        let (piece_str, target_str) = move_str.split_at(2);
        return Ok(moves::BasicMove {
            target_square: parse_square(target_str)?,
            piece_square: parse_square(piece_str)?,
        });
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
