use crate::behaviors::maze::MazeGeneration;
use crate::algorithms::MazeAlgorithm;
use crate::grid::Grid;
use crate::cell::{Coordinates, MazeType};
use crate::error::Error;
use std::collections::HashSet;

pub struct RecursiveDivision;

impl MazeGeneration for RecursiveDivision {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        match grid.maze_type {
            MazeType::Orthogonal => {}
            maze_type => {
                return Err(Error::AlgorithmUnavailableForMazeType {
                    algorithm: MazeAlgorithm::RecursiveDivision,
                    maze_type,
                });
            }
        }

        // Link all adjacent cells to start with no walls
        for y in 0..grid.height {
            for x in 0..grid.width {
                let coords = Coordinates { x, y };
                if x + 1 < grid.width {
                    grid.link(coords, Coordinates { x: x + 1, y })?;
                }
                if y + 1 < grid.height {
                    grid.link(coords, Coordinates { x, y: y + 1 })?;
                }
            }
        }

        if grid.capture_steps {
            let changed_cells = HashSet::new();
            self.capture_step(grid, &changed_cells); // Captures fully open grid
        }

        self.divide(grid, 0, 0, grid.width, grid.height)?;
        Ok(())
    }
}

impl RecursiveDivision {
    fn divide(
        &self,
        grid: &mut Grid,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Result<(), Error> {
        if width <= 1 || height <= 1 {
            return Ok(());
        }

        let divide_horizontally = if height > width {
            true // Prefer horizontal if taller
        } else if width > height {
            false // Prefer vertical if wider
        } else {
            grid.random_bool() // Random if square
        };

        if divide_horizontally {
            let wall_y = y + grid.bounded_random_usize(height - 1);
            if wall_y >= grid.height - 1 {
                return Ok(());
            }
            let passage_x = x + grid.bounded_random_usize(width);

            let mut changed_cells = HashSet::new();
            for col in x..x + width {
                if col != passage_x {
                    let coords = Coordinates { x: col, y: wall_y };
                    let below = Coordinates { x: col, y: wall_y + 1 };
                    grid.unlink(coords, below)?; // Adds wall by removing link
                    if grid.capture_steps {
                        changed_cells.insert(coords);
                        changed_cells.insert(below);
                    }
                }
            }

            if grid.capture_steps && !changed_cells.is_empty() {
                self.capture_step(grid, &changed_cells);
            }

            let top_height = wall_y - y + 1;
            let bottom_height = height - top_height;
            self.divide(grid, x, y, width, top_height)?;
            self.divide(grid, x, wall_y + 1, width, bottom_height)?;
        } else {
            let wall_x = x + grid.bounded_random_usize(width - 1);
            if wall_x >= grid.width - 1 {
                return Ok(());
            }
            let passage_y = y + grid.bounded_random_usize(height);

            let mut changed_cells = HashSet::new();
            for row in y..y + height {
                if row != passage_y {
                    let coords = Coordinates { x: wall_x, y: row };
                    let right = Coordinates { x: wall_x + 1, y: row };
                    grid.unlink(coords, right)?; // Adds wall by removing link
                    if grid.capture_steps {
                        changed_cells.insert(coords);
                        changed_cells.insert(right);
                    }
                }
            }

            if grid.capture_steps && !changed_cells.is_empty() {
                self.capture_step(grid, &changed_cells);
            }

            let left_width = wall_x - x + 1;
            let right_width = width - left_width;
            self.divide(grid, x, y, left_width, height)?;
            self.divide(grid, wall_x + 1, y, right_width, height)?;
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
                RecursiveDivision.generate(&mut grid).expect("Recursive Division maze generation failed");
                println!("Edges: {}, Expected: {}", grid.count_edges(), grid.width * grid.height - 1);
                println!("\n\nRecursive Division\n\n{}\n\n", grid.to_asci());
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
                RecursiveDivision.generate(&mut grid).expect("Recursive Division maze generation failed");
                println!("Edges: {}, Expected: {}", grid.count_edges(), grid.width * grid.height - 1);
                println!("\n\nRecursive Division\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn reject_5_x_5_delta_recursive_division_maze() {
        match Grid::new(MazeType::Delta, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                match RecursiveDivision.generate(&mut grid) {
                    Ok(()) => panic!("Successfully generated a Recursive Division maze for a Delta grid, which should have been rejected!"),
                    Err(e) => println!("As expected, Delta grid is rejected for Recursive Division maze generation: {:?}", e),
                }
            }
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }

    #[test]
    fn reject_5_x_5_sigma_recursive_division_maze() {
        match Grid::new(MazeType::Sigma, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                match RecursiveDivision.generate(&mut grid) {
                    Ok(()) => panic!("Successfully generated a Recursive Division maze for a Sigma grid, which should have been rejected!"),
                    Err(e) => println!("As expected, Sigma grid is rejected for Recursive Division maze generation: {:?}", e),
                }
            }
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }
    
    #[test]
    fn test_recursive_division_with_capture_steps() {
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 11, y: 11 };
        match Grid::new(MazeType::Orthogonal, 12, 12, start, goal, true) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                RecursiveDivision.generate(&mut grid).expect("Maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
                assert!(grid.generation_steps.is_some());
                let steps = grid.generation_steps.as_ref().unwrap(); assert!(!steps.is_empty());
                let has_linked_cells = steps.iter().any(|step| {
                    step.cells.iter().filter_map(|opt| opt.as_ref()).any(|cell| !cell.linked.is_empty())
                });
                assert!(has_linked_cells, "No cells were linked during maze generation");
                let has_open_walls = steps.iter().any(|step| {
                    step.cells.iter().filter_map(|opt| opt.as_ref()).any(|cell| !cell.open_walls.is_empty())
                });
                assert!(has_open_walls, "No cells have open walls in generation steps");
            }
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }
}
