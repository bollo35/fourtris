use crate::board::Board;
use crate::pieces::{Piece, PIECE_TYPES};
use crate::coord::Coord;
use crate::game_renderer::TetriminoType;
use crate::game_renderer::GameRenderer;

struct RenderInfo {
    previous_piece_pos: [Coord; 4],
}

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

const COOLDOWN : u32 = 10;

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
    /// This is used as part of the gravity calculation.
    frames: usize,
    /// The current level. This determines how fast the pieces fall.
    level: usize,
    /// The current score, used to determine which level has been reached.
    score: usize,
    /// The next score to make to get to the next level.
    next_level_score: usize,
    /// Counter to keep track of when to allow another rotation.
    rotation_cooldown_counter: u32,
    /// Counter to keep track of when to allow another translation.
    translation_cooldown_counter: u32,
    /// Optional rendering info
    render_info: Option<RenderInfo>,
}

// 1 unit of gravity = moving one cell
// these are the gravity constants for 60 fps
// source: https://harddrop.com/wiki/Tetris_Worlds
const GRAVITY : [f64; 15] = [
    0.01667,
    0.021017,
    0.026977,
    0.035356,
    0.04693,
    0.06361,
    0.0879,
    0.1236,
    0.1775,
    0.2598,
    0.388,
    0.59,
    0.92,
    1.46,
    2.36,
];
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    Playing,
    GameOver,
}

impl Game {
    pub fn new() -> Game {
        let mut tets = PIECE_TYPES;
        /*
        let mut rng = thread_rng();
        tets.shuffle(&mut rng);
        */

        Game {
            pieces: tets,
            current_piece: tets[0],
            board: Board::new(),
            piece_counter: 0,
            state: GameState::Playing,
            frames: 0,
            level: 1,
            score: 0,
            next_level_score: 5,
            rotation_cooldown_counter: 0,
            translation_cooldown_counter: 0,
            render_info: None,
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

        // save a copy of the piece's current position
        let previous_piece_pos = self.current_piece.position.clone();

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

        self.frames += 1;
        // THOUGHT: I could just add the gravity value to a stored value each frame, instead of
        //          recalculating the displacement every frame. Seems more efficient.
        let displacement = GRAVITY[self.level-1] * self.frames as f64;

        if (displacement as i32) > 0 || input.down {
            // reset frame counter
            self.frames = 0;

            let candidate = if input.down {
                // if the user is holding the down input, move the piece down at the drop rate
                // for that level
                let diff_displacement = (1.0/GRAVITY[self.level-1] + 1.0) * GRAVITY[self.level - 1];
                self.current_piece.apply_gravity(diff_displacement as isize)
            } else {
                self.current_piece.apply_gravity(displacement as isize)
            };

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

                // update the score based on the number of lines cleared
                if lines_cleared == 1 {
                    self.score += 1;
                } else if lines_cleared == 2 {
                    self.score += 3;
                } else if lines_cleared == 3 {
                    self.score += 5;
                } else if lines_cleared == 4 {
                    self.score += 8;
                }

                if lines_cleared > 0 {
                    // println!("SCORE: {}", self.score);
                    // do we need to go to the next level?
                    if self.score > self.next_level_score && self.level < 15 {
                        self.level += 1;
                        // println!("LEVEL {}!", self.level);
                        self.next_level_score += 5 * self.level;
                    }
                }

                if collision || at_bottom {
                    // move to the next piece
                    self.piece_counter += 1;
                    // if all of the pieces have been used, shuffle the pieces
                    if self.piece_counter == self.pieces.len() {
/*
                        let mut rng = thread_rng();
                        // shuffle the pieces
                        self.pieces.shuffle(&mut rng);
*/
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
                // println!("I don't think you should ever see this...");
                // Actually, I do think this could happen at the higher levelssti
                // println!("new position: {:?}", new_position);
            }
        }

        let current_piece_pos = self.current_piece.position.clone();
        let piece_has_moved = current_piece_pos.iter().
                                 zip(previous_piece_pos.iter()).
                                 any(|(&a, &b)| a != b);

        self.render_info = 
            if piece_has_moved {
                Some( RenderInfo {
                    previous_piece_pos,
                })
            } else {
                None
            };

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

        if let Some(render_info) = &self.render_info {
            // erase the previous location
            for c in render_info.previous_piece_pos.iter() {
                let x = c.x;
                let y = 21 - c.y;
                renderer.draw_block(x as i32, y as i32, TetriminoType::EmptySpace);
            }
        }
        // draw the active (falling) piece
        for c in self.current_piece.position.iter() {
            let x = c.x;
            let y = 21 - c.y;
            renderer.draw_block(x as i32, y as i32, TetriminoType::LiveTetrimino);
        }
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn level(&self) -> usize {
        self.level
    }
    /*
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
    */
}
