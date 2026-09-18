use crate::board::Square;


/// a 64 bit integer where each bit represents true or false for one square, 0 (least significant bit) to 63 (most significant bit) this is how a board is represented:
/// 56 57 58 59 60 61 62 63
/// 48 49 50 51 52 53 54 55
/// 40 41 42 43 44 45 46 47
/// 32 33 34 35 36 37 38 39
/// 24 25 26 27 28 29 30 31
/// 16 17 18 19 20 21 22 23
///  8  9 10 11 12 13 14 15
///  0  1  2  3  4  5  6  7
pub type Bitboard = u64;

/// iterator that iterates ovea each square on a bitboard
pub struct BitboardIterator {
    bitboard: u64,
    i: u32,
}

impl Iterator for BitboardIterator {
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 64 {
            return None;
        }

        let row = self.i / 8;
        let col = self.i % 8;

        self.i += 1;
        return Some((
            Square { col: col, row: row },
            ((self.bitboard >> self.i - 1) % 2) == 1,
        ));
    }

    type Item = (Square, bool);
}

/// creates a bitboard iterator from a bitboard
pub fn bitboard_iterator(board: Bitboard) -> BitboardIterator {
    BitboardIterator {
        bitboard: board,
        i: 0,
    }
}

/// creates a row of truthy bits on a bitboard
pub fn row(row: u32) -> Bitboard {
    (0..8).map(|x| row * 8 + x).fold(0, |y, x| 2u64.pow(x) + y)
}

/// creates a column of truthy bits on a bitboard
pub fn col(col: u32) -> Bitboard {
    (0..8).map(|x| col + 8 * x).fold(0, |y, x| 2u64.pow(x) + y)
}

/// creates a point of truthy bits on a bitboard
pub fn point(row: u32, col: u32) -> Bitboard {
    2u64.pow(row * 8 + col)
}

///// flips the bitboards values horizontaly
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

/// displaces a board (overflow is clipped)
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

/// shifts a board horizontaly
fn shift(x: u64, amount: i32) -> u64 {
    if amount >= 0 {
        x.checked_shl(amount as u32).unwrap_or(0)
    } else {
        x.checked_shr(amount.unsigned_abs()).unwrap_or(0)
    }
}

/// gets the value of a board on a point
pub fn at(board: Bitboard, row: u32, col: u32) -> bool {
    (board & point(row, col)) > 0
}

/// parses a board string to a bitboard
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

/// returns bitboard if contidion is true, 0 otherwise
pub fn bitboard_if(board: Bitboard, condition: bool) -> Bitboard {
    if condition { board } else { 0 }
}

/// returns the amount of truthy bits
pub fn bitboard_bit_count(board: Bitboard) -> u64 {
    (0..64).map(|n| 1 - (board << n) % 2).fold(0, |t, c| t + c)
}
