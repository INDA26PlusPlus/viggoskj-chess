use crate::bool_board::BoolBoard;
use crate::{bool_board, piece};

pub struct ColorBoard {
    pawns: BoolBoard,
    knights: BoolBoard,
    bishops: BoolBoard,
    rooks: BoolBoard,
    queens: BoolBoard,
    kings: BoolBoard,
}

impl ColorBoard {
    fn At(u32 row, u32 col) -> Option<Piece>
    {

    }
}

pub struct Board {
    white: ColorBoard,
    black: ColorBoard,
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut str = "".to_string();

        for row in (0..8).rev() {
            for col in 0..8 {}
        }

        return str;
    }
}

fn black_default_board() -> ColorBoard {
    ColorBoard {
        pawns: bool_board::vertical_mirror(white_default_pawn_board()),
        rooks: bool_board::vertical_mirror(white_default_rook_board()),
        knights: bool_board::vertical_mirror(white_default_knight_board()),
        bishops: bool_board::vertical_mirror(white_default_bishop_board()),
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

fn white_default_pawn_board() -> BoolBoard {
    bool_board::col(1)
}

fn white_default_rook_board() -> BoolBoard {
    return bool_board::union(bool_board::point(0, 0), bool_board::point(0, 7));
}

fn white_default_knight_board() -> BoolBoard {
    return bool_board::union(bool_board::point(0, 1), bool_board::point(0, 6));
}

fn white_default_bishop_board() -> BoolBoard {
    return bool_board::union(bool_board::point(0, 2), bool_board::point(0, 5));
}

fn white_default_queen_board() -> BoolBoard {
    return bool_board::point(0, 3);
}

fn white_default_king_board() -> BoolBoard {
    return bool_board::point(0, 4);
}

fn black_default_queen_board() -> BoolBoard {
    return bool_board::point(7, 4);
}

fn black_default_king_board() -> BoolBoard {
    return bool_board::point(7, 3);
}

pub fn create_start_board() -> Board {
    let board = Board {
        white: white_default_board(),
        black: black_default_board(),
    };

    return board;
}
