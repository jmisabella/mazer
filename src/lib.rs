use crate::grid::Grid;
use crate::error::Error;

#[allow(unused_imports)]
pub use crate::ffi::*;

pub mod cell;
pub mod grid;
pub mod direction;
pub mod request;
pub mod algorithms;
pub mod error;
pub mod ffi;


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

pub fn generate(request_json: &str) -> Result<Grid, Error> {
    return Grid::try_from(request_json);
}


