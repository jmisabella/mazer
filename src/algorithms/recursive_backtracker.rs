use crate::grid::Grid;
use crate::cell::Coordinates;
use crate::error::Error;

use std::collections::HashSet;

pub struct RecursiveBacktracker;

impl RecursiveBacktracker {
    pub fn generate(grid: &mut Grid) -> Result<(), Error> {
        // Create a stack to track the current path
        let mut stack: Vec<Coordinates> = Vec::new();
        let mut visited: HashSet<Coordinates> = HashSet::new();

        // Start at the start_coords
        stack.push(grid.start_coords);
        visited.insert(grid.start_coords);

        while let Some(current_coords) = stack.last().cloned() {
            // Get all unvisited neighbors
            let neighbors: Vec<Coordinates> = grid
                .get(current_coords)?
                .neighbors()
                .into_iter()
                .filter(|neighbor| !visited.contains(neighbor))
                .collect();

            if neighbors.is_empty() {
                // Backtrack if no unvisited neighbors
                stack.pop();
            } else {
                // Choose a random unvisited neighbor
                let random_index = {
                    let upper_bound = neighbors.len() - 1;
                    grid.bounded_random_usize(upper_bound)
                };
                let next_coords = neighbors[random_index];

                // Link current cell to the chosen neighbor
                grid.link(current_coords, next_coords);

                // Mark the neighbor as visited and push it onto the stack
                visited.insert(next_coords);
                stack.push(next_coords);
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
    fn generate_and_print_5_x_5_orthogonal_maze() {
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze());
                RecursiveBacktracker::generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
                println!("\n\nRecursive Backtracker\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze());
            }
            Err(e) => panic!("Unexpected error occurred running test: {:?}", e),
        }
    }

    #[test]
    fn generate_and_print_12_x_6_orthogonal_maze() {
        match Grid::new(MazeType::Orthogonal, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze());
                RecursiveBacktracker::generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
                println!("\n\nRecursive Backtracker\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_5_x_5_delta_maze() {
        match Grid::new(MazeType::Delta, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze());
                RecursiveBacktracker::generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
                assert!(grid.is_perfect_maze());
            }
            Err(e) => panic!("Unexpected error occurred running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_delta_maze() {
        match Grid::new(MazeType::Delta, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze());
                RecursiveBacktracker::generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
                assert!(grid.is_perfect_maze());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_5_x_5_sigma_maze() {
        match Grid::new(MazeType::Sigma, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze());
                RecursiveBacktracker::generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
                assert!(grid.is_perfect_maze());
            }
            Err(e) => panic!("Unexpected error occurred running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_sigma_maze() {
        match Grid::new(MazeType::Sigma, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze());
                RecursiveBacktracker::generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
                assert!(grid.is_perfect_maze());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

}

