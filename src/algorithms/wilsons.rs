use crate::grid::Grid;
use crate::cell::Coordinates;
use crate::error::Error;

use std::collections::HashSet;

pub struct Wilsons;

impl Wilsons {
    pub fn generate(grid: &mut Grid) -> Result<(), Error> {
        let mut visited: HashSet<Coordinates> = HashSet::new();

        // Mark the start cell as visited
        visited.insert(grid.start_coords);

        //let grid_size = grid.cells.len() * grid.cells[0].len();
        let grid_size = grid.width * grid.height;

        while visited.len() < grid_size {
            // Choose a random unvisited cell to start the random walk
            let walk_start = loop {
                //let x = grid.bounded_random_usize(grid.cells[0].len() - 1);
                //let y = grid.bounded_random_usize(grid.cells.len() - 1);
                let x = grid.bounded_random_usize(grid.width - 1);
                let y = grid.bounded_random_usize(grid.height - 1);
                let coords = Coordinates { x, y };
                if !visited.contains(&coords) {
                    break coords;
                }
            };

            // Perform a random walk
            let mut walk: Vec<Coordinates> = vec![walk_start];
            let mut walk_set: HashSet<Coordinates> = HashSet::new();
            walk_set.insert(walk_start);

            while let Some(last) = walk.last() {
                if visited.contains(last) {
                    break;
                }
                let current = *last;
                let cell = grid.get(Coordinates { x: current.x, y: current.y })?;
                //let cell = match grid.get(Coordinates { x: current.x, y: current.y }) {
                //    Some(c) => c,
                //    None => return Err(Error::OutOfBoundsCoordinates {
                //        coordinates: current,
                //        maze_width: grid.width,
                //        maze_height: grid.height,
                //    }),
                //};
                let neighbors_list: HashSet<Coordinates> = cell.neighbors();
                let neighbors: Vec<&Coordinates> = neighbors_list.iter().collect();
            
                if neighbors.is_empty() {
                    return Err(Error::EmptyList); // Handle case where there are no neighbors
                }
                let index = grid.bounded_random_usize(neighbors.len() - 1);
            
                // Choose a random neighbor
                let next = neighbors[index];
                if let Some(pos) = walk.iter().position(|&c| c == *next) {
                    // Loop detected: truncate the path
                    walk.truncate(pos + 1);
                } else {
                    walk.push(*next);
                    walk_set.insert(*next);
                }
            }
            
            // Carve the path into the maze
            walk.windows(2) // iterator producing all windows of size 2
                .filter_map(|pair| { 
                    if let [current, next] = pair {
                        Some((current, next))
                    } else {
                        None
                    }
                })
                .for_each(|(current, next)| {
                    let _ = grid.link(*current, *next);
                    visited.insert(*current);
                    visited.insert(*next);
                });
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
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Wilsons::generate(&mut grid).expect("Wilson's maze generation failed");
                println!("\n\nWilson's\n\n{}\n\n", grid.to_asci());
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
                Wilsons::generate(&mut grid).expect("Wilson's maze generation failed");
                println!("\n\nWilson's\n\n{}\n\n", grid.to_asci());
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
                Wilsons::generate(&mut grid).expect("Wilson's maze generation failed");
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
                Wilsons::generate(&mut grid).expect("Wilson's maze generation failed");
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
                Wilsons::generate(&mut grid).expect("Wilson's maze generation failed");
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
                Wilsons::generate(&mut grid).expect("Wilson's maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

}
