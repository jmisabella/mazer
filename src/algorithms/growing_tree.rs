use crate::behaviors::maze::MazeGeneration;
use crate::grid::Grid;
use crate::cell::Coordinates;
use crate::error::Error;

use std::collections::HashSet;

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum SelectionStrategy {
    Random,
    Newest,
}

pub struct GrowingTree {
    pub strategy: SelectionStrategy,
}

impl MazeGeneration for GrowingTree {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        let mut active: Vec<Coordinates> = Vec::new();
        let mut visited: HashSet<Coordinates> = HashSet::new();

        // Start with a random cell
        let start_coords = Coordinates {
            x: grid.bounded_random_usize(grid.width),
            y: grid.bounded_random_usize(grid.height),
        };
        active.push(start_coords);
        visited.insert(start_coords);

        // Capture initial state with no changed cells
        if grid.capture_steps {
            let changed_cells = HashSet::new();
            self.capture_step(grid, &changed_cells);
        }

        while !active.is_empty() {
            // Choose a cell from active list based on strategy
            let index = match self.strategy {
                SelectionStrategy::Random => grid.bounded_random_usize(active.len()),
                SelectionStrategy::Newest => active.len() - 1,
            };
            let current_coords = active[index];

            // Get unvisited neighbors
            let unvisited_neighbors: Vec<Coordinates> = if let Ok(cell) = grid.get(current_coords) {
                cell.neighbors()
                    .into_iter()
                    .filter(|neighbor| !visited.contains(neighbor))
                    .collect()
            } else {
                Vec::new()
            };

            if unvisited_neighbors.is_empty() {
                // No unvisited neighbors, remove from active list
                active.swap_remove(index);
            } else {
                // Choose a random unvisited neighbor
                let neighbor_index = grid.bounded_random_usize(unvisited_neighbors.len());
                let next_coords = unvisited_neighbors[neighbor_index];

                // Link to the neighbor
                grid.link(current_coords, next_coords)?;

                // Mark neighbor as visited and add to active list
                visited.insert(next_coords);
                active.push(next_coords);

                // Capture step with changed cells after linking
                if grid.capture_steps {
                    let mut changed_cells = HashSet::new();
                    changed_cells.insert(current_coords);
                    changed_cells.insert(next_coords);
                    self.capture_step(grid, &changed_cells);
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
    fn generate_and_print_5_x_5_orthogonal_maze() {
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                GrowingTree{ strategy: SelectionStrategy::Random }.generate(&mut grid).expect("Growing Tree maze generation failed");
                println!("\n\nGrowing Tree\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_and_print_12_x_6_orthogonal_maze() {
        match Grid::new(MazeType::Orthogonal, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                GrowingTree{ strategy: SelectionStrategy::Newest }.generate(&mut grid).expect("Growing Tree maze generation failed");
                println!("\n\nGrowing Tree\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_5_x_5_delta_maze() {
        match Grid::new(MazeType::Delta, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                GrowingTree{ strategy: SelectionStrategy::Random }.generate(&mut grid).expect("Growing Tree maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_delta_maze() {
        match Grid::new(MazeType::Delta, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                GrowingTree{ strategy: SelectionStrategy::Newest }.generate(&mut grid).expect("Growing Tree maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_5_x_5_sigma_maze() {
        match Grid::new(MazeType::Sigma, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                GrowingTree{ strategy: SelectionStrategy::Random }.generate(&mut grid).expect("Growing Tree maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_sigma_maze() {
        match Grid::new(MazeType::Sigma, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                GrowingTree{ strategy: SelectionStrategy::Newest }.generate(&mut grid).expect("Growing Tree maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_23_growing_tree_rhombille_maze() {
        match Grid::new(MazeType::Rhombille, 12, 23, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 22 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                GrowingTree{ strategy: SelectionStrategy::Newest }.generate(&mut grid).expect("Growing Tree maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }


    #[test]
    fn test_growing_tree_with_capture_steps() {
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 11, y: 11 };
        match Grid::new(MazeType::Orthogonal, 12, 12, start, goal, true) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                GrowingTree{ strategy: SelectionStrategy::Random }.generate(&mut grid).expect("Maze generation failed");
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
