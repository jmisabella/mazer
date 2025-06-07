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
            MazeType::Orthogonal => {} // Proceed for Orthogonal grids
            maze_type => {
                return Err(Error::AlgorithmUnavailableForMazeType {
                    algorithm: MazeAlgorithm::RecursiveDivision,
                    maze_type,
                });
            }
        }

        // Capture initial state if capture_steps is true
        if grid.capture_steps {
            let changed_cells = HashSet::new();
            self.capture_step(grid, &changed_cells);
        }

        // Recursively divide the grid
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
        // Stop if the region cannot be divided further
        if width <= 1 && height <= 1 {
            return Ok(());
        }

        // Ensure we can divide by requiring at least 2 cells in one dimension
        let can_divide_horizontally = height > 1;
        let can_divide_vertically = width > 1;

        if !can_divide_horizontally && !can_divide_vertically {
            return Ok(());
        }

        // Decide whether to divide horizontally or vertically
        let divide_horizontally = if !can_divide_vertically {
            true // Must divide horizontally if width <= 1
        } else if !can_divide_horizontally {
            false // Must divide vertically if height <= 1
        } else if width > height {
            false // Prefer vertical if wider
        } else if height > width {
            true // Prefer horizontal if taller
        } else {
            grid.random_bool() // Random choice if equal
        };

        if divide_horizontally {
            // Need at least 2 rows to place a wall between them
            if height < 2 {
                return Ok(());
            }
            // Choose a wall y-coordinate between rows [y, y+height-2)
            let wall_y = if height > 2 {
                y + grid.bounded_random_usize(height - 2) // [y, y+height-3]
            } else {
                y // Only one possible wall position
            };
            // Ensure wall_y is within bounds
            if wall_y >= grid.height - 1 {
                return Ok(());
            }
            // Choose a passage x-coordinate within [x, x+width-1)
            let passage_x = if width > 1 {
                x + grid.bounded_random_usize(width - 1) // [x, x+width-2]
            } else {
                x
            };
            if passage_x >= grid.width {
                return Ok(());
            }

            // Carve a passage by linking cells at the passage position
            let coords = Coordinates { x: passage_x, y: wall_y };
            let below = Coordinates { x: passage_x, y: wall_y + 1 };
            grid.link(coords, below)?;

            // Capture state after passage creation if capture_steps is true
            if grid.capture_steps {
                let mut changed_cells = HashSet::new();
                changed_cells.insert(coords);
                changed_cells.insert(below);
                self.capture_step(grid, &changed_cells);
            }

            // Recursively divide the two regions
            let top_height = wall_y - y + 1;
            let bottom_height = height - top_height;
            self.divide(grid, x, y, width, top_height)?; // Top region (includes wall_y row)
            self.divide(grid, x, wall_y + 1, width, bottom_height)?; // Bottom region
        } else {
            // Need at least 2 columns to place a wall between them
            if width < 2 {
                return Ok(());
            }
            // Choose a wall x-coordinate between columns [x, x+width-2)
            let wall_x = if width > 2 {
                x + grid.bounded_random_usize(width - 2) // [x, x+width-3]
            } else {
                x // Only one possible wall position
            };
            // Ensure wall_x is within bounds
            if wall_x >= grid.width - 1 {
                return Ok(());
            }
            // Choose a passage y-coordinate within [y, y+height-1)
            let passage_y = if height > 1 {
                y + grid.bounded_random_usize(height - 1) // [y, y+height-2]
            } else {
                y
            };
            if passage_y >= grid.height {
                return Ok(());
            }

            // Carve a passage by linking cells at the passage position
            let coords = Coordinates { x: wall_x, y: passage_y };
            let right = Coordinates { x: wall_x + 1, y: passage_y };
            grid.link(coords, right)?;

            // Capture state after passage creation if capture_steps is true
            if grid.capture_steps {
                let mut changed_cells = HashSet::new();
                changed_cells.insert(coords);
                changed_cells.insert(right);
                self.capture_step(grid, &changed_cells);
            }

            // Recursively divide the two regions
            let left_width = wall_x - x + 1;
            let right_width = width - left_width;
            self.divide(grid, x, y, left_width, height)?; // Left region (includes wall_x column)
            self.divide(grid, wall_x + 1, y, right_width, height)?; // Right region
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
    fn reject_12_x_12_polar_recursive_division_maze() {
        match Grid::new(MazeType::Polar, 12, 12, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 11 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                match RecursiveDivision.generate(&mut grid) {
                    Ok(()) => panic!("Successfully generated a Recursive Division maze for a Polar grid, which should have been rejected!"),
                    Err(e) => println!("As expected, Polar grid is rejected for Recursive Division maze generation: {:?}", e),
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

// use crate::behaviors::maze::MazeGeneration;
// use crate::algorithms::MazeAlgorithm;
// use crate::grid::Grid;
// use crate::cell::{Coordinates, MazeType};
// use crate::error::Error;

// pub struct RecursiveDivision;

// impl MazeGeneration for RecursiveDivision {
//     fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
//         match grid.maze_type {
//             MazeType::Orthogonal => {} // Proceed for Orthogonal grids
//             maze_type => {
//                 return Err(Error::AlgorithmUnavailableForMazeType {
//                     algorithm: MazeAlgorithm::RecursiveDivision,
//                     maze_type,
//                 });
//             }
//         }

//         // Capture initial state if capture_steps is true
//         if grid.capture_steps {
//             let mut grid_clone = grid.clone();
//             grid_clone.capture_steps = false;
//             grid_clone.generation_steps = None;
//             grid.generation_steps.as_mut().unwrap().push(grid_clone);
//         }

//         // Recursively divide the grid
//         self.divide(grid, 0, 0, grid.width, grid.height)?;

//         Ok(())
//     }
// }

// impl RecursiveDivision {
//     fn divide(
//         &self,
//         grid: &mut Grid,
//         x: usize,
//         y: usize,
//         width: usize,
//         height: usize,
//     ) -> Result<(), Error> {
//         // Stop if the region cannot be divided further
//         if width <= 1 && height <= 1 {
//             return Ok(());
//         }

//         // Ensure we can divide by requiring at least 2 cells in one dimension
//         let can_divide_horizontally = height > 1;
//         let can_divide_vertically = width > 1;

//         if !can_divide_horizontally && !can_divide_vertically {
//             return Ok(());
//         }

//         // Decide whether to divide horizontally or vertically
//         let divide_horizontally = if !can_divide_vertically {
//             true // Must divide horizontally if width <= 1
//         } else if !can_divide_horizontally {
//             false // Must divide vertically if height <= 1
//         } else if width > height {
//             false // Prefer vertical if wider
//         } else if height > width {
//             true // Prefer horizontal if taller
//         } else {
//             grid.random_bool() // Random choice if equal
//         };

//         if divide_horizontally {
//             // Need at least 2 rows to place a wall between them
//             if height < 2 {
//                 return Ok(());
//             }
//             // Choose a wall y-coordinate between rows [y, y+height-2)
//             let wall_y = if height > 2 {
//                 y + grid.bounded_random_usize(height - 2) // [y, y+height-3]
//             } else {
//                 y // Only one possible wall position
//             };
//             // Ensure wall_y is within bounds
//             if wall_y >= grid.height - 1 {
//                 return Ok(());
//             }
//             // Choose a passage x-coordinate within [x, x+width-1)
//             let passage_x = if width > 1 {
//                 x + grid.bounded_random_usize(width - 1) // [x, x+width-2]
//             } else {
//                 x
//             };
//             if passage_x >= grid.width {
//                 return Ok(());
//             }

//             // Carve a passage by linking cells at the passage position
//             let coords = Coordinates { x: passage_x, y: wall_y };
//             let below = Coordinates { x: passage_x, y: wall_y + 1 };
//             grid.link(coords, below)?;

//             // Capture state after passage creation if capture_steps is true
//             if grid.capture_steps {
//                 let mut grid_clone = grid.clone();
//                 grid_clone.capture_steps = false;
//                 grid_clone.generation_steps = None;
//                 grid.generation_steps.as_mut().unwrap().push(grid_clone);
//             }

//             // Recursively divide the two regions
//             let top_height = wall_y - y + 1;
//             let bottom_height = height - top_height;
//             self.divide(grid, x, y, width, top_height)?; // Top region (includes wall_y row)
//             self.divide(grid, x, wall_y + 1, width, bottom_height)?; // Bottom region
//         } else {
//             // Need at least 2 columns to place a wall between them
//             if width < 2 {
//                 return Ok(());
//             }
//             // Choose a wall x-coordinate between columns [x, x+width-2)
//             let wall_x = if width > 2 {
//                 x + grid.bounded_random_usize(width - 2) // [x, x+width-3]
//             } else {
//                 x // Only one possible wall position
//             };
//             // Ensure wall_x is within bounds
//             if wall_x >= grid.width - 1 {
//                 return Ok(());
//             }
//             // Choose a passage y-coordinate within [y, y+height-1)
//             let passage_y = if height > 1 {
//                 y + grid.bounded_random_usize(height - 1) // [y, y+height-2]
//             } else {
//                 y
//             };
//             if passage_y >= grid.height {
//                 return Ok(());
//             }

//             // Carve a passage by linking cells at the passage position
//             let coords = Coordinates { x: wall_x, y: passage_y };
//             let right = Coordinates { x: wall_x + 1, y: passage_y };
//             grid.link(coords, right)?;

//             // Capture state after passage creation if capture_steps is true
//             if grid.capture_steps {
//                 let mut grid_clone = grid.clone();
//                 grid_clone.capture_steps = false;
//                 grid_clone.generation_steps = None;
//                 grid.generation_steps.as_mut().unwrap().push(grid_clone);
//             }

//             // Recursively divide the two regions
//             let left_width = wall_x - x + 1;
//             let right_width = width - left_width;
//             self.divide(grid, x, y, left_width, height)?; // Left region (includes wall_x column)
//             self.divide(grid, wall_x + 1, y, right_width, height)?; // Right region
//         }

//         Ok(())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::cell::{MazeType, Coordinates};

//     #[test]
//     fn generate_and_print_5_x_5_orthogonal_maze() {
//         match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 RecursiveDivision.generate(&mut grid).expect("Recursive Division maze generation failed");
//                 println!("Edges: {}, Expected: {}", grid.count_edges(), grid.width * grid.height - 1);
//                 println!("\n\nRecursive Division\n\n{}\n\n", grid.to_asci());
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn generate_and_print_12_x_6_orthogonal_maze() {
//         match Grid::new(MazeType::Orthogonal, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 RecursiveDivision.generate(&mut grid).expect("Recursive Division maze generation failed");
//                 println!("Edges: {}, Expected: {}", grid.count_edges(), grid.width * grid.height - 1);
//                 println!("\n\nRecursive Division\n\n{}\n\n", grid.to_asci());
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn reject_5_x_5_delta_recursive_division_maze() {
//         match Grid::new(MazeType::Delta, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 match RecursiveDivision.generate(&mut grid) {
//                     Ok(()) => panic!("Successfully generated a Recursive Division maze for a Delta grid, which should have been rejected!"),
//                     Err(e) => println!("As expected, Delta grid is rejected for Recursive Division maze generation: {:?}", e),
//                 }
//             }
//             Err(e) => panic!("Unexpected error generating grid: {:?}", e),
//         }
//     }

//     #[test]
//     fn reject_5_x_5_sigma_recursive_division_maze() {
//         match Grid::new(MazeType::Sigma, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 match RecursiveDivision.generate(&mut grid) {
//                     Ok(()) => panic!("Successfully generated a Recursive Division maze for a Sigma grid, which should have been rejected!"),
//                     Err(e) => println!("As expected, Sigma grid is rejected for Recursive Division maze generation: {:?}", e),
//                 }
//             }
//             Err(e) => panic!("Unexpected error generating grid: {:?}", e),
//         }
//     }

//     #[test]
//     fn reject_12_x_12_polar_recursive_division_maze() {
//         match Grid::new(MazeType::Polar, 12, 12, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 11 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 match RecursiveDivision.generate(&mut grid) {
//                     Ok(()) => panic!("Successfully generated a Recursive Division maze for a Polar grid, which should have been rejected!"),
//                     Err(e) => println!("As expected, Polar grid is rejected for Recursive Division maze generation: {:?}", e),
//                 }
//             }
//             Err(e) => panic!("Unexpected error generating grid: {:?}", e),
//         }
//     }

//     #[test]
//     fn test_recursive_division_with_capture_steps() {
//         let start = Coordinates { x: 0, y: 0 };
//         let goal = Coordinates { x: 11, y: 11 };
//         match Grid::new(MazeType::Orthogonal, 12, 12, start, goal, true) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 RecursiveDivision.generate(&mut grid).expect("Maze generation failed");
//                 assert!(grid.is_perfect_maze().unwrap());
//                 assert!(grid.generation_steps.is_some());
//                 let steps = grid.generation_steps.as_ref().unwrap(); assert!(!steps.is_empty());
//                 // Check if any cells become linked across all generation steps
//                 let has_linked_cells = steps.iter().any(|step| {
//                     step.cells.iter().any(|cell| !cell.linked.is_empty())
//                 });
//                 assert!(has_linked_cells, "No cells were linked during maze generation");
//                 let has_open_walls = steps.iter().any(|step| {
//                     step.cells.iter().any(|cell| !cell.open_walls.is_empty())
//                 });
//                 assert!(has_open_walls, "No cells have open walls in generation steps");
//             }
//             Err(e) => panic!("Unexpected error generating grid: {:?}", e),
//         }
//     }

// }
