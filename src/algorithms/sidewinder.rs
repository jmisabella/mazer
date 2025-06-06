use crate::behaviors::maze::MazeGeneration;
use crate::algorithms::MazeAlgorithm;
use crate::grid::Grid;
use crate::cell::{Coordinates, MazeType};
use crate::error::Error;

pub struct Sidewinder;

impl MazeGeneration for Sidewinder {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        match grid.maze_type {
            MazeType::Orthogonal => {} // proceed with maze generation for allowed Orthogonal (square) grid type
            maze_type => {
                return Err(Error::AlgorithmUnavailableForMazeType{algorithm:MazeAlgorithm::Sidewinder, maze_type:maze_type});
            }
        }
        let rows = grid.height;
        let cols = grid.width;

        // Capture initial state
        if grid.capture_steps {
            let mut grid_clone = grid.clone();
            grid_clone.capture_steps = false;
            grid_clone.generation_steps = None;
            grid.generation_steps.as_mut().unwrap().push(grid_clone);
        }

        for row in 0..rows {
            let mut run: Vec<Coordinates> = Vec::new(); // Start a new run

            for col in 0..cols {
                let current_coords = Coordinates { x: col, y: row };
                run.push(current_coords); // Add current cell to the run

                let at_eastern_boundary = col + 1 == cols;
                let at_northern_boundary = row == 0;

                let should_close_run = at_eastern_boundary || (!at_northern_boundary && grid.random_bool());

                if should_close_run {
                    // Close the run by carving upward
                    if !at_northern_boundary {
                        // Get a random index from the run
                        let random_index = grid.bounded_random_usize(run.len() - 1);
                        let random_cell = run[random_index];

                        let above_coords = Coordinates {
                            x: random_cell.x,
                            y: random_cell.y - 1,
                        };

                        // Link the selected cell upward
                        grid.link(random_cell, above_coords)?;

                        // Capture state after linking
                        if grid.capture_steps {
                            let mut grid_clone = grid.clone();
                            grid_clone.capture_steps = false;
                            grid_clone.generation_steps = None;
                            grid.generation_steps.as_mut().unwrap().push(grid_clone);
                        }
                    }

                    run.clear(); // Reset the run
                } else if !at_eastern_boundary {
                    // Carve eastward
                    let east_coords = Coordinates {
                        x: col + 1,
                        y: row,
                    };

                    grid.link(current_coords, east_coords)?;

                    // Capture state after linking
                    if grid.capture_steps {
                        let mut grid_clone = grid.clone();
                        grid_clone.capture_steps = false;
                        grid_clone.generation_steps = None;
                        grid.generation_steps.as_mut().unwrap().push(grid_clone);
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
    fn print_5_x_5_maze() {
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Sidewinder.generate(&mut grid).expect("Sidewinder maze generation failed");
                println!("\n\nSidewinder\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }     
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn print_12_x_6_maze() {
        match Grid::new(MazeType::Orthogonal, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Sidewinder.generate(&mut grid).expect("Sidewinder maze generation failed");
                println!("\n\nSidewinder\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn reject_5_x_5_delta_binary_tree_maze() {
        match Grid::new(MazeType::Delta, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                match Sidewinder.generate(&mut grid) {
                    Ok(()) => {
                        panic!("Successfully generated a Sidewinder maze for a Delta grid, which is should have been rejected!");
                    }
                    Err(e) => {
                        println!("As expected, Delta grid is rejected for Sidewinder maze generation: {:?}", e);
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
                match Sidewinder.generate(&mut grid) {
                    Ok(()) => {
                        panic!("Successfully generated a Sidewinder maze for a Sigma grid, which is should have been rejected!");
                    }
                    Err(e) => {
                        println!("As expected, Sigma grid is rejected for Sidewinder maze generation: {:?}", e);
                    }
                }
            }    
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }

    #[test]
    fn test_sidewinder_with_capture_steps() {
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 11, y: 11 };
        match Grid::new(MazeType::Orthogonal, 12, 12, start, goal, true) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Sidewinder.generate(&mut grid).expect("Maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
                assert!(grid.generation_steps.is_some());
                assert!(grid.generation_steps.as_ref().unwrap().len() > 0);
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

// pub struct Sidewinder;

// impl MazeGeneration for Sidewinder {
//     fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
//         match grid.maze_type {
//             MazeType::Orthogonal => {} // proceed with maze generation for allowed Orthogonal (square) grid type
//             maze_type => {
//                 return Err(Error::AlgorithmUnavailableForMazeType{algorithm:MazeAlgorithm::Sidewinder, maze_type:maze_type});
//             }
//         }
//         let rows = grid.height;
//         let cols = grid.width;
//         for row in 0..rows {
//             let mut run: Vec<Coordinates> = Vec::new(); // Start a new run

//             for col in 0..cols {
//                 let current_coords = Coordinates { x: col, y: row };
//                 run.push(current_coords); // Add current cell to the run

//                 let at_eastern_boundary = col + 1 == cols;
//                 let at_northern_boundary = row == 0;

//                 let should_close_run = at_eastern_boundary || (!at_northern_boundary && grid.random_bool());

//                 if should_close_run {
//                     // Close the run by carving upward
//                     if !at_northern_boundary {
//                         // Get a random index from the run
//                         let random_index = grid.bounded_random_usize(run.len() - 1);
//                         let random_cell = run[random_index];

//                         let above_coords = Coordinates {
//                             x: random_cell.x,
//                             y: random_cell.y - 1,
//                         };

//                         // Link the selected cell upward
//                         {
//                             let current_cell = grid.get_mut(random_cell)?;
//                             current_cell.linked.insert(above_coords);
//                         }

//                         let above_cell = grid.get_mut(above_coords)?;
//                         above_cell.linked.insert(random_cell);
//                     }

//                     run.clear(); // Reset the run
//                 } else if !at_eastern_boundary {
//                     // Carve eastward
//                     let east_coords = Coordinates {
//                         x: col + 1,
//                         y: row,
//                     };

//                     {
//                         let current_cell = grid.get_mut_by_coords(col, row)?;
//                         current_cell.linked.insert(east_coords);
//                     }

//                     let east_cell = grid.get_mut_by_coords(col + 1, row)?;
//                     east_cell.linked.insert(current_coords);
//                 }
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
//     fn print_5_x_5_maze() {
//         match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 Sidewinder.generate(&mut grid).expect("Sidewinder maze generation failed");
//                 println!("\n\nSidewinder\n\n{}\n\n", grid.to_asci());
//                 assert!(grid.is_perfect_maze().unwrap());
//             }     
//             Err(e) => panic!("Unexpected error running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn print_12_x_6_maze() {
//         match Grid::new(MazeType::Orthogonal, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 Sidewinder.generate(&mut grid).expect("Sidewinder maze generation failed");
//                 println!("\n\nSidewinder\n\n{}\n\n", grid.to_asci());
//                 assert!(grid.is_perfect_maze().unwrap());
//             }
//             Err(e) => panic!("Unexpected error running test: {:?}", e),
//         }
//     }

//     #[test]
//     fn reject_5_x_5_delta_binary_tree_maze() {
//         match Grid::new(MazeType::Delta, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 match Sidewinder.generate(&mut grid) {
//                     Ok(()) => {
//                         panic!("Successfully generated a Sidewinder maze for a Delta grid, which is should have been rejected!");
//                     }
//                     Err(e) => {
//                         println!("As expected, Delta grid is rejected for Sidewinder maze generation: {:?}", e);
//                     }
//                 }
//             }    
//             Err(e) => panic!("Unexpected error generating grid: {:?}", e),
//         }
//     }

//     #[test]
//     fn reject_5_x_5_sigma_binary_tree_maze() {
//         match Grid::new(MazeType::Sigma, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
//             Ok(mut grid) => {
//                 assert!(!grid.is_perfect_maze().unwrap());
//                 match Sidewinder.generate(&mut grid) {
//                     Ok(()) => {
//                         panic!("Successfully generated a Sidewinder maze for a Sigma grid, which is should have been rejected!");
//                     }
//                     Err(e) => {
//                         println!("As expected, Sigma grid is rejected for Sidewinder maze generation: {:?}", e);
//                     }
//                 }
//             }    
//             Err(e) => panic!("Unexpected error generating grid: {:?}", e),
//         }
//     }

// }
