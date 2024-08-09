use crate::piece::{Piece, PieceKind, Player};
use crate::board::Board;

#[derive(Debug)]
pub struct ChessMove {
    pub from: i32, // The position in the bitboard
    pub to: i32,
    pub kind: PieceKind,
    pub player: Player
}

// Is it necessary to include the piece kind here?
impl ChessMove {
    pub fn new(from: i32, to: i32, kind: PieceKind, player: Player) -> Self {
        return Self { from, to, kind, player }
    }
}

pub struct MoveGen {
}

impl MoveGen {
    pub fn all(board: &Board) -> Vec<ChessMove> {
        todo!();
    }

    pub fn pawns(board: &mut Board) -> Vec<ChessMove> {
        let pawns: u64 = match board.get_turn() {
            Player::White => board.white[PieceKind::Pawn as usize],
            Player::Black => board.black[PieceKind::Pawn as usize],
        };

        let mut moves: Vec<ChessMove> = Vec::new();

        for i in 0..u64::BITS {
            if (pawns >> i) != 0 {
                moves.extend(Self::pawn(board, Board::u64_to_row_col(pawns >> i)));
            }
        }

        moves
    }

    // We need to process single pawn to be able to draw their individual bitmasks
    pub fn pawn(board: &mut Board, coords: (i32, i32)) -> Vec<ChessMove> {
        let mut bits = 0u64; // Only used for drawing

        let dir: i32;
        let pawn: u64 = Board::row_col_to_u64(coords.0, coords.1);
        let friends: u64;
        let enemies: u64;
        let col0_mask: u64 = 0x8080808080808080;
        let col7_mask: u64 = 0x0101010101010101;
        let mut moves: Vec<ChessMove> = Vec::new();

        match board.get_turn() {
            Player::White => {
                dir = -1;
                friends = board.get_occupied(Player::White);
                enemies = board.get_occupied(Player::Black);
            },
            Player::Black => {
                dir = 1;
                friends = board.get_occupied(Player::Black);
                enemies = board.get_occupied(Player::White);
            }
        }

        let single_push: u64 = shift(pawn, dir * 8) & !friends & !enemies;
        let double_push: u64 = shift(single_push, dir * 8) & !friends & !enemies;

        if single_push != 0 {
            moves.push(ChessMove::new(Board::u64_to_index(pawn), Board::u64_to_index(single_push), PieceKind::Pawn, board.get_turn()));
            bits |= single_push;
        }

        if double_push != 0 &&
            ((board.get_turn() == Player::White && pawn & 0x00ff000000000000 != 0) || // Seventh row
             (board.get_turn() == Player::Black && pawn & 0x000000000000ff00 != 0))   // Second row
        {
            moves.push(
                ChessMove::new(
                    Board::u64_to_index(pawn),
                    Board::u64_to_index(double_push),
                    PieceKind::Pawn, board.get_turn()
                ));
            bits |= double_push;
        }

        // Kills
        let (mut left_kill, mut right_kill) = (shift(pawn, 7 * dir), shift(pawn, 9 * dir));

        // Mask out kills that cross the board
        match board.get_turn() {
            Player::White => {
                left_kill &= !col7_mask;
                right_kill &= !col0_mask;
            },
            Player::Black => {
                left_kill &= !col0_mask;
                right_kill &= !col7_mask;
            }
        };

        if left_kill & enemies != 0 {
            moves.push(
                ChessMove::new(
                    Board::u64_to_index(pawn),
                    Board::u64_to_index(left_kill),
                    PieceKind::Pawn, board.get_turn()
                ));
            bits |= left_kill;
        }

        if right_kill & enemies != 0 {
            moves.push(
                ChessMove::new(
                    Board::u64_to_index(pawn),
                    Board::u64_to_index(right_kill),
                    PieceKind::Pawn, board.get_turn()
                ));
            bits |= right_kill;
        }

        // Draw the bits
        board.bits = Some(bits);

        moves
    }

    pub fn knight(board: &mut Board, coords: (i32, i32)) -> Vec<ChessMove> {
        let mut movemask = 1u64;
        let (friends, enemies) = match board.get_turn() {
            Player::White => (board.white, board.black),
            Player::Black => (board.black, board.white)
        };

        vec![]
    }

    pub fn piece_at(board: &mut Board, coords: (i32, i32)) -> Vec<ChessMove> {
        match board.at(coords).unwrap().kind {
            PieceKind::Pawn => Self::pawn(board, coords),
            PieceKind::Knight => Self::knight(board, coords),
            PieceKind::Bishop => vec![],
            PieceKind::Rook => vec![],
            PieceKind::Queen => vec![],
            PieceKind::King => vec![]
        }
    }
}

// Helper that allows us to shift with negative values
fn shift(x: u64, shamt: i32) -> u64 {
    if shamt > 0 {
        if shamt >= 64 {
            0
        } else {
            x << shamt
        }
    } else {
        if -shamt >= 64 {
            0
        } else {
            x >> -shamt
        }
    }
}
