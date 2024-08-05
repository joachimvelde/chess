
use crate::piece::{Piece, N_PIECES};
use std::collections::HashMap;
use std::fmt;

// Use the Piece enum to index the correct boards
pub struct Bitboards {
    pub black: [u64; N_PIECES],
    pub white: [u64; N_PIECES],
    pub white_turn: bool,
    pub white_castling_k: bool,
    pub white_castling_q: bool,
    pub black_castling_k: bool,
    pub black_castling_q: bool,
    pub en_passant_target: Option<String>,
    pub halfmove_clock: i32,
    pub fullmove_number: i32
}

impl Bitboards {
    pub fn new() -> Self {
        Self {
            black: [0; N_PIECES],
            white: [0; N_PIECES],
            white_turn: true,
            white_castling_k: false,
            white_castling_q: false,
            black_castling_k: false,
            black_castling_q: false,
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 0
        }
    }

    pub fn set(&mut self, piece: Piece, is_white: bool, pos: u64) {
        if is_white {
            self.white[piece as usize] |= pos;
        } else {
            self.black[piece as usize] |= pos;
        }
    }

    pub fn apply_fen(&mut self, fen: String) {
        let letter_to_piece = HashMap::from([
            ("p", Piece::Pawn),
            ("n", Piece::Knight),
            ("b", Piece::Bishop),
            ("r", Piece::Rook),
            ("q", Piece::Queen),
            ("k", Piece::King),
        ]);

        let (mut rank, mut file) = (7, 0);
        let mut index: usize = 0;

        // Piece positions
        for c in fen.chars() {
            index += 1;
            match c {
                '/' => {
                    rank -= 1;
                    file = 0;
                },
                '1'..='8' => file += c.to_digit(10).unwrap(), // Empty squares
                'a'..='z' | 'A'..='Z' => {
                    let pos = 1_u64 << (rank * 8 + file);
                    let key = c.to_lowercase().to_string();
                    if c.is_lowercase() {
                        if let Some(&piece) = letter_to_piece.get(key.as_str()) {
                            self.set(piece, false, pos);
                        }
                    } else {
                        if let Some(&piece) = letter_to_piece.get(key.as_str()) {
                            self.set(piece, true, pos);
                        }
                    }
                    file += 1;
                },
                ' ' => break,
                _ => ()
            }
        }

        // Player turn
        match fen.chars().nth(index).unwrap() {
            'w' => self.white_turn = true,
            'b' => self.white_turn = false,
            _ => ()
        }
        index += 2; // Skip next space

        // Castling
        while fen.chars().nth(index).unwrap() != ' ' {
            match fen.chars().nth(index).unwrap() {
                'K' => self.white_castling_k = true,
                'Q' => self.white_castling_q = true,
                'k' => self.black_castling_k = true,
                'q' => self.black_castling_q = true,
                _ => ()
            }
            index += 1;
        }
        index += 1; // Skip space

        // En passant target square
        if fen.chars().nth(index).unwrap() == '_' {
            self.en_passant_target = None;
            index += 2;
        } else {
            self.en_passant_target = Some(fen[index..=index+1].to_string());
            index += 3;
        }

        // Halfmove clock
        let end = fen[index..].find(' ').unwrap_or(fen.len() - index) + index;
        self.halfmove_clock = fen[index..end].parse().unwrap_or(0);
        index = end + 1;

        // Fullmove number
        let end = fen[index..].find(' ').unwrap_or(fen.len() - index) + index;
        self.fullmove_number = fen[index..end].parse().unwrap_or(0);
    }
}

impl fmt::Display for Bitboards {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn format_bitboard(bitboard: u64) -> String {
            let mut output = String::new();
            for rank in (0..8).rev() {
                for file in 0..8 {
                    let pos = 1 << (rank * 8 + file);
                    if bitboard & pos != 0 {
                        output.push('1');
                    } else {
                        output.push('0');
                    }
                }
                output.push('\n');
            }
            return output;
        }


        write!(f, "--- [BOARD] ---\n")?;
        write!(f, "white_turn: {}\n", self.white_turn)?;
        write!(f, "white_castling_k: {}\n", self.white_castling_k)?;
        write!(f, "white_castling_q: {}\n", self.white_castling_q)?;
        write!(f, "black_castling_k: {}\n", self.black_castling_k)?;
        write!(f, "black_castling_q: {}\n", self.black_castling_q)?;
        write!(f, "en_passant_target: {}\n", self.en_passant_target.as_deref().unwrap_or("_"))?;
        write!(f, "halfmove_clock: {}\n", self.halfmove_clock)?;
        write!(f, "fullmove_number: {}\n", self.fullmove_number)?;

        write!(f, "\nBlack bitboards")?;
        for (i, bb) in self.black.iter().enumerate() {
            write!(f, "\nPiece {}: \n{}", i, format_bitboard(*bb))?;
        }

        write!(f, "\nWhite bitboards")?;
        for (i, bb) in self.white.iter().enumerate() {
            write!(f, "\nPiece {}: \n{}", i, format_bitboard(*bb))?;
        }

        Ok(())
    }
}
