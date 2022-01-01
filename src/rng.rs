/// This trait is intended to be used to
/// permutate the order of the pieces in
/// the game after the current permutation
/// has been used.
pub trait Rng {
    fn next(&mut self) -> usize;
}
