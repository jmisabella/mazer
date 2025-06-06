
use crate::behaviors::maze::MazeGeneration;
use crate::grid::Grid;
use crate::cell::Coordinates;
use crate::error::Error;

use std::collections::HashSet;

pub struct RecursiveBacktracker;

impl MazeGeneration for RecursiveBacktracker {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        // Create a stack to track the current path
        let mut stack: Vec<Coordinates> = Vec::new();
        let mut visited: HashSet<Coordinates> = HashSet::new();

        // Start at the start_coords
        stack.push(grid.start_coords);
        visited.insert(grid.start_coords);

        // Capture initial state if capture_steps is true
        if grid.capture_steps {
            let mut grid_clone = grid.clone();
            grid_clone.capture_steps = false;
            grid_clone.generation_steps = None;
            grid.generation_steps.as_mut().unwrap().push(grid_clone);
        }

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
                grid.link(current_coords, next_coords)?;

                // Capture state after linking if capture_steps is true
                if grid.capture_steps {
                    let mut grid_clone = grid.clone();
                    grid_clone.capture_steps = false;
                    grid_clone.generation_steps = None;
                    grid.generation_steps.as_mut().unwrap().push(grid_clone);
                }

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
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                RecursiveBacktracker.generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
                println!("\n\nRecursive Backtracker\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error occurred running test: {:?}", e),
        }
    }

    #[test]
    fn generate_and_print_12_x_6_orthogonal_maze() {
        match Grid::new(MazeType::Orthogonal, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                RecursiveBacktracker.generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
                println!("\n\nRecursive Backtracker\n\n{}\n\n", grid.to_asci());
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
                RecursiveBacktracker.generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error occurred running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_delta_maze() {
        match Grid::new(MazeType::Delta, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                RecursiveBacktracker.generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
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
                RecursiveBacktracker.generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error occurred running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_sigma_maze() {
        match Grid::new(MazeType::Sigma, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                RecursiveBacktracker.generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_12_polar_maze() {
        match Grid::new(MazeType::Polar, 12, 12, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 11 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                RecursiveBacktracker.generate(&mut grid).expect("Maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }
    
    #[test]
    fn generate_12_x_6_polar_maze() {
        match Grid::new(MazeType::Polar, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                RecursiveBacktracker.generate(&mut grid).expect("Maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn test_recursive_backtracker_with_capture_steps() {
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 19, y: 19 };
        match Grid::new(MazeType::Orthogonal, 20, 20, start, goal, true) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                RecursiveBacktracker.generate(&mut grid).expect("Maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
                assert!(grid.generation_steps.is_some());
                assert!(grid.generation_steps.as_ref().unwrap().len() > 0);
            }
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }

}

// use crate::behaviors::maze::MazeGeneration;
// use crate::grid::Grid;
// use crate::cell::Coordinates;
// use crate::error::Error;

// use std::collections::HashSet;

// pub struct RecursiveBacktracker;

// impl MazeGeneration for RecursiveBacktracker {
//     fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
//         // Create a stack to track the current path
//         let mut stack: Vec<Coordinates> = Vec::new();
//         let mut visited: HashSet<Coordinates> = HashSet::new();

//         // Start at the start_coords
//         stack.push(grid.start_coords);
//         visited.insert(grid.start_coords);

//         while let Some(current_coords) = stack.last().cloned() {
//             // Get all unvisited neighbors
//             let neighbors: Vec<Coordinates> = grid
//                 .get(current_coords)?
//                 .neighbors()
//                 .into_iter()
//                 .filter(|neighbor| !visited.contains(neighbor))
//                 .collect();

//             if neighbors.is_empty() {
//                 // Backtrack if no unvisited neighbors
//                 stack.pop();
//             } else {
//                 // Choose a random unvisited neighbor
//                 let random_index = {
//                     let upper_bound = neighbors.len() - 1;
//                     grid.bounded_random_usize(upper_bound)
//                 };
//                 let next_coords = neighbors[random_index];

//                 // Link current cell to the chosen neighbor
//                 grid.link(current_coords, next_coords)?;

//                 // Mark the neighbor as visited and push it onto the stack
//                 visited.insert(next_coords);
//                 stack.push(next_coords);
//             }
//         }
//         Ok(())
//     }

// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::cell::{ MazeType, Coordinates };
    
//     #[test]
//     fn generate_and_print_5_x_5_orthogonal_maze() {
//         match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 RecursiveBacktracker.generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
//                 println!("\n\nRecursive Backtracker\n\n{}\n\n", grid.to_asci());
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error occurred running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn generate_and_print_12_x_6_orthogonal_maze() {
//         match Grid::new(MazeType::Orthogonal, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 RecursiveBacktracker.generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
//                 println!("\n\nRecursive Backtracker\n\n{}\n\n", grid.to_asci());
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn generate_5_x_5_delta_maze() {
//         match Grid::new(MazeType::Delta, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 RecursiveBacktracker.generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error occurred running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn generate_12_x_6_delta_maze() {
//         match Grid::new(MazeType::Delta, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 RecursiveBacktracker.generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn generate_5_x_5_sigma_maze() {
//         match Grid::new(MazeType::Sigma, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 RecursiveBacktracker.generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error occurred running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn generate_12_x_6_sigma_maze() {
//         match Grid::new(MazeType::Sigma, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 RecursiveBacktracker.generate(&mut grid).expect("RecursiveBacktracker maze generation failed");
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn generate_12_x_12_polar_maze() {
//         match Grid::new(MazeType::Polar, 12, 12, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 11 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 RecursiveBacktracker.generate(&mut grid).expect("Maze generation failed");
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error running test: {:?}", e),
//         }
//     }
    
//     #[test]
//     fn generate_12_x_6_polar_maze() {
//         match Grid::new(MazeType::Polar, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 RecursiveBacktracker.generate(&mut grid).expect("Maze generation failed");
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn test_recursive_backtracker_with_capture_steps() {
//         let start = Coordinates { x: 0, y: 0 };
//         let goal = Coordinates { x: 19, y: 19 };
//         match Grid::new(MazeType::Orthogonal, 20, 20, start, goal, true) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 RecursiveBacktracker.generate(&mut grid).expect("Maze generation failed");
//                 assert!(grid.is_perfect_maze().unwrap());
//                 assert!(grid.generation_steps.is_some());
//                 assert!(grid.generation_steps.as_ref().unwrap().len() > 0);
//             }
//             Err(e) => panic!("Unexpected error generating grid: {:?}", e),
//         }
//     }

// }

