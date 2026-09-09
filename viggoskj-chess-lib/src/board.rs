use crate::bit_board::BitBoard;
use crate::chess_error::ChessError;
use crate::piece::{Piece, PieceType};
use crate::{bit_board, piece};

#[derive(Copy, Clone)]
pub struct ColorBoard {
    pub pawns: BitBoard,
    pub knights: BitBoard,
    pub bishops: BitBoard,
    pub rooks: BitBoard,
    pub queens: BitBoard,
    pub kings: BitBoard,
}

impl ColorBoard {
    pub fn mask(&self) -> BitBoard {
        self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.kings
    }
}

impl ColorBoard {
    fn at(&self, row: u32, col: u32) -> Option<piece::PieceType> {
        if bit_board::at(self.pawns, row, col) {
            return Some(piece::PieceType::Pawn);
        }
        if bit_board::at(self.knights, row, col) {
            return Some(piece::PieceType::Knight);
        }
        if bit_board::at(self.bishops, row, col) {
            return Some(piece::PieceType::Bishop);
        }
        if bit_board::at(self.rooks, row, col) {
            return Some(piece::PieceType::Rook);
        }
        if bit_board::at(self.queens, row, col) {
            return Some(piece::PieceType::Queen);
        }
        if bit_board::at(self.kings, row, col) {
            return Some(piece::PieceType::King);
        }

        None
    }
}

pub struct Board {
    pub white: ColorBoard,
    pub black: ColorBoard,
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
                board_position: bit_board::point(row, col),
            });
        }

        if let Some(piece_type) = self.black.at(row, col) {
            return Some(Piece {
                piece_type: piece_type,
                piece_color: crate::game::Color::Black,
                board_position: bit_board::point(row, col),
            });
        }

        return None;
    }

    pub fn mask(&self) -> BitBoard {
        self.white.mask() | self.black.mask()
    }
}

fn black_default_board() -> ColorBoard {
    ColorBoard {
        pawns: black_default_pawn_board(),
        rooks: black_default_rook_board(),
        knights: black_default_knight_board(),
        bishops: black_default_bishop_board(),
        queens: black_default_queen_board(),
        kings: black_default_king_board(),
    }
}

fn white_default_board() -> ColorBoard {
    ColorBoard {
        pawns: white_default_pawn_board(),
        rooks: white_default_rook_board(),
        knights: white_default_knight_board(),
        bishops: white_default_bishop_board(),
        queens: white_default_queen_board(),
        kings: white_default_king_board(),
    }
}

fn white_default_pawn_board() -> BitBoard {
    bit_board::row(1)
}

fn white_default_rook_board() -> BitBoard {
    return bit_board::union(bit_board::point(0, 0), bit_board::point(0, 7));
}

fn white_default_knight_board() -> BitBoard {
    return bit_board::union(bit_board::point(0, 1), bit_board::point(0, 6));
}

fn white_default_bishop_board() -> BitBoard {
    return bit_board::union(bit_board::point(0, 2), bit_board::point(0, 5));
}

fn black_default_queen_board() -> BitBoard {
    return bit_board::point(7, 3);
}

fn black_default_king_board() -> BitBoard {
    return bit_board::point(7, 4);
}

fn black_default_pawn_board() -> BitBoard {
    bit_board::row(6)
}

fn black_default_rook_board() -> BitBoard {
    return bit_board::union(bit_board::point(7, 0), bit_board::point(7, 7));
}

fn black_default_knight_board() -> BitBoard {
    return bit_board::union(bit_board::point(7, 1), bit_board::point(7, 6));
}

fn black_default_bishop_board() -> BitBoard {
    return bit_board::union(bit_board::point(7, 2), bit_board::point(7, 5));
}

fn white_default_queen_board() -> BitBoard {
    return bit_board::point(0, 3);
}

fn white_default_king_board() -> BitBoard {
    return bit_board::point(0, 4);
}

pub fn create_start_board() -> Board {
    let board = Board {
        white: white_default_board(),
        black: black_default_board(),
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
