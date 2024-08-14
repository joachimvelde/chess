use std::fmt;

pub const N_PIECES: usize = 6;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Player {
    Black,
    White
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub player: Player,
    pub kind: PieceKind,
    pub index: i32, // The piece's index in the u64
}

impl PieceKind {
    pub fn iterator() -> std::slice::Iter<'static, PieceKind> {
        static PIECES: [PieceKind; N_PIECES] = [
            PieceKind::Pawn,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Rook,
            PieceKind::Queen,
            PieceKind::King
        ];

        PIECES.iter()
    }
}

impl Piece {
    pub fn new(player: Player, kind: PieceKind, index: i32) -> Self {
        Self { player, kind, index }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::Black => write!(f, "Black"),
            Player::White => write!(f, "White")
        }
    }
}
