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
    #[cfg(feature="full_redraw")]
    fn draw_board(&mut self);
    fn draw_block(&mut self, x: u8, y: u8, piece_type: TetriminoType);
    fn draw_score(&mut self, score: u32);
    fn draw_level(&mut self, level: usize);
}
