use crate::bitboard::{Bitboard, bitboard_iterator, point};
use crate::chess_error::ChessError;
use crate::game::{Color};
use crate::moves::BasicMove;
use crate::piece::{Piece, PieceType, if_piece_type};
use crate::{bitboard, piece};
use crate::{instantiation, moves};


/// the bitboards of all the diferent piece types of a color (the black or white sides pieces)
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ColorBoard {
    pub pawns: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub rooks: Bitboard,
    pub queens: Bitboard,
    pub kings: Bitboard,
}

/// the full chess board contianing two color boards
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Board {
    pub white: ColorBoard,
    pub black: ColorBoard,
}

/// singel square on a chess board
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Square {
    pub row: u32,
    pub col: u32,
}

impl ColorBoard {
    /// returns the whole bitboard mask of the board
    pub fn mask(&self) -> Bitboard {
        self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.kings
    }
}

impl ColorBoard {
    fn at(&self, row: u32, col: u32) -> Option<piece::PieceType> {
        if bitboard::at(self.pawns, row, col) {
            return Some(piece::PieceType::Pawn);
        }
        if bitboard::at(self.knights, row, col) {
            return Some(piece::PieceType::Knight);
        }
        if bitboard::at(self.bishops, row, col) {
            return Some(piece::PieceType::Bishop);
        }
        if bitboard::at(self.rooks, row, col) {
            return Some(piece::PieceType::Rook);
        }
        if bitboard::at(self.queens, row, col) {
            return Some(piece::PieceType::Queen);
        }
        if bitboard::at(self.kings, row, col) {
            return Some(piece::PieceType::King);
        }

        None
    }
}
impl ToString for Board {
    fn to_string(&self) -> String {
        (0..8)
            .rev()
            .map(|row| {
                (0..8).map(move |col| match self.get_pice(row, col) {
                    Some(b) => b.to_char(),
                    None => '-',
                })
            })
            .fold(String::new(), |all, row| {
                all + &row.fold(String::new(), |mut all, c| {
                    all.push(c);
                    all
                }) + "\n"
            })
    }
}

impl Board {
    /// gets a piece at a location on the board
    pub fn get_pice(&self, row: u32, col: u32) -> Option<Piece> {
        if let Some(piece_type) = self.white.at(row, col) {
            return Some(Piece {
                piece_type: piece_type,
                piece_color: crate::game::Color::White,
                board_position: bitboard::point(row, col),
            });
        }

        if let Some(piece_type) = self.black.at(row, col) {
            return Some(Piece {
                piece_type: piece_type,
                piece_color: crate::game::Color::Black,
                board_position: bitboard::point(row, col),
            });
        }

        return None;
    }

    pub fn assert_get_pice(&self, row: u32, col: u32) -> Result<Piece, ChessError> {
        match self.get_pice(row, col) {
            Some(t) => Ok(t),
            None => Err(ChessError::EmptySquare),
        }
    }

    /// gets the mask of the whole board
    pub fn mask(&self) -> Bitboard {
        self.white.mask() | self.black.mask()
    }
}

/// gets a bitboard representation of a square
pub fn square_bitboard(square: Square) -> Bitboard {
    point(square.row, square.col)
}

pub(crate) fn create_start_board() -> Board {
    let board: Board = Board {
        white: instantiation::white_default_board(),
        black: instantiation::black_default_board(),
    };

    return board;
}

/// turns a point on a bitboard to a square
pub fn to_square(bitboard: Bitboard) -> Result<Square, ChessError> {
    // TODO  Fing fix
    let truthy = bitboard_iterator(bitboard)
        .filter(|(_, truthy)| *truthy)
        .map(|(square, _)| square);

    for v in truthy {
        return Ok(v);
    }

    Err(ChessError::InvalidSquare)
}

/// validates a square is within bounds
pub fn validate_square(row: u32, col: u32) -> Result<(), ChessError> {
    if row >= 8 || col >= 8 {
        Err(ChessError::InvalidSquare)
    } else {
        Ok(())
    }
}

pub(crate) fn initialy_legal_basic_moves_bitboard(
    board: &Board,
    piece_square: Square,
    playing: Color,
) -> Result<(Piece, Bitboard), ChessError> {
    let piece = match board.get_pice(piece_square.row, piece_square.col) {
        Some(t) => t,
        _ => {
            return Err(ChessError::InvalidMove {
                reason: crate::chess_error::InvalidMoveReason::NoTargetPiece,
            });
        }
    };

    let (playing, waiting) = select_playing_board(board, playing);

    let moves = moves::legal_basic_moves_bitboard(piece, playing.mask(), waiting.mask());

    Ok((piece, moves & !(square_bitboard(piece_square))))
}

// todo, actually move the move enforcement into the game layer
pub(crate) fn do_basic_move(
    board: &Board,
    chess_move: BasicMove,
    playing: Color,
) -> Result<(Board, Piece), ChessError> {
    let piece = board.assert_get_pice(chess_move.piece_square.row, chess_move.piece_square.col)?;

    let target_mask = point(chess_move.target_square.row, chess_move.target_square.col);

    if piece.piece_color != playing {
        return Err(ChessError::InvalidMove {
            reason: crate::chess_error::InvalidMoveReason::WrongColor,
        });
    }

    let (playing_board, waiting_board) = select_playing_board(board, playing);

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

    Ok((
        new_board(new_playing_board, new_wating_board, playing),
        piece,
    ))
}

/// selects the board of the color
pub fn select_playing_board(board: &Board, color: Color) -> (ColorBoard, ColorBoard) {
    match color {
        Color::Black => (board.black, board.white),
        Color::White => (board.white, board.black),
    }
}

pub(crate) fn new_board(
    new_playing_board: ColorBoard,
    new_waiting_board: ColorBoard,
    playing: Color,
) -> Board {
    Board {
        white: match playing {
            Color::Black => new_waiting_board,
            Color::White => new_playing_board,
        },
        black: match playing {
            Color::White => new_waiting_board,
            Color::Black => new_playing_board,
        },
    }
}

pub(crate) fn back_row(color: Color) -> u32 {
    match color {
        Color::White => 0,
        Color::Black => 7,
    }
}
