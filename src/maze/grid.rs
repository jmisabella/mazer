use crate::maze::cell::MazeType;
use crate::maze::cell::Cell;
use crate::maze::cell::Coordinates;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone)]
pub struct Grid {
    width: u32,
    height: u32,
    maze_type: MazeType,
    cells: Vec<Vec<Cell>>,
    seed: ThreadRng,
    start_coords: Coordinates,
    goal_coords: Coordinates,
}