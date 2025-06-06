use crate::behaviors::maze::MazeGeneration;
use crate::algorithms::MazeAlgorithm;
use crate::grid::Grid;
use crate::cell::{Coordinates, MazeType};
use crate::error::Error;

pub struct BinaryTree;

impl MazeGeneration for BinaryTree {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        match grid.maze_type {
            MazeType::Orthogonal => {} // proceed with maze generation for allowed Orthogonal (square) grid type
            maze_type => {
                return Err(Error::AlgorithmUnavailableForMazeType{algorithm:MazeAlgorithm::BinaryTree, maze_type:maze_type});
            }
        }
        let rows = grid.height;
        let cols = grid.width;
        for row in 0..rows {
            for col in 0..cols {
                // Determine the existence of neighbors
                let right_exists = col + 1 < cols;
                let down_exists = row + 1 < rows;

                // Determine the direction to carve passage before borrowing `grid.cells`
                let carve_down = if right_exists && down_exists {
                    grid.random_bool() // Randomly decide between down and right
                } else {
                    // If only one neighbor exists, decide based on its existence
                    !right_exists
                };
                // Scope for mutable access to `grid.cells`
                if carve_down {
                    if down_exists {
                        let (current_coords, down_coords) = (
                            Coordinates { x: col, y: row },
                            Coordinates { x: col, y: row + 1 },
                        );
                        let index1 = grid.get_flattened_index(col, row);
                        grid.cells[index1].linked.insert(down_coords);
                        let index2 = grid.get_flattened_index(col, row + 1);
                        grid.cells[index2].linked.insert(current_coords);
                    }
                } else {
                    if right_exists {
                        let (current_coords, right_coords) = (
                            Coordinates { x: col, y: row },
                            Coordinates { x: col + 1, y: row },
                        );
                        let index1 = grid.get_flattened_index(col, row);
                        grid.cells[index1].linked.insert(right_coords);
                        let index2 = grid.get_flattened_index(col + 1, row);
                        grid.cells[index2].linked.insert(current_coords);
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
    use crate::cell::{ MazeType, Coordinates };
    
    #[test]
    fn print_5_x_5_orthogonal_maze() {
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                BinaryTree.generate(&mut grid).expect("BinaryTree maze generation failed");
                println!("\n\nBinary Tree\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }    
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }

    #[test]
    fn print_12_x_24_orthogonal_maze() {
        match Grid::new(MazeType::Orthogonal, 12, 24, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 23 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                BinaryTree.generate(&mut grid).expect("BinaryTree maze generation failed");
                println!("\n\nBinary Tree\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }    
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }

    #[test]
    fn reject_5_x_5_delta_binary_tree_maze() {
        match Grid::new(MazeType::Delta, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                match BinaryTree.generate(&mut grid) {
                    Ok(()) => {
                        panic!("Successfully generated a BinaryTree maze for a Delta grid, which is should have been rejected!");
                    }
                    Err(e) => {
                        println!("As expected, Delta grid is rejected for BinaryTree maze generation: {:?}", e);
                    }
                }
            }    
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }

    #[test]
    fn reject_5_x_5_sigma_binary_tree_maze() {
        match Grid::new(MazeType::Sigma, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                match BinaryTree.generate(&mut grid) {
                    Ok(()) => {
                        panic!("Successfully generated a BinaryTree maze for a Sigma grid, which is should have been rejected!");
                    }
                    Err(e) => {
                        println!("As expected, Sigma grid is rejected for BinaryTree maze generation: {:?}", e);
                    }
                }
            }    
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }
}
