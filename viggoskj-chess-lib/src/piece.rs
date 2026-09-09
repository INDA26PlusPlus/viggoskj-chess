use std::ptr::read;

use crate::bit_board::{BitBoard, displace};
use crate::board::validate_square;
use crate::chess_error::ChessError;
use crate::game::Color;
use crate::{bit_board, game};

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Queen,
    King,
    Knight,
}

impl PieceType {
    pub fn to_char(&self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Rook => 'r',
            PieceType::Bishop => 'b',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
            PieceType::Knight => 'h',
        }
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_color: game::Color,
    pub piece_type: PieceType,
    pub board_position: BitBoard,
}

#[derive(Copy, Clone)]
pub struct Move {
    pub origin_row: u32,
    pub origin_col: u32,
    pub target_row: u32,
    pub target_col: u32,
}

pub fn parse_move(move_str: &str) -> Result<Move, ChessError> {
    if move_str.len() != 4 {
        return Err(ChessError::InvalidMove);
    } else {
        return Ok(Move {
            origin_col: parse_col(move_str.chars().nth(0).unwrap())?,
            origin_row: parse_row(move_str.chars().nth(1).unwrap())?,
            target_col: parse_col(move_str.chars().nth(2).unwrap())?,
            target_row: parse_row(move_str.chars().nth(3).unwrap())?,
        })
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
        _ => Err(ChessError::InvalidSquare)
    }
}

pub fn validate_move(chess_move: Move) -> Result<(), ChessError>
{
    validate_square(chess_move.origin_row, chess_move.origin_col)?;
    validate_square(chess_move.target_row, chess_move.target_col)?;
    Ok(())
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
        _ => Err(ChessError::InvalidSquare)
    }
}

pub fn piece_basic_move_bit_board(
    piece: Piece,
    solid_mask: BitBoard,
    attack_mask: BitBoard,
) -> BitBoard {
    match piece.piece_type {
        PieceType::Pawn => single_move_bit_board(
            match piece.piece_color {
                Color::White => 1,
                Color::Black => -1,
            },
            0,
            piece.board_position,
            solid_mask,
        ),
        PieceType::Rook => cross_move_bit_board(piece.board_position, solid_mask, attack_mask),
        PieceType::Bishop => x_move_bit_board(piece.board_position, solid_mask, attack_mask),
        PieceType::Queen => {
            cross_move_bit_board(piece.board_position, solid_mask, attack_mask)
                | x_move_bit_board(piece.board_position, solid_mask, attack_mask)
        }
        PieceType::King => king_move_bit_board(piece.board_position),
        PieceType::Knight => knight_move_bit_board(piece.board_position),
    }
}

fn cross_move_bit_board(
    piece_placement: BitBoard,
    solid_mask: BitBoard,
    attack_mask: BitBoard,
) -> BitBoard {
    move_bit_board(1, 0, piece_placement, solid_mask, attack_mask)
        | move_bit_board(0, 1, piece_placement, solid_mask, attack_mask)
        | move_bit_board(-1, 0, piece_placement, solid_mask, attack_mask)
        | move_bit_board(0, -1, piece_placement, solid_mask, attack_mask)
}

fn x_move_bit_board(
    piece_placement: BitBoard,
    solid_mask: BitBoard,
    attack_mask: BitBoard,
) -> BitBoard {
    move_bit_board(1, 1, piece_placement, solid_mask, attack_mask)
        | move_bit_board(1, -1, piece_placement, solid_mask, attack_mask)
        | move_bit_board(-1, -1, piece_placement, solid_mask, attack_mask)
        | move_bit_board(-1, 1, piece_placement, solid_mask, attack_mask)
}

fn king_move_bit_board(piece_placement: BitBoard) -> BitBoard {
    displace(piece_placement, -1, -1)
        | displace(piece_placement, -1, 0)
        | displace(piece_placement, -1, 1)
        | displace(piece_placement, 0, -1)
        | displace(piece_placement, 0, 1)
        | displace(piece_placement, 1, -1)
        | displace(piece_placement, 1, 0)
        | displace(piece_placement, 1, 1)
}

fn knight_move_bit_board(piece_placement: BitBoard) -> BitBoard {
    displace(piece_placement, -2, -1)
        | displace(piece_placement, -2, 1)
        | displace(piece_placement, -1, -2)
        | displace(piece_placement, -1, 2)
        | displace(piece_placement, 1, -2)
        | displace(piece_placement, 1, 2)
        | displace(piece_placement, 2, -1)
        | displace(piece_placement, 2, 1)
}

fn move_bit_board(
    row_move: i32,
    col_move: i32,
    piece_placement: BitBoard,
    solid_mask: BitBoard,
    attack_mask: BitBoard,
) -> BitBoard {
    let mut prev = u64::MAX;
    let mut now: u64 = piece_placement;

    while prev != now {
        prev = now;
        now = single_move_bit_board(row_move, col_move, now, solid_mask) & !(attack_mask);
    }

    now = single_move_bit_board(row_move, col_move, now, solid_mask);

    return now & (!piece_placement);
}

fn single_move_bit_board(
    row_move: i32,
    col_move: i32,
    piece_placement: BitBoard,
    solid_mask: BitBoard,
) -> BitBoard {
    (piece_placement | displace(piece_placement, row_move, col_move)) & (!solid_mask)
}
