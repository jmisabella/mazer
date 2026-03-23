use crate::behaviors::maze::MazeGeneration;
use crate::grid::Grid;
use crate::cell::Coordinates;
use crate::error::Error;

use std::collections::HashSet;

pub struct AldousBroder;

impl MazeGeneration for AldousBroder {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        // Step 1: Calculate the number of actual cells and initialize visited tracking
        let total_cells = grid.cells.iter().filter(|opt| opt.is_some()).count();
        let mut visited = HashSet::new();

        // Step 2: Choose a random starting cell that exists
        let start_coords = loop {
            let rand_x = grid.bounded_random_usize(grid.width);
            let rand_y = grid.bounded_random_usize(grid.height);
            let coords = Coordinates { x: rand_x, y: rand_y };
            if grid.get(coords).is_ok() {
                break coords;
            }
        };
        let mut current_coords = start_coords;
        visited.insert(current_coords);
        let mut visited_count = 1;

        // Capture initial state if capture_steps is true
        if grid.capture_steps {
            let mut grid_clone = grid.clone();
            grid_clone.capture_steps = false;
            grid_clone.generation_steps = None;
            grid.generation_steps.as_mut().unwrap().push(grid_clone);
        }

        // Step 3: Loop until all existing cells are visited
        while visited_count < total_cells {
            if let Ok(current_cell) = grid.get(current_coords) {
                // Get neighbors that exist (i.e., have Some(Cell))
                let neighbors: Vec<Coordinates> = current_cell
                    .neighbors()
                    .iter()
                    .filter(|&&coords| grid.get(coords).is_ok())
                    .cloned()
                    .collect();

                if !neighbors.is_empty() {
                    // Pick a random neighbor
                    let random_index = grid.bounded_random_usize(neighbors.len());
                    let random_neighbor = neighbors[random_index];

                    // If the neighbor hasn't been visited, link it and update visited
                    if !visited.contains(&random_neighbor) {
                        grid.link(current_coords, random_neighbor)?;
                        visited.insert(random_neighbor);
                        visited_count += 1;

                        if grid.capture_steps {
                            let mut changed_cells = HashSet::new();
                            changed_cells.insert(current_coords);
                            changed_cells.insert(random_neighbor);
                            self.capture_step(grid, &changed_cells);
                        }
                    }

                    // Move to the selected neighbor
                    current_coords = random_neighbor;
                } else {
                    // If no unvisited neighbors, jump to another unvisited existing cell
                    current_coords = loop {
                        let rand_x = grid.bounded_random_usize(grid.width);
                        let rand_y = grid.bounded_random_usize(grid.height);
                        let coords = Coordinates { x: rand_x, y: rand_y };
                        if grid.get(coords).is_ok() && !visited.contains(&coords) {
                            break coords;
                        }
                    };
                }
            } else {
                // This should not occur with proper movement logic
                return Err(Error::InvalidCellCoordinates {
                    coordinates: current_coords,
                });
            }
        }
        Ok(())
    }
}

// impl MazeGeneration for AldousBroder {
//     fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
//         // Step 1: Initialize visited tracking using Vec<Vec<bool>>
//         let rows = grid.height;
//         let cols = grid.width;
//         let mut visited = vec![vec![false; cols]; rows];
//         let total_cells = rows * cols;

//         let rand_x = grid.bounded_random_usize(cols);
//         let rand_y = grid.bounded_random_usize(rows);
//         // Step 2: Choose a random starting cell
//         let mut current_coords = Coordinates {
//             x: rand_x,
//             y: rand_y,
//         };
//         visited[current_coords.y][current_coords.x] = true; // Mark as visited
//         let mut visited_count = 1;

//         // Capture initial state if capture_steps is true
//         if grid.capture_steps {
//             let mut grid_clone = grid.clone();
//             grid_clone.capture_steps = false;
//             grid_clone.generation_steps = None;
//             grid.generation_steps.as_mut().unwrap().push(grid_clone);
//         }

//         // Step 3: Loop until all cells are visited
//         while visited_count < total_cells {
//             if let Ok(current_cell) = grid.get(current_coords) {
//                 // Get neighbors of the current cell
//                 let neighbors: Vec<Coordinates> = current_cell
//                     .neighbors()
//                     .iter()
//                     .filter(|&&coords| coords.y < rows && coords.x < cols)
//                     .cloned()
//                     .collect();

//                 if !neighbors.is_empty() {
//                     // Pick a random neighbor
//                     let random_index = grid.bounded_random_usize(neighbors.len());
//                     let random_neighbor = neighbors[random_index];

//                     // If the neighbor hasn't been visited, link it and update visited
//                     if !visited[random_neighbor.y][random_neighbor.x] {
//                         grid.link(current_coords, random_neighbor)?;
//                         visited[random_neighbor.y][random_neighbor.x] = true;
//                         visited_count += 1;

//                         if grid.capture_steps {
//                             let mut changed_cells = HashSet::new();
//                             changed_cells.insert(current_coords);
//                             changed_cells.insert(random_neighbor);
//                             self.capture_step(grid, &changed_cells);
//                         }
//                     }

//                     // Move to the selected neighbor
//                     current_coords = random_neighbor;
//                 }
//             } else {
//                 // If the current cell is invalid (edge case), break out
//                 break;
//             }
//         }
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::{ MazeType, Coordinates };
    
    #[test]
    fn generate_and_print_5_x_5_orthogonal_maze() {
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                AldousBroder.generate(&mut grid).expect("AldousBroder maze generation failed");
                println!("\n\nAldous Broder\n\n{}\n\n", grid.to_asci());
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
                AldousBroder.generate(&mut grid).expect("AldousBroder maze generation failed");
                println!("\n\nAldous Broder\n\n{}\n\n", grid.to_asci());
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
                AldousBroder.generate(&mut grid).expect("AldousBroder maze generation failed");
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
                AldousBroder.generate(&mut grid).expect("AldousBroder maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }
    
    #[test]
    fn generate_12_x_12_delta_maze() {
        match Grid::new(MazeType::Delta, 12, 12, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 11 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                AldousBroder.generate(&mut grid).expect("AldousBroder maze generation failed");
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
                AldousBroder.generate(&mut grid).expect("AldousBroder maze generation failed");
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
                AldousBroder.generate(&mut grid).expect("AldousBroder maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }
    
    #[test]
    fn generate_12_x_6_rhombic_maze_aldous_broder() {
        match Grid::new(MazeType::Rhombic, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                AldousBroder.generate(&mut grid).expect("AldousBroder maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn test_aldous_broder_with_capture_steps() {
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 11, y: 11 };
        match Grid::new(MazeType::Orthogonal, 12, 12, start, goal, true) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                AldousBroder.generate(&mut grid).expect("Maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
                assert!(grid.generation_steps.is_some());
                let steps = grid.generation_steps.as_ref().unwrap();
                assert!(!steps.is_empty());
                // Check if any cells become linked across all generation steps
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
