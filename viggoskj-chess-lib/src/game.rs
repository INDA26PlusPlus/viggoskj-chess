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

    let self_mask = match game.turn {
        Color::Black => game.board.black.mask(),
        Color::White => game.board.white.mask(),
    };

    let attack_mask = match game.turn {
        Color::Black => game.board.white.mask(),
        Color::White => game.board.black.mask(),
    };

    Ok((
        piece,
        piece_basic_move_bit_board(piece, self_mask, attack_mask),
    ))
}

pub fn move_piece(game: &Game, chess_move: piece::Move) -> Result<Game, ChessError> {
    let (piece, move_set) = piece_moves(game, chess_move.piece_square)?;
    let target_mask = point(chess_move.target_square.row, chess_move.target_square.col);

    if piece.piece_color != game.turn {
        return Err(ChessError::InvalidMove {
            reason: crate::chess_error::InvalidMoveReason::WrongColor,
        });
    }

    if (move_set & target_mask) == 0 {
        return Err(ChessError::InvalidMove {
            reason: crate::chess_error::InvalidMoveReason::NotAMoveOption,
        });
    }

    let (playing_board, waiting_board) = match game.turn {
        Color::Black => (&game.board.black, &game.board.white),
        Color::White => (&game.board.white, &game.board.black),
    };

    let new_wating_board = ColorBoard {
        pawns: waiting_board.pawns & !target_mask,
        knights: waiting_board.knights & !target_mask,
        bishops: waiting_board.bishops & !target_mask,
        rooks: waiting_board.rooks & !target_mask,
        queens: waiting_board.queens & !target_mask,
        kings: waiting_board.kings & !target_mask,
    };

    let new_playing_board = ColorBoard {
        bishops: playing_board.bishops & (!piece.board_position)
            | if_piece_type(piece.piece_type, PieceType::Bishop, target_mask),
        kings: playing_board.kings & (!piece.board_position)
            | if_piece_type(piece.piece_type, PieceType::King, target_mask),
        knights: playing_board.knights & (!piece.board_position)
            | if_piece_type(piece.piece_type, PieceType::Knight, target_mask),
        pawns: playing_board.pawns & (!piece.board_position)
            | if_piece_type(piece.piece_type, PieceType::Pawn, target_mask),
        queens: playing_board.queens & (!piece.board_position)
            | if_piece_type(piece.piece_type, PieceType::Queen, target_mask),
        rooks: playing_board.rooks & (!piece.board_position)
            | if_piece_type(piece.piece_type, PieceType::Rook, target_mask),
    };

    let new_board = Board{
        white: match game.turn {
            Color::Black => new_wating_board,
            Color::White => new_playing_board,
        },
        black: match game.turn {
            Color::White => new_wating_board,
            Color::Black => new_playing_board,
        },
    };

    Ok(Game {
        turn: game.turn.other(),
        board: new_board
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

#[derive(Copy, Clone, PartialEq)]
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
