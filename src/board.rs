use crate::coord::Coord;
use crate::pieces::Piece;
use crate::pieces::PieceType;
use crate::game_renderer::TetriminoType;

use core::ops::Range;

const BOARD_WIDTH: usize  =  10;
const BOARD_HEIGHT: usize  =  22;
pub struct Board {
    content: [[TetriminoType; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Board {
        Board {
            // this will make an empty board
            content: Default::default(),
        }
    }

    /// Returns the `TetriminoType` at the specified xy coordinate.
    pub fn tetrimino_type_at(&self, x: u8, y: u8) -> TetriminoType {
        self.content[y as usize][x as usize]
    }

    /// Checks to see if a set of coordinates are within the accepted
    /// bounds of the playfield. In order to support moving a piece
    /// on the last row, a y-value of -1 is considered within the
    /// bounds of the playfield.
    pub fn is_tetrimino_within_bounds(&self, coords: &[Coord; 4]) -> bool {
        coords.iter().all(|&c| 0 <= c.x && c.x < BOARD_WIDTH as i32 && 
                               -1 <= c.y && c.y < BOARD_HEIGHT as i32)
    }

    #[cfg(test)]
    /// A helper function allowing random write access to the board for testing.
    pub fn add_tetrimino_at(&mut self, x: usize, y: usize, tet_type: TetriminoType) {
        if x < BOARD_WIDTH && y < BOARD_HEIGHT {
            self.content[y][x] = tet_type;
        } else {
            panic!("Invalid x or y coordinate ({},{})", x, y);
        }
    }

    // THIS FUNCTION SHOULD ONLY BE CALLED AFTER VERIFYING THAT
    // THE COORDINATES ARE WITHIN THE BOARD SIZE
    /// Returns `true` if any of the tetriminos in a piece are already
    /// occupied in the board.
    pub fn is_occupied(&self, coords: &[Coord; 4]) -> bool {
        // would these coordinates overlap with 
        // any of the pieces already settled on the board?
        // NOTE: the cast only holds if the coordinates have already been
        //       verified to be within the borders of the game board
        coords.iter().any(|&c| { 
            if c.y < 0 {
                false
            } else {
                self.content[c.y as usize][c.x as usize] != TetriminoType::EmptySpace
            }
        })
    }

    /// Returns `true` if any of the tetriminos in a piece are at the
    /// bottom of the board.
    pub fn is_at_the_bottom(&self, coords: &[Coord; 4]) -> bool {
        // at least one y coordinate should be equal to -1
        // and none of the coordinates should be less than -1
        // -1 is used to allow for movement on the last row
        coords.iter().any(|&c| c.y == -1) && !coords.iter().any(|&c| c.y < -1)
    }

    /// Adds a piece to the board at the coordinates.
    /// Returns the range of y-values occupied by the piece.
    /// The y-values constrain the search for lines to remove.
    pub fn add_piece(&mut self, piece: &Piece) -> Range<usize> {
        let tet_type = 
            match piece.piece_type {
                PieceType::I(_) => TetriminoType::I,
                PieceType::O    => TetriminoType::O,
                PieceType::J    => TetriminoType::J,
                PieceType::L    => TetriminoType::L,
                PieceType::S    => TetriminoType::S,
                PieceType::Z    => TetriminoType::Z,
                PieceType::T    => TetriminoType::T,
            };

        // add pieces to the board
        for c in piece.position.iter() {
            // NOTE: assumption is that these have been verified to be within the board bounds
            self.content[c.y as usize][c.x as usize] = tet_type;
        }


        // determine y coordinate range
        // the y range determines where to check for completed lines
        let mut y_min : i32 =  400;
        let mut y_max : i32 = -400;
        for c in piece.position.iter() {
            if c.y < y_min {
                y_min = c.y;
            }

            if c.y > y_max {
                y_max = c.y;
            }
        }

        (y_min as usize)..((y_max + 1) as usize)
    }

    /// Updates the board by clearing any full lines within
    /// the specified y-range.
    /// Returns the number of lines cleared.
    pub fn clear_lines(&mut self, y_range: Range<usize>) -> u32 {
        // this will hold the indices of lines to be removed
        // at most 4 lines will be removed
        // -1 will indicate there are no more lines to remove
        let mut completed_lines : [i32; 4] = [-1, -1, -1, -1];
        let mut idx = 0;
        let mut lines_cleared = 0;
        // check for completed lines and mark all entries with a 2
        for y in y_range {
            // save all indices where there isn't a line to clear
            // NOTE: y must be within the board bounds
            let is_completed_line = self.content[y as usize].iter().all(|&val| val != TetriminoType::EmptySpace);

            // mark line for deletion if it's a completed line
            if is_completed_line {
                completed_lines[idx] = y as i32;
                idx += 1;
                lines_cleared += 1;
            }
        }

        // clear completed lines.
        for (cleared_so_far, y) in completed_lines.iter().enumerate() {
            // exit if there are no more lines to clear
            // (indicated by -1)
            if *y == -1 {
                break;
            }

            // adjust the y coordinate for the lines already removed
            let real_y = *y - cleared_so_far as i32; 
            // shift all the grid rows above this line down
            // the last grid row won't have another row to copy from, so ignore that row until
            // the end
            for i in real_y..(BOARD_HEIGHT as i32 - 1) {
                for x in 0..BOARD_WIDTH {
                    self.content[i as usize][x] = self.content[i as usize + 1][x];
                }
            }

            // set the upper most grid row to all zeroes, indicating nothing is there
            for x in self.content[BOARD_HEIGHT-1].iter_mut() {
                *x = TetriminoType::EmptySpace;
            }
        }

        lines_cleared
    }
}
