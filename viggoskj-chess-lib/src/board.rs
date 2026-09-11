use crate::bitboard::{BitBoard, point};
use crate::chess_error::ChessError;
use crate::piece::{Piece, PieceType};
use crate::{bitboard, piece};
use crate::instantiation;

#[derive(Copy, Clone)]
pub struct ColorBoard {
    pub pawns: BitBoard,
    pub knights: BitBoard,
    pub bishops: BitBoard,
    pub rooks: BitBoard,
    pub queens: BitBoard,
    pub kings: BitBoard,
}

pub struct Board {
    pub white: ColorBoard,
    pub black: ColorBoard,
}

#[derive(Copy, Clone)]
pub struct Square {
    pub row: u32,
    pub col: u32,
}

impl ColorBoard {
    pub fn mask(&self) -> BitBoard {
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
                (0..8).map(move |col| match self.white.at(row, col) {
                    Some(w) => w.to_char(),
                    None => match self.black.at(row, col) {
                        Some(b) => b.to_char().to_ascii_uppercase(),
                        None => '-',
                    },
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

    pub fn mask(&self) -> BitBoard {
        self.white.mask() | self.black.mask()
    }
}

pub fn square_bitboard (square: Square) -> BitBoard
{
    point(square.row, square.col)
}


pub fn create_start_board() -> Board {
    let board: Board = Board {
        white: instantiation::white_default_board(),
        black: instantiation::black_default_board(),
    };

    return board;
}

pub fn validate_square(row: u32, col: u32) -> Result<(), ChessError> {
    if row >= 8 || col >= 8 {
        Err(ChessError::InvalidSquare)
    } else {
        Ok(())
    }
}
