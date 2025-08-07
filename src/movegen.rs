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
    (-1, -1), (-1, 0), (-1, 1),
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

    pub fn is_castling(&self) -> bool {
        self.kind == PieceKind::King && {
            let from = Board::index_to_row_col(self.from);
            let to = Board::index_to_row_col(self.to);
            (to.1 - from.1).abs() == 2
        }
    }
}

pub struct MoveGen {
}

// NOTE: Does the board have to be mutably borrowed in all these?
impl MoveGen {
    pub fn all(board: &mut Board, player: Player) -> Vec<ChessMove> {
        let (queen, king): (u64, u64) = match player {
            Player::White => (board.white[PieceKind::Queen as usize], board.white[PieceKind::King as usize]),
            Player::Black => (board.black[PieceKind::Queen as usize], board.black[PieceKind::King as usize]),
        };

        let mut moves: Vec<ChessMove> = Vec::new();
        moves.extend(Self::pawns(board, player));
        moves.extend(Self::knights(board, player));
        moves.extend(Self::bishops(board, player));
        moves.extend(Self::rooks(board, player));
        moves.extend(Self::queen(board, player, Board::u64_to_row_col(queen)));
        moves.extend(Self::king(board, player, Board::u64_to_row_col(king)));

        moves
    }

    pub fn all_no_castling(board: &mut Board, player: Player) -> Vec<ChessMove> {
        let (queen, king): (u64, u64) = match player {
            Player::White => (board.white[PieceKind::Queen as usize], board.white[PieceKind::King as usize]),
            Player::Black => (board.black[PieceKind::Queen as usize], board.black[PieceKind::King as usize]),
        };

        let mut moves: Vec<ChessMove> = Vec::new();
        moves.extend(Self::pawns(board, player));
        moves.extend(Self::knights(board, player));
        moves.extend(Self::bishops(board, player));
        moves.extend(Self::rooks(board, player));
        moves.extend(Self::queen(board, player, Board::u64_to_row_col(queen)));
        moves.extend(Self::king_no_castling(board, player, Board::u64_to_row_col(king)));

        moves
    }

    pub fn pawn(board: &mut Board, player: Player, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let friends = board.get_occupied(player);

        let mut moves: Vec<ChessMove> = Vec::new();

        match player {
            Player::White => {
                // Straight single tile moves
                if !board.is_occupied((x - 1, y)) {
                    moves.push(
                        ChessMove::new(
                            Board::row_col_to_index(x, y),
                            Board::row_col_to_index(x - 1, y),
                            PieceKind::Pawn,
                            player,
                    ));
                   
                    // Straight doulbe tile moves
                    if x == 6 && !board.is_occupied((x - 2, y)) {
                        moves.push(
                            ChessMove::new(
                                Board::row_col_to_index(x, y),
                                Board::row_col_to_index(x - 2, y),
                                PieceKind::Pawn,
                                player,
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
                                player,
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
                            player,
                    ));
                   
                    // Straight doulbe tile moves
                    if x == 1 && !board.is_occupied((x + 2, y)) {
                        moves.push(
                            ChessMove::new(
                                Board::row_col_to_index(x, y),
                                Board::row_col_to_index(x + 2, y),
                                PieceKind::Pawn,
                                player,
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
                                player,
                        ));
                    }
                }
            }
        }

        moves
    }

    pub fn pawns(board: &mut Board, player: Player) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let pawns = match player {
            Player::White => board.white[PieceKind::Pawn as usize],
            Player::Black => board.black[PieceKind::Pawn as usize]
        };

        for i in 0..u64::BITS {
            if (pawns >> i) & 0b1 != 0 {
                moves.extend(Self::pawn(board, player, Board::index_to_row_col(i as i32)));
            }
        }

        moves
    }

    // Needed because pawns have special kill moves
    pub fn pawn_attacks(board: &mut Board, player: Player) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();

        let pawns = match player {
            Player::White => board.white[PieceKind::Pawn as usize],
            Player::Black => board.black[PieceKind::Pawn as usize]
        };
        
        for i in 0..u64::BITS {
            if (pawns >> i) & 0b1 != 0 {
                let (row, col) = Board::u64_to_row_col(1u64 << i);

                let attacks = match player {
                    Player::White => vec![(row - 1, col - 1), (row - 1, col + 1)],
                    Player::Black => vec![(row + 1, col - 1), (row + 1, col + 1)],
                };

                for (to_r, to_c) in attacks {
                    if board.is_valid((to_r, to_c), board.get_occupied(player)) {
                        moves.push(ChessMove::new(
                                Board::row_col_to_index(row, col),
                                Board::row_col_to_index(to_r, to_c),
                                PieceKind::Pawn,
                                player,
                        ));
                    }
                }
            }
        }

        moves
    }

    pub fn knight(board: &mut Board, player: Player, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let friends = board.get_occupied(player);

        let mut moves: Vec<ChessMove> = Vec::new();

        for &(dx, dy) in &KNIGHT_MOVES {
            let (to_x, to_y) = (x + dx, y + dy);
            if board.is_valid((to_x, to_y), friends) {
                moves.push(
                    ChessMove::new(
                        Board::row_col_to_index(x, y),
                        Board::row_col_to_index(to_x, to_y),
                        PieceKind::Knight,
                        player,
                    ));
            }
        }

        moves
    }

    pub fn knights(board: &mut Board, player: Player) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let knights = match player {
            Player::White => board.white[PieceKind::Knight as usize],
            Player::Black => board.black[PieceKind::Knight as usize]
        };

        for i in 0..u64::BITS {
            if (knights >> i) & 0b1 != 0 {
                moves.extend(Self::knight(board, player, Board::index_to_row_col(i as i32)));
            }
        }

        moves
    }

    pub fn bishop(board: &mut Board, player: Player, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let dirs = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        let friends = board.get_occupied(player);
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
                            player,
                        ));
                } else {
                    if Board::row_col_to_u64(to_x, to_y) & friends == 0 {
                        moves.push(
                            ChessMove::new(
                                Board::row_col_to_index(x, y),
                                Board::row_col_to_index(to_x, to_y),
                                PieceKind::Bishop,
                                player,
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

    pub fn bishops(board: &mut Board, player: Player) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let knights = match player {
            Player::White => board.white[PieceKind::Bishop as usize],
            Player::Black => board.black[PieceKind::Bishop as usize]
        };

        for i in 0..u64::BITS {
            if (knights >> i) & 0b1 != 0 {
                moves.extend(Self::bishop(board, player, Board::index_to_row_col(i as i32)));
            }
        }

        moves
    }

    pub fn rook(board: &mut Board, player: Player, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let friends = board.get_occupied(player);
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
                            player,
                        ));
                } else {
                    if Board::row_col_to_u64(to_x, to_y) & friends == 0 {
                        moves.push(
                            ChessMove::new(
                                Board::row_col_to_index(x, y),
                                Board::row_col_to_index(to_x, to_y),
                                PieceKind::Rook,
                                player,
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

    pub fn rooks(board: &mut Board, player: Player) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let rooks = match player {
            Player::White => board.white[PieceKind::Rook as usize],
            Player::Black => board.black[PieceKind::Rook as usize]
        };

        for i in 0..u64::BITS {
            if (rooks >> i) & 0b1 != 0 {
                moves.extend(Self::rook(board, player, Board::index_to_row_col(i as i32)));
            }
        }

        moves
    }

    // NOTE: Why do we have the x and y parameters?
    pub fn queen(board: &mut Board, player: Player, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0),
        (-1, -1), (-1, 1), (1, -1), (1, 1)];
        let friends = board.get_occupied(player);
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
                            player,
                        ));
                } else {
                    if Board::row_col_to_u64(to_x, to_y) & friends == 0 {
                        moves.push(
                            ChessMove::new(
                                Board::row_col_to_index(x, y),
                                Board::row_col_to_index(to_x, to_y),
                                PieceKind::Queen,
                                player,
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

    pub fn king_no_castling(board: &mut Board, player: Player, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let friends = board.get_occupied(player);

        let mut moves: Vec<ChessMove> = Vec::new();

        for &(dx, dy) in &KING_MOVES {
            let (to_x, to_y) = (x + dx, y + dy);
            if board.is_valid((to_x, to_y), friends) {
                moves.push(
                    ChessMove::new(
                        Board::row_col_to_index(x, y),
                        Board::row_col_to_index(to_x, to_y),
                        PieceKind::King,
                        player,
                    ));
            }
        }

        moves
    }

    pub fn king(board: &mut Board, player: Player, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let friends = board.get_occupied(player);

        let mut moves: Vec<ChessMove> = Vec::new();

        for &(dx, dy) in &KING_MOVES {
            let (to_x, to_y) = (x + dx, y + dy);
            if board.is_valid((to_x, to_y), friends) {
                moves.push(
                    ChessMove::new(
                        Board::row_col_to_index(x, y),
                        Board::row_col_to_index(to_x, to_y),
                        PieceKind::King,
                        player,
                    ));
            }
        }

        moves.extend(Self::castling(board, (x, y)));

        moves
    }

    pub fn piece_at(board: &mut Board, coords: (i32, i32)) -> Vec<ChessMove> {
        let piece = board.at(coords).unwrap();
        let player = piece.player;

        match board.at(coords).unwrap().kind {
            PieceKind::Pawn => Self::pawn(board, player, coords),
            PieceKind::Knight => Self::knight(board, player, coords),
            PieceKind::Bishop => Self::bishop(board, player, coords),
            PieceKind::Rook => Self::rook(board, player, coords),
            PieceKind::Queen => Self::queen(board, player, coords),
            PieceKind::King => Self::king(board, player, coords)
        }
    }

    /*
     * ----- RULES FOR CASTLING -----
     * 1. King nor rook cannot have previously moved
     * 2. There are no pieces between the king and the rook
     * 3. The king is not in check
     * 4. The king does not pass through or finish on a square that is attacked by an enemy piece
     */
    fn castling(board: &mut Board, (x, y): (i32, i32)) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = vec![];
        let player = board.get_turn();

        if board.can_castle_kingside() {
            let to = match player {
                Player::White => Board::row_col_to_index(7, 6), // g1
                Player::Black => Board::row_col_to_index(0, 6), // g8
            };

            moves.push(ChessMove::new(
                    Board::row_col_to_index(x, y), // Are we misusing x and y coordinates as
                                                   // row/col?
                    to,
                    PieceKind::King,
                    player,
            ));
        }

        if board.can_castle_queenside() {
            let to = match player {
                Player::White => Board::row_col_to_index(7, 2), // c1
                Player::Black => Board::row_col_to_index(0, 2), // c8
            };

            moves.push(ChessMove::new(
                    Board::row_col_to_index(x, y),
                    to,
                    PieceKind::King,
                    player,
            ));
        }

        moves
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
