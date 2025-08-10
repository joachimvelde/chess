use crate::piece::{Piece, PieceKind, N_PIECES, Player};
use crate::movegen::{ChessMove, MoveGen};
use std::collections::HashMap;
use std::fmt;

// Use the PieceKind enum to index the correct boards
pub struct Board {
    pub black: [u64; N_PIECES],
    pub white: [u64; N_PIECES],
    turn: Player,
    pub white_castling_k: bool, // Castling rights
    pub white_castling_q: bool,
    pub black_castling_k: bool,
    pub black_castling_q: bool,
    pub en_passant_target: Option<i32>, // Board index
    pub halfmove_clock: i32,
    pub fullmove_number: i32,

    // For drawing
    selected_piece: Option<Piece>,
    pub bits: Option<u64>,
    pub promoting: Option<u64>
}

impl Board {
    pub fn new() -> Self {
        Self {
            black: [0; N_PIECES],
            white: [0; N_PIECES],
            turn: Player::White,
            white_castling_k: false,
            white_castling_q: false,
            black_castling_k: false,
            black_castling_q: false,
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 0,
            selected_piece: None,
            bits: None,
            promoting: None
        }
    }

    pub fn set(&mut self, piece: PieceKind, is_white: bool, pos: u64) {
        if is_white {
            self.white[piece as usize] |= pos;
        } else {
            self.black[piece as usize] |= pos;
        }
    }

    pub fn reset(&mut self) {
        self.black = [0; N_PIECES];
        self.white = [0; N_PIECES];
        self.deselect();
        self.promoting = None;
        self.apply_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    }

    pub fn apply_fen(&mut self, fen: String) {
        let letter_to_piece = HashMap::from([
            ("p", PieceKind::Pawn),
            ("n", PieceKind::Knight),
            ("b", PieceKind::Bishop),
            ("r", PieceKind::Rook),
            ("q", PieceKind::Queen),
            ("k", PieceKind::King),
        ]);

        // Chess uses rank which is just row, but backwards.
        // I will use normal rows and columns where top left is (0, 0)
        let (mut row, mut col) = (0, 0);
        let mut index: usize = 0;

        // PieceKind positions
        for c in fen.chars() {
            index += 1;
            match c {
                '/' => {
                    row += 1;
                    col = 0;
                },
                '1'..='8' => col += c.to_digit(10).unwrap(), // Empty squares
                'a'..='z' | 'A'..='Z' => {
                    let pos = 1_u64 << (row * 8 + col);
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
                    col += 1;
                },
                ' ' => break,
                _ => ()
            }
        }

        // Player turn
        match fen.chars().nth(index).unwrap() {
            'w' => self.turn = Player::White,
            'b' => self.turn = Player::Black,
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
        if fen.chars().nth(index).unwrap() == '-' {
            self.en_passant_target = None;
            index += 2;
        } else {
            let square = &fen[index..index + 1];
            let rank =  8 - (square.chars().nth(1).unwrap() as i32 - 'a' as i32);
            let file = square.chars().nth(0).unwrap() as i32 - 'a' as i32;
            self.en_passant_target = Some(Self::row_col_to_index(rank, file));
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

    pub fn is_game_over(&self) -> bool {
        self.white[PieceKind::King as usize] == 0 || self.black[PieceKind::King as usize] == 0
    }

    pub fn at(&self, coords: (i32, i32)) -> Option<Piece> {
        let bit = Board::row_col_to_u64(coords.0, coords.1);

        for player in [Player::Black, Player::White] {
            let boards = match player {
                Player::Black => self.black,
                Player::White => self.white
            };

            for (i, &board) in boards.iter().enumerate() {
                if board & bit != 0 {
                    let kind = match i {
                        0 => PieceKind::Pawn,
                        1 => PieceKind::Knight,
                        2 => PieceKind::Bishop,
                        3 => PieceKind::Rook,
                        4 => PieceKind::Queen,
                        5 => PieceKind::King,
                        _ => unreachable!()
                    };

                    return Some(Piece::new(player, kind, Self::row_col_to_index(coords.0, coords.1)));
                }
            }
        }

        None
    }

    pub fn get_occupied(&self, player: Player) -> u64 {
        let mut pieces: u64 = 0;

        for &kind in PieceKind::iterator() {
            match player {
                Player::White => pieces |= self.white[kind as usize],
                Player::Black => pieces |= self.black[kind as usize]
            }
        }

        pieces
    }

    pub fn get_empty(&self) -> u64 {
        !(self.get_occupied(Player::White) | self.get_occupied(Player::Black))
    }

    pub fn select(&mut self, coords: (i32, i32)) {
        self.selected_piece = self.at(coords);
    }

    pub fn deselect(&mut self) {
        self.selected_piece = None;
        self.bits = None;
    }

    pub fn is_selected(&self) -> bool {
        return self.selected_piece.is_some();
    }

    pub fn get_selected(&self) -> Piece {
        assert!(self.is_selected());
        return self.selected_piece.unwrap();
    }

    pub fn promote_to(&mut self, kind: PieceKind) {
        assert!(self.promoting.is_some());

        let piece = self.at(Self::u64_to_row_col(self.promoting.unwrap())).unwrap();

        match piece.player {
            Player::White => {
                self.white[kind as usize] |= self.promoting.unwrap();
                self.white[PieceKind::Pawn as usize] &= !self.promoting.unwrap();
            },
            Player::Black => {
                self.black[kind as usize] |= self.promoting.unwrap();
                self.black[PieceKind::Pawn as usize] &= !self.promoting.unwrap();
            }
        }

        self.promoting = None;
    }

    pub fn opponent(player: Player) -> Player {
        match player {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }

    pub fn index_to_row_col(pos: i32) -> (i32, i32) {
        return (pos / 8, pos % 8);
    }

    pub fn row_col_to_index(row: i32, col: i32) -> i32 {
        return row * 8 + col;
    }

    pub fn get_rows_and_cols(bitboard: u64) -> Vec<(i32, i32)> {
        let mut res: Vec<(i32, i32)> = Vec::new();
        for pos in 0..64 {
            if (bitboard & (1_u64 << pos)) != 0 {
                res.push(Self::index_to_row_col(pos));
            }
        }
        return res;
    }

    pub fn index_to_u64(index: i32) -> u64 {
        1_u64 << index
    }

    pub fn u64_to_index(x: u64) -> i32 {
        x.trailing_zeros() as i32
    }

    pub fn u64_to_row_col(x: u64) -> (i32, i32) {
        Self::index_to_row_col(Self::u64_to_index(x))
    }

    pub fn row_col_to_u64(row: i32, col: i32) -> u64 {
        Self::index_to_u64(Self::row_col_to_index(row, col))
    }

    pub fn get_turn(&self) -> Player {
        self.turn
    }

    pub fn swap_turns(&mut self) {
        match self.turn {
            Player::White => self.turn = Player::Black,
            Player::Black => self.turn = Player::White
        }
    }

    pub fn apply_move(&mut self, m: ChessMove) {
        self.en_passant_target = None;

        // Handle kills
        let victim = self.at(Self::index_to_row_col(m.to));

        // Check for en passant capture
        if m.kind == PieceKind::Pawn && victim.is_none() {
            let (from_row, from_col) = Self::index_to_row_col(m.from);
            let (_, to_col) = Self::index_to_row_col(m.to);

            // If pawn moved diagonally two squares, it's an en passant move
            // NOTE: Maybe we could store this in the ChessMove struct instead?
            if from_col != to_col {
                let kill_row = from_row;
                let kill_index = Self::row_col_to_index(kill_row, to_col);

                // Kill the pawn
                match Self::opponent(m.player) {
                    Player::White => self.white[PieceKind::Pawn as usize] ^= 1u64 << kill_index,
                    Player::Black => self.black[PieceKind::Pawn as usize] ^= 1u64 << kill_index,
                }
            }
        }

        // Normal captures
        if victim.is_some() {
            match victim.unwrap().player {
                Player::White => self.white[victim.unwrap().kind as usize] ^= 1u64 << m.to,
                Player::Black => self.black[victim.unwrap().kind as usize] ^= 1u64 << m.to,
            }
        }

        // Move the piece
        match m.player {
            Player::White => self.white[m.kind as usize] ^= 1_u64 << m.from | 1_u64 << m.to,
            Player::Black => self.black[m.kind as usize] ^= 1_u64 << m.from | 1_u64 << m.to
        }

        // Set en passant target if a pawn moves doubly
        if m.kind == PieceKind::Pawn {
            let (from_row, from_col) = Self::index_to_row_col(m.from);
            let (to_row, _) = Self::index_to_row_col(m.to);

            if (from_row - to_row).abs() == 2 {
                let en_passant_row = (from_row + to_row) / 2;
                self.en_passant_target = Some(Self::row_col_to_index(en_passant_row, from_col));
            }
        }

        // Move the rook if castling
        if m.is_castling() {
            let king_to = Self::index_to_row_col(m.to);

            // Kingside
            match (m.player, king_to) {
                (Player::White, (7, 6)) => {
                    let from = Self::row_col_to_index(7, 7);
                    let to = Self::row_col_to_index(7, 5);
                    self.white[PieceKind::Rook as usize] ^= 1_u64 << from | 1_u64 << to;
                }
                _ => ()
            }

            // Queenside
            match (m.player, king_to) {
                (Player::White, (7, 2)) => {
                    let from = Self::row_col_to_index(7, 0);
                    let to = Self::row_col_to_index(7, 3);
                    self.white[PieceKind::Rook as usize] ^= 1_u64 << from | 1_u64 << to;
                }
                _ => ()
            }

            // Kingside
            match (m.player, king_to) {
                (Player::Black, (0, 6)) => {
                    let from = Self::row_col_to_index(0, 7);
                    let to = Self::row_col_to_index(0, 5);
                    self.black[PieceKind::Rook as usize] ^= 1_u64 << from | 1_u64 << to;
                }
                _ => ()
            }

            // Queenside
            match (m.player, king_to) {
                (Player::Black, (0, 2)) => {
                    let from = Self::row_col_to_index(0, 0);
                    let to = Self::row_col_to_index(0, 3);
                    self.black[PieceKind::Rook as usize] ^= 1_u64 << from | 1_u64 << to;
                }
                _ => ()
            }
        }

        // Promotions
        match (m.player, m.kind, Board::index_to_row_col(m.to).0) {
            (Player::White, PieceKind::Pawn, 0) | (Player::Black, PieceKind::Pawn, 7) => self.promoting = Some(Board::index_to_u64(m.to)),
            _  => ()
        }

        // Castling rights
        match (m.kind, m.player) {
            (PieceKind::King, Player::White) => {
                self.white_castling_k = false;
                self.white_castling_q = false;
            },
            (PieceKind::Rook, Player::White) => {
                match Self::index_to_row_col(m.from) {
                    (7, 0) => self.white_castling_q = false,
                    (7, 7)=> self.white_castling_k = false,
                    _ => (),
                }
            }
            (PieceKind::King, Player::Black) => {
                self.black_castling_k = false;
                self.black_castling_q = false;
            },
            (PieceKind::Rook, Player::Black) => {
                match Self::index_to_row_col(m.from) {
                    (0, 0) => self.black_castling_q = false,
                    (0, 7)=> self.black_castling_k = false,
                    _ => (),
                }
            },
            _ => (),
        }

        // Cannot castle without a rook
        if victim.is_some() {
            if victim.unwrap().kind == PieceKind::Rook {
                match (victim.unwrap().player, Self::index_to_row_col(m.to)) {
                    (Player::White, (7, 0)) => self.white_castling_q = false,
                    (Player::White, (7, 7)) => self.white_castling_k = false,
                    (Player::Black, (0, 0)) => self.black_castling_q = false,
                    (Player::Black, (0, 7)) => self.black_castling_k = false,
                    _ => (),
                }
            }
        }

        self.swap_turns();
    }

    pub fn is_valid(&self, (x, y): (i32, i32), friends: u64) -> bool {
        Self::in_bounds((x, y)) && Self::row_col_to_u64(x, y) & friends == 0
    }

    pub fn in_bounds(coords: (i32, i32)) -> bool {
        coords.0 >= 0 && coords.0 <= 7 && coords.1 >= 0 && coords.1 <= 7
    }

    pub fn is_occupied(&self, coords: (i32, i32)) -> bool {
        if !Self::in_bounds(coords) {
            return true;
        }

        let bits = Board::row_col_to_u64(coords.0, coords.1);
        for piece in PieceKind::iterator() {
            if (self.black[*piece as usize] & bits) != 0 || (self.white[*piece as usize] & bits) != 0 {
                return true;
            }
        }

        return false;
    }
    
    // Right side
    pub fn can_castle_kingside(&mut self) -> bool {
        let player = self.get_turn();

        match player {
            Player::White => {
                self.white_castling_k &&
                    self.is_path_clear_castling(player, true) &&
                    !self.is_king_in_check(player) &&
                    !self.is_square_attacked_by((7, 5), Player::Black) &&
                    !self.is_square_attacked_by((7, 6), Player::Black)

            },
            Player::Black => {
                self.black_castling_k &&
                    self.is_path_clear_castling(player, true) &&
                    !self.is_king_in_check(player) &&
                    !self.is_square_attacked_by((0, 5), Player::White) &&
                    !self.is_square_attacked_by((0, 6), Player::White)
            },
        }
    }

    // Left side
    pub fn can_castle_queenside(&mut self) -> bool {
        let player = self.get_turn();
        match player {
            Player::White => {
                self.white_castling_q &&
                    self.is_path_clear_castling(player, false) &&
                    !self.is_king_in_check(player) &&
                    !self.is_square_attacked_by((7, 2), Player::Black) &&
                    !self.is_square_attacked_by((7, 3), Player::Black)
            },
            Player::Black => {
                self.black_castling_q &&
                    self.is_path_clear_castling(player, false) &&
                    !self.is_king_in_check(player) &&
                    !self.is_square_attacked_by((0, 2), Player::White) &&
                    !self.is_square_attacked_by((0, 3), Player::White)
            },
        }
    }

    pub fn is_path_clear_castling(&self, player: Player, kingside: bool) -> bool {
        let all = !self.get_empty();

        let path_mask = match (player, kingside) {
            (Player::White, true) =>0x6000000000000000,
            (Player::White, false) =>0x0e00000000000000,
            (Player::Black, true) => 0x60,
            (Player::Black, false) => 0x0e,
        };
        
        (all & path_mask) == 0
    }

    fn is_king_in_check(&mut self, player: Player) -> bool {
        let king_board = match player {
            Player::White => self.white[PieceKind::King as usize],
            Player::Black => self.black[PieceKind::King as usize],
        };

        let king = Self::u64_to_row_col(king_board);

        self.is_square_attacked_by(king, Self::opponent(player))
    }

    // NOTE: Generates all available moves - can be optimised
    fn is_square_attacked_by(&mut self, (row, col): (i32, i32), player: Player) -> bool {
        let mut all = MoveGen::all_no_castling(self, player);
        all.extend(MoveGen::pawn_attacks(self, player));

        all.iter()
            .any(|x| x.to == Self::row_col_to_index(row, col))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn format_bitboard(bitboard: u64) -> String {
            let mut output = String::new();
            for row in (0..8).rev() {
                for col in 0..8 {
                    let pos = 1 << (row * 8 + col);
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
        write!(f, "turn: {}\n", self.turn)?;
        write!(f, "white_castling_k: {}\n", self.white_castling_k)?;
        write!(f, "white_castling_q: {}\n", self.white_castling_q)?;
        write!(f, "black_castling_k: {}\n", self.black_castling_k)?;
        write!(f, "black_castling_q: {}\n", self.black_castling_q)?;
        write!(f, "en_passant_target: {}\n", self.en_passant_target.unwrap_or(-1))?;
        write!(f, "halfmove_clock: {}\n", self.halfmove_clock)?;
        write!(f, "fullmove_number: {}\n", self.fullmove_number)?;

        write!(f, "\nBlack bitboards")?;
        for (i, bb) in self.black.iter().enumerate() {
            write!(f, "\nPieceKind {}: \n{}", i, format_bitboard(*bb))?;
        }

        write!(f, "\nWhite bitboards")?;
        for (i, bb) in self.white.iter().enumerate() {
            write!(f, "\nPieceKind {}: \n{}", i, format_bitboard(*bb))?;
        }

        Ok(())
    }
}
