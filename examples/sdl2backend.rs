extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::ttf::Font;

extern crate rand;
use rand::Rng;
use fourtris::game::{Game, GameState, Input};
use fourtris::game_renderer::{GameRenderer, TetriminoType};

use std::time::Duration;
use std::path::Path;

// ---------------------------
//         CONSTANTS
// ---------------------------
const PADDING : u32 = 80;
const BLOCK_WIDTH : u32 = 20;
const PLAYFIELD_WIDTH : u32 = BLOCK_WIDTH * 10;
const PLAYFIELD_HEIGHT : u32 = BLOCK_WIDTH * 22;
const WINDOW_WIDTH : u32 = 2 * PADDING + PLAYFIELD_WIDTH;
const WINDOW_HEIGHT : u32 = PLAYFIELD_HEIGHT;


pub struct Randy {
    rng: rand::rngs::ThreadRng
}

impl Randy {
    pub fn new() -> Randy {
        let rng = rand::thread_rng();
        Randy {
            rng
        }
    }
}

impl fourtris::rng::Rng for Randy {
    fn next(&mut self) -> usize {
        self.rng.gen_range(0..7)
    }
}

pub struct Sdl2Backend<'a, 'b> {
    canvas: &'a mut Canvas<Window>,
    font: &'b Font<'b, 'b>,
}

impl Sdl2Backend<'_, '_> {
    pub fn new<'a>(canvas: &'a mut Canvas<Window>, font: &'a Font) -> Sdl2Backend<'a, 'a> {
        Sdl2Backend {
            canvas,
            font,
        }
    }
}

impl GameRenderer for Sdl2Backend<'_, '_> {
    fn draw_board(&mut self) {
        // clear the screen to white
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();


        // draw the playing field
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        let playfield = Rect::new(PADDING as i32,
                                  0,
                                  PLAYFIELD_WIDTH,
                                  PLAYFIELD_HEIGHT);
        self.canvas.fill_rect(playfield).unwrap();

    }

    fn draw_block(&mut self, x: u8, y: u8, tetrimino_type: TetriminoType) {
        match tetrimino_type {
            TetriminoType::I => {
                self.canvas.set_draw_color(Color::RGB(0, 0, 200));
            },
            TetriminoType::O => {
                self.canvas.set_draw_color(Color::RGB(0, 200, 0));
            },
            TetriminoType::J => {
                self.canvas.set_draw_color(Color::RGB(0, 200, 200));
            },
            TetriminoType::L => {
                self.canvas.set_draw_color(Color::RGB(200, 0, 0));
            },
            TetriminoType::S => {
                self.canvas.set_draw_color(Color::RGB(200, 0, 200));
            },
            TetriminoType::Z => {
                self.canvas.set_draw_color(Color::RGB(200, 200, 0));
            },
            TetriminoType::T => {
                self.canvas.set_draw_color(Color::RGB(100, 200, 100));
            },
            TetriminoType::EmptySpace => {
                self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            },
        };

        let real_x = x as i32 * BLOCK_WIDTH  as i32 + PADDING as i32;
        let real_y = y as i32 * BLOCK_WIDTH  as i32;
        let rect = Rect::new(real_x,
                             real_y,
                             BLOCK_WIDTH,
                             BLOCK_WIDTH);

        self.canvas.fill_rect(rect).unwrap();
    }

    // I don't feel like implementing these, but here is where they really belong
    fn draw_score(&mut self, score: u32) {
        // create a texture for the numerical score
        let text_foreground_color = Color::RGB(255, 0, 0);
        let text_background_color = Color::RGB(255, 255, 255);
        let texture_creator = self.canvas.texture_creator();

        // create score texture
        let render_score_string_shaded = self.font.render("SCORE").
            shaded(text_foreground_color, text_background_color).unwrap();
        let score_string_texture = Texture::from_surface(&render_score_string_shaded, &texture_creator).unwrap();
        let score_string_rect = Rect::new((PADDING + PLAYFIELD_WIDTH + 5) as i32,
                                          0,
                                          render_score_string_shaded.width(),
                                          render_score_string_shaded.height());
        // draw the letters
        self.canvas.copy(&score_string_texture, None, Some(score_string_rect)).unwrap();

        // create score texture
        let render_score_value_shaded = self.font.render(&format!("{}", score)).
            shaded(text_foreground_color, text_background_color).unwrap();
        let score_value_texture = Texture::from_surface(&render_score_value_shaded, &texture_creator).unwrap();

        let x_pos = PADDING + PLAYFIELD_WIDTH + 5 + (PADDING - render_score_value_shaded.width())/ 2;
        let score_value_rect = Rect::new(x_pos as i32,
                                         (score_string_rect.height() + 5) as i32,
                                         render_score_value_shaded.width(),
                                         render_score_value_shaded.height());
        self.canvas.copy(&score_value_texture, None, Some(score_value_rect)).unwrap();
    }

    fn draw_level(&mut self, level: usize) {

        // create a texture for the numerical score
        let text_foreground_color = Color::RGB(255, 0, 0);
        let text_background_color = Color::RGB(255, 255, 255);
        let texture_creator = self.canvas.texture_creator();

        // create level texture
        let render_level_string_shaded = self.font.render("LEVEL").
            shaded(text_foreground_color, text_background_color).unwrap();
        let level_string_texture = Texture::from_surface(&render_level_string_shaded, &texture_creator).unwrap();
        let level_string_rect = Rect::new(5,
                                          0,
                                          render_level_string_shaded.width(),
                                          render_level_string_shaded.height());
        // draw the letters
        self.canvas.copy(&level_string_texture, None, Some(level_string_rect)).unwrap();

        // create score texture
        let render_level_value_shaded = self.font.render(&format!("{}", level)).
            shaded(text_foreground_color, text_background_color).unwrap();
        let level_value_texture = Texture::from_surface(&render_level_value_shaded, &texture_creator).unwrap();

        let x_pos = (PADDING - render_level_value_shaded.width())/ 2;
        let level_value_rect = Rect::new(x_pos as i32,
                                         (level_string_rect.height() + 5)  as i32,
                                         render_level_value_shaded.width(),
                                         render_level_value_shaded.height());
        self.canvas.copy(&level_value_texture, None, Some(level_value_rect)).unwrap();
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Kinda Tetris", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let sdl_ttf_context = sdl2::ttf::init().unwrap();
    let font_path = Path::new("Raleway-Bold.ttf");
    let font = sdl_ttf_context.load_font(font_path, 20).unwrap();

    let mut randy = Randy::new();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut game = Game::new(&mut randy);
    let mut input : Input = Default::default();

    'playing: loop {
        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'playing
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Left  => input.left       = true,
                        Keycode::Right => input.right      = true,
                        Keycode::Down  => input.down       = true,
                        Keycode::Q     => input.ccw_rotate = true,
                        Keycode::W     => input.cw_rotate  = true,
                        _ => {},
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Left  => input.left       = false,
                        Keycode::Right => input.right      = false,
                        Keycode::Down  => input.down       = false,
                        Keycode::Q     => input.ccw_rotate = false,
                        Keycode::W     => input.cw_rotate  = false,
                        _ => {},
                    }
                },
                _ => {},
            }
        }

        // run the game loop
        let state = game.run_loop(&input, &mut randy);

        match state {
            GameState::GameOver =>  {
                println!("GAME OVER MAN!");
                println!("You made it to level {}", game.level());
                println!("Final score: {}", game.score());
                break 'playing;
            },
            _ => {},
        };

        // create a scope so I can borrow mutably
        {
            // clear the screen to black
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            let mut backend = Sdl2Backend::new(&mut canvas, &font);
            game.draw(&mut backend);
        }

        canvas.present();
        // sleep between frames
        // 16 milliseconds is ~ 60 fps
        std::thread::sleep(Duration::from_millis(16));
    }
}
