use crate::bitboard;
use crate::board;


pub fn black_default_board() -> board::ColorBoard {
    board::ColorBoard {
        pawns: black_default_pawn_board(),
        rooks: black_default_rook_board(),
        knights: black_default_knight_board(),
        bishops: black_default_bishop_board(),
        queens: black_default_queen_board(),
        kings: black_default_king_board(),
    }
}

pub fn white_default_board() -> board::ColorBoard {
    board::ColorBoard {
        pawns: white_default_pawn_board(),
        rooks: white_default_rook_board(),
        knights: white_default_knight_board(),
        bishops: white_default_bishop_board(),
        queens: white_default_queen_board(),
        kings: white_default_king_board(),
    }
}

pub fn white_default_pawn_board() -> bitboard::BitBoard {
    bitboard::row(1)
}

pub fn white_default_rook_board() -> bitboard::BitBoard {
    white_default_left_rook_board() | white_default_right_rook_board()
}

pub fn white_default_left_rook_board() -> bitboard::BitBoard {
    bitboard::point(0, 0)
}

pub fn white_default_right_rook_board() -> bitboard::BitBoard {
    bitboard::point(0, 7)
}

pub fn white_default_knight_board() -> bitboard::BitBoard {
    bitboard::point(0, 1) | bitboard::point(0, 6)
}

pub fn white_default_bishop_board() -> bitboard::BitBoard {
    bitboard::point(0, 2) | bitboard::point(0, 5)
}

pub fn black_default_queen_board() -> bitboard::BitBoard {
    bitboard::point(7, 3)
}

pub(crate) fn black_default_king_board() -> bitboard::BitBoard {
    bitboard::point(7, 4)
}

pub fn black_default_pawn_board() -> bitboard::BitBoard {
    bitboard::row(6)
}

pub fn black_default_rook_board() -> bitboard::BitBoard {
    black_default_left_rook_board() | black_default_right_rook_board()
}

pub fn black_default_left_rook_board() -> bitboard::BitBoard {
    bitboard::point(7, 0)
}

pub fn black_default_right_rook_board() -> bitboard::BitBoard {
    bitboard::point(7, 7)
}

pub fn black_default_knight_board() -> bitboard::BitBoard {
    bitboard::point(7, 1) | bitboard::point(7, 6)
}

pub fn black_default_bishop_board() -> bitboard::BitBoard {
    bitboard::point(7, 2) | bitboard::point(7, 5)
}

pub fn white_default_queen_board() -> bitboard::BitBoard {
    bitboard::point(0, 3)
}

pub fn white_default_king_board() -> bitboard::BitBoard {
    bitboard::point(0, 4)
}