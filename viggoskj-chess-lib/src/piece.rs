use crate::bit_board::{BitBoard, displace};
use crate::game::Color;
use crate::{bit_board, game};

#[derive(Copy, Clone)]
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
