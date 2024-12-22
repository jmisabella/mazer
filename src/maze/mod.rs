use crate::maze::grid::Grid;

pub mod cell;
pub mod grid;
pub mod direction;
pub mod request;
pub mod generators;
pub mod algorithms;

// algorithms: BinaryTree, Sidewinder, AldousBroder, HuntAndKill, RecursiveBacktracker
// maze_types: Orthogonal, Delta, Hex, Polar
// example json maze request, all fields are required:
// {
//     "maze_type": "Orthogonal",
//     "width": 12,
//     "height": 12,
//     "algorithm": "RecursiveBacktracker",
//     "start": { "x": 0, "y": 0 },
//     "goal": { "x": 11, "y": 11 }
// }
pub fn generate(json: &str) -> Grid {
    return Grid::from_json(json);
}