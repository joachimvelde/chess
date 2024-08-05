pub const N_PIECES: usize = 6;

#[derive(Copy, Clone, Debug)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}
