use crate::maze::cell::MazeType;
use crate::maze::cell::Cell;
use crate::maze::cell::Coordinates;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone)] // Traits similar to Scala's case class behavior
pub struct Grid {
    width: u32,
    height: u32,
    maze_type: MazeType,
    cells: Vec<Vec<Cell>>,
    seed: ThreadRng,
    startCoords: Coordinates,
    goalCoords: Coordinates,
}