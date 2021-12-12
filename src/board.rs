use crate::coord::Coord;

const BOARD_WIDTH: usize =  10;
const BOARD_HEIGHT: usize =  22;
pub struct Board {
    content: [[isize; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Board {
        Board {
            // default boolean value is false
            // this will make the board empty
            content: Default::default(),
        }
    }

    pub fn is_tetrimino_within_bounds(&self, coords: &[Coord; 4]) -> bool {
        coords.iter().all(|&c| 0 <= c.x && c.x < BOARD_WIDTH as isize && 
                              0 <= c.y && c.y < BOARD_HEIGHT as isize)
    }

    pub fn is_not_vacant_at(&self, x: usize, y: usize) -> bool {
        (x <= BOARD_WIDTH) &&
        (y <= BOARD_HEIGHT) &&
        self.content[y][x] == 1
    }

    // THIS FUNCTION SHOULD ONLY BE CALLED AFTER VERIFYING THAT
    // THE COORDINATES ARE WITHIN THE BOARD SIZE
    pub fn is_occupied(&self, coords: &[Coord; 4]) -> bool {
        // would these coordinates overlap with 
        // any of the pieces already settled on the board?
        // NOTE: the cast only holds if the coordinates have already been
        //       verified to be within the borders of the game board
        coords.iter().any(|&c| self.content[c.y as usize][c.x as usize] == 1)
    }

    pub fn is_at_the_bottom(&self, coords: &[Coord; 4]) -> bool {
        // at least one y coordinate should be equal to zero
        // and none of the coordinates should be less than zero
        coords.iter().any(|&c| c.y == 0) && !coords.iter().any(|&c| c.y < 0)
    }

    pub fn add_piece(&mut self, coords: &[Coord; 4]) -> u32 {
        // add pieces to the board
        for c in coords.iter() {
            // NOTE: assumption is that these have been verified to be within the board bounds
            self.content[c.y as usize][c.x as usize] = 1;
        }

        // determine y coordinate range
        // the y range determines where to check for completed lines
        let mut y_min : isize =  400;
        let mut y_max : isize = -400;
        for c in coords.iter() {
            if c.y < y_min {
                y_min = c.y;
            }

            if c.y > y_max {
                y_max = c.y;
            }
        }

        let mut lines_cleared = 0;
        // check for completed lines and mark all entries with a 2
        for y in y_min..=y_max {
            // save all indices where there isn't a line to clear
            // NOTE: y must be within the board bounds
            let is_completed_line = self.content[y as usize].iter().all(|&val| val == 1);

            // mark line for deletion if it's a completed line
            if is_completed_line {
                for val in self.content[y as usize].iter_mut() {
                    *val = 2;
                }
                lines_cleared += 1;
            }
        }

        // clear completed lines.
        // this is done naively.
        // Start checking for completed lines at y_min.
        // Each time we find a completed line, replace that line with the
        // contents of the grid row above it and shift the rest of the grid
        // rows accordingly.
        // Also, set the top row of the grid to empty.
        let mut y = y_min;
        // decrement each time we test a line in the expected
        // range of where the new tetrimino landed
        let mut line_counter = y_max - y_min + 1;
        while line_counter > 0 {
            let to_delete = self.content[y as usize][0] == 2;
            if to_delete {
                // shift all the grid rows above this line down
                // the last grid row won't have another row to copy from, so ignore that row until
                // the end
                for i in y..(BOARD_HEIGHT as isize -1) {
                    for x in 0..BOARD_WIDTH {
                        self.content[i as usize][x] = self.content[i as usize + 1][x];
                    }
                }

                // set the upper most grid row to all zeroes, indicating nothing is there
                for x in self.content[BOARD_HEIGHT-1].iter_mut() {
                    *x = 0;
                }
            } else {
                y += 1;
            }
            line_counter -= 1;
        }

        lines_cleared
    }

    pub fn is_board_full(&self) -> bool {
        self.content[BOARD_HEIGHT-3].iter().any(|&c| c == 1)
    }
}
