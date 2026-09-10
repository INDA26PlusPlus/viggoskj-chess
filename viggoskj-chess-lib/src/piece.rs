use crate::bit_board::{BitBoard, displace, point};
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

#[derive(Copy, Clone)]
pub struct Square {
    pub row: u32,
    pub col: u32,
}

pub fn square_bitboard (square: Square) -> BitBoard
{
    point(square.row, square.col)
}

pub fn parse_move(move_str: &str) -> Result<BasicMove, ChessError> {
    if move_str.len() != 4 {
        return Err(ChessError::InvalidMoveString);
    } else {
        let (piece_str, target_str) = move_str.split_at(2);
        return Ok(BasicMove {
            target_square: parse_square(target_str)?,
            piece_square: parse_square(piece_str)?,
        });
    }
}

pub fn parse_square(move_str: &str) -> Result<Square, ChessError> {
    if move_str.len() != 2 {
        return Err(ChessError::InvalidMoveString);
    } else {
        return Ok(Square {
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

pub fn validate_move(chess_move: BasicMove) -> Result<(), ChessError> {
    validate_square(chess_move.piece_square.row, chess_move.piece_square.col)?;
    validate_square(chess_move.target_square.row, chess_move.target_square.col)?;
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
        _ => Err(ChessError::InvalidMoveString),
    }
}

pub fn piece_basic_move_bit_board(
    piece: Piece,
    self_mask: BitBoard,
    attack_mask: BitBoard,
) -> BitBoard {
    let forward = match piece.piece_color {
        Color::White => 1,
        Color::Black => -1,
    };
    match piece.piece_type {
        PieceType::Pawn => {
            single_move_bit_board(forward, 0, piece.board_position, self_mask)
                | pawn_capture_bit_board(piece.board_position, attack_mask, forward)
                | pawn_double_step_board(piece.board_position, self_mask, attack_mask, forward)
        }
        PieceType::Rook => cross_move_bit_board(piece.board_position, self_mask, attack_mask),
        PieceType::Bishop => x_move_bit_board(piece.board_position, self_mask, attack_mask),
        PieceType::Queen => {
            cross_move_bit_board(piece.board_position, self_mask, attack_mask)
                | x_move_bit_board(piece.board_position, self_mask, attack_mask)
        }
        PieceType::King => king_move_bit_board(piece.board_position, self_mask),
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

fn king_move_bit_board(piece_placement: BitBoard, self_mask: BitBoard) -> BitBoard {
    (displace(piece_placement, -1, -1)
        | displace(piece_placement, -1, 0)
        | displace(piece_placement, -1, 1)
        | displace(piece_placement, 0, -1)
        | displace(piece_placement, 0, 1)
        | displace(piece_placement, 1, -1)
        | displace(piece_placement, 1, 0)
        | displace(piece_placement, 1, 1))
        & !self_mask
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
    self_mask: BitBoard,
) -> BitBoard {
    (displace(piece_placement, row_move, col_move)) & (!self_mask) | piece_placement
}

fn pawn_capture_bit_board(
    piece_placement: BitBoard,
    attack_mask: BitBoard,
    forwards: i32,
) -> BitBoard {
    displace(piece_placement, forwards, 1) & attack_mask
        | displace(piece_placement, forwards, -1) & attack_mask
}

fn pawn_double_step_board(
    piece_placement: BitBoard,
    self_mask: BitBoard,
    attack_mask: BitBoard,
    forward: i32,
) -> BitBoard {
    let mut now: u64 = piece_placement;
    now = single_move_bit_board(forward, 0, now, self_mask) & !(attack_mask);
    now = single_move_bit_board(forward, 0, now, self_mask);
    return now & (!piece_placement);
}

pub fn castling_board(full_mask: BitBoard, row: u32) -> BitBoard {
    kingside_castling_move(full_mask, row) | queenside_castling_move(full_mask, row)
}

pub fn kingside_castling_move(full_mask: BitBoard, row: u32) -> BitBoard {
    if full_mask & !(point(row, 5) | point(row, 4)) == full_mask {
        point(row, 6)
    } else {
        0
    }
}

pub fn queenside_castling_move(full_mask: BitBoard, row: u32) -> BitBoard {
    if full_mask & !(point(row, 1) | point(row, 2) | point(row, 3)) == full_mask {
        point(row, 2)
    } else {
        0
    }
}
