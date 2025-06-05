use crate::piece::{Piece, PieceKind, Player};
use crate::board::Board;

/*
 * Some of the functions in this file have the same structure, where the only diffierence is the
 * dirs-variable and piece kind. Consider creating a function that handles this.
 * */

const KNIGHT_MOVES: [(i32, i32); 8] = [
    (-2, -1), (-1, -2), (1, -2), (2, -1),
    (-2, 1), (-1, 2), (1, 2), (2, 1)
];

const KING_MOVES: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (1, 1),
    (0, -1), (0, 1),
    (1, -1), (1, 0), (1, 1)
];

#[derive(Debug, Copy, Clone)]
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
    pub fn all(board: &mut Board) -> Vec<ChessMove> {
        let (queen, king): (u64, u64) = match board.get_turn() {
            Player::White => (board.white[PieceKind::Queen as usize], board.white[PieceKind::King as usize]),
            Player::Black => (board.black[PieceKind::Queen as usize], board.black[PieceKind::King as usize]),
        };

        let mut moves: Vec<ChessMove> = Vec::new();
        moves.extend(Self::pawns(board));
        moves.extend(Self::knights(board));
        moves.extend(Self::bishops(board));
        moves.extend(Self::rooks(board));
        moves.extend(Self::queen(board, Board::u64_to_row_col(queen)));
        moves.extend(Self::king(board, Board::u64_to_row_col(king)));

        moves
    }

    pub fn pawn(board: &mut Board, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let friends = board.get_occupied(board.get_turn());

        let mut moves: Vec<ChessMove> = Vec::new();

        match board.get_turn() {
            Player::White => {
                // Straight single tile moves
                if !board.is_occupied((x - 1, y)) {
                    moves.push(
                        ChessMove::new(
                            Board::row_col_to_index(x, y),
                            Board::row_col_to_index(x - 1, y),
                            PieceKind::Pawn,
                            board.get_turn()
                    ));
                   
                    // Straight doulbe tile moves
                    if x == 6 && !board.is_occupied((x - 2, y)) {
                        moves.push(
                            ChessMove::new(
                                Board::row_col_to_index(x, y),
                                Board::row_col_to_index(x - 2, y),
                                PieceKind::Pawn,
                                board.get_turn()
                            ));
                    }
                }

                // Murders
                for &(to_x, to_y) in &[(x - 1, y - 1), (x - 1, y + 1)] {
                    if board.is_valid((to_x, to_y), friends) && board.is_occupied((to_x, to_y)) {
                        moves.push(
                            ChessMove::new(
                                Board::row_col_to_index(x, y),
                                Board::row_col_to_index(to_x, to_y),
                                PieceKind::Pawn,
                                board.get_turn()
                        ));
                    }
                }
            },
            Player::Black => {
                // Straight single tile moves
                if !board.is_occupied((x + 1, y)) {
                    moves.push(
                        ChessMove::new(
                            Board::row_col_to_index(x, y),
                            Board::row_col_to_index(x + 1, y),
                            PieceKind::Pawn,
                            board.get_turn()
                    ));
                   
                    // Straight doulbe tile moves
                    if x == 1 && !board.is_occupied((x + 2, y)) {
                        moves.push(
                            ChessMove::new(
                                Board::row_col_to_index(x, y),
                                Board::row_col_to_index(x + 2, y),
                                PieceKind::Pawn,
                                board.get_turn()
                            ));
                    }
                }

                // Murders
                for &(to_x, to_y) in &[(x + 1, y - 1), (x + 1, y + 1)] {
                    if board.is_valid((to_x, to_y), friends) && board.is_occupied((to_x, to_y)) {
                        moves.push(
                            ChessMove::new(
                                Board::row_col_to_index(x, y),
                                Board::row_col_to_index(to_x, to_y),
                                PieceKind::Pawn,
                                board.get_turn()
                        ));
                    }
                }
            }
        }

        moves
    }

    pub fn pawns(board: &mut Board) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let pawns = match board.get_turn() {
            Player::White => board.white[PieceKind::Pawn as usize],
            Player::Black => board.black[PieceKind::Pawn as usize]
        };
        
        for i in 0..u64::BITS {
            if (pawns >> i) & 0b1 != 0 {
                moves.extend(Self::pawn(board, Board::index_to_row_col(i as i32)));
            }
        }

        moves
    }

    pub fn knight(board: &mut Board, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let friends = board.get_occupied(board.get_turn());

        let mut moves: Vec<ChessMove> = Vec::new();

        for &(dx, dy) in &KNIGHT_MOVES {
            let (to_x, to_y) = (x + dx, y + dy);
            if board.is_valid((to_x, to_y), friends) {
                moves.push(
                    ChessMove::new(
                        Board::row_col_to_index(x, y),
                        Board::row_col_to_index(to_x, to_y),
                        PieceKind::Knight,
                        board.get_turn()
                ));
            }
        }

        moves
    }

    pub fn knights(board: &mut Board) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let knights = match board.get_turn() {
            Player::White => board.white[PieceKind::Knight as usize],
            Player::Black => board.black[PieceKind::Knight as usize]
        };
        
        for i in 0..u64::BITS {
            if (knights >> i) & 0b1 != 0 {
                moves.extend(Self::knight(board, Board::index_to_row_col(i as i32)));
            }
        }

        moves
    }

    pub fn bishop(board: &mut Board, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let dirs = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        let friends = board.get_occupied(board.get_turn());
        let mut moves: Vec<ChessMove> = Vec::new();

        for (dx, dy) in dirs {
            let (mut to_x, mut to_y) = (x, y);
            loop {
                to_x += dx;
                to_y += dy;

                if !Board::in_bounds((to_x, to_y)) {
                    break;
                }

                if !board.is_occupied((to_x, to_y)) {
                    moves.push(
                        ChessMove::new(
                            Board::row_col_to_index(x, y),
                            Board::row_col_to_index(to_x, to_y),
                            PieceKind::Bishop,
                            board.get_turn()
                    ));
                } else {
                    if Board::row_col_to_u64(to_x, to_y) & friends == 0 {
                        moves.push(
                            ChessMove::new(
                                Board::row_col_to_index(x, y),
                                Board::row_col_to_index(to_x, to_y),
                                PieceKind::Bishop,
                                board.get_turn()
                        ));
                        break;
                    } else {
                        break;
                    }
                }
            }
        }

        moves
    }

    pub fn bishops(board: &mut Board) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let knights = match board.get_turn() {
            Player::White => board.white[PieceKind::Bishop as usize],
            Player::Black => board.black[PieceKind::Bishop as usize]
        };
        
        for i in 0..u64::BITS {
            if (knights >> i) & 0b1 != 0 {
                moves.extend(Self::bishop(board, Board::index_to_row_col(i as i32)));
            }
        }

        moves
    }

    pub fn rook(board: &mut Board, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let friends = board.get_occupied(board.get_turn());
        let mut moves: Vec<ChessMove> = Vec::new();

        for (dx, dy) in dirs {
            let (mut to_x, mut to_y) = (x, y);
            loop {
                to_x += dx;
                to_y += dy;

                if !Board::in_bounds((to_x, to_y)) {
                    break;
                }

                if !board.is_occupied((to_x, to_y)) {
                    moves.push(
                        ChessMove::new(
                            Board::row_col_to_index(x, y),
                            Board::row_col_to_index(to_x, to_y),
                            PieceKind::Rook,
                            board.get_turn()
                    ));
                } else {
                    if Board::row_col_to_u64(to_x, to_y) & friends == 0 {
                        moves.push(
                            ChessMove::new(
                                Board::row_col_to_index(x, y),
                                Board::row_col_to_index(to_x, to_y),
                                PieceKind::Rook,
                                board.get_turn()
                        ));
                        break;
                    } else {
                        break;
                    }
                }
            }
        }

        moves
    }

    pub fn rooks(board: &mut Board) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let rooks = match board.get_turn() {
            Player::White => board.white[PieceKind::Rook as usize],
            Player::Black => board.black[PieceKind::Rook as usize]
        };
        
        for i in 0..u64::BITS {
            if (rooks >> i) & 0b1 != 0 {
                moves.extend(Self::rook(board, Board::index_to_row_col(i as i32)));
            }
        }

        moves
    }

    pub fn queen(board: &mut Board, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0),
                    (-1, -1), (-1, 1), (1, -1), (1, 1)];
        let friends = board.get_occupied(board.get_turn());
        let mut moves: Vec<ChessMove> = Vec::new();

        for (dx, dy) in dirs {
            let (mut to_x, mut to_y) = (x, y);
            loop {
                to_x += dx;
                to_y += dy;

                if !Board::in_bounds((to_x, to_y)) {
                    break;
                }

                if !board.is_occupied((to_x, to_y)) {
                    moves.push(
                        ChessMove::new(
                            Board::row_col_to_index(x, y),
                            Board::row_col_to_index(to_x, to_y),
                            PieceKind::Queen,
                            board.get_turn()
                    ));
                } else {
                    if Board::row_col_to_u64(to_x, to_y) & friends == 0 {
                        moves.push(
                            ChessMove::new(
                                Board::row_col_to_index(x, y),
                                Board::row_col_to_index(to_x, to_y),
                                PieceKind::Queen,
                                board.get_turn()
                        ));
                        break;
                    } else {
                        break;
                    }
                }
            }
        }

        moves
    }

    // BUG: King can't kill top right btw
    // The same to knight - could combine into new function, perhaps
    pub fn king(board: &mut Board, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let friends = board.get_occupied(board.get_turn());

        let mut moves: Vec<ChessMove> = Vec::new();

        for &(dx, dy) in &KING_MOVES {
            let (to_x, to_y) = (x + dx, y + dy);
            if board.is_valid((to_x, to_y), friends) {
                moves.push(
                    ChessMove::new(
                        Board::row_col_to_index(x, y),
                        Board::row_col_to_index(to_x, to_y),
                        PieceKind::King,
                        board.get_turn()
                ));
            }
        }

        moves
    }

    pub fn piece_at(board: &mut Board, coords: (i32, i32)) -> Vec<ChessMove> {
        match board.at(coords).unwrap().kind {
            PieceKind::Pawn => Self::pawn(board, coords),
            PieceKind::Knight => Self::knight(board, coords),
            PieceKind::Bishop => Self::bishop(board, coords),
            PieceKind::Rook => Self::rook(board, coords),
            PieceKind::Queen => Self::queen(board, coords),
            PieceKind::King => Self::king(board, coords)
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
