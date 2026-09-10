pub type BitBoard = u64;

pub fn row(row: u32) -> BitBoard {
    (0..8).map(|x| row * 8 + x).fold(0, |y, x| 2u64.pow(x) + y)
}

pub fn col(col: u32) -> BitBoard {
    (0..8).map(|x| col + 8 * x).fold(0, |y, x| 2u64.pow(x) + y)
}

pub fn point(row: u32, col: u32) -> BitBoard {
    2u64.pow(row * 8 + col)
}

pub fn displace(board: BitBoard, rows: i32, cols: i32) -> BitBoard {
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

pub fn at(board: BitBoard, row: u32, col: u32) -> bool {
    (board & point(row, col)) > 0
}

pub fn empty() -> BitBoard {
    0
}

pub fn full() -> BitBoard {
    BitBoard::MAX
}

pub fn bitboard_string(board: BitBoard) -> String {
    (0..8).rev()
        .map(|row| (0..8).map(move |col| at(board, row, col)))
        .fold(String::new(), |all, row| {
            all + &row.fold(String::new(), |mut all, c| {
                all.push(if c { '1' } else { '0' });
                all
            }) + "\n"
        })
}
