#[derive(Copy, Clone, Debug, PartialEq)]
/// Defines the different Tetrimino states for use by the renderer.
pub enum TetriminoType {
    EmptySpace,
    I,
    O,
    J,
    L,
    S,
    Z,
    T,
}

impl Default for TetriminoType {
    fn default() -> Self { TetriminoType::EmptySpace }
}

/// Define a trait for drawing the game state.
/// This allows the use of multiple backends.
pub trait GameRenderer {
    fn draw_block(&mut self, x: i32, y: i32, piece_type: TetriminoType);
}
