
use crate::maze::grid::Grid;
use crate::maze::algorithms::binary_tree::BinaryTree;
use crate::maze::algorithms::sidewinder::Sidewinder;
use serde::{ Serialize, Deserialize };

pub mod binary_tree;
pub mod sidewinder;


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
            _ => unimplemented!()
        }
        return grid.clone();
    }
}