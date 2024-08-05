use crate::{WIDTH, HEIGHT};
use crate::board::Board;
use crate::piece::{Piece, N_PIECES};

use raylib::prelude::*;

pub fn draw_tiles(d: &mut RaylibDrawHandle) {
    let tile_dim = WIDTH / 8;

    for row in 0..8 {
        for col in 0..8 {
            let colour = if (row + col) % 2 == 0 { Color::from_hex("fceee3").unwrap() } else { Color::from_hex("58391d").unwrap() };
            d.draw_rectangle(col * tile_dim, row * tile_dim, tile_dim, tile_dim, colour);
        }
    }
}

pub fn draw_pieces(d: &mut RaylibDrawHandle, board: &Board, black_textures: &Vec<Texture2D>, white_textures: &Vec<Texture2D>) {
    let tile_dim = WIDTH as f32 / 8.0;
    for piece in Piece::iterator() {
        // White pieces
        for (row, col) in Board::get_rows_and_cols(board.white[*piece as usize]) {
            d.draw_texture_ex(
                white_textures.get(*piece as usize).unwrap(),
                Vector2::new(col as f32 * tile_dim, row as f32 * tile_dim),
                0.00, // Rotation
                0.053,  // Scale
                Color::WHITE);
        }

        // Black pieces
        for (row, col) in Board::get_rows_and_cols(board.black[*piece as usize]) {
            d.draw_texture_ex(
                black_textures.get(*piece as usize).unwrap(),
                Vector2::new(col as f32 * tile_dim, row as f32 * tile_dim),
                0.00,  // Rotation
                0.05,  // Scale
                Color::WHITE);
        }
    }
}

pub fn draw(rl: &mut RaylibHandle, thread: &RaylibThread, board: &Board, black_textures: &Vec<Texture2D>, white_textures: &Vec<Texture2D>) {
    let mut d = rl.begin_drawing(thread);
    draw_tiles(&mut d);
    draw_pieces(&mut d, board, black_textures, white_textures);
}
