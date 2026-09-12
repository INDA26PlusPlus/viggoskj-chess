use crate::advanced_moves::can_try_kingside_castle;

pub type Bitboard = u64;

pub fn row(row: u32) -> Bitboard {
    (0..8).map(|x| row * 8 + x).fold(0, |y, x| 2u64.pow(x) + y)
}

pub fn col(col: u32) -> Bitboard {
    (0..8).map(|x| col + 8 * x).fold(0, |y, x| 2u64.pow(x) + y)
}

pub fn point(row: u32, col: u32) -> Bitboard {
    2u64.pow(row * 8 + col)
}

pub fn horizontal_flip(board: Bitboard) -> Bitboard {
    let mut result = 0;

    for row in 0..8 {
        for col in 0..8 {
            if board & point(row, col) != 0 {
                result |= point(row, 7 - col);
            }
        }
    }

    result
}

pub fn displace(board: Bitboard, rows: i32, cols: i32) -> Bitboard {
    let mut result = board;

    result = shift(result, rows * 8);

    if cols > 0 {
        for _ in 0..cols {
            result = (result & 0x7F7F7F7F7F7F7F7F) << 1;
        }
    } else {
        for _ in 0..-cols {
            result = (result & 0xFEFEFEFEFEFEFEFE) >> 1;
        }
    }

    result
}

fn shift(x: u64, amount: i32) -> u64 {
    if amount >= 0 {
        x.checked_shl(amount as u32).unwrap_or(0)
    } else {
        x.checked_shr(amount.unsigned_abs()).unwrap_or(0)
    }
}

pub fn at(board: Bitboard, row: u32, col: u32) -> bool {
    (board & point(row, col)) > 0
}

pub fn bitboard_string(board: Bitboard) -> String {
    (0..8)
        .rev()
        .map(|row| (0..8).map(move |col| at(board, row, col)))
        .fold(String::new(), |all, row| {
            all + &row.fold(String::new(), |mut all, c| {
                all.push(if c { '1' } else { '0' });
                all
            }) + "\n"
        })
}

pub fn bitboard_if(board: Bitboard, condition: bool) -> Bitboard {
    if condition { board } else { 0 }
}