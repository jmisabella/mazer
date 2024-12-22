
use crate::grid::Grid;
use crate::algorithms::binary_tree::BinaryTree;
use crate::algorithms::sidewinder::Sidewinder;
use crate::algorithms::aldous_broder::AldousBroder;
use crate::algorithms::hunt_and_kill::HuntAndKill;
use crate::algorithms::recursive_backtracker::RecursiveBacktracker;
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
        let maze = generate(json);
        assert!(maze.is_perfect_maze());
        println!("\n\nRecursive Backtracker\n\n{}\n\n", maze.to_asci());

    }
}