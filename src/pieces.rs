use crate::coord::Coord;
/// This describes the different orientations for the I piece.
/// The I piece is 4 tetriminoes long, so it doesn't rotate as
/// nicely as the other pieces. The integer values assigned 
/// correspond to the appropriate offset values in the I_CW_OFFSETS array.
/// The orientations are as follows:
/// HorizontalDown
/// [ ][ ][ ][ ]
/// [ ][ ][ ][ ]
/// [o][o][o][o]
/// [ ][ ][ ][ ]
/// VerticalLeft
/// [ ][o][ ][ ]
/// [ ][o][ ][ ]
/// [ ][o][ ][ ]
/// [ ][o][ ][ ]
/// HorizontalUp
/// [ ][ ][ ][ ]
/// [o][o][o][o]
/// [ ][ ][ ][ ]
/// [ ][ ][ ][ ]
/// VerticalRight
/// [ ][ ][o][ ]
/// [ ][ ][o][ ]
/// [ ][ ][o][ ]
/// [ ][ ][o][ ]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Orientation {
    HorizontalDown = 0,
    VerticalLeft   = 1,
    HorizontalUp   = 2,
    VerticalRight  = 3,
}

/// Represents the 7 pieces.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    IType(Orientation), 
    OType,
    JType,
    LType,
    SType,
    ZType,
    TType,
}

// ---------------------------------------------------------------
//            Initial Piece coordinates
// ---------------------------------------------------------------
// CONVENTION ALERT: The center point of these pieces is the first
//                   coordinate of the array.
//                   In the case of the I piece, and endpoint was
//                   chosen. This piece gets treated differently
//                   than the others. Also, the O piece never gets
//                   rotated.
// All these coordinates describe the pieces at their spawn point
// Each array has a diagram to show which array index corresponds
// to which tetrimino.

// [3][2][1][0]
const I_COORDS: [Coord; 4] = [
    Coord { x: 6, y: 20 },
    Coord { x: 5, y: 20 },
    Coord { x: 4, y: 20 },
    Coord { x: 3, y: 20 },
];

// [1][3]
// [0][2]
const O_COORDS: [Coord; 4] = [
    Coord { x: 4, y: 20 },
    Coord { x: 4, y: 21 },
    Coord { x: 5, y: 20 },
    Coord { x: 5, y: 21 },
];

// [2]
// [1][0][3]
const J_COORDS : [Coord; 4] = [
    Coord { x: 4, y: 20 },
    Coord { x: 3, y: 20 },
    Coord { x: 3, y: 21 },
    Coord { x: 5, y: 20 },
];

//       [3]
// [1][0][2]
const L_COORDS : [Coord; 4] = [
    Coord { x: 4, y: 20 },
    Coord { x: 3, y: 20 },
    Coord { x: 5, y: 20 },
    Coord { x: 5, y: 21 },
];

//    [2][3]
// [1][0]
const S_COORDS : [Coord; 4] = [
    Coord { x: 4, y: 20 },
    Coord { x: 3, y: 20 },
    Coord { x: 4, y: 21 },
    Coord { x: 5, y: 21 },
];

// [1][2]
//    [0][3]
const Z_COORDS : [Coord; 4] = [
    Coord { x: 4, y: 20 },
    Coord { x: 3, y: 21 },
    Coord { x: 4, y: 21 },
    Coord { x: 5, y: 20 },
];

//    [3]
// [1][0][2]
const T_COORDS : [Coord; 4] = [
    Coord { x: 4, y: 20 },
    Coord { x: 3, y: 20 },
    Coord { x: 4, y: 21 },
    Coord { x: 5, y: 20 },
];

// The I piece doesn't really have a center point.
// Instead, the code treats one of the end points as
// the center point. In order to rotate the piece
// properly, there is an additional translation offset.
// This offset changes based on the direction of rotation.
// The counterclockwise rotation offset coordinates, are just
// clockwise rotations of the clockwise offsets.
// How did I discover this? I worked it out by hand.
const I_CW_OFFSETS : [Coord; 4] = [
    Coord { x: -2, y: -1 },  // Horizontal down
    Coord { x: -1, y:  2 },  // Vertical left
    Coord { x:  2, y:  1 },  // Horizontal up
    Coord { x:  1, y: -2 },  // Vertical right
];

/// Takes all the coordinates for a piece and adds an offset to them.
fn add_offset(coords: &[Coord; 4], offset: Coord) -> [Coord; 4] {
    let mut new_pos : [Coord; 4] = Default::default();
    for (old, new) in coords.iter().zip(new_pos.iter_mut()) {
        *new = *old + offset;
    }

    new_pos
}

/// Returns an array of Coord all relative to the center point (i.e., the first point in the array)
fn make_relative(coords: &[Coord; 4]) -> [Coord; 4] {
    let center_point = coords[0];
    add_offset(&coords, Coord { x: -center_point.x, y: -center_point.y})
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// Represents a tetris piece
pub struct Piece {
    pub piece_type: PieceType,
    /// The coordinates of the individual tetriminoes
    pub position: [Coord; 4],
}



impl Piece {
    /// Calculate the new location of a piece if moved left by one space
    pub fn move_left(&self) -> Piece {
        let offset = Coord { x: -1, y: 0 };
        Piece {
            position: add_offset(&self.position, offset),
            .. *self
        }
    }

    /// Calculate the new location of a piece if moved right by one space
    pub fn move_right(&self) -> Piece {
        let offset = Coord { x: 1, y: 0 };

        Piece {
            position: add_offset(&self.position, offset),
            .. *self
        }
    }

    /// Calculate the new location of a piece if moved down by one space
    pub fn apply_gravity(&self, displacement: u32) -> Piece {
        let offset = Coord { x: 0, y: -1 * displacement as i32 };

        Piece {
            position: add_offset(&self.position, offset),
            .. *self
        }
    }

    /// Calculate the new tetrimino locations for a piece rotated clockwise
    pub fn cw_rot(&self) -> Piece {
        // The idea for rotating is:
        // 1. Make all the points relative one of the piece's tetriminoes.
        // 2. Rotate 90 degrees around that point.
        // 3. Translate the point back to it's origin
        //     - In the case of the I piece, there's an associated adjustment

        // save center point for translation (step 3)
        let center_coord = self.position[0];
        let relative_coords = make_relative(&self.position);
        let rel_rotated_coords = 
            [ Coord {x: relative_coords[0].y, y: -relative_coords[0].x}, 
              Coord {x: relative_coords[1].y, y: -relative_coords[1].x}, 
              Coord {x: relative_coords[2].y, y: -relative_coords[2].x}, 
              Coord {x: relative_coords[3].y, y: -relative_coords[3].x} ];

        match self.piece_type {
           PieceType::IType(orientation) => {
               let offset = I_CW_OFFSETS[orientation as usize];
               let new_position = add_offset(&rel_rotated_coords, offset + center_coord);
               let new_piece_type = 
                   match orientation {
                       Orientation::HorizontalDown => PieceType::IType(Orientation::VerticalLeft),
                       Orientation::VerticalLeft   => PieceType::IType(Orientation::HorizontalUp),
                       Orientation::HorizontalUp   => PieceType::IType(Orientation::VerticalRight),
                       Orientation::VerticalRight  => PieceType::IType(Orientation::HorizontalDown),
                   };

               Piece {
                   piece_type: new_piece_type,
                   position: new_position,
               }
           },
           PieceType::OType => {
               // why would you try to rotate a square??
               self.clone()
           },
           _ => {
               let new_position = add_offset(&rel_rotated_coords, center_coord);
               Piece { position: new_position, .. *self }
           }
        }
    }

    /// Calculate the new tetrimino locations for a piece rotated counterclockwise
    pub fn ccw_rot(&self) -> Piece {
        // The idea for rotating is:
        // 1. Make all the points relative one of the piece's tetriminoes.
        // 2. Rotate -90 degrees around that point.
        // 3. Translate the point back to it's origin
        //     - In the case of the I piece, there's an associated adjustment

        // save center point for translation
        let center_coord = self.position[0];
        let relative_coords = make_relative(&self.position);
        let rel_rotated_coords = 
            [ Coord {x: -relative_coords[0].y, y: relative_coords[0].x}, 
              Coord {x: -relative_coords[1].y, y: relative_coords[1].x}, 
              Coord {x: -relative_coords[2].y, y: relative_coords[2].x}, 
              Coord {x: -relative_coords[3].y, y: relative_coords[3].x} ];

        match self.piece_type {
           PieceType::IType(orientation) => {
               let cw_offset = I_CW_OFFSETS[orientation as usize];
               // the offset when rotating counter clockwise, happens to the be 90 degree clockwise
               // rotation of the clockwise offset. If you want to prove it to yourself, just draw
               // it out.
               let offset = Coord { x: cw_offset.y, y: -cw_offset.x };
               let new_position = add_offset(&rel_rotated_coords, center_coord + offset);
               let new_piece_type = 
                   match orientation {
                       Orientation::HorizontalDown => PieceType::IType(Orientation::VerticalRight),
                       Orientation::VerticalLeft   => PieceType::IType(Orientation::HorizontalDown),
                       Orientation::HorizontalUp   => PieceType::IType(Orientation::VerticalLeft),
                       Orientation::VerticalRight  => PieceType::IType(Orientation::HorizontalUp),
                   };

               Piece {
                   piece_type: new_piece_type,
                   position: new_position,
               }
           },
           PieceType::OType => {
               // why would you try to rotate a square??
               self.clone()
           },
           _ => {
               let new_position = add_offset(&rel_rotated_coords, center_coord);
               Piece { position: new_position, .. *self }
           }
        }

    }
}

pub const PIECE_TYPES : [Piece; 7] = [
    Piece { piece_type: PieceType::IType(Orientation::HorizontalDown),
                position: I_COORDS, },
    Piece { piece_type: PieceType::OType,
                position: O_COORDS, },
    Piece { piece_type: PieceType::JType,
                position: J_COORDS, },
    Piece { piece_type: PieceType::LType,
                position: L_COORDS, },
    Piece { piece_type: PieceType::SType,
                position: S_COORDS, },
    Piece { piece_type: PieceType::ZType,
                position: Z_COORDS, },
    Piece { piece_type: PieceType::TType,
                position: T_COORDS, },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_coordinate_test() {
        let coords = [
            Coord {x: 1, y: -2}, // make coordinates relative to (1, -2)
            Coord {x: 2, y: -4},
            Coord {x: -1, y: 2},
            Coord {x: 1, y: 3},
        ];

        let relative_coords = make_relative(&coords);

        let expected_results = [
            Coord {x: 0, y: 0}, // make coordinates relative to (1, -2)
            Coord {x: 1, y: -2},
            Coord {x: -2, y: 4},
            Coord {x: 0, y: 5},
        ];
        
        assert_eq!(relative_coords, expected_results);
    }

    #[test]
    fn add_offset_test0() {
        let offset = Coord { x: 1, y: 1 };

        let coords = [
            Coord {x: 3, y: 4},
            Coord {x: 2, y: -5},
            Coord {x: -7, y: 2},
            Coord {x: 7, y: -3},
        ];

        let new_coords = add_offset(&coords, offset);

        let expected_results = [
            Coord {x: 4, y: 5},
            Coord {x: 3, y: -4},
            Coord {x: -6, y: 3},
            Coord {x: 8, y: -2},
        ];

        assert_eq!(new_coords, expected_results);
    }

    #[test]
    fn add_offset_test1() {
        let offset = Coord { x: 1, y: -2 };

        let coords = [
            Coord {x: 3, y: 4},
            Coord {x: 2, y: -5},
            Coord {x: -7, y: 2},
            Coord {x: 7, y: -3},
        ];

        let new_coords = add_offset(&coords, offset);

        let expected_results = [
            Coord {x: 4, y: 2},
            Coord {x: 3, y: -7},
            Coord {x: -6, y: 0},
            Coord {x: 8, y: -5},
        ];

        assert_eq!(new_coords, expected_results);
    }

    #[test]
    fn add_offset_test2() {
        let offset = Coord { x: -3, y: -2 };

        let coords = [
            Coord {x: 3, y: 4},
            Coord {x: 2, y: -5},
            Coord {x: -7, y: 2},
            Coord {x: 7, y: -3},
        ];

        let new_coords = add_offset(&coords, offset);

        let expected_results = [
            Coord {x: 0, y: 2},
            Coord {x: -1, y: -7},
            Coord {x: -10, y: 0},
            Coord {x: 4, y: -5},
        ];

        assert_eq!(new_coords, expected_results);
    }

    #[test]
    fn add_offset_test4() {
        let offset = Coord { x: -3, y: 2 };

        let coords = [
            Coord {x: 3, y: 4},
            Coord {x: 2, y: -5},
            Coord {x: -7, y: 2},
            Coord {x: 7, y: -3},
        ];

        let new_coords = add_offset(&coords, offset);

        let expected_results = [
            Coord {x: 0, y: 6},
            Coord {x: -1, y: -3},
            Coord {x: -10, y: 4},
            Coord {x: 4, y: -1},
        ];

        assert_eq!(new_coords, expected_results);
    }

    #[test]
    fn add_offset_test5() {
        let offset = Coord { x: 0, y: 0 };

        let coords = [
            Coord {x: 3, y: 4},
            Coord {x: 2, y: -5},
            Coord {x: -7, y: 2},
            Coord {x: 7, y: -3},
        ];

        let new_coords = add_offset(&coords, offset);

        let expected_results = [
            Coord {x: 3, y: 4},
            Coord {x: 2, y: -5},
            Coord {x: -7, y: 2},
            Coord {x: 7, y: -3},
        ];

        assert_eq!(new_coords, expected_results);
    }

    // ------------------------------------
    //         I PIECE ROTATIONS
    // ------------------------------------
    #[test]
    // Test rotating from HorizontalDown to VerticalLeft
    // [22]| [ ][ ][ ][ ]      [ ][a][ ][ ]
    // [21]| [ ][ ][ ][ ]  -\  [ ][b][ ][ ]
    // [20]| [a][b][c][d]  -/  [ ][c][ ][ ]
    // [19]| [ ][ ][ ][ ]      [ ][d][ ][ ]
    //     +-------------------------------
    //       [3][4][5][6]      [3][4][5][6]
    // a = index 3
    // b = index 2
    // c = index 1
    // d = index 0
    fn i_piece_cw_rot_horizdown_to_vertleft() {
        // get the I piece in the HorizontalDown orientation
        let piece = PIECE_TYPES[0].clone();

        let rotated = piece.cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::IType(Orientation::VerticalLeft),
                position: [
                    Coord { x: 4, y: 19 }, // d
                    Coord { x: 4, y: 20 }, // c
                    Coord { x: 4, y: 21 }, // b
                    Coord { x: 4, y: 22 }, // a
                ],
            };

        assert_eq!(rotated, expected_result);
    }

    #[test]
    // Test rotating from HorizontalDown to HorizUp
    // [22]| [ ][ ][ ][ ]      [ ][a][ ][ ]      [ ][ ][ ][ ]
    // [21]| [ ][ ][ ][ ]  -\  [ ][b][ ][ ]  -\  [d][c][b][a]
    // [20]| [a][b][c][d]  -/  [ ][c][ ][ ]  -/  [ ][ ][ ][ ]
    // [19]| [ ][ ][ ][ ]      [ ][d][ ][ ]      [ ][ ][ ][ ]
    //     +------------------------------------------------
    //       [3][4][5][6]      [3][4][5][6]      [3][4][5][6]
    // a = index 3
    // b = index 2
    // c = index 1
    // d = index 0
    fn i_piece_cw_rot_horizdown_to_horizup() {
        // get the I piece in the HorizontalDown orientation
        let piece = PIECE_TYPES[0].clone();

        let rotated = piece.cw_rot().cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::IType(Orientation::HorizontalUp),
                position: [
                    Coord { x: 3, y: 21 }, // d
                    Coord { x: 4, y: 21 }, // c
                    Coord { x: 5, y: 21 }, // b
                    Coord { x: 6, y: 21 }, // a
                ],
            };

        assert_eq!(rotated, expected_result);
    }

    #[test]
    // Test rotating from HorizontalDown to VerticalRight
    // [22]| [ ][ ][ ][ ]      [ ][a][ ][ ]      [ ][ ][ ][ ]      [ ][ ][d][ ]
    // [21]| [ ][ ][ ][ ]  -\  [ ][b][ ][ ]  -\  [d][c][b][a]  -\  [ ][ ][c][ ]
    // [20]| [a][b][c][d]  -/  [ ][c][ ][ ]  -/  [ ][ ][ ][ ]  -/  [ ][ ][b][ ]
    // [19]| [ ][ ][ ][ ]      [ ][d][ ][ ]      [ ][ ][ ][ ]      [ ][ ][a][ ]
    //     +-------------------------------------------------------------------
    //       [3][4][5][6]      [3][4][5][6]      [3][4][5][6]      [3][4][5][6]
    // a = index 3
    // b = index 2
    // c = index 1
    // d = index 0
    fn i_piece_cw_rot_horizdown_to_vertright() {
        // get the I piece in the HorizontalDown orientation
        let piece = PIECE_TYPES[0].clone();

        let rotated = piece.cw_rot().cw_rot().cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::IType(Orientation::VerticalRight),
                position: [
                    Coord { x: 5, y: 22 }, // d
                    Coord { x: 5, y: 21 }, // c
                    Coord { x: 5, y: 20 }, // b
                    Coord { x: 5, y: 19 }, // a
                ],
            };

        
        
        assert_eq!(rotated, expected_result);
    }

    #[test]
    fn i_piece_cw_rot_horizdown_to_horizdown() {
        // get the I piece in the HorizontalDown orientation
        let piece = PIECE_TYPES[0].clone();

        // 4 rotations should take us right back to where we started
        let rotated = piece.cw_rot().cw_rot().cw_rot().cw_rot();

        let expected_result = PIECE_TYPES[0].clone();

        assert_eq!(rotated, expected_result);
    }

    #[test]
    // Test rotating from HorizontalDown to VerticalRight
    // [22]| [ ][ ][ ][ ]      [ ][ ][d][ ]
    // [21]| [ ][ ][ ][ ]  -\  [ ][ ][c][ ]
    // [20]| [a][b][c][d]  -/  [ ][ ][b][ ]
    // [19]| [ ][ ][ ][ ]      [ ][ ][a][ ]
    //     +-------------------------------
    //       [3][4][5][6]      [3][4][5][6]
    // a = index 3
    // b = index 2
    // c = index 1
    // d = index 0
    fn i_piece_counter_cw_rot_horizdown_to_vertleft() {
        // get the I piece in the HorizontalDown orientation
        let piece = PIECE_TYPES[0].clone();

        let rotated = piece.ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::IType(Orientation::VerticalRight),
                position: [
                    Coord { x: 5, y: 22 }, // d
                    Coord { x: 5, y: 21 }, // c
                    Coord { x: 5, y: 20 }, // b
                    Coord { x: 5, y: 19 }, // a
                ],
            };

        assert_eq!(rotated, expected_result);
    }

    #[test]
    // Test rotating from HorizontalDown to HorizUp
    // [22]| [ ][ ][ ][ ]      [ ][ ][d][ ]      [ ][ ][ ][ ]
    // [21]| [ ][ ][ ][ ]  -\  [ ][ ][c][ ]  -\  [d][c][b][a]
    // [20]| [a][b][c][d]  -/  [ ][ ][b][ ]  -/  [ ][ ][ ][ ]
    // [19]| [ ][ ][ ][ ]      [ ][ ][a][ ]      [ ][ ][ ][ ]
    //     +------------------------------------------------
    //       [3][4][5][6]      [3][4][5][6]      [3][4][5][6]
    // a = index 3
    // b = index 2
    // c = index 1
    // d = index 0
    fn i_piece_counter_cw_rot_horizdown_to_horizup() {
        // get the I piece in the HorizontalDown orientation
        let piece = PIECE_TYPES[0].clone();

        let rotated = piece.ccw_rot().ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::IType(Orientation::HorizontalUp),
                position: [
                    Coord { x: 3, y: 21 }, // d
                    Coord { x: 4, y: 21 }, // c
                    Coord { x: 5, y: 21 }, // b
                    Coord { x: 6, y: 21 }, // a
                ],
            };

        assert_eq!(rotated, expected_result);
    }

    #[test]
    // Test rotating from HorizontalDown to VerticalLeft
    // [22]| [ ][ ][ ][ ]      [ ][ ][d][ ]      [ ][ ][ ][ ]      [ ][a][ ][ ]
    // [21]| [ ][ ][ ][ ]  -\  [ ][ ][c][ ]  -\  [d][c][b][a]  -\  [ ][b][ ][ ]
    // [20]| [a][b][c][d]  -/  [ ][ ][b][ ]  -/  [ ][ ][ ][ ]  -/  [ ][c][ ][ ]
    // [19]| [ ][ ][ ][ ]      [ ][ ][a][ ]      [ ][ ][ ][ ]      [ ][d][ ][ ]
    //     +-------------------------------------------------------------------
    //       [3][4][5][6]      [3][4][5][6]      [3][4][5][6]      [3][4][5][6]
    // a = index 3
    // b = index 2
    // c = index 1
    // d = index 0
    fn i_piece_counter_cw_rot_horizdown_to_vertright() {
        // get the I piece in the HorizontalDown orientation
        let piece = PIECE_TYPES[0].clone();

        let rotated = piece.ccw_rot().ccw_rot().ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::IType(Orientation::VerticalLeft),
                position: [
                    Coord { x: 4, y: 19 }, // d
                    Coord { x: 4, y: 20 }, // c
                    Coord { x: 4, y: 21 }, // b
                    Coord { x: 4, y: 22 }, // a
                ],
            };
        
        assert_eq!(rotated, expected_result);
    }

    #[test]
    fn i_piece_counter_cw_rot_horizdown_to_horizdown() {
        // get the I piece in the HorizontalDown orientation
        let piece = PIECE_TYPES[0].clone();

        // 4 rotations should take us right back to where we started
        let rotated = piece.ccw_rot().ccw_rot().ccw_rot().ccw_rot();

        let expected_result = PIECE_TYPES[0].clone();

        assert_eq!(rotated, expected_result);
    }

    // ------------------------------------
    //         O PIECE ROTATIONS
    // ------------------------------------
    #[test]
    fn o_piece_cw_rot_doesnt_change() {
        let piece = PIECE_TYPES[1].clone();

        let rotated = piece.cw_rot();

        let expected_result = PIECE_TYPES[1].clone();

        assert_eq!(rotated, expected_result);
    }

    #[test]
    fn o_piece_ccw_rot_doesnt_change() {
        let piece = PIECE_TYPES[1].clone();

        let rotated = piece.ccw_rot();

        let expected_result = PIECE_TYPES[1].clone();

        assert_eq!(rotated, expected_result);
    }

    // ------------------------------------
    //         J PIECE ROTATIONS
    // ------------------------------------
    #[test]
    // [21]| [c][ ][ ]    [ ][b][c]
    // [20]| [b][a][d] => [ ][a][ ]
    // [19]| [ ][ ][ ]    [ ][d][ ]
    //     + ----------------------
    //       [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn j_piece_single_cw_rot() {
        let piece = PIECE_TYPES[2].clone();

        let rotated = piece.cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::JType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 21 }, // b
                    Coord { x: 5, y: 21 }, // c
                    Coord { x: 4, y: 19 }, // d
                ],
            };

        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [c][ ][ ]    [ ][b][c]    [ ][ ][ ]
    // [20]| [b][a][d] => [ ][a][ ] => [d][a][b]
    // [19]| [ ][ ][ ]    [ ][d][ ]    [ ][ ][c]
    //     + -----------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn j_piece_2_cw_rot() {
        let piece = PIECE_TYPES[2].clone();

        let rotated = piece.cw_rot().cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::JType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 5, y: 20 }, // b
                    Coord { x: 5, y: 19 }, // c
                    Coord { x: 3, y: 20 }, // d
                ],
            };

        assert_eq!(rotated, expected_result);
    }
    #[test]
    // [21]| [c][ ][ ]    [ ][b][c]    [ ][ ][ ]    [ ][d][ ]
    // [20]| [b][a][d] => [ ][a][ ] => [d][a][b] => [ ][a][ ]
    // [19]| [ ][ ][ ]    [ ][d][ ]    [ ][ ][c]    [c][b][ ]
    //     + ------------------------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn j_piece_3_cw_rot() {
        let piece = PIECE_TYPES[2].clone();

        let rotated = piece.cw_rot().cw_rot().cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::JType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 19 }, // b
                    Coord { x: 3, y: 19 }, // c
                    Coord { x: 4, y: 21 }, // d
                ],
            };

        assert_eq!(rotated, expected_result);
    }

    #[test]
    fn j_piece_4_cw_rot_no_change() {
        let piece = PIECE_TYPES[2].clone();

        let rotated = piece.cw_rot().cw_rot().cw_rot().cw_rot();

        let expected_result = PIECE_TYPES[2];

        assert_eq!(rotated, expected_result);
    }
    #[test]
    // [21]| [c][ ][ ]    [ ][d][ ]
    // [20]| [b][a][d] => [ ][a][ ]
    // [19]| [ ][ ][ ]    [c][b][ ]
    //     + ----------------------
    //       [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn j_piece_single_ccw_rot() {
        let piece = PIECE_TYPES[2].clone();

        let rotated = piece.ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::JType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 19 }, // b
                    Coord { x: 3, y: 19 }, // c
                    Coord { x: 4, y: 21 }, // d
                ],
            };

        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [c][ ][ ]    [ ][d][ ]    [ ][ ][ ]
    // [20]| [b][a][d] => [ ][a][ ] => [d][a][b]
    // [19]| [ ][ ][ ]    [c][b][ ]    [ ][ ][c]
    //     + -----------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn j_piece_2_ccw_rot() {
        let piece = PIECE_TYPES[2].clone();

        let rotated = piece.ccw_rot().ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::JType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 5, y: 20 }, // b
                    Coord { x: 5, y: 19 }, // c
                    Coord { x: 3, y: 20 }, // d
                ],
            };

        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [c][ ][ ]    [ ][d][ ]    [ ][ ][ ]    [ ][b][c]
    // [20]| [b][a][d] => [ ][a][ ] => [d][a][b] => [ ][a][ ]
    // [19]| [ ][ ][ ]    [c][b][ ]    [ ][ ][c]    [ ][d][ ]
    //     + ------------------------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn j_piece_3_ccw_rot() {
        let piece = PIECE_TYPES[2].clone();

        let rotated = piece.ccw_rot().ccw_rot().ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::JType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 21 }, // b
                    Coord { x: 5, y: 21 }, // c
                    Coord { x: 4, y: 19 }, // d
                ],
            };

        assert_eq!(rotated, expected_result);
    }

    #[test]
    fn j_piece_4_ccw_rot_no_change() {
        let piece = PIECE_TYPES[2].clone();

        let rotated = piece.ccw_rot().ccw_rot().ccw_rot().ccw_rot();

        let expected_result = PIECE_TYPES[2];

        assert_eq!(rotated, expected_result);
    }

    // ------------------------------------
    //         L PIECE ROTATIONS
    // ------------------------------------
    #[test]
    // [21]| [ ][ ][d]    [ ][b][ ]
    // [20]| [b][a][c] => [ ][a][ ]
    // [19]| [ ][ ][ ]    [ ][c][d]
    //     + ----------------------
    //       [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn l_piece_single_cw_rot() {
        let piece = PIECE_TYPES[3].clone();

        let rotated = piece.cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::LType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 21 }, // b
                    Coord { x: 4, y: 19 }, // c
                    Coord { x: 5, y: 19 }, // d
                ],
            };

        assert_eq!(rotated, expected_result);
        
    }

    #[test]
    // [21]| [ ][ ][d]    [ ][b][ ]    [ ][ ][ ]
    // [20]| [b][a][c] => [ ][a][ ] => [c][a][b]
    // [19]| [ ][ ][ ]    [ ][c][d]    [d][ ][ ]
    //     + -----------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn l_piece_2_cw_rot() {
        let piece = PIECE_TYPES[3].clone();

        let rotated = piece.cw_rot().cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::LType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 5, y: 20 }, // b
                    Coord { x: 3, y: 20 }, // c
                    Coord { x: 3, y: 19 }, // d
                ],
            };

        assert_eq!(rotated, expected_result);
        
    }

    #[test]
    // [21]| [ ][ ][d]    [ ][b][ ]    [ ][ ][ ]    [d][c][ ]
    // [20]| [b][a][c] => [ ][a][ ] => [c][a][b] => [ ][a][ ]
    // [19]| [ ][ ][ ]    [ ][c][d]    [d][ ][ ]    [ ][b][ ]
    //     + ------------------------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn l_piece_3_cw_rot() {
        let piece = PIECE_TYPES[3].clone();

        let rotated = piece.cw_rot().cw_rot().cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::LType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 19 }, // b
                    Coord { x: 4, y: 21 }, // c
                    Coord { x: 3, y: 21 }, // d
                ],
            };

        assert_eq!(rotated, expected_result);
        
    }

    #[test]
    fn l_piece_4_cw_rot_no_change() {
        let piece = PIECE_TYPES[3].clone();

        let rotated = piece.cw_rot().cw_rot().cw_rot().cw_rot();

        let expected_result = PIECE_TYPES[3].clone();

        assert_eq!(rotated, expected_result);
    }
    #[test]
    // [21]| [ ][ ][d]    [d][c][ ]
    // [20]| [b][a][c] => [ ][a][ ]
    // [19]| [ ][ ][ ]    [ ][b][ ]
    //     + ----------------------
    //       [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn l_piece_single_ccw_rot() {
        let piece = PIECE_TYPES[3].clone();

        let rotated = piece.ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::LType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 19 }, // b
                    Coord { x: 4, y: 21 }, // c
                    Coord { x: 3, y: 21 }, // d
                ],
            };

        assert_eq!(rotated, expected_result);
        
    }

    #[test]
    // [21]| [ ][ ][d]    [d][c][ ]    [ ][ ][ ]
    // [20]| [b][a][c] => [ ][a][ ] => [c][a][b]
    // [19]| [ ][ ][ ]    [ ][b][ ]    [d][ ][ ]
    //     + -----------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn l_piece_2_ccw_rot() {
        let piece = PIECE_TYPES[3].clone();

        let rotated = piece.ccw_rot().ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::LType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 5, y: 20 }, // b
                    Coord { x: 3, y: 20 }, // c
                    Coord { x: 3, y: 19 }, // d
                ],
            };

        assert_eq!(rotated, expected_result);
        
    }

    #[test]
    // [21]| [ ][ ][d]    [d][c][ ]    [ ][ ][ ]    [ ][b][ ]
    // [20]| [b][a][c] => [ ][a][ ] => [c][a][b] => [ ][a][ ]
    // [19]| [ ][ ][ ]    [ ][b][ ]    [d][ ][ ]    [ ][c][d]
    //     + ------------------------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn l_piece_3_ccw_rot() {
        let piece = PIECE_TYPES[3].clone();

        let rotated = piece.ccw_rot().ccw_rot().ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::LType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 21 }, // b
                    Coord { x: 4, y: 19 }, // c
                    Coord { x: 5, y: 19 }, // d
                ],
            };

        assert_eq!(rotated, expected_result);
        
    }

    #[test]
    fn l_piece_4_ccw_rot_no_change() {
        let piece = PIECE_TYPES[3].clone();

        let rotated = piece.ccw_rot().ccw_rot().ccw_rot().ccw_rot();

        let expected_result = PIECE_TYPES[3].clone();

        assert_eq!(rotated, expected_result);
    }

    // ------------------------------------
    //         S PIECE ROTATIONS
    // ------------------------------------
    #[test]
    // [21]| [ ][c][d]    [ ][b][ ]
    // [20]| [b][a][ ] => [ ][a][c]
    // [19]| [ ][ ][ ]    [ ][ ][d]
    //     + ----------------------
    //       [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn s_piece_single_cw_rot() {
        let piece = PIECE_TYPES[4].clone();

        let rotated = piece.cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::SType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 21 }, // b
                    Coord { x: 5, y: 20 }, // c
                    Coord { x: 5, y: 19 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [ ][c][d]    [ ][b][ ]    [ ][ ][ ]
    // [20]| [b][a][ ] => [ ][a][c] => [ ][a][b]
    // [19]| [ ][ ][ ]    [ ][ ][d]    [d][c][ ]
    //     + -----------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn s_piece_2_cw_rot() {
        let piece = PIECE_TYPES[4].clone();

        let rotated = piece.cw_rot().cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::SType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 5, y: 20 }, // b
                    Coord { x: 4, y: 19 }, // c
                    Coord { x: 3, y: 19 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [ ][c][d]    [ ][b][ ]    [ ][ ][ ]    [d][ ][ ]
    // [20]| [b][a][ ] => [ ][a][c] => [ ][a][b] => [c][a][ ]
    // [19]| [ ][ ][ ]    [ ][ ][d]    [d][c][ ]    [ ][b][ ]
    //     + ------------------------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn s_piece_3_cw_rot() {
        let piece = PIECE_TYPES[4].clone();

        let rotated = piece.cw_rot().cw_rot().cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::SType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 19 }, // b
                    Coord { x: 3, y: 20 }, // c
                    Coord { x: 3, y: 21 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    fn s_piece_4_cw_rot_no_change() {
        let piece = PIECE_TYPES[4].clone();

        let rotated = piece.cw_rot().cw_rot().cw_rot().cw_rot();

        let expected_result = PIECE_TYPES[4].clone();

        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [ ][c][d]    [d][ ][ ]
    // [20]| [b][a][ ] => [c][a][ ]
    // [19]| [ ][ ][ ]    [ ][b][ ]
    //     + ----------------------
    //       [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn s_piece_single_ccw_rot() {
        let piece = PIECE_TYPES[4].clone();

        let rotated = piece.ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::SType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 19 }, // b
                    Coord { x: 3, y: 20 }, // c
                    Coord { x: 3, y: 21 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [ ][c][d]    [d][ ][ ]    [ ][ ][ ]
    // [20]| [b][a][ ] => [c][a][ ] => [ ][a][b]
    // [19]| [ ][ ][ ]    [ ][b][ ]    [d][c][ ]
    //     + -----------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn s_piece_2_ccw_rot() {
        let piece = PIECE_TYPES[4].clone();

        let rotated = piece.ccw_rot().ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::SType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 5, y: 20 }, // b
                    Coord { x: 4, y: 19 }, // c
                    Coord { x: 3, y: 19 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [ ][c][d]    [d][ ][ ]    [ ][ ][ ]    [ ][b][ ]
    // [20]| [b][a][ ] => [c][a][ ] => [ ][a][b] => [ ][a][c]
    // [19]| [ ][ ][ ]    [ ][b][ ]    [d][c][ ]    [ ][ ][d]
    //     + ------------------------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn s_piece_3_ccw_rot() {
        let piece = PIECE_TYPES[4].clone();

        let rotated = piece.ccw_rot().ccw_rot().ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::SType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 21 }, // b
                    Coord { x: 5, y: 20 }, // c
                    Coord { x: 5, y: 19 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    fn s_piece_4_ccw_rot_no_change() {
        let piece = PIECE_TYPES[4].clone();

        let rotated = piece.ccw_rot().ccw_rot().ccw_rot().ccw_rot();

        let expected_result = PIECE_TYPES[4].clone();

        assert_eq!(rotated, expected_result);
    }

    // ------------------------------------
    //         Z PIECE ROTATIONS
    // ------------------------------------
    #[test]
    // [21]| [b][c][ ]    [ ][ ][b]
    // [20]| [ ][a][d] => [ ][a][c]
    // [19]| [ ][ ][ ]    [ ][d][ ]
    //     + ----------------------
    //       [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn z_piece_single_cw_rot() {
        let piece = PIECE_TYPES[5].clone();

        let rotated = piece.cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::ZType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 5, y: 21 }, // b
                    Coord { x: 5, y: 20 }, // c
                    Coord { x: 4, y: 19 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [b][c][ ]    [ ][ ][b]    [ ][ ][ ]
    // [20]| [ ][a][d] => [ ][a][c] => [d][a][ ]
    // [19]| [ ][ ][ ]    [ ][d][ ]    [ ][c][b]
    //     + -----------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn z_piece_2_cw_rot() {
        let piece = PIECE_TYPES[5].clone();

        let rotated = piece.cw_rot().cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::ZType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 5, y: 19 }, // b
                    Coord { x: 4, y: 19 }, // c
                    Coord { x: 3, y: 20 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [b][c][ ]    [ ][ ][b]    [ ][ ][ ]    [ ][d][ ]
    // [20]| [ ][a][d] => [ ][a][c] => [d][a][ ] => [c][a][ ]
    // [19]| [ ][ ][ ]    [ ][d][ ]    [ ][c][b]    [b][ ][ ]
    //     + ------------------------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn z_piece_3_cw_rot() {
        let piece = PIECE_TYPES[5].clone();

        let rotated = piece.cw_rot().cw_rot().cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::ZType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 3, y: 19 }, // b
                    Coord { x: 3, y: 20 }, // c
                    Coord { x: 4, y: 21 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    fn z_piece_4_cw_rot_no_change() {
        let piece = PIECE_TYPES[5].clone();

        let rotated = piece.cw_rot().cw_rot().cw_rot().cw_rot();

        let expected_result = PIECE_TYPES[5];

        assert_eq!(rotated, expected_result);
    }
    #[test]
    // [21]| [b][c][ ]    [ ][d][ ]
    // [20]| [ ][a][d] => [c][a][ ]
    // [19]| [ ][ ][ ]    [b][ ][ ]
    //     + ----------------------
    //       [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn z_piece_single_ccw_rot() {
        let piece = PIECE_TYPES[5].clone();

        let rotated = piece.ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::ZType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 3, y: 19 }, // b
                    Coord { x: 3, y: 20 }, // c
                    Coord { x: 4, y: 21 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [b][c][ ]    [ ][d][ ]    [ ][ ][ ]
    // [20]| [ ][a][d] => [c][a][ ] => [d][a][ ]
    // [19]| [ ][ ][ ]    [b][ ][ ]    [ ][c][b]
    //     + -----------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn z_piece_2_ccw_rot() {
        let piece = PIECE_TYPES[5].clone();

        let rotated = piece.ccw_rot().ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::ZType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 5, y: 19 }, // b
                    Coord { x: 4, y: 19 }, // c
                    Coord { x: 3, y: 20 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [b][c][ ]    [ ][d][ ]    [ ][ ][ ]    [ ][ ][b]
    // [20]| [ ][a][d] => [c][a][ ] => [d][a][ ] => [ ][a][c]
    // [19]| [ ][ ][ ]    [b][ ][ ]    [ ][c][b]    [ ][d][ ]
    //     + ------------------------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn z_piece_3_ccw_rot() {
        let piece = PIECE_TYPES[5].clone();

        let rotated = piece.ccw_rot().ccw_rot().ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::ZType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 5, y: 21 }, // b
                    Coord { x: 5, y: 20 }, // c
                    Coord { x: 4, y: 19 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    fn z_piece_4_ccw_rot_no_change() {
        let piece = PIECE_TYPES[5].clone();

        let rotated = piece.ccw_rot().ccw_rot().ccw_rot().ccw_rot();

        let expected_result = PIECE_TYPES[5];
        assert_eq!(rotated, expected_result);
    }

    // ------------------------------------
    //         T PIECE ROTATIONS
    // ------------------------------------
    #[test]
    // [21]| [ ][c][ ]    [ ][b][ ]
    // [20]| [b][a][d] => [ ][a][c]
    // [19]| [ ][ ][ ]    [ ][d][ ]
    //     + ----------------------
    //       [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn t_piece_single_cw_rot() {
        let piece = PIECE_TYPES[6].clone();

        let rotated = piece.cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::TType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 21 }, // b
                    Coord { x: 5, y: 20 }, // c
                    Coord { x: 4, y: 19 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [ ][c][ ]    [ ][b][ ]    [ ][ ][ ]
    // [20]| [b][a][d] => [ ][a][c] => [d][a][b]
    // [19]| [ ][ ][ ]    [ ][d][ ]    [ ][c][ ]
    //     + -----------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn t_piece_2_cw_rot() {
        let piece = PIECE_TYPES[6].clone();

        let rotated = piece.cw_rot().cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::TType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 5, y: 20 }, // b
                    Coord { x: 4, y: 19 }, // c
                    Coord { x: 3, y: 20 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [ ][c][ ]    [ ][b][ ]    [ ][ ][ ]    [ ][d][ ]
    // [20]| [b][a][d] => [ ][a][c] => [d][a][b] => [c][a][ ]
    // [19]| [ ][ ][ ]    [ ][d][ ]    [ ][c][ ]    [ ][b][ ]
    //     + ------------------------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn t_piece_3_cw_rot() {
        let piece = PIECE_TYPES[6].clone();

        let rotated = piece.cw_rot().cw_rot().cw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::TType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 19 }, // b
                    Coord { x: 3, y: 20 }, // c
                    Coord { x: 4, y: 21 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    fn t_piece_4_cw_rot_no_change() {
        let piece = PIECE_TYPES[6].clone();

        let rotated = piece.cw_rot().cw_rot().cw_rot().cw_rot();

        let expected_result = PIECE_TYPES[6].clone();

        assert_eq!(rotated, expected_result);
    }
    #[test]
    // [21]| [ ][c][ ]    [ ][d][ ]
    // [20]| [b][a][d] => [c][a][ ]
    // [19]| [ ][ ][ ]    [ ][b][ ]
    //     + ----------------------
    //       [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn t_piece_single_ccw_rot() {
        let piece = PIECE_TYPES[6].clone();

        let rotated = piece.ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::TType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 19 }, // b
                    Coord { x: 3, y: 20 }, // c
                    Coord { x: 4, y: 21 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [ ][c][ ]    [ ][d][ ]    [ ][ ][ ]
    // [20]| [b][a][d] => [c][a][ ] => [d][a][b]
    // [19]| [ ][ ][ ]    [ ][b][ ]    [ ][c][ ]
    //     + -----------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn t_piece_2_ccw_rot() {
        let piece = PIECE_TYPES[6].clone();

        let rotated = piece.ccw_rot().ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::TType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 5, y: 20 }, // b
                    Coord { x: 4, y: 19 }, // c
                    Coord { x: 3, y: 20 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    // [21]| [ ][c][ ]    [ ][d][ ]    [ ][ ][ ]    [ ][b][ ]
    // [20]| [b][a][d] => [c][a][ ] => [d][a][b] => [ ][a][c]
    // [19]| [ ][ ][ ]    [ ][b][ ]    [ ][c][ ]    [ ][d][ ]
    //     + ------------------------------------------------
    //       [3][4][5]    [3][4][5]    [3][4][5]    [3][4][5]
    //
    // a = index 0
    // b = index 1
    // c = index 2
    // d = index 3
    fn t_piece_3_ccw_rot() {
        let piece = PIECE_TYPES[6].clone();

        let rotated = piece.ccw_rot().ccw_rot().ccw_rot();

        let expected_result =
            Piece {
                piece_type: PieceType::TType,
                position: [
                    Coord { x: 4, y: 20 }, // a
                    Coord { x: 4, y: 21 }, // b
                    Coord { x: 5, y: 20 }, // c
                    Coord { x: 4, y: 19 }, // d
                ],
            };
        assert_eq!(rotated, expected_result);
    }

    #[test]
    fn t_piece_4_ccw_rot_no_change() {
        let piece = PIECE_TYPES[6].clone();

        let rotated = piece.ccw_rot().ccw_rot().ccw_rot().ccw_rot();

        let expected_result = PIECE_TYPES[6].clone();

        assert_eq!(rotated, expected_result);
    }
}
