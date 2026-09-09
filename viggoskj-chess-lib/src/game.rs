use std::iter::Enumerate;

use crate::{
    Piece::Square,
    bit_board::{self, BitBoard, point},
    board::{Board, ColorBoard, validate_square},
    chess_error::ChessError,
    game::Color::White,
    piece::{self, Piece, PieceType, piece_basic_move_bit_board},
};

pub struct Game {
    pub board: Board,
    pub turn: Color,
}

pub fn piece_moves(
    game: &Game,
    piece_square: piece::Square,
) -> Result<(Piece, BitBoard), ChessError> {
    let piece = match game.board.get_pice(piece_square.row, piece_square.col) {
        Some(t) => t,
        _ => {
            return Err(ChessError::InvalidMove {
                reason: crate::chess_error::InvalidMoveReason::NoTargetPiece,
            });
        }
    };
    let solid_mask = match game.turn {
        Color::Black => game.board.black.mask(),
        Color::White => game.board.white.mask(),
    };

    let attack_mask = match game.turn {
        Color::Black => game.board.white.mask(),
        Color::White => game.board.black.mask(),
    };

    Ok((
        piece,
        piece_basic_move_bit_board(piece, solid_mask, attack_mask),
    ))
}

pub fn move_piece(game: &Game, chess_move: piece::Move) -> Result<Game, ChessError> {
    let (piece, move_set) = piece_moves(game, chess_move.piece_square)?;
    let target_mask = point(chess_move.target_square.row, chess_move.target_square.col);

    if (move_set & target_mask) == 0 {
        return Err(ChessError::InvalidMove {
            reason: crate::chess_error::InvalidMoveReason::NotAMoveOption,
        });
    }

    let new_board = Board {
        black: ColorBoard {
            pawns: game.board.black.pawns & !target_mask,
            knights: game.board.black.knights & !target_mask,
            bishops: game.board.black.bishops & !target_mask,
            rooks: game.board.black.rooks & !target_mask,
            queens: game.board.black.queens & !target_mask,
            kings: game.board.black.kings & !target_mask,
        },
        white: ColorBoard {
            bishops: game.board.white.bishops & (!piece.board_position)
                | if_piece_type(piece.piece_type, PieceType::Bishop, target_mask),
            kings: game.board.white.kings & (!piece.board_position)
                | if_piece_type(piece.piece_type, PieceType::King, target_mask),
            knights: game.board.white.knights & (!piece.board_position)
                | if_piece_type(piece.piece_type, PieceType::Knight, target_mask),
            pawns: game.board.white.pawns & (!piece.board_position)
                | if_piece_type(piece.piece_type, PieceType::Pawn, target_mask),
            queens: game.board.white.queens & (!piece.board_position)
                | if_piece_type(piece.piece_type, PieceType::Queen, target_mask),
            rooks: game.board.white.rooks & (!piece.board_position)
                | if_piece_type(piece.piece_type, PieceType::Rook, target_mask),
        },
    };

    Ok(Game {
        turn: game.turn,
        board: new_board,
    })
}

fn if_piece_type(
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
