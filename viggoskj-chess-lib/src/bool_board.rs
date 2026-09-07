pub type BoolBoard = [[bool; 8]; 8];

pub fn union(a: BoolBoard, b: BoolBoard) -> BoolBoard {
    [
        [a[0][0] | b[0][0], a[0][1] | b[0][1], a[0][2] | b[0][2], a[0][3] | b[0][3], a[0][4] | b[0][4], a[0][5] | b[0][5], a[0][6] | b[0][6], a[0][7] | b[0][7]],
        [a[1][0] | b[1][0], a[1][1] | b[1][1], a[1][2] | b[1][2], a[1][3] | b[1][3], a[1][4] | b[1][4], a[1][5] | b[1][5], a[1][6] | b[1][6], a[1][7] | b[1][7]],
        [a[2][0] | b[2][0], a[2][1] | b[2][1], a[2][2] | b[2][2], a[2][3] | b[2][3], a[2][4] | b[2][4], a[2][5] | b[2][5], a[2][6] | b[2][6], a[2][7] | b[2][7]],
        [a[3][0] | b[3][0], a[3][1] | b[3][1], a[3][2] | b[3][2], a[3][3] | b[3][3], a[3][4] | b[3][4], a[3][5] | b[3][5], a[3][6] | b[3][6], a[3][7] | b[3][7]],
        [a[4][0] | b[4][0], a[4][1] | b[4][1], a[4][2] | b[4][2], a[4][3] | b[4][3], a[4][4] | b[4][4], a[4][5] | b[4][5], a[4][6] | b[4][6], a[4][7] | b[4][7]],
        [a[5][0] | b[5][0], a[5][1] | b[5][1], a[5][2] | b[5][2], a[5][3] | b[5][3], a[5][4] | b[5][4], a[5][5] | b[5][5], a[5][6] | b[5][6], a[5][7] | b[5][7]],
        [a[6][0] | b[6][0], a[6][1] | b[6][1], a[6][2] | b[6][2], a[6][3] | b[6][3], a[6][4] | b[6][4], a[6][5] | b[6][5], a[6][6] | b[6][6], a[6][7] | b[6][7]],
        [a[7][0] | b[7][0], a[7][1] | b[7][1], a[7][2] | b[7][2], a[7][3] | b[7][3], a[7][4] | b[7][4], a[7][5] | b[7][5], a[7][6] | b[7][6], a[7][7] | b[7][7]],
    ]
}

pub fn not(a: BoolBoard) -> BoolBoard {
    [
        [!a[0][0], !a[0][1], !a[0][2], !a[0][3], !a[0][4], !a[0][5], !a[0][6], !a[0][7]],
        [!a[1][0], !a[1][1], !a[1][2], !a[1][3], !a[1][4], !a[1][5], !a[1][6], !a[1][7]],
        [!a[2][0], !a[2][1], !a[2][2], !a[2][3], !a[2][4], !a[2][5], !a[2][6], !a[2][7]],
        [!a[3][0], !a[3][1], !a[3][2], !a[3][3], !a[3][4], !a[3][5], !a[3][6], !a[3][7]],
        [!a[4][0], !a[4][1], !a[4][2], !a[4][3], !a[4][4], !a[4][5], !a[4][6], !a[4][7]],
        [!a[5][0], !a[5][1], !a[5][2], !a[5][3], !a[5][4], !a[5][5], !a[5][6], !a[5][7]],
        [!a[6][0], !a[6][1], !a[6][2], !a[6][3], !a[6][4], !a[6][5], !a[6][6], !a[6][7]],
        [!a[7][0], !a[7][1], !a[7][2], !a[7][3], !a[7][4], !a[7][5], !a[7][6], !a[7][7]],
    ]
}

pub fn vertical_mirror(a: BoolBoard) -> BoolBoard {
    [
        [a[7][0], a[7][1], a[7][2], a[7][3], a[7][4], a[7][5], a[7][6], a[7][7]],
        [a[6][0], a[6][1], a[6][2], a[6][3], a[6][4], a[6][5], a[6][6], a[6][7]],
        [a[5][0], a[5][1], a[5][2], a[5][3], a[5][4], a[5][5], a[5][6], a[5][7]],
        [a[4][0], a[4][1], a[4][2], a[4][3], a[4][4], a[4][5], a[4][6], a[4][7]],
        [a[3][0], a[3][1], a[3][2], a[3][3], a[3][4], a[3][5], a[3][6], a[3][7]],
        [a[2][0], a[2][1], a[2][2], a[2][3], a[2][4], a[2][5], a[2][6], a[2][7]],
        [a[1][0], a[1][1], a[1][2], a[1][3], a[1][4], a[1][5], a[1][6], a[1][7]],
        [a[0][0], a[0][1], a[0][2], a[0][3], a[0][4], a[0][5], a[0][6], a[0][7]],
    ]
}

pub fn row(row_index: usize) -> BoolBoard {
    let mut board = empty();

    for i in 0..8 {
        board[row_index][i] = true;
    }

    return board;
}

pub fn col(col_index: usize) -> BoolBoard {
    let mut board = empty();

    for i in 0..8 {
        board[i][col_index] = true;
    }

    return board;
}

pub fn point(row: usize, col: usize) -> BoolBoard
{
    let mut board = empty();
    board[col][row] = true;
    return board;
}

pub fn empty() -> BoolBoard {
    [
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
    ]
}

pub fn full() -> BoolBoard {
    [
        [true, true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true, true],
    ]
}