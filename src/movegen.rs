use crate::piece::{PieceKind, Player};
use crate::board::Board;

#[derive(Debug)]
pub struct ChessMove {
    pub from: i32, // The position in the bitboard
    pub to: i32,
    pub kind: PieceKind,
    pub is_white: bool
}

impl ChessMove {
    pub fn new(from: i32, to: i32, kind: PieceKind, is_white: bool) -> Self {
        return Self { from, to, kind, is_white }
    }
}

pub struct MoveGen {
}

// This is fine for now, but later we should look into magic bitboards
impl MoveGen {
    pub fn all(board: &Board) -> Vec<ChessMove> {
        todo!();
    }

    pub fn pawns(board: &Board) -> Vec<ChessMove> {
        // Finds pawns
        let pawns: Vec<(i32, i32)>;
        if board.turn == Player::White {
            pawns = Board::get_rows_and_cols(board.white[PieceKind::Pawn as usize]);
        } else {
            pawns = Board::get_rows_and_cols(board.black[PieceKind::Pawn as usize]);
        }

        // Create possible moves
        let mut moves: Vec<ChessMove> = Vec::new();
        let dir: i32 = if board.turn == Player::White { 1 } else { -1 };
        for (row, col) in pawns {
            moves.push(
                ChessMove::new(
                    Board::row_col_to_index(row, col),
                    Board::row_col_to_index(row + dir, col),
                    PieceKind::Pawn,
                    true
                ));
        }
        
        // Filter out invalid ones
        return moves.into_iter()
            .filter(|m| !board.is_occupied(Board::index_to_row_col(m.to)))
            .collect();
    }
}
