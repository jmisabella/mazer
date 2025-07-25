use crate::behaviors::maze::MazeGeneration;
use crate::grid::Grid;
use crate::cell::Coordinates;
use crate::error::Error;

use std::collections::HashSet;

pub struct Wilsons;

impl MazeGeneration for Wilsons {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        let mut visited: HashSet<Coordinates> = HashSet::new();

        // Mark the start cell as visited
        visited.insert(grid.start_coords);

        // Capture initial state if capture_steps is true
        if grid.capture_steps {
            let changed_cells = HashSet::new();
            self.capture_step(grid, &changed_cells);
        }

        // Count only valid cells (Some(Cell)) in the grid
        let total_cells = grid.cells.iter().filter(|opt| opt.is_some()).count();

        while visited.len() < total_cells {
            // Choose a random unvisited cell that exists to start the walk
            let walk_start = loop {
                let x = grid.bounded_random_usize(grid.width);
                let y = grid.bounded_random_usize(grid.height);
                let coords = Coordinates { x, y };
                if grid.get(coords).is_ok() && !visited.contains(&coords) {
                    break coords;
                }
            };

            // Perform a random walk
            let mut walk: Vec<Coordinates> = vec![walk_start];
            let mut walk_set: HashSet<Coordinates> = HashSet::new();
            walk_set.insert(walk_start);

            while let Some(&current) = walk.last() {
                if visited.contains(&current) {
                    break; // Path hit a visited cell, carve it
                }

                // Get valid neighbors (in-bounds and existing)
                let cell = grid.get(current)?;
                let neighbors: Vec<Coordinates> = cell
                    .neighbors()
                    .into_iter()
                    .filter(|&coords| grid.get(coords).is_ok())
                    .collect();

                if neighbors.is_empty() {
                    break; // No valid moves, end this walk
                }

                // Pick a random neighbor
                let index = grid.bounded_random_usize(neighbors.len());
                let next = neighbors[index];

                if let Some(pos) = walk.iter().position(|&c| c == next) {
                    // Loop detected: truncate the path
                    walk.truncate(pos + 1);
                    walk_set.clear();
                    walk.iter().for_each(|&c| { walk_set.insert(c); });
                } else {
                    walk.push(next);
                    walk_set.insert(next);
                }
            }

            // Carve the path into the maze
            for pair in walk.windows(2) {
                let (current, next) = (pair[0], pair[1]);
                grid.link(current, next)?;
                visited.insert(current);
                visited.insert(next);

                if grid.capture_steps {
                    let mut changed_cells = HashSet::new();
                    changed_cells.insert(current);
                    changed_cells.insert(next);
                    self.capture_step(grid, &changed_cells);
                }
            }
        }

        Ok(())
    }
}

// impl MazeGeneration for Wilsons {
//     fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
//         let mut visited: HashSet<Coordinates> = HashSet::new();

//         // Mark the start cell as visited
//         visited.insert(grid.start_coords);

//         // Capture initial state if capture_steps is true
//         if grid.capture_steps {
//             let changed_cells = HashSet::new();
//             self.capture_step(grid, &changed_cells);
//         }

//         let grid_size = grid.width * grid.height;

//         while visited.len() < grid_size {
//             // Choose a random unvisited cell to start the random walk
//             let walk_start = loop {
//                 let x = grid.bounded_random_usize(grid.width);
//                 let y = grid.bounded_random_usize(grid.height);
//                 let coords = Coordinates { x, y };
//                 if !visited.contains(&coords) {
//                     break coords;
//                 }
//             };

//             // Perform a random walk
//             let mut walk: Vec<Coordinates> = vec![walk_start];
//             let mut walk_set: HashSet<Coordinates> = HashSet::new();
//             walk_set.insert(walk_start);

//             while let Some(last) = walk.last() {
//                 if visited.contains(last) {
//                     break;
//                 }
//                 let current = *last;
//                 let cell = grid.get(Coordinates { x: current.x, y: current.y })?;
//                 let neighbors_list: HashSet<Coordinates> = cell.neighbors();
//                 let neighbors: Vec<&Coordinates> = neighbors_list.iter().collect();
            
//                 if neighbors.is_empty() {
//                     return Err(Error::EmptyList); // Handle case where there are no neighbors
//                 }
//                 let index = grid.bounded_random_usize(neighbors.len());
            
//                 // Choose a random neighbor
//                 let next = neighbors[index];
//                 if let Some(pos) = walk.iter().position(|&c| c == *next) {
//                     // Loop detected: truncate the path
//                     walk.truncate(pos + 1);
//                 } else {
//                     walk.push(*next);
//                     walk_set.insert(*next);
//                 }
//             }
            
//             // Carve the path into the maze
//             walk.windows(2)
//                 .filter_map(|pair| { 
//                     if let [current, next] = pair {
//                         Some((current, next))
//                     } else {
//                         None
//                     }
//                 })
//                 .for_each(|(current, next)| {
//                     let _ = grid.link(*current, *next);
//                     visited.insert(*current);
//                     visited.insert(*next);
//                     // Capture state after each link if capture_steps is true
//                     if grid.capture_steps {
//                         let mut changed_cells = HashSet::new();
//                         changed_cells.insert(*current);
//                         changed_cells.insert(*next);
//                         self.capture_step(grid, &changed_cells);
//                     }
//                 });

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
                Wilsons.generate(&mut grid).expect("Wilson's maze generation failed");
                println!("\n\nWilson's\n\n{}\n\n", grid.to_asci());
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
                Wilsons.generate(&mut grid).expect("Wilson's maze generation failed");
                println!("\n\nWilson's\n\n{}\n\n", grid.to_asci());
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
                Wilsons.generate(&mut grid).expect("Wilson's maze generation failed");
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
                Wilsons.generate(&mut grid).expect("Wilson's maze generation failed");
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
                Wilsons.generate(&mut grid).expect("Wilson's maze generation failed");
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
                Wilsons.generate(&mut grid).expect("Wilson's maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_upsilon_maze() {
        match Grid::new(MazeType::Upsilon, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Wilsons.generate(&mut grid).expect("Wilson's maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_rhombic_maze_wilsons() {
        match Grid::new(MazeType::Rhombic, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Wilsons.generate(&mut grid).expect("Wilsons maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn test_wilsons_with_capture_steps() {
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 11, y: 11 };
        match Grid::new(MazeType::Orthogonal, 12, 12, start, goal, true) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Wilsons.generate(&mut grid).expect("Maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
                assert!(grid.generation_steps.is_some());
                let steps = grid.generation_steps.as_ref().unwrap(); assert!(!steps.is_empty());
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
