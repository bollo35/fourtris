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

// CONVENTION ALERT: The center point of these pieces is the first
//                   coordinate of the array.
// All these coordinates describe the pieces at their spawn point
const I_COORDS: [Coord; 4] = [
    Coord { x: 6, y: 20 },
    Coord { x: 4, y: 20 },
    Coord { x: 5, y: 20 },
    Coord { x: 3, y: 20 },
];

const O_COORDS: [Coord; 4] = [
    Coord { x: 4, y: 20 },
    Coord { x: 4, y: 21 },
    Coord { x: 5, y: 20 },
    Coord { x: 5, y: 21 },
];

const J_COORDS : [Coord; 4] = [
    Coord { x: 4, y: 20 },
    Coord { x: 3, y: 20 },
    Coord { x: 3, y: 21 },
    Coord { x: 5, y: 20 },
];

const L_COORDS : [Coord; 4] = [
    Coord { x: 4, y: 20 },
    Coord { x: 3, y: 20 },
    Coord { x: 5, y: 20 },
    Coord { x: 5, y: 21 },
];

const S_COORDS : [Coord; 4] = [
    Coord { x: 4, y: 20 },
    Coord { x: 3, y: 20 },
    Coord { x: 4, y: 21 },
    Coord { x: 5, y: 21 },
];

const Z_COORDS : [Coord; 4] = [
    Coord { x: 4, y: 20 },
    Coord { x: 3, y: 21 },
    Coord { x: 4, y: 21 },
    Coord { x: 5, y: 20 },
];

const T_COORDS : [Coord; 4] = [
    Coord { x: 4, y: 20 },
    Coord { x: 3, y: 20 },
    Coord { x: 4, y: 21 },
    Coord { x: 5, y: 20 },
];

// The I piece doesn't really have a center point.
// Thus, the code treats one of the end points as
// the center point. In order to rotate the piece
// properly, there is an additional translation offset.
// This offset changes based on the direction of rotation.
// The counterclockwise rotation offset coordinates, are just
// rotations of the clockwise offsets.
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

#[derive(Debug, Copy, Clone)]
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
    pub fn apply_gravity(&self, displacement: isize) -> Piece {
        let offset = if displacement > 0 {
            Coord { x: 0, y: -displacement }
        } else {
            Coord { x: 0, y: displacement }
        };

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
