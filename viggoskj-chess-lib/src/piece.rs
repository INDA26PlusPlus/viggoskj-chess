use crate::bitboard::{BitBoard, displace, point};
use crate::board::{Square, validate_square};
use crate::chess_error::ChessError;
use crate::game::Color;
use crate::moves::BasicMove;
use crate::{bitboard, game};

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Queen,
    King,
    Knight,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_color: game::Color,
    pub piece_type: PieceType,
    pub board_position: BitBoard,
}


impl PieceType {
    pub fn to_char(&self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Rook => 'r',
            PieceType::Bishop => 'b',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
            PieceType::Knight => 'n',
        }
    }
}

pub fn validate_move(chess_move: BasicMove) -> Result<(), ChessError> {
    validate_square(chess_move.piece_square.row, chess_move.piece_square.col)?;
    validate_square(chess_move.target_square.row, chess_move.target_square.col)?;
    Ok(())
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
    if full_mask & !(point(row, 6) | point(row, 5)) == full_mask {
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

pub fn if_piece_type(
    piece_type: PieceType,
    required_piece_type: PieceType,
    board: BitBoard,
) -> BitBoard {
    if piece_type == required_piece_type {
        board
    } else {
        0
    }
}