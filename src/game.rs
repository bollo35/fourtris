use crate::board::Board;
use crate::pieces::{Piece, PieceType, PIECE_TYPES};
use crate::coord::Coord;
use crate::game_renderer::TetriminoType;
use crate::game_renderer::GameRenderer;
use crate::rng::Rng;

#[derive(Default)]
struct RenderInfo {
    previous_piece_pos: Option<[Coord; 4]>,
    newly_settled_pieces: Option<[Coord; 4]>,
    lines_cleared: bool,
    new_score: Option<u32>,
    new_level: Option<usize>,
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
    piece_index: usize,
    /// Indicates whether the game is still active.
    state: GameState,
    /// Represents the current displacement of the active piece.
    displacement: f32,
    /// The current level. This determines how fast the pieces fall.
    level: usize,
    /// The current score, used to determine which level has been reached.
    score: u32,
    /// The next score to make to get to the next level.
    next_level_score: u32,
    /// Counter to keep track of when to allow another rotation.
    rotation_cooldown_counter: u32,
    /// Counter to keep track of when to allow another translation.
    translation_cooldown_counter: u32,
    /// Rendering info
    render_info: RenderInfo,
}

// 1 unit of gravity = moving one cell
// these are the gravity constants for 60 fps
// source: https://harddrop.com/wiki/Tetris_Worlds
const GRAVITY : [f32; 15] = [
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
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        let mut tets = PIECE_TYPES;
        // do a knuth shuffle to permuate the pieces
        for i in 0..tets.len() {
            let index = rng.next();
            if index != i {
                // swap i and index
                let temp = tets[i];
                tets[i] = tets[index];
                tets[index] = temp;
            }
        }

        Game {
            pieces: tets,
            current_piece: tets[0],
            board: Board::new(),
            piece_index: 0,
            state: GameState::Playing,
            displacement: 0.0,
            level: 1,
            score: 0,
            next_level_score: 5,
            rotation_cooldown_counter: 0,
            translation_cooldown_counter: 0,
            render_info: Default::default(),
        }
    }

    pub fn run_loop<R: Rng>(&mut self, input: &Input, rng: &mut R) -> GameState {
        match self.state {
            GameState::GameOver => return self.state,
            _ => {},
        }

        // reset render info
        self.render_info = Default::default();

        // save a copy of the piece's current position
        let previous_piece = self.current_piece.clone();

        // -------------------------
        //    HORIZONTAL MOVEMENT
        // -------------------------
        if self.translation_cooldown_counter > 0 {
            self.translation_cooldown_counter -= 1;
        } else { // counter is u32, so hitting this branch means it's equal to zero
            let translated_piece = 
                if input.left && !input.right {
                    Some(self.current_piece.move_left())
                } else if input.right && !input.left {
                    Some(self.current_piece.move_right())
                } else {
                    None
                };

            // if the translate piece is within the playfield
            // and it doesn't collide with any of the pieces on the board
            // accept the translation
            if let Some(candidate) = translated_piece {
                if self.board.is_tetrimino_within_bounds(&candidate.position) &&
                !self.board.is_occupied(&candidate.position) {
                    // update the current piece information
                    self.current_piece = candidate;
                    // only apply the translation cooldown if the piece
                    // has successfully been moved
                    self.translation_cooldown_counter = COOLDOWN;
                }
            }
        }


        // --------------------
        //    PIECE ROTATION
        // --------------------
        if self.rotation_cooldown_counter > 0 {
            self.rotation_cooldown_counter -= 1;
        } else { // counter is u32, so hitting this branch means it's equal to zero
            let rotated_piece = 
                if input.cw_rotate && !input.ccw_rotate {
                    Some(self.current_piece.cw_rot())
                } else if input.ccw_rotate && !input.cw_rotate {
                    Some(self.current_piece.ccw_rot())
                } else {
                    None
                };

            if let Some(candidate) = rotated_piece {
                if self.board.is_tetrimino_within_bounds(&candidate.position) &&
                   !self.board.is_occupied(&candidate.position) {
                    // update the current piece information
                    self.current_piece = candidate;
                    // only apply the rotation cooldown if the piece
                    // has successfully been rotated
                    self.rotation_cooldown_counter = COOLDOWN;
                }
            }
        }

        // -----------------------
        //    VERTICAL MOVEMENT
        // -----------------------

        self.displacement += GRAVITY[self.level-1];

        if (self.displacement as u32) > 0 || input.down {

            // choose the displacement value we will apply
            let displacement =
                if input.down {
                    // move the piece down at least 1 cell per frame while the user is holding the
                    // down button
                    core::cmp::max(1, self.displacement as u32 + 1)
                } else {
                    self.displacement as u32
                };

            // reset internal displacement
            self.displacement = 0.0;

            let candidate = self.current_piece.apply_gravity(displacement);
            let new_position = candidate.position;

            if self.board.is_tetrimino_within_bounds(&new_position) {
                // if the new position is occupied, then that means we should be "settled" at our
                // current state
                // OR if it hits the bottom
                let collision = self.board.is_occupied(&new_position);
                let at_bottom = self.board.is_at_the_bottom(&new_position);

                let possibly_settled_piece = 
                    if collision {
                        Some(self.current_piece)
                    } else if at_bottom {
                        Some(candidate)
                    } else {
                        None
                    };

                if let Some(settled_piece) = possibly_settled_piece {
                    // add the piece to the board
                    let y_range = self.board.add_piece(&settled_piece);

                    // determine how many lines were cleared after adding the piece
                    let lines_cleared = self.board.clear_lines(y_range);

                    // update the score based on the number of lines cleared
                    self.score +=
                        if lines_cleared == 1 {
                            1
                        } else if lines_cleared == 2 {
                            3
                        } else if lines_cleared == 3 {
                            5
                        } else if lines_cleared == 4 {
                            8
                        } else {
                            0
                        };

                    // see if we need to go to the next level
                    let off_to_a_new_level = self.score > self.next_level_score && self.level < 15;
                    if off_to_a_new_level {
                        self.level += 1;
                        self.next_level_score += 5 * (self.level+1) as u32;
                    }

                    // save render info
                    // TODO: can we make the render info only get compiled if performing a
                    //       parial redraw?
                    let lines_were_cleared = lines_cleared > 0; // intermediate variable to shorten line length
                    self.render_info.lines_cleared = lines_were_cleared;
                    self.render_info.new_score = if lines_were_cleared { Some(self.score) } else { None };
                    self.render_info.new_level = if off_to_a_new_level { Some(self.level) } else { None };
                    // save the position of these pieces for the next render cycle
                    self.render_info.newly_settled_pieces = Some(settled_piece.position);

                    // move to the next piece
                    self.piece_index += 1;
                    // if all of the pieces have been used, shuffle the pieces
                    if self.piece_index == self.pieces.len() {
                        // do a knuth shuffle to create a permutation of the pieces
                        for i in 0..self.pieces.len() {
                            let index = rng.next();
                            if index != i {
                                // swap i and index
                                let temp = self.pieces[i];
                                self.pieces[i] = self.pieces[index];
                                self.pieces[index] = temp;
                            }
                        }
                        // reset the index
                        self.piece_index = 0;
                    }
                    // set new current piece
                    self.current_piece = self.pieces[self.piece_index];
                } else {
                    // current piece did not collide with existing tetriminoes
                    // and did not hit the bottom...it may continue its descent
                    self.current_piece = candidate;
                }
            } else {
                // tetrimino is outside of the bounds, which means it is past the bottom
                // println!("I don't think you should ever see this...");
                // Actually, I do think this could happen at the higher levels
                // println!("new position: {:?}", new_position);
            }
        }

        let piece_has_moved = self.current_piece.position.iter().
                                 zip(previous_piece.position.iter()).
                                 any(|(&a, &b)| a != b);
        self.render_info.previous_piece_pos = 
            if piece_has_moved {
                Some(previous_piece.position.clone())
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

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn level(&self) -> u8 {
        self.level as u8
    }

    /// Draw the game state using the provided renderer.
    pub fn draw<G: GameRenderer>(&self, renderer: &mut G) {
        self._draw(renderer);
    }

    #[cfg(not(any(feature="partial_redraw", feature="full_redraw")))]
    compile_error!("Must enable feature \"partial_redraw\" or feature \"full_redraw\".");

    #[cfg(all(feature="partial_redraw", feature="full_redraw"))]
    compile_error!("feature \"partial_redraw\" and feature \"full_redraw\" cannot be enabled at the same time");

    #[cfg(feature="partial_redraw")]
    fn _draw<G: GameRenderer>(&self, renderer: &mut G) {

        if let Some(score) = self.render_info.new_score {
            renderer.draw_score(score);
        }

        if let Some(level) = self.render_info.new_level {
            renderer.draw_level(level);
        }
        
        // make updates to the board as necessary
        if self.render_info.lines_cleared {
            // redraw the board
            for y in 0..22 {
                for x in 0..10 {
                    let real_y = 21 - y;
                    renderer.draw_block(x as u8, real_y as u8, self.board.tetrimino_type_at(x, y));
                }
            }
        } else {
            // do all erasing first, in case a new piece may have some overlap with the
            // previous position
            if let Some(previous_pos) = &self.render_info.previous_piece_pos {
                // erase the previous location
                for c in previous_pos.iter() {
                    let x = c.x;
                    let y = 21 - c.y;
                    renderer.draw_block(x as u8, y as u8, TetriminoType::EmptySpace);
                }
            }

            // draw any newly settled pieces
            if let Some (newly_settled_pieces) = &self.render_info.newly_settled_pieces {
                for c in newly_settled_pieces.iter() {
                    let x = c.x;
                    let y = 21 - c.y;
                    renderer.draw_block(x as u8, y as u8, self.board.tetrimino_type_at(c.x as u8, c.y as u8));
                }
            }
        }

        // draw the active (falling) piece
        let tet_type =
            match self.current_piece.piece_type {
                PieceType::IType(_) => TetriminoType::I,
                PieceType::OType    => TetriminoType::O,
                PieceType::JType    => TetriminoType::J,
                PieceType::LType    => TetriminoType::L,
                PieceType::SType    => TetriminoType::S,
                PieceType::ZType    => TetriminoType::Z,
                PieceType::TType    => TetriminoType::T,
            };
        for c in self.current_piece.position.iter() {
            let x = c.x;
            let y = 21 - c.y;
            renderer.draw_block(x as u8, y as u8, tet_type);
        }
    }


    #[cfg(feature="full_redraw")]
    pub fn _draw<G: GameRenderer>(&self, renderer: &mut G) {
        renderer.draw_board();
        renderer.draw_score(self.score);
        renderer.draw_level(self.level);

        // redraw the board
        for y in 0..22 {
            for x in 0..10 {
                let real_y = 21 - y;
                renderer.draw_block(x as u8, real_y as u8, self.board.tetrimino_type_at(x, y));
            }
        }

        // draw the active (falling) piece
        let tet_type =
            match self.current_piece.piece_type {
                PieceType::IType(_) => TetriminoType::I,
                PieceType::OType    => TetriminoType::O,
                PieceType::JType    => TetriminoType::J,
                PieceType::LType    => TetriminoType::L,
                PieceType::SType    => TetriminoType::S,
                PieceType::ZType    => TetriminoType::Z,
                PieceType::TType    => TetriminoType::T,
            };
        for c in self.current_piece.position.iter() {
            let x = c.x;
            let y = 21 - c.y;
            renderer.draw_block(x as u8, y as u8, tet_type);
        }
    }
}
