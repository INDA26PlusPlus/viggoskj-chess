use crate::{
    advanced_moves::{
        self, can_promote, can_try_kingside_castle, can_try_queenside_castle, do_kingside_castling,
        do_promotion, do_queenside_castling, legal_kingside_castling_move,
        legal_queenside_castling_move_bitboard,
    },
    bitboard::{self, Bitboard, bitboard_if, bitboard_string, point},
    board::{
        self, Board, ColorBoard, Square, legal_basic_moves_bitboard, square_bitboard,
        validate_square,
    },
    chess_error::ChessError,
    game::Color::White,
    instantiation,
    moves::{AdvancedMove, BasicMove, Move},
    piece::{Piece, PieceType},
};

#[derive(Debug, PartialEq)]
pub struct Game {
    pub(crate) white_rook_left_moved: bool,
    pub(crate) white_rook_right_moved: bool,
    pub(crate) white_king_moved: bool,
    pub(crate) black_rook_left_moved: bool,
    pub(crate) black_rook_right_moved: bool,
    pub(crate) black_king_moved: bool,

    pub(crate) white_en_pessant: Bitboard,
    pub(crate) black_en_pessant: Bitboard,

    pub board: Board,
    pub turn: Color,
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

    if piece.piece_type == PieceType::King {
        if can_try_kingside_castle(game) {
            if board::square_bitboard(chess_move.target_square)
                & legal_kingside_castling_move(game.board.mask(), piece_square.row)
                > 0
            {
                return Ok(AdvancedMove::KingSideCastle);
            }
        }

        if can_try_queenside_castle(game) {
            if board::square_bitboard(chess_move.target_square)
                & legal_queenside_castling_move_bitboard(game.board.mask(), piece_square.row)
                > 0
            {
                return Ok(AdvancedMove::QueenSideCastle);
            }
        }
    }

    Err(ChessError::InvalidMove {
        reason: crate::chess_error::InvalidMoveReason::NotAMoveOption,
    })
}

pub fn play_move(game: &Game, chess_move: Move) -> Result<Game, ChessError> {
    match chess_move {
        Move::Basic { chess_move: basic } => {
            if let Ok(advanced_move) = resolve_advanced_move(game, basic) {
                play_advanced_move(game, advanced_move)
            } else {
                play_basic_move(game, basic)
            }
        }
        Move::Advanced { chess_move } => play_advanced_move(game, chess_move),
    }
}

pub fn pice_moves_bitboard(game: &Game, target_piece: Square) -> Result<Bitboard, ChessError> {
    let basic = board::legal_basic_moves_bitboard(&game.board, target_piece, game.turn);
    let advanced = advanced_moves::legal_advanced_moves_bitboard(game, target_piece);

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

pub fn play_basic_move(game: &Game, chess_move: BasicMove) -> Result<Game, ChessError> {
    let piece_mask = point(chess_move.piece_square.row, chess_move.piece_square.col);

    let (new_board, moved_piece) = board::do_basic_move(&game.board, chess_move, game.turn)?;

    Ok(Game {
        black_rook_left_moved: game.black_rook_left_moved
            | piece_moved(
                bitboard_if(
                    instantiation::black_default_left_rook_board(),
                    moved_piece.piece_type == PieceType::Rook && game.turn == Color::Black,
                ),
                piece_mask,
            ),
        black_rook_right_moved: game.black_rook_right_moved
            | piece_moved(
                bitboard_if(
                    instantiation::black_default_right_rook_board(),
                    moved_piece.piece_type == PieceType::Rook && game.turn == Color::Black,
                ),
                piece_mask,
            ),
        black_king_moved: game.black_king_moved
            | piece_moved(
                bitboard_if(
                    instantiation::black_default_king_board(),
                    moved_piece.piece_type == PieceType::King && game.turn == Color::Black,
                ),
                piece_mask,
            ),
        white_rook_left_moved: game.white_rook_left_moved
            | piece_moved(
                bitboard_if(
                    instantiation::white_default_left_rook_board(),
                    moved_piece.piece_type == PieceType::Rook && game.turn == Color::White,
                ),
                piece_mask,
            ),
        white_rook_right_moved: game.white_rook_right_moved
            | piece_moved(
                bitboard_if(
                    instantiation::white_default_right_rook_board(),
                    moved_piece.piece_type == PieceType::Rook && game.turn == Color::White,
                ),
                piece_mask,
            ),
        white_king_moved: game.white_king_moved
            | piece_moved(
                bitboard_if(
                    instantiation::white_default_king_board(),
                    moved_piece.piece_type == PieceType::King && game.turn == Color::White,
                ),
                piece_mask,
            ),
        turn: game.turn.other(),
        white_en_pessant: bitboard::bitboard_if(
            point(2, chess_move.piece_square.col),
            game.turn == Color::White
                && moved_piece.piece_type == PieceType::Pawn
                && chess_move.piece_square.row == 1
                && chess_move.target_square.row == 3,
        ),
        black_en_pessant: bitboard::bitboard_if(
            point(5, chess_move.piece_square.col),
            game.turn == Color::Black
                && moved_piece.piece_type == PieceType::Pawn
                && chess_move.piece_square.row == 6
                && chess_move.target_square.row == 4,
        ),
        board: new_board,
    })
}

pub fn play_promotion(
    game: &Game,
    promotion_type: PieceType,
    basic_move: BasicMove,
) -> Result<Game, ChessError> {
    if !(advanced_moves::can_promote(&game.board, basic_move.piece_square, game.turn)?) {
        return Err(ChessError::InvalidMove {
            reason: crate::chess_error::InvalidMoveReason::NotAMoveOption,
        });
    }

    do_promotion(game, promotion_type, basic_move)
}

pub fn play_advanced_move(game: &Game, chess_move: AdvancedMove) -> Result<Game, ChessError> {
    match chess_move {
        AdvancedMove::KingSideCastle => do_kingside_castling(game),
        AdvancedMove::QueenSideCastle => do_queenside_castling(game),
        AdvancedMove::Promotion {
            piece_type,
            basic_move,
        } => play_promotion(game, piece_type, basic_move),
    }
}

pub struct MoveResult {}

#[derive(Copy, Clone, PartialEq, Debug)]
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

fn piece_moved(piece_position: Bitboard, moved_piece: Bitboard) -> bool {
    (piece_position & moved_piece) > 0
}

pub fn playing_board(game: &Game) -> (ColorBoard, ColorBoard) {
    match game.turn {
        Color::Black => (game.board.black, game.board.white),
        Color::White => (game.board.white, game.board.black),
    }
}
