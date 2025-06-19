use crate::behaviors::maze::MazeGeneration;
use crate::grid::Grid;
use crate::cell::Coordinates;
use crate::error::Error;

use std::collections::HashSet;

pub struct HuntAndKill;

impl MazeGeneration for HuntAndKill {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        let mut visited = HashSet::new();
        let mut current_coords = Coordinates {
            x: grid.bounded_random_usize(grid.width),
            y: grid.bounded_random_usize(grid.height),
        };
        visited.insert(current_coords);

        // Capture initial state with no changed cells
        if grid.capture_steps {
            let changed_cells = HashSet::new();
            self.capture_step(grid, &changed_cells);
        }

        loop {
            // Kill Phase: Randomly carve passages to unvisited neighbors
            while let Some(next_coords) = Self::random_unvisited_neighbor(grid, &current_coords, &visited) {
                // Link the current cell with the chosen neighbor
                grid.link(current_coords, next_coords)?;
                visited.insert(next_coords);
                current_coords = next_coords;

                // Capture step with changed cells after linking
                if grid.capture_steps {
                    let mut changed_cells = HashSet::new();
                    changed_cells.insert(current_coords);
                    changed_cells.insert(next_coords);
                    self.capture_step(grid, &changed_cells);
                }
            }

            // Hunt Phase: Find the first unvisited cell with at least one visited neighbor
            if let Some((new_coords, neighbor)) = Self::find_hunt_target(grid, &visited) {
                // Link the new cell with one of its visited neighbors
                grid.link(new_coords, neighbor)?;
                visited.insert(new_coords);
                current_coords = new_coords;

                // Capture step with changed cells after linking
                if grid.capture_steps {
                    let mut changed_cells = HashSet::new();
                    changed_cells.insert(new_coords);
                    changed_cells.insert(neighbor);
                    self.capture_step(grid, &changed_cells);
                }
            } else {
                // No more unvisited cells, maze generation complete
                break;
            }
        }
        Ok(())
    }
}

impl HuntAndKill {
    /// Finds a random unvisited neighbor of the current cell.
    fn random_unvisited_neighbor(
        grid: &mut Grid,
        coords: &Coordinates,
        visited: &HashSet<Coordinates>,
    ) -> Option<Coordinates> {
        if let Ok(current_cell) = grid.get(*coords) {
            let neighbors: Vec<_> = current_cell
                .neighbors()
                .into_iter()
                .filter(|neighbor| !visited.contains(neighbor))
                .collect();

            if neighbors.is_empty() {
                None
            } else {
                let index = grid.bounded_random_usize(neighbors.len());
                Some(neighbors[index])
            }
        } else {
            None
        }
    }

    /// Finds the first unvisited cell with at least one visited neighbor.
    fn find_hunt_target(
        grid: &Grid,
        visited: &HashSet<Coordinates>,
    ) -> Option<(Coordinates, Coordinates)> {
        (0..grid.height)
            .flat_map(|y| (0..grid.width).map(move |x| Coordinates { x, y }))
            .find_map(|coords| {
                if visited.contains(&coords) {
                    None
                } else {
                    match grid.get(coords) {
                        Ok(current_cell) => current_cell
                            .neighbors()
                            .into_iter()
                            .find(|neighbor| visited.contains(neighbor))
                            .map(|neighbor| (coords, neighbor)),
                        Err(_) => None,
                    }
                }
            })
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
                HuntAndKill.generate(&mut grid).expect("HuntAndKill maze generation failed");
                println!("\n\nHunt-and-Kill\n\n{}\n\n", grid.to_asci());
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
                HuntAndKill.generate(&mut grid).expect("HuntAndKill maze generation failed");
                println!("\n\nHunt-and-Kill\n\n{}\n\n", grid.to_asci());
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
                HuntAndKill.generate(&mut grid).expect("HuntAndKill maze generation failed");
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
                HuntAndKill.generate(&mut grid).expect("HuntAndKill maze generation failed");
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
                HuntAndKill.generate(&mut grid).expect("HuntAndKill maze generation failed");
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
                HuntAndKill.generate(&mut grid).expect("HuntAndKill maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_23_hunt_and_kill_rhombille_maze() {
        match Grid::new(MazeType::Rhombille, 12, 23, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 22 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                HuntAndKill.generate(&mut grid).expect("Growing Tree maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn test_hunt_and_kill_with_capture_steps() {
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 11, y: 11 };
        match Grid::new(MazeType::Orthogonal, 12, 12, start, goal, true) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                HuntAndKill.generate(&mut grid).expect("Maze generation failed");
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
