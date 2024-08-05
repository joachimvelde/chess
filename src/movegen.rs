use crate::piece::Piece;
use crate::board::Board;

#[derive(Debug)]
pub struct ChessMove {
    pub from: i32, // The position in the bitboard
    pub to: i32,
    pub kind: Piece,
    pub is_white: bool
}

impl ChessMove {
    pub fn new(from: i32, to: i32, kind: Piece, is_white: bool) -> Self {
        return Self { from, to, kind, is_white }
    }
}

pub struct MoveGen {
}

impl MoveGen {
    pub fn all(board: &Board) -> Vec<ChessMove> {
        todo!();
    }

    pub fn pawns(board: &Board) -> Vec<ChessMove> {
        // Finds pawns
        let pawns: Vec<(i32, i32)>;
        if board.white_turn {
            pawns = Board::get_rows_and_cols(board.white[Piece::Pawn as usize]);
        } else {
            pawns = Board::get_rows_and_cols(board.black[Piece::Pawn as usize]);
        }

        // Create possible moves
        let mut moves: Vec<ChessMove> = Vec::new();
        let dir: i32 = if board.white_turn { 1 } else { -1 };
        for (row, col) in pawns {
            moves.push(
                ChessMove::new(
                    Board::row_col_to_index(row, col),
                    Board::row_col_to_index(row + dir, col),
                    Piece::Pawn,
                    true
                ));
        }
        
        // Filter out invalid ones
        return moves.into_iter()
            .filter(|m| !board.is_occupied(Board::index_to_row_col(m.to)))
            .collect();
    }
}
