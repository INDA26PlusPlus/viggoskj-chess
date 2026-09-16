use std::vec;

use crate::{
    advanced_moves::{
        self, can_promote, can_try_kingside_castle, can_try_queenside_castle, do_kingside_castling,
        do_promotion, do_queenside_castling, initialy_legal_advanced_moves_bitboard,
        legal_kingside_castling_move, legal_queenside_castling_move_bitboard,
    },
    bitboard::{
        self, Bitboard, bitboard_bit_count, bitboard_if, bitboard_iterator, bitboard_string, point,
    },
    board::{
        self, Board, ColorBoard, Square, initialy_legal_basic_moves_bitboard, select_playing_board,
        square_bitboard, to_square, validate_square,
    },
    check::is_check,
    chess_error::{ChessError, InvalidMoveReason::NoTargetPiece},
    game::Color::White,
    instantiation,
    moves::{self, AdvancedMove, BasicMove, Move, PossibleMove, PossibleMovesBitboard},
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

    if piece.piece_type == PieceType::Pawn {
        let forward = match piece.piece_color {
            Color::White => 1,
            Color::Black => -1,
        };

        let en_pessant_moves =
            moves::pawn_capture_bitboard(piece.board_position, u64::max_value(), forward)
                & wating_en_pessant_pawns(game);

        if en_pessant_moves & square_bitboard(chess_move.target_square) != 0 {
            return Ok(AdvancedMove::EnPessant {
                basic_move: chess_move,
            });
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

pub fn legal_basic_moves_bitboard(
    game: &Game,
    piece_square: Square,
) -> Result<(Piece, Bitboard), ChessError> {
    let (piece, moves) = initialy_legal_basic_moves_bitboard(&game.board, piece_square, game.turn)?;
    Ok((piece, trim_illegal_moves(game, piece, moves)))
}

pub fn legal_advanced_moves_bitboard(
    game: &Game,
    piece_square: Square,
) -> Result<(Piece, Bitboard), ChessError> {
    let (piece, moves) = initialy_legal_advanced_moves_bitboard(&game, piece_square)?;
    Ok((piece, trim_illegal_moves(game, piece, moves)))
}

pub fn legal_moves_bitboard(game: &Game, target_piece: Square) -> Result<Bitboard, ChessError> {
    let (_, basic_moves) = legal_basic_moves_bitboard(&game, target_piece)?;

    let (_, advanced_moves) = legal_advanced_moves_bitboard(game, target_piece)?;

    return Ok(basic_moves | advanced_moves);
}

pub fn initialy_legal_moves_bitboard(
    game: &Game,
    target_piece: Square,
) -> Result<Bitboard, ChessError> {

    let (_, basic_moves) =
        board::initialy_legal_basic_moves_bitboard(&game.board, target_piece, game.turn)?;

    let (_, advanced_moves) =
        advanced_moves::initialy_legal_advanced_moves_bitboard(game, target_piece)?;

    return Ok(basic_moves | advanced_moves);
}

pub fn play_basic_move(game: &Game, chess_move: BasicMove) -> Result<Game, ChessError> {
    let piece_mask = point(chess_move.piece_square.row, chess_move.piece_square.col);

    let (new_board, moved_piece) = board::do_basic_move(&game.board, chess_move, game.turn)?;

    let (_, available_moves_mask) =
        initialy_legal_basic_moves_bitboard(&game.board, chess_move.piece_square, game.turn)?;

    if square_bitboard(chess_move.target_square) & available_moves_mask == 0 {
        return Err(ChessError::InvalidMove {
            reason: crate::chess_error::InvalidMoveReason::NotAMoveOption,
        });
    }

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
        AdvancedMove::EnPessant { basic_move } => advanced_moves::do_en_pessant(game, basic_move),
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

pub fn wating_en_pessant_pawns(game: &Game) -> Bitboard {
    match game.turn {
        Color::Black => game.white_en_pessant,
        Color::White => game.black_en_pessant,
    }
}

pub fn possible_legal_moves(game: &Game) -> Vec<PossibleMove> {
    Vec::from_iter(
        possible_legal_move_bitboards(game)
            .iter()
            .map(|possible| {
                bitboard_iterator(possible.moves)
                    .filter(|(_, truthy)| *truthy)
                    .map(|(square, _)| PossibleMove {
                        piece: possible.piece,
                        taget_square: square,
                    })
            })
            .flatten(),
    )
}

pub fn possible_initialy_legal_oponent_turn(game: &Game) -> Vec<PossibleMovesBitboard> {
    possible_initialy_legal_move_bitboards(&if_other_turn(game))
}

pub fn possible_legal_move_bitboards(game: &Game) -> Vec<PossibleMovesBitboard> {
    let mut moves: Vec<PossibleMovesBitboard> = Vec::new();

    let (playing, _) = playing_board(game);

    let mut mask = playing.mask();

    for i in 0..64 {
        let bit = mask % 2;
        mask = mask >> 1;
        if bit == 1 {
            let row = i / 8;
            let col = i % 8;
            moves.push(PossibleMovesBitboard {
                moves: match legal_moves_bitboard(game, Square { row: row, col: col }) {
                    Ok(b) => b,
                    Err(ChessError::InvalidMove {
                        reason: NoTargetPiece,
                    }) => 0,
                    Err(e) => Err(e).unwrap(),
                },
                piece: game.board.get_pice(row, col).unwrap(),
            });
        }
    }

    return moves;
}

pub fn possible_initialy_legal_move_bitboards(game: &Game) -> Vec<PossibleMovesBitboard> {
    let mut moves: Vec<PossibleMovesBitboard> = Vec::new();

    let (playing, _) = playing_board(game);

    let mut mask = playing.mask();

    for i in 0..64 {
        let bit = mask % 2;
        mask = mask >> 1;
        if bit == 1 {
            let row = i / 8;
            let col = i % 8;
            moves.push(PossibleMovesBitboard {
                moves: match initialy_legal_moves_bitboard(game, Square { row: row, col: col }) {
                    Ok(b) => b,
                    Err(ChessError::InvalidMove {
                        reason: NoTargetPiece,
                    }) => 0,
                    Err(e) => Err(e).unwrap(),
                },
                piece: game.board.get_pice(row, col).unwrap(),
            });
        }
    }

    return moves;
}

pub fn if_other_turn(game: &Game) -> Game {
    Game {
        white_rook_left_moved: game.white_rook_left_moved,
        white_rook_right_moved: game.white_rook_right_moved,
        white_king_moved: game.white_king_moved,
        black_rook_left_moved: game.black_rook_left_moved,
        black_rook_right_moved: game.black_rook_right_moved,
        black_king_moved: game.black_king_moved,
        white_en_pessant: game.white_en_pessant,
        black_en_pessant: game.black_en_pessant,
        board: game.board,
        turn: game.turn.other(),
    }
}

pub fn trim_illegal_moves(game: &Game, piece: Piece, moves: Bitboard) -> Bitboard {
    bitboard_iterator(moves)
        .filter(|(_, truthy)| *truthy)
        .filter(|(square, _)| {

            if let Ok(game_res) = play_move(
                game,
                moves::Move::Basic {
                    chess_move: BasicMove {
                        piece_square: to_square(piece.board_position).unwrap(),
                        target_square: *square,
                    },
                },
            ) {
                if !is_check(&if_other_turn(&game_res)) {
                    return true;
                }
            }
            false
        })
        .fold(0, |acc, (square, _)| {
            acc | bitboard::point(square.row, square.col)
        })
}
