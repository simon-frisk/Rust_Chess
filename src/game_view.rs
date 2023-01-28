use std::path::Path;

use graphics::{Context, DrawState, Graphics, Image, Rectangle};
use opengl_graphics::{Texture, TextureSettings};
use piston::{Button, GenericEvent, MouseButton};

use crate::{
    game_state::GameState,
    piece::{Piece, PieceType},
    CELL_SIZE,
};

const DARK_COLOR: [f32; 4] = [0.2, 0.0, 0.0, 1.0];
const LIGHT_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];

pub struct GameView<'a> {
    game_state: &'a mut GameState,
    mouse_pos: [f64; 2],
}

impl<'a> GameView<'a> {
    pub fn new(game_state: &'a mut GameState) -> GameView {
        GameView {
            game_state,
            mouse_pos: [0.0, 0.0],
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.mouse_pos = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let x = (self.mouse_pos[0] / CELL_SIZE) as u8;
            let y = (self.mouse_pos[1] / CELL_SIZE) as u8;

            self.game_state.selected_piece_index = None;
            for (i, piece) in self.game_state.pieces.iter().enumerate() {
                if piece.pos.x == x && piece.pos.y == y {
                    self.game_state.selected_piece_index = Some(i);
                    break;
                }
            }
        }
    }

    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        for x in 0..9 {
            for y in 0..9 {
                let mut color = if (x + y) % 2 == 0 {
                    LIGHT_COLOR
                } else {
                    DARK_COLOR
                };
                if let Some(selected_piece_index) = self.game_state.selected_piece_index {
                    let selected_piece = &self.game_state.pieces[selected_piece_index];
                    if selected_piece.pos.x == x && selected_piece.pos.y == y {
                        color = [0.0, 1.0, 0.0, 1.0];
                    }
                }
                let rect: [f64; 4] = [
                    CELL_SIZE * (x as f64),
                    CELL_SIZE * (y as f64),
                    CELL_SIZE,
                    CELL_SIZE,
                ];

                Rectangle::new(color).draw(rect, &c.draw_state, c.transform, g);
            }
        }
        for (i, piece) in self.game_state.pieces.iter().enumerate() {
            let i: i8 = i as i8; // Will never have too many pieces

            let rect: [f64; 4] = [
                CELL_SIZE * (piece.pos.x as f64) + CELL_SIZE / 4.0,
                CELL_SIZE * (piece.pos.y as f64) + CELL_SIZE / 4.0,
                CELL_SIZE / 2.0,
                CELL_SIZE / 2.0,
            ];
            Rectangle::new([0.0, 0.0, 1.0, 1.0]).draw(rect, &c.draw_state, c.transform, g);
        }
    }
}
