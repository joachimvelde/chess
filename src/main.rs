mod piece;
mod bitboards;

use bitboards::*;
use piece::*;

fn main() {
    let mut board = Bitboards::new();
    board.apply_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    println!("{}", board);
}
