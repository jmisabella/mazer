use crate::behaviors::maze::MazeGeneration;
use crate::grid::Grid;
use crate::cell::Coordinates;
use crate::error::Error;

use std::collections::{BinaryHeap, HashSet};
use rand::Rng;

// A structure to hold frontier cells with their weights for Prim's algorithm
#[derive(Eq, PartialEq)]
struct FrontierCell {
    coords: Coordinates,
    weight: u32, // Random weight for choosing the next cell
}

impl Ord for FrontierCell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering to make BinaryHeap a min-heap (lower weights first)
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for FrontierCell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Prims;

impl MazeGeneration for Prims {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        let mut visited: HashSet<Coordinates> = HashSet::new();
        let mut frontier: BinaryHeap<FrontierCell> = BinaryHeap::new();
        let mut rng = rand::thread_rng();

        // Step 1: Choose a random starting cell
        let start_coords = Coordinates {
            x: grid.bounded_random_usize(grid.width - 1),
            y: grid.bounded_random_usize(grid.height - 1),
        };
        visited.insert(start_coords);
        
        // Step 2: Add all neighbors of the starting cell to the frontier
        if let Ok(start_cell) = grid.get(start_coords) {
            for &neighbor_coords in start_cell.neighbors().iter() {
                frontier.push(FrontierCell {
                    coords: neighbor_coords,
                    weight: rng.gen(), // Assign a random weight
                });
            }
        }

        // Step 3: Process the frontier until it's empty
        while let Some(FrontierCell { coords, .. }) = frontier.pop() {
            if visited.contains(&coords) {
                continue; // Skip if already visited
            }

            // Mark the cell as visited
            visited.insert(coords);

            // Get neighbors and release the borrow
            let (visited_neighbors, unvisited_neighbors) = if let Ok(cell) = grid.get(coords) {
                let visited_neighbors: Vec<Coordinates> = cell
                    .neighbors()
                    .into_iter()
                    .filter(|neighbor| visited.contains(neighbor))
                    .collect();
                let unvisited_neighbors: Vec<Coordinates> = cell
                    .neighbors()
                    .into_iter()
                    .filter(|neighbor| !visited.contains(neighbor))
                    .collect();
                (visited_neighbors, unvisited_neighbors)
            } else {
                continue;
            };

            // Link to a visited neighbor if available
            if !visited_neighbors.is_empty() {
                let neighbor_index = grid.bounded_random_usize(visited_neighbors.len() - 1);
                let neighbor_coords = visited_neighbors[neighbor_index];
                grid.link(coords, neighbor_coords)?;
            }

            // Add unvisited neighbors to the frontier
            for neighbor_coords in unvisited_neighbors {
                frontier.push(FrontierCell {
                    coords: neighbor_coords,
                    weight: rng.gen(), // Assign a random weight
                });
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
    fn generate_and_print_5_x_5_orthogonal_maze() {
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Prims.generate(&mut grid).expect("Prim's maze generation failed");
                println!("\n\nPrim's\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_and_print_12_x_6_orthogonal_maze() {
        match Grid::new(MazeType::Orthogonal, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Prims.generate(&mut grid).expect("Prim's maze generation failed");
                println!("\n\nPrim's\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_5_x_5_delta_maze() {
        match Grid::new(MazeType::Delta, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Prims.generate(&mut grid).expect("Prim's maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_delta_maze() {
        match Grid::new(MazeType::Delta, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Prims.generate(&mut grid).expect("Prim's maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_5_x_5_sigma_maze() {
        match Grid::new(MazeType::Sigma, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Prims.generate(&mut grid).expect("Prim's maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_sigma_maze() {
        match Grid::new(MazeType::Sigma, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Prims.generate(&mut grid).expect("Prim's maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_12_polar_maze() {
        match Grid::new(MazeType::Polar, 12, 12, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 11 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Prims.generate(&mut grid).expect("Prim's maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_polar_maze() {
        match Grid::new(MazeType::Polar, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Prims.generate(&mut grid).expect("Prim's maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }
}