use std::f32::consts::E;

use crate::{
    advanced_moves::{do_kingside_castling, do_queenside_castling}, bit_board::{self, BitBoard, bitboard_string, point}, board::{self, Board, ColorBoard, Square, validate_square}, chess_error::ChessError, game::Color::White, moves::{AdvancedMove, BasicMove}, piece::{
        Piece, PieceType, castling_board, kingside_castling_move, piece_basic_move_bit_board, queenside_castling_move, square_bitboard,
    },
};

pub struct Game {
    pub(crate) white_rook_left_moved: bool,
    pub(crate) white_rook_right_moved: bool,
    pub(crate) white_king_moved: bool,
    pub(crate) black_rook_left_moved: bool,
    pub(crate) black_rook_right_moved: bool,
    pub(crate) black_king_moved: bool,
    pub board: Board,
    pub turn: Color,
}

pub fn piece_basic_moves_bitboard(
    game: &Game,
    piece_square: Square,
) -> Result<(Piece, BitBoard), ChessError> {
    let piece = match game.board.get_pice(piece_square.row, piece_square.col) {
        Some(t) => t,
        _ => {
            return Err(ChessError::InvalidMove {
                reason: crate::chess_error::InvalidMoveReason::NoTargetPiece,
            });
        }
    };

    let self_mask = match game.turn {
        Color::Black => game.board.black.mask(),
        Color::White => game.board.white.mask(),
    };

    let attack_mask = match game.turn {
        Color::Black => game.board.white.mask(),
        Color::White => game.board.black.mask(),
    };

    let moves = piece_basic_move_bit_board(piece, self_mask, attack_mask);

    Ok((piece, moves))
}

pub fn piece_advanced_moves_bitboard(
    game: &Game,
    piece_square: Square,
) -> Result<(Piece, BitBoard), ChessError> {
    let piece = match game.board.get_pice(piece_square.row, piece_square.col) {
        Some(t) => t,
        _ => {
            return Err(ChessError::InvalidMove {
                reason: crate::chess_error::InvalidMoveReason::NoTargetPiece,
            });
        }
    };

    let self_mask = match game.turn {
        Color::Black => game.board.black.mask(),
        Color::White => game.board.white.mask(),
    };
    let mut moves = 0;

    if piece.piece_type == PieceType::King {
        if game.turn == Color::White {
            if !game.white_king_moved && !game.white_rook_left_moved && !game.white_rook_right_moved
            {
                moves |= castling_board(self_mask, 0);
            }
        } else if game.turn == Color::Black {
            if !game.black_king_moved && !game.black_rook_left_moved && !game.black_rook_right_moved
            {
                moves |= castling_board(self_mask, 7);
            }
        }
    }

    Ok((piece, moves))
}

pub fn resolve_advanced_move(
    game: &Game,
    chess_move: BasicMove,
) -> Result<AdvancedMove, ChessError> {
    let piece_square = chess_move.piece_square;

    let piece = match game.board.get_pice(piece_square.row, piece_square.col) {
        Some(t) => t,
        _ => {
            return Err(ChessError::InvalidMove {
                reason: crate::chess_error::InvalidMoveReason::NoTargetPiece,
            });
        }
    };

    let self_mask = match game.turn {
        Color::Black => game.board.black.mask(),
        Color::White => game.board.white.mask(),
    };

    let attack_mask = match game.turn {
        Color::Black => game.board.white.mask(),
        Color::White => game.board.black.mask(),
    };

    if piece.piece_type == PieceType::King {
        if game.turn == Color::White {
            if !game.white_king_moved && !game.white_rook_left_moved && !game.white_rook_right_moved
            {
                if square_bitboard(chess_move.target_square)
                    & kingside_castling_move(game.board.mask(), 0)
                    > 0
                {
                    return Ok(AdvancedMove::KingSideCastle);
                }

                if square_bitboard(chess_move.target_square)
                    & queenside_castling_move(game.board.mask(), 0)
                    > 0
                {
                    return Ok(AdvancedMove::QueenSideCastle);
                }
            }
        } else if game.turn == Color::Black {
            if !game.black_king_moved && !game.black_rook_left_moved && !game.black_rook_right_moved
            {
                if square_bitboard(chess_move.target_square)
                    & kingside_castling_move(game.board.mask(), 7)
                    > 0
                {
                    return Ok(AdvancedMove::KingSideCastle);
                }

                if square_bitboard(chess_move.target_square)
                    & queenside_castling_move(game.board.mask(), 7)
                    > 0
                {
                    return Ok(AdvancedMove::QueenSideCastle);
                }
            }
        }
    }

    Err(ChessError::InvalidMove {
        reason: crate::chess_error::InvalidMoveReason::NotAMoveOption,
    })
}

pub fn move_piece(game: &Game, chess_move: BasicMove) -> Result<Game, ChessError> {
    if let Ok(advanced_move) = resolve_advanced_move(game, chess_move) {
        advanced_move_piece(game, advanced_move)
    } else {
        basic_move_piece(game, chess_move)
    }
}

pub fn pice_moves_bitboard(game: &Game, target_piece: Square) -> Result<BitBoard, ChessError> {
    let basic = piece_basic_moves_bitboard(game, target_piece);
    let advanced = piece_advanced_moves_bitboard(game, target_piece);

    match advanced {
        Ok(board) => Ok(board.1),
        Err(ChessError::InvalidMove {
            reason: crate::chess_error::InvalidMoveReason::NotAMoveOption,
        }) => match basic {
            Ok(board) => Ok(board.1),
            Err(v) => Err(v),
        },
        Err(v) => Err(v),
    }
}

pub fn basic_move_piece(game: &Game, chess_move: BasicMove) -> Result<Game, ChessError> {
    let (piece, move_set) = piece_basic_moves_bitboard(game, chess_move.piece_square)?;
    let target_mask = point(chess_move.target_square.row, chess_move.target_square.col);
    let piece_mask = point(chess_move.piece_square.row, chess_move.piece_square.col);

    if piece.piece_color != game.turn {
        return Err(ChessError::InvalidMove {
            reason: crate::chess_error::InvalidMoveReason::WrongColor,
        });
    }

    if (move_set & target_mask) == 0 {
        return Err(ChessError::InvalidMove {
            reason: crate::chess_error::InvalidMoveReason::NotAMoveOption,
        });
    }

    let (playing_board, waiting_board) = playing_board(game);

    let new_wating_board = ColorBoard {
        pawns: waiting_board.pawns & !target_mask,
        knights: waiting_board.knights & !target_mask,
        bishops: waiting_board.bishops & !target_mask,
        rooks: waiting_board.rooks & !target_mask,
        queens: waiting_board.queens & !target_mask,
        kings: waiting_board.kings & !target_mask,
    };

    let new_playing_board = ColorBoard {
        bishops: playing_board.bishops & (!piece.board_position)
            | if_piece_type(piece.piece_type, PieceType::Bishop, target_mask),
        kings: playing_board.kings & (!piece.board_position)
            | if_piece_type(piece.piece_type, PieceType::King, target_mask),
        knights: playing_board.knights & (!piece.board_position)
            | if_piece_type(piece.piece_type, PieceType::Knight, target_mask),
        pawns: playing_board.pawns & (!piece.board_position)
            | if_piece_type(piece.piece_type, PieceType::Pawn, target_mask),
        queens: playing_board.queens & (!piece.board_position)
            | if_piece_type(piece.piece_type, PieceType::Queen, target_mask),
        rooks: playing_board.rooks & (!piece.board_position)
            | if_piece_type(piece.piece_type, PieceType::Rook, target_mask),
    };

    Ok(Game {
        black_rook_left_moved: game.black_rook_left_moved
            | piece_moved(board::black_default_left_rook_board(), piece_mask),
        black_rook_right_moved: game.black_rook_right_moved
            | piece_moved(board::black_default_right_rook_board(), piece_mask),
        black_king_moved: game.black_king_moved
            | piece_moved(board::black_default_king_board(), piece_mask),
        white_rook_left_moved: game.white_rook_left_moved
            | piece_moved(board::white_default_left_rook_board(), piece_mask),
        white_rook_right_moved: game.white_rook_right_moved
            | piece_moved(board::white_default_right_rook_board(), piece_mask),
        white_king_moved: game.white_king_moved
            | piece_moved(board::white_default_king_board(), piece_mask),
        turn: game.turn.other(),
        board: new_board(game, new_playing_board, new_wating_board),
    })
}

pub fn advanced_move_piece(
    game: &Game,
    chess_move: AdvancedMove,
) -> Result<Game, ChessError> {
    match chess_move {
        AdvancedMove::KingSideCastle => do_kingside_castling(game),
        AdvancedMove::QueenSideCastle => do_queenside_castling(game),
    }
}

fn if_piece_type(
    piece_type: PieceType,
    required_piece_type: PieceType,
    board: BitBoard,
) -> BitBoard {
    if piece_type == required_piece_type {
        board
    } else {
        0
    }
}

pub struct MoveResult {}

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn other(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

fn piece_moved(piece_position: BitBoard, moved_piece: BitBoard) -> bool {
    (piece_position & moved_piece) > 0
}

pub fn playing_board(game: &Game) -> (ColorBoard, ColorBoard) {
    match game.turn {
        Color::Black => (game.board.black, game.board.white),
        Color::White => (game.board.white, game.board.black),
    }
}

pub fn new_board(game: &Game, new_playing_board: ColorBoard, new_waiting_board: ColorBoard) -> Board {
    Board {
        white: match game.turn {
            Color::Black => new_waiting_board,
            Color::White => new_playing_board,
        },
        black: match game.turn {
            Color::White => new_waiting_board,
            Color::Black => new_playing_board,
        },
    }
}
