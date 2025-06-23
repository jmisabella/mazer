use crate::behaviors::maze::MazeGeneration;
use crate::algorithms::MazeAlgorithm;
use crate::grid::Grid;
use crate::cell::{Coordinates, MazeType};
use crate::error::Error;

use std::collections::HashSet;

pub struct BinaryTree;

impl MazeGeneration for BinaryTree {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        match grid.maze_type {
            MazeType::Orthogonal => {} // proceed with maze generation for allowed Orthogonal (square) grid type
            MazeType::Rhombille => {} // proceed with maze generation for allowed Rhombille (diamond) grid type
            maze_type => {
                return Err(Error::AlgorithmUnavailableForMazeType {
                    algorithm: MazeAlgorithm::BinaryTree,
                    maze_type: maze_type,
                });
            }
        }
        let rows = grid.height;
        let cols = grid.width;

        // Capture initial state if capture_steps is true
        if grid.capture_steps {
            let changed_cells = HashSet::new(); // Empty set for initial state
            self.capture_step(grid, &changed_cells);
        }

        for row in 0..rows {
            for col in 0..cols {
                let current_coords = Coordinates { x: col, y: row };
                let right_exists = col + 1 < cols;
                let down_exists = row + 1 < rows;
                let carve_down = if right_exists && down_exists {
                    grid.random_bool() // Randomly decide between down and right
                } else {
                    !right_exists
                };
                if carve_down {
                    if down_exists {
                        let down_coords = Coordinates { x: col, y: row + 1 };
                        grid.link(current_coords, down_coords)?;
                        if grid.capture_steps {
                            let mut changed_cells = HashSet::new();
                            changed_cells.insert(current_coords);
                            changed_cells.insert(down_coords);
                            self.capture_step(grid, &changed_cells);
                        }
                    }
                } else {
                    if right_exists {
                        let right_coords = Coordinates { x: col + 1, y: row };
                        grid.link(current_coords, right_coords)?;
                        if grid.capture_steps {
                            let mut changed_cells = HashSet::new();
                            changed_cells.insert(current_coords);
                            changed_cells.insert(right_coords);
                            self.capture_step(grid, &changed_cells);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::{MazeType, Coordinates};

    #[test]
    fn print_5_x_5_orthogonal_maze() {
        match Grid::new(
            MazeType::Orthogonal,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
            false,
        ) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                BinaryTree
                    .generate(&mut grid)
                    .expect("BinaryTree maze generation failed");
                println!("\n\nBinary Tree\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }

    #[test]
    fn print_12_x_24_orthogonal_maze() {
        match Grid::new(
            MazeType::Orthogonal,
            12,
            24,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 11, y: 23 },
            false,
        ) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                BinaryTree
                    .generate(&mut grid)
                    .expect("BinaryTree maze generation failed");
                println!("\n\nBinary Tree\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }

    #[test]
    fn reject_5_x_5_delta_binary_tree_maze() {
        match Grid::new(
            MazeType::Delta,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
            false,
        ) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                match BinaryTree.generate(&mut grid) {
                    Ok(()) => {
                        panic!("Successfully generated a BinaryTree maze for a Delta grid, which should have been rejected!");
                    }
                    Err(e) => {
                        println!(
                            "As expected, Delta grid is rejected for BinaryTree maze generation: {:?}",
                            e
                        );
                    }
                }
            }
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }

    #[test]
    fn reject_5_x_5_sigma_binary_tree_maze() {
        match Grid::new(
            MazeType::Sigma,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
            false,
        ) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                match BinaryTree.generate(&mut grid) {
                    Ok(()) => {
                        panic!("Successfully generated a BinaryTree maze for a Sigma grid, which should have been rejected!");
                    }
                    Err(e) => {
                        println!(
                            "As expected, Sigma grid is rejected for BinaryTree maze generation: {:?}",
                            e
                        );
                    }
                }
            }
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_rhombille_maze_binary_tree() {
        match Grid::new(MazeType::Rhombille, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                BinaryTree.generate(&mut grid).expect("BinaryTree maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn test_binary_tree_with_capture_steps() {
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 19, y: 19 };
        match Grid::new(MazeType::Orthogonal, 20, 20, start, goal, true) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                BinaryTree.generate(&mut grid).expect("Maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
                assert!(grid.generation_steps.is_some());
                let steps = grid.generation_steps.as_ref().unwrap();
                assert!(!steps.is_empty());
                // Check if any cells become linked across all generation steps
                let has_linked_cells = steps.iter().any(|step| {
                    step.cells.iter().any(|cell| !cell.linked.is_empty())
                });
                assert!(has_linked_cells, "No cells were linked during maze generation");
                let has_open_walls = steps.iter().any(|step| {
                    step.cells.iter().any(|cell| !cell.open_walls.is_empty())
                });
                assert!(has_open_walls, "No cells have open walls in generation steps");
            }
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }
}
