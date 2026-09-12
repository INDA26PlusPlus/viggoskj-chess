use crate::bitboard::{Bitboard, bitboard_if, displace, point};
use crate::board::Square;
use crate::board;
use crate::chess_error::ChessError;
use crate::game::Color;
use crate::instantiation;
use crate::piece::{Piece, PieceType};

#[derive(Copy, Clone)]
pub struct BasicMove {
    pub piece_square: Square,
    pub target_square: Square,
}

#[derive(Copy, Clone)]
pub enum AdvancedMove {
    KingSideCastle,
    QueenSideCastle,
    Promotion {
        piece_type: PieceType,
        basic_move: BasicMove,
    },
}

#[derive(Copy, Clone)]
pub enum Move {
    Advanced { chess_move: AdvancedMove },
    Basic { chess_move: BasicMove },
}


pub fn validate_move(chess_move: BasicMove) -> Result<(), ChessError> {
    board::validate_square(chess_move.piece_square.row, chess_move.piece_square.col)?;
    board::validate_square(chess_move.target_square.row, chess_move.target_square.col)?;
    Ok(())
}

pub fn legal_basic_moves_bitboard(
    piece: Piece,
    self_mask: Bitboard,
    attack_mask: Bitboard,
) -> Bitboard {
    let forward = match piece.piece_color {
        Color::White => 1,
        Color::Black => -1,
    };

    match piece.piece_type {
        PieceType::Pawn => {
            single_move_bitboard(forward, 0, piece.board_position, self_mask)
                | pawn_capture_bitboard(piece.board_position, attack_mask, forward)
                | bitboard_if(
                    pawn_double_step_board(piece.board_position, self_mask, attack_mask, forward),
                    piece.board_position
                        & (instantiation::black_default_pawn_board() | instantiation::white_default_pawn_board())
                        != 0,
                )
        }
        PieceType::Rook => cross_move_bitboard(piece.board_position, self_mask, attack_mask),
        PieceType::Bishop => x_move_bitboard(piece.board_position, self_mask, attack_mask),
        PieceType::Queen => {
            cross_move_bitboard(piece.board_position, self_mask, attack_mask)
                | x_move_bitboard(piece.board_position, self_mask, attack_mask)
        }
        PieceType::King => king_move_bitboard(piece.board_position, self_mask),
        PieceType::Knight => knight_move_bitboard(piece.board_position),
    }
}

fn cross_move_bitboard(
    piece_placement: Bitboard,
    solid_mask: Bitboard,
    attack_mask: Bitboard,
) -> Bitboard {
    move_bitboard(1, 0, piece_placement, solid_mask, attack_mask)
        | move_bitboard(0, 1, piece_placement, solid_mask, attack_mask)
        | move_bitboard(-1, 0, piece_placement, solid_mask, attack_mask)
        | move_bitboard(0, -1, piece_placement, solid_mask, attack_mask)
}

fn x_move_bitboard(
    piece_placement: Bitboard,
    solid_mask: Bitboard,
    attack_mask: Bitboard,
) -> Bitboard {
    move_bitboard(1, 1, piece_placement, solid_mask, attack_mask)
        | move_bitboard(1, -1, piece_placement, solid_mask, attack_mask)
        | move_bitboard(-1, -1, piece_placement, solid_mask, attack_mask)
        | move_bitboard(-1, 1, piece_placement, solid_mask, attack_mask)
}

fn king_move_bitboard(piece_placement: Bitboard, self_mask: Bitboard) -> Bitboard {
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

fn knight_move_bitboard(piece_placement: Bitboard) -> Bitboard {
    displace(piece_placement, -2, -1)
        | displace(piece_placement, -2, 1)
        | displace(piece_placement, -1, -2)
        | displace(piece_placement, -1, 2)
        | displace(piece_placement, 1, -2)
        | displace(piece_placement, 1, 2)
        | displace(piece_placement, 2, -1)
        | displace(piece_placement, 2, 1)
}

fn move_bitboard(
    row_move: i32,
    col_move: i32,
    piece_placement: Bitboard,
    solid_mask: Bitboard,
    attack_mask: Bitboard,
) -> Bitboard {
    let mut prev = u64::MAX;
    let mut now: u64 = piece_placement;

    while prev != now {
        prev = now;
        now = single_move_bitboard(row_move, col_move, now, solid_mask) & !(attack_mask);
    }

    now = single_move_bitboard(row_move, col_move, now, solid_mask);
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

fn pawn_capture_bitboard(
    piece_placement: Bitboard,
    attack_mask: Bitboard,
    forwards: i32,
) -> Bitboard {
    displace(piece_placement, forwards, 1) & attack_mask
        | displace(piece_placement, forwards, -1) & attack_mask
}

fn pawn_double_step_board(
    piece_placement: Bitboard,
    self_mask: Bitboard,
    attack_mask: Bitboard,
    forward: i32,
) -> Bitboard {
    let mut now: u64 = piece_placement;
    now = single_move_bitboard(forward, 0, now, self_mask) & !(attack_mask);
    now = single_move_bitboard(forward, 0, now, self_mask);
    return now & (!piece_placement);
}
