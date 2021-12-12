use crate::board::Board;
use crate::pieces::{Piece, PIECE_TYPES};
use crate::game_renderer::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Default)]
pub struct Input {
    /// true when user attempts to move the piece left
    pub left: bool,
    /// true when user attempts to move the piece right
    pub right: bool,
    /// true when user attempts to move the piece down
    pub down: bool,
    /// true when user wishes to rotate a piece clockwise
    pub cw_rotate: bool,
    /// true when user wishes to rotate a piece counterclockwise
    pub ccw_rotate: bool,
}

const COOLDOWN : u32 = 5;

pub struct Game {
    /// Holds all possible pieces and their spawn locations.
    /// Gets shuffled after all pieces have been used.
    pieces: [Piece; 7],
    /// The currently falling piece
    current_piece: Piece,
    /// The playing board
    board: Board,
    /// Index for the pieces array.
    piece_counter: usize,
    /// Indicates whether the game is still active.
    state: GameState,
    /// Delay to control how fast the pieces fall.
    gravity_delay: u32,
    /// Counter to keep track of when to apply gravity.
    gravity_cooldown_timer: u32,
    /// Counter to keep track of when to allow another rotation.
    rotation_cooldown_counter: u32,
    /// Counter to keep track of when to allow another translation.
    translation_cooldown_counter: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    Playing,
    GameOver,
}

impl Game {
    pub fn new() -> Game {
        let mut rng = thread_rng();
        let mut tets = PIECE_TYPES;
        tets.shuffle(&mut rng);

        Game {
            pieces: tets,
            current_piece: tets[0],
            board: Board::new(),
            piece_counter: 0,
            state: GameState::Playing,
            gravity_delay: 30,
            gravity_cooldown_timer: 0,
            rotation_cooldown_counter: 0,
            translation_cooldown_counter: 0,
        }
    }

    pub fn run_loop(&mut self, input: &Input) -> GameState {
        match self.state {
            GameState::GameOver => return self.state,
            _ => {},
        }
        // -------------------------
        //    HORIZONTAL MOVEMENT
        // -------------------------
        if self.translation_cooldown_counter > 0 {
            self.translation_cooldown_counter -= 1;
        }

        if self.translation_cooldown_counter ==  0 { 

            self.translation_cooldown_counter = COOLDOWN;
            // if the player is pressing both the left and right buttons
            // don't move the piece
            if input.left && !input.right {
                let candidate = self.current_piece.move_left();

                // calculate the new position of the piece
                let new_position = candidate.position;

                // check if the piece is within the bounds of the board
                // and that moving the piece does not cause a collision
                if self.board.is_tetrimino_within_bounds(&new_position) &&
                   !self.board.is_occupied(&new_position) {
                    // update the current piece with the new position
                    self.current_piece = candidate;
                }
            } else if input.right && !input.left {
                let candidate = self.current_piece.move_right();

                // calculate the new position of the piece
                let new_position = candidate.position;

                // check if the piece is within the bounds of the board
                // and that moving the piece does not cause a collision
                if self.board.is_tetrimino_within_bounds(&new_position) &&
                   !self.board.is_occupied(&new_position) {
                    // update the current piece with the new position
                    self.current_piece = candidate;
                }
            } else {
                // if both buttons for translation were pressed, then let the player try again
                self.translation_cooldown_counter = 0;
            }
        }


        // --------------------
        //    PIECE ROTATION
        // --------------------
        if self.rotation_cooldown_counter > 0 {
            self.rotation_cooldown_counter -= 1;
        }

        if self.rotation_cooldown_counter == 0 {
            self.rotation_cooldown_counter = COOLDOWN;
            // if the player is trying to rotate both clockwise and counterclockwise
            // don't bother rotating the piece
            if input.cw_rotate && !input.ccw_rotate {
                let candidate = self.current_piece.cw_rot();

                // calculate the new position of the piece
                let new_position = candidate.position;


                // check if the rotated piece is within the bounds of the board
                // and that rotating the piece does not cause a collision
                if self.board.is_tetrimino_within_bounds(&new_position) &&
                   !self.board.is_occupied(&new_position) {
                    // update the position of the current piece
                    self.current_piece = candidate;
                }
            } else if input.ccw_rotate && !input.cw_rotate {
                let candidate = self.current_piece.ccw_rot();

                // calculate the new position of the piece
                let new_position = candidate.position;

                // check if the rotated piece is within the bounds of the board
                // and that rotating the piece does not cause a collision
                if self.board.is_tetrimino_within_bounds(&new_position) &&
                   !self.board.is_occupied(&new_position) {
                    // update the position of the current piece
                    self.current_piece = candidate;
                }
            } else {
                // if both buttons for rotation were pressed, then let the player rotate again
                self.rotation_cooldown_counter = 0;
            }
        }

        // -----------------------
        //    VERTICAL MOVEMENT
        // -----------------------
        if self.gravity_cooldown_timer == self.gravity_delay || input.down {
            // reset the gravity countdown timer
            self.gravity_cooldown_timer = 0;

            let candidate = self.current_piece.apply_gravity();

            let new_position = candidate.position;

            if self.board.is_tetrimino_within_bounds(&new_position) {
                // if the new position is occupied, then that means we should be "settled" at our
                // current state
                // OR if it hits the bottom
                let collision = self.board.is_occupied(&new_position);
                let at_bottom = self.board.is_at_the_bottom(&new_position);

                let lines_cleared = if collision {
                    self.board.add_piece(&self.current_piece.position)
                } else if at_bottom {
                    self.board.add_piece(&new_position)
                } else {
                    0
                };

                if lines_cleared > 0 {
                    println!("Cleared {} lines!", lines_cleared);
                }

                if collision || at_bottom {
                    // move to the next piece
                    self.piece_counter += 1;
                    // if all of the pieces have been used, shuffle the pieces
                    if self.piece_counter == self.pieces.len() {
                        let mut rng = thread_rng();
                        // shuffle the pieces
                        self.pieces.shuffle(&mut rng);
                        // reset the counter
                        self.piece_counter = 0;
                    }
                    // set new current piece
                    self.current_piece = self.pieces[self.piece_counter];
                } else {
                    self.current_piece = candidate;
                }
            } else {
                // tetrimino is outside of the bounds, which means it is past the bottom
                println!("I don't think you should ever see this...");
                println!("new position: {:?}", new_position);
            }
        }

        self.gravity_cooldown_timer += 1;

        // Is the game over?
        if self.board.is_board_full() {
            self.state = GameState::GameOver;
            GameState::GameOver
        } else {
            self.state = GameState::Playing;
            GameState::Playing
        }
    }

    /// Draw the game state using the provided renderer.
    pub fn draw(&self, renderer: &mut dyn GameRenderer) {
        // draw all settled pieces
        for y in 0..22 {
            for x in 0..10 {
                let real_y = 21 - y;
                if self.board.is_not_vacant_at(x as usize, y as usize) {
                    renderer.draw_block(x as i32, real_y as i32, TetriminoType::SettledTetrimino);
                }
            }
        }

        // draw the active (falling) piece
        for c in self.current_piece.position.iter() {
            let x = c.x;
            let y = 21 - c.y;
            renderer.draw_block(x as i32, y as i32, TetriminoType::LiveTetrimino);
        }
    }

    /// Function for printing the board to the console.
    /// Used for debugging.
    pub fn print_board(&self) {
        for y in 0..22 {
            print!("{:2}", 21 - y);
            for x in 0..10 {
                let real_y = 21 - y;
                if self.current_piece.position.iter().any(|c| c.x == x && c.y == real_y) {
                    print!("[o]");
                } else if self.board.is_not_vacant_at(x as usize, real_y as usize) {
                    print!("[x]");
                } else {
                    print!("[ ]");
                }
            }
            println!();
        }

        print!("  ");
        for x in 0..10 {
            print!("={}=", x);
        }
        println!();
    }
}
