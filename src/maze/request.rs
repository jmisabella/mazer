use crate::maze::cell::Coordinates;
use crate::maze::cell::MazeType;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Algorithm {
    BinaryTree,
    Sidewinder,
    AldousBroder,
    Wilsons,
    HuntAndKill,
    RecursiveBacktracker
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MazeRequest {
    maze_type: MazeType,
    width: u32,
    height: u32,
    algorithm: Algorithm,
    start: Coordinates,
    goal: Coordinates,
}