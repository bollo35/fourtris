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

    #[cfg(test)]
    fn new_test() -> Game {
        Game {
            pieces: PIECE_TYPES,
            current_piece: PIECE_TYPES[0],
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

    fn handle_horizontal_input<P>(input: &Input, piece: &Piece, accept_new_position: P) 
        -> Option<Piece> where 
        P : Fn(&Piece) -> bool {
        let translated_piece =
            if input.left && !input.right {
                Some(piece.move_left())
            } else if input.right && !input.left {
                Some(piece.move_right())
            } else {
                None
            };

        // if the translated piece is within the playfield
        // and it doesn't collide with any of the pieces on the board
        // accept the translation
        translated_piece.filter(accept_new_position) 
    }

    fn handle_rotation_input<P>(input: &Input, piece: &Piece, accept_new_position: P)
        -> Option<Piece> where 
        P : Fn(&Piece) -> bool {
        let rotated_piece = 
            if input.cw_rotate && !input.ccw_rotate {
                Some(piece.cw_rot())
            } else if input.ccw_rotate && !input.cw_rotate {
                Some(piece.ccw_rot())
            } else {
                None
            };
        // if the rotated piece is within the playfield
        // and it doesn't collide with any of the pieces on the board
        // accept the rotation
        rotated_piece.filter(accept_new_position)
    }

    // TODO: try to make less ugly
    fn handle_vertical_movement(piece: &Piece, board: &Board, displacement: u32)
        -> (Piece, bool) {
        let mut relocated_piece = *piece;
        let mut is_settled = false;
        for _ in 1..=displacement {
            let previous_piece = relocated_piece;
            relocated_piece = relocated_piece.apply_gravity(1);

            if board.is_tetrimino_within_bounds(&relocated_piece.position) {
                let collision = board.is_occupied(&relocated_piece.position);
                let at_bottom = board.is_at_the_bottom(&relocated_piece.position);

                relocated_piece  =
                    if collision || at_bottom {
                        // if there's been a collision, the piece should stay at its previous location
                        previous_piece
                    }  else {
                        // if there's been no collision or if it's at the bottom, the translation
                        // is valid
                        relocated_piece
                    };

                is_settled = collision || at_bottom;
                if is_settled {
                    break;
                }
            }
        }

        (relocated_piece, is_settled)
        /*
        let relocated_piece = piece.apply_gravity(displacement);

        if board.is_tetrimino_within_bounds(&relocated_piece.position) {
            let collision = board.is_occupied(&relocated_piece.position);
            let at_bottom = board.is_at_the_bottom(&relocated_piece.position);

            let return_piece =
                if collision {
                    // if there's been a collision, the piece should stay at its previous location
                    *piece
                }  else {
                    // if there's been no collision or if it's at the bottom, the translation
                    // is valid
                    relocated_piece
                };

            let piece_settled = collision || at_bottom;
            (return_piece, piece_settled)

        } else {
            // um...do something
            panic!("No idea why we got here...");
        }
        */
    }

    pub fn run_loop<R: Rng>(&mut self, input: &Input, rng: &mut R) -> GameState {
        match self.state {
            GameState::GameOver => return self.state,
            _ => {},
        }

        let valid_piece_location = |p: &Piece| { 
            self.board.is_tetrimino_within_bounds(&p.position) &&
            !self.board.is_occupied(&p.position)
        };

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
            let translated_piece = Game::handle_horizontal_input(
                                               &input,
                                               &self.current_piece,
                                               valid_piece_location);

            if let Some(candidate) = translated_piece {
                // update the current piece information
                self.current_piece = candidate;
                // only apply the translation cooldown if the piece
                // has successfully been moved
                self.translation_cooldown_counter = COOLDOWN;
            }
        }


        // --------------------
        //    PIECE ROTATION
        // --------------------
        if self.rotation_cooldown_counter > 0 {
            self.rotation_cooldown_counter -= 1;
        } else { // counter is u32, so hitting this branch means it's equal to zero
            let rotated_piece = Game::handle_rotation_input(
                                            &input,
                                            &self.current_piece,
                                            valid_piece_location);

            if let Some(candidate) = rotated_piece {
                // update the current piece information
                self.current_piece = candidate;
                // only apply the rotation cooldown if the piece
                // has successfully been rotated
                self.rotation_cooldown_counter = COOLDOWN;
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
            self.displacement = if input.down { 0.0 } else { self.displacement - displacement as f32 };


            let (updated_piece, is_settled) = Game::handle_vertical_movement(
                                                          &self.current_piece,
                                                          &self.board,
                                                          displacement);

            if is_settled {
                // add the piece to the board
                let y_range = self.board.add_piece(&updated_piece);

                // determine how many lines were cleraed after adding this piece
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

                let off_to_a_new_level = self.score > self.next_level_score && self.level < 15;
                if off_to_a_new_level {
                    self.level += 1;
                    self.next_level_score += 5 * (self.level + 1) as u32;
                }
                // save render info
                // TODO: can we make the render info only get compiled if performing a
                //       parial redraw?
                let lines_were_cleared = lines_cleared > 0; // intermediate variable to shorten line length
                self.render_info.lines_cleared = lines_were_cleared;
                self.render_info.new_score = if lines_were_cleared { Some(self.score) } else { None };
                self.render_info.new_level = if off_to_a_new_level { Some(self.level) } else { None };
                // save the position of these pieces for the next render cycle
                self.render_info.newly_settled_pieces = Some(updated_piece.position);

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
                self.current_piece = updated_piece;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translation_allowed_when_predicate_yields_true() {

        let piece = PIECE_TYPES[0];

        let all_translations_allowed = |_p: &Piece| true;

        let input = Input {
            left: false,
            right: true,
            down: false,
            cw_rotate: false,
            ccw_rotate: false,
        };

        let updated_piece = Game::handle_horizontal_input(&input, &piece, all_translations_allowed).unwrap();

        // don't check the actual value of the translation
        // just ensure that the positions are different
        assert_ne!(updated_piece.position, piece.position);
    }

    #[test]
    fn translation_inhibited_when_predicate_yields_false() {
        let piece = PIECE_TYPES[0];

        let no_translation_allowed = |_p: &Piece| false;

        let input = Input {
            left: false,
            right: true,
            down: false,
            cw_rotate: false,
            ccw_rotate: false,
        };

        let updated_piece = Game::handle_horizontal_input(&input, &piece, no_translation_allowed);

        assert_eq!(updated_piece, None);
    }

    #[test]
    fn no_translation_if_both_inputs_true() {
        let piece = PIECE_TYPES[0];

        let all_translation_allowed = |_p: &Piece| true;

        let input = Input {
            left: true,
            right: true,
            down: false,
            cw_rotate: false,
            ccw_rotate: false,
        };

        let updated_piece = Game::handle_horizontal_input(&input, &piece, all_translation_allowed);

        assert_eq!(updated_piece, None);
    }

    #[test]
    fn rotation_inhibited_when_predicate_yields_false() {
        let piece = PIECE_TYPES[0];

        let no_rotation_allowed = |_p: &Piece| false;

        let input = Input {
            left: false,
            right: false,
            down: false,
            cw_rotate: true,
            ccw_rotate: false,
        };

        let updated_piece = Game::handle_rotation_input(&input, &piece, no_rotation_allowed);

        assert_eq!(updated_piece, None);
    }

    #[test]
    fn rotation_allowed_when_predicate_yields_true() {
        let mut piece = PIECE_TYPES[0];
        for c in piece.position.iter_mut() {
            // rotating while at the top of the screen would place
            // the piece outside 
            c.y -= 5;
        }

        let all_rotation_allowed = |_p: &Piece| true;

        let input = Input {
            left: false,
            right: false,
            down: false,
            cw_rotate: true,
            ccw_rotate: false,
        };

        let updated_piece = Game::handle_rotation_input(&input, &piece, all_rotation_allowed).unwrap();

        // don't check the rotated value, the rotations are verified in other tests
        // just make sure that the rotated value differs from the original value
        assert_ne!(updated_piece, piece);
    }

    #[test]
    fn no_rotation_if_both_inputs_true() {
        let piece = PIECE_TYPES[0];

        let all_rotation_allowed = |_p: &Piece| true;

        let input = Input {
            left: false,
            right: false,
            down: false,
            cw_rotate: true,
            ccw_rotate: true,
        };

        let updated_piece = Game::handle_rotation_input(&input, &piece, all_rotation_allowed);

        assert_eq!(updated_piece, None);
    }

    #[test]
    fn fast_moving_piece_settles_appropriately() {
        let mut board = Board::new();

        let y = 19;
        for x in 0..10 { // board is 10 tetriminos wide
            board.add_tetrimino_at(x, y, TetriminoType::I);
        }

        let piece = PIECE_TYPES[0]; // this is the I type, which spawns at y = 20

        let displacement = 15; // set a ridiculous displacement

        let (updated_piece, is_settled) = Game::handle_vertical_movement(&piece, &board, displacement);

        // check to see if the piece settled
        assert_eq!(is_settled, true);

        // check to see if it settled in the right location
        assert_eq!(updated_piece.position[0].y, 20);
        assert_eq!(updated_piece.position[1].y, 20);
        assert_eq!(updated_piece.position[2].y, 20);
        assert_eq!(updated_piece.position[3].y, 20);
    }

    // Rng implementation that doesn't shuffle anything
    struct Randy {
        indexes: [usize; 7],
        i: usize,
    }

    impl Randy {
        fn new() -> Randy {
            Randy {
                indexes: [0, 1, 2, 3, 4, 5, 6],
                i: 0,
            }
        }
    }

    impl Rng for Randy {
        fn next(&mut self) -> usize {
            let r = self.indexes[self.i];
            self.i = (self.i + 1) % 6;
            r
        }
    }

    #[test]
    fn piece_can_move_horizontally_on_the_last_row() {
        let mut game = Game::new_test();
        
        let mut input = Input {
            down: true,
            .. Default::default()
        };

        let mut randy = Randy::new();

        // run 20 iterations of the game to move the piece to the bottom
        // NOTE: the first piece chosen is the I piece
        for _ in 0..20 {
            let _ = game.run_loop(&input, &mut randy);
        }

        input.left = true;

        // run iteration of the main loop
        let _ = game.run_loop(&input, &mut randy);

        assert_ne!(game.board.tetrimino_type_at(5, 0), TetriminoType::EmptySpace);
        assert_ne!(game.board.tetrimino_type_at(4, 0), TetriminoType::EmptySpace);
        assert_ne!(game.board.tetrimino_type_at(3, 0), TetriminoType::EmptySpace);
        assert_ne!(game.board.tetrimino_type_at(2, 0), TetriminoType::EmptySpace);
    }
}
