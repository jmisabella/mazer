use crate::behaviors::maze::MazeGeneration;
use crate::grid::Grid;
use crate::cell::Coordinates;
use crate::error::Error;

use std::collections::HashSet;

pub struct GrowingTree;

impl MazeGeneration for GrowingTree {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        let mut active: Vec<Coordinates> = Vec::new();
        let mut visited: HashSet<Coordinates> = HashSet::new();

        // Start with a random cell
        let start_coords = Coordinates {
            x: grid.bounded_random_usize(grid.width - 1),
            y: grid.bounded_random_usize(grid.height - 1),
        };
        active.push(start_coords);
        visited.insert(start_coords);

        // Capture initial state if capture_steps is true
        if grid.capture_steps {
            let mut grid_clone = grid.clone();
            grid_clone.capture_steps = false;
            grid_clone.generation_steps = None;
            grid.generation_steps.as_mut().unwrap().push(grid_clone);
        }

        while !active.is_empty() {
            // Choose a random cell from active list (can be modified for other strategies)
            let index = grid.bounded_random_usize(active.len() - 1);
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
                let neighbor_index = grid.bounded_random_usize(unvisited_neighbors.len() - 1);
                let next_coords = unvisited_neighbors[neighbor_index];

                // Link to the neighbor
                grid.link(current_coords, next_coords)?;

                // Mark neighbor as visited and add to active list
                visited.insert(next_coords);
                active.push(next_coords);

                // Capture step after linking if capture_steps is true
                if grid.capture_steps {
                    let mut grid_clone = grid.clone();
                    grid_clone.capture_steps = false;
                    grid_clone.generation_steps = None;
                    grid.generation_steps.as_mut().unwrap().push(grid_clone);
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
                GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
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
                GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
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
                GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
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
                GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
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
                GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
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
                GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
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
                GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
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
                GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
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
                GrowingTree.generate(&mut grid).expect("Maze generation failed");
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

// pub struct GrowingTree;

// impl MazeGeneration for GrowingTree {
//     fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
//         let mut active: Vec<Coordinates> = Vec::new();
//         let mut visited: HashSet<Coordinates> = HashSet::new();

//         // Start with a random cell
//         let start_coords = Coordinates {
//             x: grid.bounded_random_usize(grid.width - 1),
//             y: grid.bounded_random_usize(grid.height - 1),
//         };
//         active.push(start_coords);
//         visited.insert(start_coords);

//         while !active.is_empty() {
//             // Choose a random cell from active list (can be modified for other strategies)
//             let index = grid.bounded_random_usize(active.len() - 1);
//             let current_coords = active[index];

//             // Get unvisited neighbors
//             let unvisited_neighbors: Vec<Coordinates> = if let Ok(cell) = grid.get(current_coords) {
//                 cell.neighbors()
//                     .into_iter()
//                     .filter(|neighbor| !visited.contains(neighbor))
//                     .collect()
//             } else {
//                 Vec::new()
//             };

//             if unvisited_neighbors.is_empty() {
//                 // No unvisited neighbors, remove from active list
//                 active.swap_remove(index);
//             } else {
//                 // Choose a random unvisited neighbor
//                 let neighbor_index = grid.bounded_random_usize(unvisited_neighbors.len() - 1);
//                 let next_coords = unvisited_neighbors[neighbor_index];

//                 // Link to the neighbor
//                 grid.link(current_coords, next_coords)?;

//                 // Mark neighbor as visited and add to active list
//                 visited.insert(next_coords);
//                 active.push(next_coords);
//             }
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
//                 GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
//                 println!("\n\nGrowing Tree\n\n{}\n\n", grid.to_asci());
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
//                 GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
//                 println!("\n\nGrowing Tree\n\n{}\n\n", grid.to_asci());
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
//                 GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn generate_12_x_6_delta_maze() {
//         match Grid::new(MazeType::Delta, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
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
//                 GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn generate_12_x_6_sigma_maze() {
//         match Grid::new(MazeType::Sigma, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
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
//                 GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
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
//                 GrowingTree.generate(&mut grid).expect("Growing Tree maze generation failed");
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error running test: {:?}", e),
//         }
//     }
// }