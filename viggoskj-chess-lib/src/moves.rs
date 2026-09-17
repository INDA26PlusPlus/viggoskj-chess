use crate::bitboard::{Bitboard, bitboard_if, displace};
use crate::board::{Square, to_square};
use crate::board::{self};
use crate::chess_error::ChessError;
use crate::game::Color;
use crate::instantiation;
use crate::piece::{Piece, PieceType};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BasicMove {
    pub piece_square: Square,
    pub target_square: Square,
}

#[derive(Copy, Clone)]
pub struct PossibleMove {
    pub piece: Piece,
    pub taget_square: Square,
}

#[derive(Copy, Clone)]
pub struct PossibleMovesBitboard {
    pub piece: Piece,
    pub moves: Bitboard,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AdvancedMove {
    KingSideCastle,
    QueenSideCastle,
    EnPessant {
        basic_move: BasicMove,
    },
    Promotion {
        piece_type: PieceType,
        basic_move: BasicMove,
    },
}

#[derive(Copy, Clone, Debug)]
pub enum Move {
    Advanced { chess_move: AdvancedMove },
    Basic { chess_move: BasicMove },
}

/// validates a move is within the board
pub fn validate_move(chess_move: BasicMove) -> Result<(), ChessError> {
    board::validate_square(chess_move.piece_square.row, chess_move.piece_square.col)?;
    board::validate_square(chess_move.target_square.row, chess_move.target_square.col)?;
    Ok(())
}

/// converts a possible move in to a playable move
pub fn possible_move_to_move(possible: PossibleMove) -> Move {
    Move::Basic {
        chess_move: BasicMove {
            piece_square: to_square(possible.piece.board_position).unwrap(),
            target_square: possible.taget_square,
        },
    }
}

pub(crate) fn legal_basic_moves_bitboard(
    piece: Piece,
    playing_mask: Bitboard,
    waiting_mask: Bitboard,
) -> Bitboard {
    let forward = match piece.piece_color {
        Color::White => 1,
        Color::Black => -1,
    };

    match piece.piece_type {
        PieceType::Pawn => pawn_move_bitboard(piece, forward, playing_mask, waiting_mask),
        PieceType::Rook => cross_move_bitboard(piece.board_position, playing_mask, waiting_mask),
        PieceType::Bishop => x_move_bitboard(piece.board_position, playing_mask, waiting_mask),
        PieceType::Queen => {
            cross_move_bitboard(piece.board_position, playing_mask, waiting_mask)
                | x_move_bitboard(piece.board_position, playing_mask, waiting_mask)
        }
        PieceType::King => king_move_bitboard(piece.board_position, playing_mask),
        PieceType::Knight => knight_move_bitboard(piece.board_position, playing_mask),
    }
}

fn pawn_move_bitboard(
    piece: Piece,
    forward: i32,
    playing_mask: Bitboard,
    wating_mask: Bitboard,
) -> Bitboard {
    (single_move_bitboard(forward, 0, piece.board_position, playing_mask) & (!wating_mask))
        | pawn_capture_bitboard(piece.board_position, wating_mask, forward)
        | bitboard_if(
            pawn_double_step_board(piece.board_position, playing_mask, wating_mask, forward),
            piece.board_position
                & (instantiation::black_default_pawn_board()
                    | instantiation::white_default_pawn_board())
                != 0,
        )
}

fn cross_move_bitboard(
    piece_placement: Bitboard,
    playing_mask: Bitboard,
    wating_mask: Bitboard,
) -> Bitboard {
    move_bitboard(1, 0, piece_placement, playing_mask, wating_mask)
        | move_bitboard(0, 1, piece_placement, playing_mask, wating_mask)
        | move_bitboard(-1, 0, piece_placement, playing_mask, wating_mask)
        | move_bitboard(0, -1, piece_placement, playing_mask, wating_mask)
}

fn x_move_bitboard(
    piece_placement: Bitboard,
    playing_mask: Bitboard,
    wating_mask: Bitboard,
) -> Bitboard {
    move_bitboard(1, 1, piece_placement, playing_mask, wating_mask)
        | move_bitboard(1, -1, piece_placement, playing_mask, wating_mask)
        | move_bitboard(-1, -1, piece_placement, playing_mask, wating_mask)
        | move_bitboard(-1, 1, piece_placement, playing_mask, wating_mask)
}

fn king_move_bitboard(piece_placement: Bitboard, playing_mask: Bitboard) -> Bitboard {
    (displace(piece_placement, -1, -1)
        | displace(piece_placement, -1, 0)
        | displace(piece_placement, -1, 1)
        | displace(piece_placement, 0, -1)
        | displace(piece_placement, 0, 1)
        | displace(piece_placement, 1, -1)
        | displace(piece_placement, 1, 0)
        | displace(piece_placement, 1, 1))
        & (!playing_mask)
}

fn knight_move_bitboard(piece_placement: Bitboard, playing_mask: Bitboard) -> Bitboard {
    (displace(piece_placement, -2, -1)
        | displace(piece_placement, -2, 1)
        | displace(piece_placement, -1, -2)
        | displace(piece_placement, -1, 2)
        | displace(piece_placement, 1, -2)
        | displace(piece_placement, 1, 2)
        | displace(piece_placement, 2, -1)
        | displace(piece_placement, 2, 1))
        & (!playing_mask)
}

fn move_bitboard(
    row_move: i32,
    col_move: i32,
    piece_placement: Bitboard,
    playing_mask: Bitboard,
    wating_mask: Bitboard,
) -> Bitboard {
    let mut prev = u64::MAX;
    let mut now: u64 = piece_placement;

    while prev != now {
        prev = now;
        now = single_move_bitboard(row_move, col_move, now, playing_mask) & !(wating_mask);
    }

    now = single_move_bitboard(row_move, col_move, now, playing_mask);
    return now & (!piece_placement);
}

fn single_move_bitboard(
    row_move: i32,
    col_move: i32,
    piece_placement: Bitboard,
    self_mask: Bitboard,
) -> Bitboard {
    (displace(piece_placement, row_move, col_move)) & (!self_mask) | piece_placement
}

pub(crate) fn pawn_capture_bitboard(
    piece_placement: Bitboard,
    wating_mask: Bitboard,
    forwards: i32,
) -> Bitboard {
    displace(piece_placement, forwards, 1) & wating_mask
        | displace(piece_placement, forwards, -1) & wating_mask
}

fn pawn_double_step_board(
    piece_placement: Bitboard,
    self_mask: Bitboard,
    wating_mask: Bitboard,
    forward: i32,
) -> Bitboard {
    let mut now: u64 = piece_placement;
    now = single_move_bitboard(forward, 0, now, self_mask) & !(wating_mask);
    now = single_move_bitboard(forward, 0, now, self_mask);
    return now & (!piece_placement);
}
