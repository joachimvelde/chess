use crate::piece::{Piece, PieceKind, Player};
use crate::board::Board;

// TODO: Decide if kind and player are actually needed
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

    pub fn pawns(board: &Board) -> Vec<ChessMove> {
        let dir: i32;
        let pawns: u64;
        let friends: u64;
        let enemies: u64;
        let mut moves: Vec<ChessMove> = Vec::new();

        match board.get_turn() {
            Player::White => {
                dir = 1;
                pawns = board.white[PieceKind::Pawn as usize];
                friends = board.get_occupied(Player::White);
                enemies = board.get_occupied(Player::Black);
            },
            Player::Black => {
                dir = -1;
                pawns = board.black[PieceKind::Pawn as usize];
                friends = board.get_occupied(Player::Black);
                enemies = board.get_occupied(Player::White);
            }
        }

        for i in 0..u64::BITS {
            if (pawns >> i) != 0 {
                let single_push = shift(1u64, i as i32 + dir * 8) & !friends & !enemies;
                let double_push = shift(single_push, dir * 8) & !friends & !enemies;

                if single_push != 0 {
                    moves.push(ChessMove::new(i as i32, Board::u64_to_index(single_push), PieceKind::Pawn, board.get_turn()));
                }

                if double_push != 0 &&
                    ((board.get_turn() == Player::White && Board::index_to_row_col(i as i32).0 == 1) ||
                     (board.get_turn() == Player::Black && Board::index_to_row_col(i as i32).0 == 6))
                {
                    moves.push(
                        ChessMove::new(
                            i as i32,
                            Board::u64_to_index(double_push),
                            PieceKind::Pawn, board.get_turn()
                    ));
                }

                let (left_kill, right_kill) = (shift(1u64, i as i32 + 7 * dir), shift(1u64, i as i32 + 9 * dir));

                // TODO: Make sure edge pieces cannot capture across the board
                if left_kill & enemies != 0 {
                    moves.push(
                        ChessMove::new(
                            i as i32,
                            Board::u64_to_index(left_kill),
                            PieceKind::Pawn, board.get_turn()
                        ));
                }

                if right_kill & enemies != 0 {
                    moves.push(
                        ChessMove::new(
                            i as i32,
                            Board::u64_to_index(right_kill),
                            PieceKind::Pawn, board.get_turn()
                        ));
                }
            }
        }

        moves
    }

    pub fn piece_at(board: &Board, coords: (i32, i32)) -> Vec<ChessMove> {
        match board.at(coords).unwrap().kind {
            PieceKind::Pawn => {
                Self::pawns(board)
                .into_iter()
                .filter(|m| Board::index_to_row_col(m.from) == coords)
                .collect()
            }
            PieceKind::Knight => vec![],
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
