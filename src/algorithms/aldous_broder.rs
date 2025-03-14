use crate::grid::Grid;
use crate::cell::Coordinates;
use crate::error::Error;

pub struct AldousBroder;

impl AldousBroder {
    pub fn generate(grid: &mut Grid) -> Result<(), Error> {
        // Step 1: Initialize visited tracking using Vec<Vec<bool>>
        let rows = grid.height;
        let cols = grid.width;
        let mut visited = vec![vec![false; cols]; rows];
        let total_cells = rows * cols;

        let rand_x = grid.bounded_random_usize(cols - 1);
        let rand_y = grid.bounded_random_usize(rows - 1);
        // Step 2: Choose a random starting cell
        let mut current_coords = Coordinates {
            x: rand_x,
            y: rand_y,
        };
        visited[current_coords.y][current_coords.x] = true; // Mark as visited
        let mut visited_count = 1;

        // Step 3: Loop until all cells are visited
        while visited_count < total_cells {
            if let Ok(current_cell) = grid.get(current_coords) {
                // Get neighbors of the current cell
                let neighbors: Vec<Coordinates> = current_cell
                    .neighbors()
                    .iter()
                    .filter(|&&coords| coords.y < rows && coords.x < cols)
                    .cloned()
                    .collect();

                if !neighbors.is_empty() {
                    // Pick a random neighbor
                    let random_index = grid.bounded_random_usize(neighbors.len() - 1);
                    let random_neighbor = neighbors[random_index];

                    // If the neighbor hasn't been visited, link it and update visited
                    if !visited[random_neighbor.y][random_neighbor.x] {
                        grid.link(current_coords, random_neighbor)?;
                        visited[random_neighbor.y][random_neighbor.x] = true;
                        visited_count += 1;
                    }

                    // Move to the selected neighbor
                    current_coords = random_neighbor;
                }
            } else {
                // If the current cell is invalid (edge case), break out
                break;
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
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                AldousBroder::generate(&mut grid).expect("AldousBroder maze generation failed");
                println!("\n\nAldous Broder\n\n{}\n\n", grid.to_asci());
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
                AldousBroder::generate(&mut grid).expect("AldousBroder maze generation failed");
                println!("\n\nAldous Broder\n\n{}\n\n", grid.to_asci());
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
                AldousBroder::generate(&mut grid).expect("AldousBroder maze generation failed");
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
                AldousBroder::generate(&mut grid).expect("AldousBroder maze generation failed");
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
                AldousBroder::generate(&mut grid).expect("AldousBroder maze generation failed");
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
                AldousBroder::generate(&mut grid).expect("AldousBroder maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

}
