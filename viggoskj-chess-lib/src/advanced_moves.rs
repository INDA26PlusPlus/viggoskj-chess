use crate::bitboard::Bitboard;
use crate::board::{back_row, legal_basic_moves_bitboard};
use crate::moves;
use crate::piece::Piece;
use crate::{
    bitboard::point,
    board::{self, Board, ColorBoard, Square, square_bitboard},
    chess_error::ChessError,
    game::{
        self,
        Color::{self, Black, White},
        Game, playing_board,
    },
    moves::{AdvancedMove, BasicMove},
    piece::{self, PieceType},
};

pub fn legal_advanced_moves_bitboard(
    game: &Game,
    piece_square: Square,
) -> Result<(Piece, Bitboard), ChessError> {
    let piece = match game.board.get_pice(piece_square.row, piece_square.col) {
        Some(t) => t,
        _ => {
            return Err(ChessError::InvalidMove {
                reason: crate::chess_error::InvalidMoveReason::NoTargetPiece,
            });
        }
    };

    let mut moves = 0;

    if piece.piece_type == PieceType::King {
        if can_try_kingside_castle(game) {
            moves |= legal_kingside_castling_move(game.board.mask(), piece_square.row);
        }

        if can_try_queenside_castle(game) {
            moves |= legal_queenside_castling_move_bitboard(game.board.mask(), piece_square.row);
        }
    }

    if piece.piece_type == PieceType::Pawn && can_promote(&game.board, piece_square, game.turn)? {
        moves |= legal_basic_moves_bitboard(&game.board, piece_square, game.turn)?.1;
    }

    Ok((piece, moves & !(square_bitboard(piece_square))))
}

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
    let king_row = back_row(game.turn);
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
        white_en_pessant: 0,
        black_en_pessant: 0,
    })
}

pub fn do_queenside_castling(game: &Game) -> Result<Game, ChessError> {
    let king_row = back_row(game.turn);
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
        black_en_pessant: 0,
        white_en_pessant: 0,
    })
}

pub fn can_promote(board: &Board, pawn_square: Square, playing: Color) -> Result<bool, ChessError> {
    let piece = match board.get_pice(pawn_square.row, pawn_square.col) {
        Some(t) => t,
        _ => {
            return Err(ChessError::InvalidMove {
                reason: crate::chess_error::InvalidMoveReason::NoTargetPiece,
            });
        }
    };

    if piece.piece_type != PieceType::Pawn {
        return Err(ChessError::InvalidMove {
            reason: crate::chess_error::InvalidMoveReason::NotAMoveOption,
        });
    }

    let (playing, waiting) = board::select_playing_board(board, playing);

    let move_mask = moves::legal_basic_moves_bitboard(piece, playing.mask(), waiting.mask());

    Ok(square_bitboard(pawn_square) & move_mask != 0)
}

pub fn do_promotion(
    game: &Game,
    promotion_type: PieceType,
    basic_move: BasicMove,
) -> Result<Game, ChessError> {
    let (new_board, _) = board::do_basic_move(&game.board, basic_move, game.turn)?;

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
        pawns: playing_board.pawns & !(square_bitboard(basic_move.target_square)),
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
        black_en_pessant: 0,
        white_en_pessant: 0,
    })
}

pub fn legal_kingside_castling_move(full_mask: Bitboard, row: u32) -> Bitboard {
    if full_mask & !(point(row, 6) | point(row, 5)) == full_mask {
        point(row, 6)
    } else {
        0
    }
}

pub fn legal_queenside_castling_move_bitboard(full_mask: Bitboard, row: u32) -> Bitboard {
    if full_mask & !(point(row, 1) | point(row, 2) | point(row, 3)) == full_mask {
        point(row, 2)
    } else {
        0
    }
}
