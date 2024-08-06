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
    // board.apply_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    board.apply_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2".to_string());

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
        update(&rl, &mut board);
        draw(&mut rl, &thread, &board, &black_textures, &white_textures);
    }
}

fn update(rl: &RaylibHandle, board: &mut Board) {
    if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
        let (row, col) = ((rl.get_mouse_y() as f32 / 100.0).floor() as i32, (rl.get_mouse_x() as f32 / 100.0).floor() as i32);

        let piece = board.at((row, col));
        if piece.is_some() && piece.unwrap().player == board.turn {
            board.select((row, col));
        }
    }
}
