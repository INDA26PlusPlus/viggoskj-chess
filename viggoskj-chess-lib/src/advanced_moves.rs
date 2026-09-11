use crate::{
    bitboard::point,
    board::{self, Board, ColorBoard, square_bitboard},
    chess_error::ChessError,
    game::{
        self,
        Color::{self, Black, White},
        Game, playing_board,
    },
    moves::{AdvancedMove, BasicMove},
    piece::{self, PieceType},
};

pub fn can_try_queenside_castle(game: &Game) -> bool {
    if game.turn == Color::White {
        if !game.white_king_moved && !game.white_rook_left_moved {
            return true;
        }
    } else if game.turn == Color::Black {
        if !game.black_king_moved && !game.black_rook_left_moved {
            return true;
        }
    }
    return false;
}

pub fn can_try_kingside_castle(game: &Game) -> bool {
    if game.turn == Color::White {
        if !game.white_king_moved && !game.white_rook_right_moved {
            return true;
        }
    } else if game.turn == Color::Black {
        if !game.black_king_moved && !game.black_rook_right_moved {
            return true;
        }
    }
    return false;
}

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
        board: board::new_board(new_playing_board, waiting_board, game.turn),
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
        board: board::new_board(new_playing_board, waiting_board, game.turn),
        turn: game.turn.other(),
        black_king_moved: game.black_king_moved | (game.turn == Black),
        black_rook_left_moved: game.black_rook_left_moved | (game.turn == Black),
        black_rook_right_moved: game.black_rook_right_moved,
        white_king_moved: game.white_king_moved | (game.turn == White),
        white_rook_left_moved: game.white_rook_left_moved | (game.turn == White),
        white_rook_right_moved: game.white_rook_right_moved,
    })
}

pub fn do_promotion(
    game: &Game,
    promotion_type: PieceType,
    basic_move: BasicMove,
) -> Result<Game, ChessError> {
    let new_board = board::basic_move_piece(&game.board, basic_move, game.turn)?;

    let (playing_board, waiting_board) = board::select_playing_board(&new_board, game.turn);

    let new_playing = ColorBoard {
        bishops: playing_board.bishops
            | piece::if_piece_type(
                PieceType::Bishop,
                promotion_type,
                square_bitboard(basic_move.target_square),
            ),
        kings: playing_board.kings
            | piece::if_piece_type(
                PieceType::King,
                promotion_type,
                square_bitboard(basic_move.target_square),
            ),
        knights: playing_board.knights
            | piece::if_piece_type(
                PieceType::Knight,
                promotion_type,
                square_bitboard(basic_move.target_square),
            ),
        pawns: playing_board.pawns& !(square_bitboard(basic_move.target_square)),
        queens: playing_board.queens
            | piece::if_piece_type(
                PieceType::Queen,
                promotion_type,
                square_bitboard(basic_move.target_square),
            ),
        rooks: playing_board.rooks
            | piece::if_piece_type(
                PieceType::Rook,
                promotion_type,
                square_bitboard(basic_move.target_square),
            ),
    };

    Ok(Game {
        board: board::new_board(new_playing, waiting_board, game.turn),
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
