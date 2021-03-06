use core::ops;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl ops::Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Coord {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Coord {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
