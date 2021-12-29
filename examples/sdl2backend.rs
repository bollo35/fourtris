extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

use tetris::game::{Game, GameState, Input};
use tetris::game_renderer::{GameRenderer, TetriminoType};

use std::time::Duration;

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
            TetriminoType::EmptySpace => {
                self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            },
        };

        let rect = Rect::new(x * self.block_width as i32,
                             y * self.block_width as i32,
                             self.block_width,
                             self.block_width);

        self.canvas.fill_rect(rect);
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let block_width = 20;
    let window = video_subsystem.window("Kinda Tetris", 10*block_width, 22*block_width)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game = Game::new();

    let mut input : Input = Default::default();

    let mut level = game.level();

    'playing: loop {
        // clear the screen to black
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

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
        let state = game.run_loop(&input);
        if game.level() != level {
            level = game.level();
            println!("Level {}!", level);
        }

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
            let mut backend = Sdl2Backend::new(&mut canvas, block_width);
            game.draw(&mut backend);
        }

        canvas.present();
        // sleep between frames
        // 16 milliseconds is ~ 60 fps
        std::thread::sleep(Duration::from_millis(16));
    }

    /*
    // The following code is for ad hoc testing
    let mut game = Game::new();

    let mut input : Input = Default::default();
    input.ccw_rotate = true;
    input.down = true;
    for i in 0..500 {
        let state = game.run_loop(&input);

        println!("{:?}", state);
        println!("i = {}", i);
        game.print_board();
    }
    */
}
