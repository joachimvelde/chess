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
        let pawns: u64;
        let friends: u64;
        let enemies: u64;
        let mut moves: Vec<ChessMove> = Vec::new();

        match board.get_turn() {
            Player::White => {
                pawns = board.white[PieceKind::Pawn as usize];
                friends = board.get_occupied(Player::Black);
                enemies = board.get_occupied(Player::White);
            },
            Player::Black => {
                pawns = board.black[PieceKind::Pawn as usize];
                friends = board.get_occupied(Player::White);
                enemies = board.get_occupied(Player::Black);
            }
        }

        for i in 0..u64::BITS {
            let bit = (pawns >> i) & 0b1;
            if bit != 0 {
                let forward: i32 = match board.get_turn() {
                    Player::White => 8,
                    Player::Black => -8
                };
                let single_push = Self::shift(1u64, i as i32 + forward) & !friends & !enemies;
                let double_push = Self::shift(single_push, forward) & !friends & !enemies;

                if single_push != 0 {
                    moves.push(ChessMove::new(i as i32, Board::u64_to_index(single_push), PieceKind::Pawn, board.get_turn()));
                }

                if double_push != 0 &&
                    ((board.get_turn() == Player::White && Board::index_to_row_col(i as i32).0 == 1) ||
                     (board.get_turn() == Player::Black && Board::index_to_row_col(i as i32).0 == 6))
                {
                    moves.push(ChessMove::new(
                            i as i32,
                            Board::u64_to_index(double_push),
                            PieceKind::Pawn, board.get_turn()
                    ));
                }

                // TODO: Kills
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

    // Helper that allows us to shift with negative values
    fn shift(x: u64, shamt: i32) -> u64 {
        match shamt > 0 {
            true =>  x <<  shamt,
            false => x >> -shamt
        }
    }
}
