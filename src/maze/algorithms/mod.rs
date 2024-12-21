
use crate::maze::grid::Grid;
use crate::maze::algorithms::binary_tree::BinaryTree;
use crate::maze::algorithms::sidewinder::Sidewinder;
use crate::maze::algorithms::aldous_broder::AldousBroder;
use crate::maze::algorithms::hunt_and_kill::HuntAndKill;
use crate::maze::algorithms::recursive_backtracker::RecursiveBacktracker;
use serde::{ Serialize, Deserialize };

pub mod binary_tree;
pub mod sidewinder;
pub mod aldous_broder;
pub mod hunt_and_kill;
pub mod recursive_backtracker;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MazeAlgorithm {
    BinaryTree,
    Sidewinder,
    AldousBroder,
    Wilsons,
    HuntAndKill,
    RecursiveBacktracker
}

impl MazeAlgorithm {
    pub fn generate(&self, grid: &mut Grid) -> Grid {
        match self {
            MazeAlgorithm::BinaryTree => BinaryTree::generate(grid),
            MazeAlgorithm::Sidewinder => Sidewinder::generate(grid),
            MazeAlgorithm::AldousBroder => AldousBroder::generate(grid),
            MazeAlgorithm::HuntAndKill => HuntAndKill::generate(grid),
            MazeAlgorithm::RecursiveBacktracker => RecursiveBacktracker::generate(grid),
            _ => unimplemented!()
        }
        return grid.clone();
    }
}