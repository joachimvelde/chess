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

impl Piece {
    pub fn iterator() -> std::slice::Iter<'static, Piece> {
        static PIECES: [Piece; N_PIECES] = [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King
        ];

        return PIECES.iter();
    }
}
