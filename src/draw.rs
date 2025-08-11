use crate::{WIDTH, HEIGHT};
use crate::board::Board;
use crate::piece::{Player, PieceKind, N_PIECES};
use crate::movegen::MoveGen;

use raylib::prelude::*;

// This exists in the raylib documentation, but is not defined for me, so I am defining it myself
pub fn check_collision_point_rec(v: Vector2, rect: Rectangle) -> bool {
    v.x >= rect.x && v.x <= (rect.x + rect.width) && v.y >= rect.y && v.y <= (rect.y + rect.height)
}

fn mark_tile(d: &mut RaylibDrawHandle, row: i32, col: i32, colour: Color) {
    let tile_dim = WIDTH / 8;
    d.draw_rectangle(tile_dim * col, tile_dim * row, tile_dim, tile_dim, colour);
}

fn draw_tiles(d: &mut RaylibDrawHandle, board: &mut Board) {
    let tile_dim = WIDTH / 8;

    for row in 0..8 {
        for col in 0..8 {
            let colour = if (row + col) % 2 == 0 { Color::from_hex("fceee3").unwrap() } else { Color::from_hex("58391d").unwrap() };
            d.draw_rectangle(col * tile_dim, row * tile_dim, tile_dim, tile_dim, colour);
        }
    }

    for row in 0..8 {
        for col in 0..8 {
            if board.is_selected() && Board::index_to_row_col(board.get_selected().index) == (row, col) {
                mark_tile(d, row, col, Color::from_hex("ff0f5f").unwrap().fade(0.7));

                for m in MoveGen::piece_at(board, (row, col)) {
                    let coords = Board::index_to_row_col(m.to);
                    mark_tile(d, coords.0, coords.1, Color::from_hex("ff0f5f").unwrap().fade(0.7));
                }
            }
        }
    }
}

fn draw_pieces(d: &mut RaylibDrawHandle, board: &Board, black_textures: &Vec<Texture2D>, white_textures: &Vec<Texture2D>) {
    let tile_dim = WIDTH as f32 / 8.0;
    for piece in PieceKind::iterator() {
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

fn draw_bits(d: &mut RaylibDrawHandle, board: &Board) {
    if board.bits.is_none() {
        return;
    }

    let tile_dim = WIDTH / 8;
    for row in 0..8 {
        for col in 0..8 {
            let (bit, tint) = match (board.bits.unwrap() >> (row * 8 + col)) & 1u64 {
                1u64 => (1, Color::LIME.fade(0.7)),
                _default => (0, Color::PINK.fade(0.0))
            };

            mark_tile(d, row, col, tint);
            d.draw_text(&bit.to_string(), col * tile_dim + 40, row * tile_dim + 35, 40, Color::WHITE);
        }
    }
}

fn draw_promotion_ui(d: &mut RaylibDrawHandle, board: &mut Board, mouse: Vector2,black_textures: &Vec<Texture2D>, white_textures: &Vec<Texture2D>) {
    let width = WIDTH / 8 * (N_PIECES as i32 - 2);
    let tile_dim = HEIGHT / 8;
    let x = WIDTH / 2 - width / 2;
    let y = HEIGHT / 2 - tile_dim / 2;
    let margin = 7.5;

    d.draw_rectangle(x, y, width, tile_dim, Color::WHITESMOKE);

    let rect = Rectangle::new(x as f32 - margin, y as f32 - margin, width as f32 + margin * 2.0, tile_dim as f32 + margin * 2.0);
    d.draw_rectangle_lines_ex(rect, margin, Color::BLACK);

    let mut i = -1;
    for &piece in PieceKind::iterator() {
        if piece == PieceKind::Pawn || piece == PieceKind::King {
            i += 1;
            continue;
        }

        // Mouse hover effect
        let piece_rect = Rectangle::new(
            (x + tile_dim * i) as f32,
            y as f32,
            tile_dim as f32,
            tile_dim as f32 
        );
        if check_collision_point_rec(mouse, piece_rect) {
            d.draw_rectangle_rec(piece_rect, Color::PLUM);
        }

        // Draw promotion pieces
        let texture = match board.get_turn() {
            // NOTE: These are opposite because apply_move() calls swap_turns()
            Player::White => black_textures.get(piece as usize).unwrap(),
            Player::Black => white_textures.get(piece as usize).unwrap()
        };
        
        d.draw_texture_ex(
            texture,
            Vector2::new((x + tile_dim * i) as f32, y as f32),
            0.00, // Rotation
            0.053,  // Scale
            Color::WHITE);

        i += 1;
    }
}

pub fn draw_menu(rl: &mut RaylibHandle, thread: &RaylibThread, board: &mut Board) {
    todo!()
}

pub fn draw(rl: &mut RaylibHandle, thread: &RaylibThread, board: &mut Board, mouse: Vector2, black_textures: &Vec<Texture2D>, white_textures: &Vec<Texture2D>, show_bits: bool) {
    let mut d = rl.begin_drawing(thread);
    draw_tiles(&mut d, board);
    draw_pieces(&mut d, board, black_textures, white_textures);
    if show_bits {
        draw_bits(&mut d, board);
    }
    if board.promoting.is_some() {
        draw_promotion_ui(&mut d, board, mouse, black_textures, white_textures);
    }
}
