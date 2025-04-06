
use crate::error::Error;
use crate::grid::Grid;
use crate::algorithms::binary_tree::BinaryTree;
use crate::algorithms::sidewinder::Sidewinder;
use crate::algorithms::aldous_broder::AldousBroder;
use crate::algorithms::wilsons::Wilsons;
use crate::algorithms::hunt_and_kill::HuntAndKill;
use crate::algorithms::recursive_backtracker::RecursiveBacktracker;

use serde::{ Serialize, Deserialize };
use std::fmt;

pub mod binary_tree;
pub mod sidewinder;
pub mod aldous_broder;
pub mod wilsons;
pub mod hunt_and_kill;
pub mod recursive_backtracker;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MazeAlgorithm {
    BinaryTree,
    Sidewinder,
    AldousBroder,
    Wilsons,
    HuntAndKill,
    RecursiveBacktracker
}

impl MazeAlgorithm {
    pub fn generate<'a>(&self, grid: &'a mut Grid) -> Result<&'a Grid, Error> {
        match self {
            MazeAlgorithm::BinaryTree => BinaryTree::generate(grid)?,
            MazeAlgorithm::Sidewinder => Sidewinder::generate(grid)?,
            MazeAlgorithm::AldousBroder => AldousBroder::generate(grid)?,
            MazeAlgorithm::Wilsons => Wilsons::generate(grid)?,
            MazeAlgorithm::HuntAndKill => HuntAndKill::generate(grid)?,
            MazeAlgorithm::RecursiveBacktracker => RecursiveBacktracker::generate(grid)?,
        };
    
        let start = grid.start_coords;
        let goal = grid.goal_coords;
    
        // Set distances on all cells
        let all_distances = grid.distances(start);
        for (coords, distance) in all_distances {
            if let Ok(cell) = grid.get_mut(coords) {
                cell.distance = distance as i32;
            }
        }
    
        // Mark solution path
        if let Ok(path) = grid.get_path_to(start.x, start.y, goal.x, goal.y) {
            for coords in path.keys() {
                if let Ok(cell) = grid.get_mut(*coords) {
                    cell.on_solution_path = true;
                }
            }
        }

        for cell in grid.cells.iter_mut() {
            cell.set_open_walls();
        }

        Ok(grid)
    }

}

impl fmt::Display for MazeAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match serde_json::to_string(&self) {
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