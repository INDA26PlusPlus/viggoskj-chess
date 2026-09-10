use crate::{
    bit_board::point,
    board::{Board, ColorBoard},
    chess_error::ChessError,
    game::{
        Color::{self, Black, White},
        Game, new_board, playing_board,
    },
};

pub fn do_kingside_castling(game: &Game) -> Result<Game, ChessError> {
    let king_row = king_row(game);
    let (playing_board, waiting_board) = playing_board(game);

    let new_playing_board = ColorBoard {
        pawns: playing_board.pawns,
        knights: playing_board.knights,
        bishops: playing_board.bishops,
        rooks: playing_board.rooks & (!point(king_row, 7)) | point(king_row, 5),
        queens: playing_board.queens,
        kings: point(king_row, 6),
    };

    Ok(Game {
        board: new_board(game, new_playing_board, waiting_board),
        turn: game.turn.other(),
        black_king_moved: game.black_king_moved | (game.turn == Black),
        black_rook_left_moved: game.black_rook_left_moved,
        black_rook_right_moved: game.black_rook_right_moved | (game.turn == Black),
        white_king_moved: game.white_king_moved | (game.turn == White),
        white_rook_left_moved: game.white_rook_left_moved,
        white_rook_right_moved: game.white_rook_right_moved | (game.turn == White),
    })
}

pub fn do_queenside_castling(game: &Game) -> Result<Game, ChessError> {
    let king_row = king_row(game);
    let (playing_board, waiting_board) = playing_board(game);

    let new_playing_board = ColorBoard {
        pawns: playing_board.pawns,
        knights: playing_board.knights,
        bishops: playing_board.bishops,
        rooks: playing_board.rooks & (!point(king_row, 0)) | point(king_row, 3),
        queens: playing_board.queens,
        kings: point(king_row, 2),
    };

    Ok(Game {
        board: new_board(game, new_playing_board, waiting_board),
        turn: game.turn.other(),
        black_king_moved: game.black_king_moved | (game.turn == Black),
        black_rook_left_moved: game.black_rook_left_moved | (game.turn == Black),
        black_rook_right_moved: game.black_rook_right_moved,
        white_king_moved: game.white_king_moved | (game.turn == White),
        white_rook_left_moved: game.white_rook_left_moved | (game.turn == White),
        white_rook_right_moved: game.white_rook_right_moved,
    })
}

fn king_row(game: &Game) -> u32 {
    match game.turn {
        White => 0,
        Black => 7,
    }
}
