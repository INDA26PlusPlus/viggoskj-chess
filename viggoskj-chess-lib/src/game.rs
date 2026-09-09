use std::iter::Enumerate;

use crate::{
    bit_board::{BitBoard, point},
    board::{Board, ColorBoard, validate_square},
    chess_error::ChessError,
    game::Color::White,
    piece::{self, Piece, PieceType, piece_basic_move_bit_board},
};

pub struct Game {
    pub board: Board,
    pub turn: Color,
}

pub fn move_piece(
    game: &Game,
    chess_move: piece::Move
) -> Result<Game, ChessError> {
    piece::validate_move(chess_move)?;

    let piece = match game.board.get_pice(chess_move.origin_row, chess_move.origin_col) {
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

    if (move_set & point(chess_move.target_row, chess_move.target_col)) == 0 {
        return Err(ChessError::InvalidMove);
    }

    let new_board = Board {
        black: game.board.black,
        white: ColorBoard {
            bishops: game.board.white.bishops & (!piece.board_position)
                | if_piece_type(piece.piece_type, PieceType::Bishop, point(chess_move.target_row, chess_move.target_col)),
            kings: game.board.white.kings & (!piece.board_position)
                | if_piece_type(piece.piece_type, PieceType::King, point(chess_move.target_row, chess_move.target_col)),
            knights: game.board.white.knights & (!piece.board_position)
                | if_piece_type(piece.piece_type, PieceType::Knight, point(chess_move.target_row, chess_move.target_col)),
            pawns: game.board.white.pawns & (!piece.board_position)
                | if_piece_type(piece.piece_type, PieceType::Pawn, point(chess_move.target_row, chess_move.target_col)),
            queens: game.board.white.queens & (!piece.board_position)
                | if_piece_type(piece.piece_type, PieceType::Queen, point(chess_move.target_row, chess_move.target_col)),
            rooks: game.board.white.rooks & (!piece.board_position)
                | if_piece_type(piece.piece_type, PieceType::Rook, point(chess_move.target_row, chess_move.target_col)),
        },
    };

    Ok(Game {
        turn: game.turn.other(),
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
