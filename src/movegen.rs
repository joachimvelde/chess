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
                    board.turn
                ));
        }
        
        // Filter out invalid ones
        return moves.into_iter()
            .filter(|m| !board.is_occupied(Board::index_to_row_col(m.to)))
            .collect();
    }

    pub fn piece_at(board: &Board, coords: (i32, i32)) -> Vec<ChessMove> {
        let piece: Piece = board.at(coords).expect("Not piece at position");
        let mut moves: Vec<ChessMove> = Vec::new();

        let empty_squares = board.get_empty_squares();
        
        let dir: i32 = if board.turn == Player::White { 1 } else { -1 };

        match piece.kind {
            PieceKind::Pawn => {
                let single_push = Self::shift(Board::index_to_u64(piece.index), 8 * dir) & empty_squares;
                moves.push(ChessMove::new(
                        Board::row_col_to_index(coords.0, coords.1), Board::u64_to_index(single_push),
                        piece.kind,
                        piece.player
                ));

                if (coords.0 == 1 && piece.player == Player::White) || (coords.0 == 6 && piece.player == Player::Black)  {
                    let double_push = Self::shift(Board::index_to_u64(piece.index), 16 * dir) & empty_squares;
                    moves.push(ChessMove::new(
                            Board::row_col_to_index(coords.0, coords.1), Board::u64_to_index(double_push),
                            piece.kind,
                            piece.player
                    ));
                }

                // TODO: Kills
            },
            PieceKind::Knight => (),
            PieceKind::Bishop => (),
            PieceKind::Rook => (),
            PieceKind::Queen => (),
            PieceKind::King => ()
        }

        moves
    }

    // Helper that allows us to shift with negative values
    fn shift(x: u64, shamt: i32) -> u64 {
        match shamt > 0 {
            true =>  x <<  shamt,
            false => x >> -shamt
        }
    }
}
