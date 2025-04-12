use crate::{Grid, Error};

pub trait MazeGeneration {
    /// Generate a maze on the provided grid.
    fn generate(&self, grid: &mut Grid) -> Result<(), Error>;
}
