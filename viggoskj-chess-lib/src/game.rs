use std::iter::Enumerate;

use crate::{
    bit_board::point,
    board::{Board, ColorBoard, validate_square},
    chess_error::ChessError,
    game::Color::White,
    piece::piece_basic_move_bit_board,
};

pub struct Game {
    pub board: Board,
    pub turn: Color,
}

pub fn move_piece(
    game: &Game,
    row1: u32,
    col1: u32,
    row2: u32,
    col2: u32,
) -> Result<Game, ChessError> {
    validate_square(row1, col1)?;
    validate_square(row2, col2)?;

    let piece = match game.board.get_pice(row1, col1) {
        Some(t) => t,
        _ => return Err(ChessError::InvalidMove),
    };

    let solid_mask = match game.turn {
        Color::Black => game.board.black.mask(),
        Color::White => game.board.white.mask(),
    };

    let attack_mask = match game.turn {
        Color::Black => game.board.white.mask(),
        Color::White => game.board.black.mask(),
    };

    let move_set = piece_basic_move_bit_board(piece, solid_mask, attack_mask);

    if (move_set & point(row2, col2)) == 0 {
        return Err(ChessError::InvalidMove);
    }

    let new_board = Board {
        black: game.board.black,
        white: ColorBoard {
            bishops: game.board.white.bishops
                & (!(match piece.piece_type {
                    crate::piece::PieceType::Bishop => piece.board_position,
                    _ => 0,
                }))
                | (match piece.piece_type {
                    crate::piece::PieceType::Bishop => point(row2, col2),
                    _ => 0,
                }),
            kings: game.board.white.kings
                & (!(match piece.piece_type {
                    crate::piece::PieceType::King => piece.board_position,
                    _ => 0,
                }))
                | (match piece.piece_type {
                    crate::piece::PieceType::King => point(row2, col2),
                    _ => 0,
                }),
            knights: game.board.white.knights
                & (!(match piece.piece_type {
                    crate::piece::PieceType::Knight => piece.board_position,
                    _ => 0,
                }))
                | (match piece.piece_type {
                    crate::piece::PieceType::Knight => point(row2, col2),
                    _ => 0,
                }),
            pawns: game.board.white.pawns
                & (!(match piece.piece_type {
                    crate::piece::PieceType::Pawn => piece.board_position,
                    _ => 0,
                }))
                | (match piece.piece_type {
                    crate::piece::PieceType::Pawn => point(row2, col2),
                    _ => 0,
                }),
            queens: game.board.white.queens
                & (!(match piece.piece_type {
                    crate::piece::PieceType::Queen => piece.board_position,
                    _ => 0,
                }))
                | (match piece.piece_type {
                    crate::piece::PieceType::Queen => point(row2, col2),
                    _ => 0,
                }),
            rooks: game.board.white.rooks
                & (!(match piece.piece_type {
                    crate::piece::PieceType::Rook => piece.board_position,
                    _ => 0,
                }))
                | (match piece.piece_type {
                    crate::piece::PieceType::Rook => point(row2, col2),
                    _ => 0,
                }),
        },
    };

    Ok(Game {
        turn: game.turn.other(),
        board: new_board,
    })
}

pub struct MoveResult {}

#[derive(Copy, Clone)]
pub enum Color {
    Black,
    White,
}

impl Color {
    fn other(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
