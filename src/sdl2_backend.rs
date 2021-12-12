extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use crate::game_renderer::GameRenderer;
use crate::game_renderer::TetriminoType;

pub struct Sdl2Backend<'a> {
    canvas: &'a mut Canvas<Window>,
    block_width: u32,
}

impl Sdl2Backend<'_> {
    pub fn new(canvas: &mut Canvas<Window>, block_width: u32) -> Sdl2Backend {

        Sdl2Backend {
            canvas,
            block_width
        }
    }
}
impl GameRenderer for Sdl2Backend<'_> {
    fn draw_block(&mut self, x: i32, y: i32, tetrimino_type: TetriminoType) {
        match tetrimino_type {
            TetriminoType::LiveTetrimino => {
                self.canvas.set_draw_color(Color::RGB(0, 0, 200));
            },
            TetriminoType::SettledTetrimino => {
                self.canvas.set_draw_color(Color::RGB(127, 127, 127));
            },
        };

        let rect = Rect::new(x * self.block_width as i32,
                             y * self.block_width as i32,
                             self.block_width,
                             self.block_width);

        self.canvas.fill_rect(rect);
    }
}
