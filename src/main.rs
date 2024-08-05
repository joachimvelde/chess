mod piece;
mod board;
mod movegen;
mod draw;

use piece::*;
use board::*;
use movegen::*;
use draw::*;

use raylib::prelude::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Chess")
        .build();
    rl.set_target_fps(60);

    // Create the board
    let mut board = Board::new();
    board.apply_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    // load textures
    let black_textures = vec![
        rl.load_texture(&thread, "media/black_pawn.png").unwrap(),
        rl.load_texture(&thread, "media/black_knight.png").unwrap(),
        rl.load_texture(&thread, "media/black_bishop.png").unwrap(),
        rl.load_texture(&thread, "media/black_rook.png").unwrap(),
        rl.load_texture(&thread, "media/black_queen.png").unwrap(),
        rl.load_texture(&thread, "media/black_king.png").unwrap(),
    ];

    let white_textures = vec![
        rl.load_texture(&thread, "media/white_pawn.png").unwrap(),
        rl.load_texture(&thread, "media/white_knight.png").unwrap(),
        rl.load_texture(&thread, "media/white_bishop.png").unwrap(),
        rl.load_texture(&thread, "media/white_rook.png").unwrap(),
        rl.load_texture(&thread, "media/white_queen.png").unwrap(),
        rl.load_texture(&thread, "media/white_king.png").unwrap(),
    ];

    // Main game loop
    while !rl.window_should_close() {
        draw(&mut rl, &thread, &board, &black_textures, &white_textures);

        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
            for m in MoveGen::pawns(&board) {
                board.apply_move(m);
            }
        }
    }
}
