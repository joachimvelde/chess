mod piece;
mod board;
mod movegen;
mod draw;

use piece::*;
use board::*;
use movegen::*;
use draw::*;

use raylib::prelude::*;
use std::env;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

fn main() {
    // Handle command arg to turn on drawing bits
    let mut show_bits = false;
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("--d")) {
        show_bits = true;
    }

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Chess")
        .build();
    rl.set_target_fps(60);

    // Create the board
    let mut board = Board::new();
    // board.reset();
    board.apply_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    // load textures   NOTE: These are way too high res
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
        draw(&mut rl, &thread, &mut board, &black_textures, &white_textures, show_bits);
    }
}

fn update(rl: &RaylibHandle, board: &mut Board) {
    if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_MIDDLE) {
        board.reset();
    }

    //  TODO: make a separate update function for different modes (pvp, pvb, bot-only)
    // Auto-play
    // if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
    //     let moves = MoveGen::all(board);
    //     board.apply_move(moves[0]);
    // }

    if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
        let (row, col) = ((rl.get_mouse_y() as f32 / 100.0).floor() as i32, (rl.get_mouse_x() as f32 / 100.0).floor() as i32);

        if board.promoting {

        } else {
            let piece = board.at((row, col));
            if piece.is_some() && piece.unwrap().player == board.get_turn() {
                board.select((row, col));
            } else if board.is_selected() {
                for m in MoveGen::piece_at(board, Board::index_to_row_col(board.get_selected().index)) {
                    if m.to == Board::row_col_to_index(row, col) {
                        board.apply_move(m);
                    }
                }
                board.deselect();
            }
        }
    }
}
