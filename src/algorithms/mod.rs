use std::fmt;
use serde::{ Serialize, Deserialize };
use crate::behaviors::display::JsonDisplay;
use crate::behaviors::maze::MazeGeneration;
use crate::error::Error;
use crate::grid::Grid;
use crate::algorithms::binary_tree::BinaryTree;
use crate::algorithms::sidewinder::Sidewinder;
use crate::algorithms::aldous_broder::AldousBroder;
use crate::algorithms::wilsons::Wilsons;
use crate::algorithms::hunt_and_kill::HuntAndKill;
use crate::algorithms::recursive_backtracker::RecursiveBacktracker;
use crate::algorithms::prims::Prims;
use crate::algorithms::kruskals::Kruskals;
use crate::algorithms::growing_tree::GrowingTree;
use crate::algorithms::ellers::Ellers;
use crate::algorithms::recursive_division::RecursiveDivision;

pub mod binary_tree;
pub mod sidewinder;
pub mod aldous_broder;
pub mod wilsons;
pub mod hunt_and_kill;
pub mod recursive_backtracker;
pub mod prims;
pub mod kruskals;
pub mod growing_tree;
pub mod ellers;
pub mod recursive_division;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MazeAlgorithm {
    BinaryTree,
    Sidewinder,
    AldousBroder,
    Wilsons,
    HuntAndKill,
    RecursiveBacktracker,
    Prims,
    Kruskals,
    GrowingTree,
    Ellers,
    RecursiveDivision,
}

impl MazeAlgorithm {
    pub fn generate<'a>(&self, grid: &'a mut Grid) -> Result<&'a Grid, Error> {
        match self {
            MazeAlgorithm::BinaryTree => BinaryTree.build(grid),
            MazeAlgorithm::Sidewinder => Sidewinder.build(grid),
            MazeAlgorithm::AldousBroder => AldousBroder.build(grid),
            MazeAlgorithm::Wilsons => Wilsons.build(grid),
            MazeAlgorithm::HuntAndKill => HuntAndKill.build(grid),
            MazeAlgorithm::RecursiveBacktracker => RecursiveBacktracker.build(grid),
            MazeAlgorithm::Prims => Prims.build(grid),
            MazeAlgorithm::Kruskals => Kruskals.build(grid),
            MazeAlgorithm::GrowingTree => GrowingTree.build(grid),
            MazeAlgorithm::Ellers => Ellers.build(grid),
            MazeAlgorithm::RecursiveDivision => RecursiveDivision.build(grid),
        }
    }
}

impl fmt::Display for MazeAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_json() {
            Ok(json) => write!(f, "{}", json),
            Err(_) => Err(fmt::Error),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::generate;

    #[test]
    fn test_recursive_backtracker_orthogonal_12_x_12_maze_generation_from_json() {
        let json = r#"
        {
            "maze_type": "Orthogonal",
            "width": 12,
            "height": 12,
            "algorithm": "RecursiveBacktracker",
            "start": { "x": 0, "y": 0 },
            "goal": { "x": 11, "y": 11 }
        }
        "#;
        match generate(json) {
            Ok(maze) => {
                assert!(maze.is_perfect_maze().unwrap());
                println!("\n\nRecursive Backtracker\n\n{}\n\n", maze.to_asci());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }
    
    #[test]
    fn test_recursive_backtracker_orthogonal_400_x_400_maze_generation_from_json() {
        let json = r#"
        {
            "maze_type": "Orthogonal",
            "width": 400,
            "height": 400,
            "algorithm": "RecursiveBacktracker",
            "start": { "x": 0, "y": 0 },
            "goal": { "x": 399, "y": 399 }
        }
        "#;
        match generate(json) {
            Ok(maze) => {
                assert!(maze.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }
}