use crate::behaviors::maze::MazeGeneration;
use crate::algorithms::MazeAlgorithm;
use crate::grid::Grid;
use crate::cell::{Coordinates, MazeType};
use crate::error::Error;

use std::collections::HashMap;

use rand::prelude::SliceRandom;

pub struct Ellers;

impl MazeGeneration for Ellers {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        match grid.maze_type {
            MazeType::Orthogonal => {} // Proceed for Orthogonal grids
            maze_type => {
                return Err(Error::AlgorithmUnavailableForMazeType {
                    algorithm: MazeAlgorithm::Ellers,
                    maze_type,
                });
            }
        }

        let rows = grid.height;
        let cols = grid.width;
        let mut set_for_cell: HashMap<Coordinates, usize> = HashMap::new();
        let mut next_set_id = 0;

        for row in 0..rows {
            // Step 1: Initialize sets for unassigned cells in the current row
            for col in 0..cols {
                let coords = Coordinates { x: col, y: row };
                if !set_for_cell.contains_key(&coords) {
                    set_for_cell.insert(coords, next_set_id);
                    next_set_id += 1;
                }
            }

            // Step 2: Randomly join adjacent cells in the same row
            for col in 0..cols - 1 {
                let current_coords = Coordinates { x: col, y: row };
                let right_coords = Coordinates { x: col + 1, y: row };
                let current_set = *set_for_cell.get(&current_coords).unwrap();
                let right_set = *set_for_cell.get(&right_coords).unwrap();

                if current_set != right_set && grid.random_bool() {
                    // Link the cells and merge sets
                    grid.link(current_coords, right_coords)?;
                    // Update all cells in right_set to current_set
                    set_for_cell
                        .iter_mut()
                        .filter(|(_, set)| **set == right_set)
                        .for_each(|(_coords, set)| *set = current_set);
                }
            }

            // Step 3: Connect to the next row (except for the last row)
            if row < rows - 1 {
                // Group cells by set
                let mut cells_by_set: HashMap<usize, Vec<Coordinates>> = HashMap::new();
                for col in 0..cols {
                    let coords = Coordinates { x: col, y: row };
                    let set_id = *set_for_cell.get(&coords).unwrap();
                    cells_by_set
                        .entry(set_id)
                        .or_insert_with(Vec::new)
                        .push(coords);
                }

                // For each set, make at least one vertical connection
                for (_set_id, cells) in cells_by_set {
                    // Shuffle cells to randomize connections
                    let mut cells = cells;
                    cells.shuffle(&mut rand::thread_rng());

                    // Make at least one connection
                    let connect_count = 1 + grid.bounded_random_usize(cells.len() - 1); // At least 1
                    for &cell_coords in cells.iter().take(connect_count) {
                        let down_coords = Coordinates {
                            x: cell_coords.x,
                            y: cell_coords.y + 1,
                        };
                        grid.link(cell_coords, down_coords)?;
                        // Assign the set to the cell below
                        set_for_cell.insert(down_coords, set_for_cell[&cell_coords]);
                    }
                }
            }
        }

        // Step 4: Final row - connect all cells in different sets
        for col in 0..cols - 1 {
            let current_coords = Coordinates { x: col, y: rows - 1 };
            let right_coords = Coordinates { x: col + 1, y: rows - 1 };
            let current_set = *set_for_cell.get(&current_coords).unwrap();
            let right_set = *set_for_cell.get(&right_coords).unwrap();

            if current_set != right_set {
                grid.link(current_coords, right_coords)?;
                // Update sets
                set_for_cell
                    .iter_mut()
                    .filter(|(_, set)| **set == right_set)
                    .for_each(|(_coords, set)| *set = current_set);
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
                Ellers.generate(&mut grid).expect("Eller's maze generation failed");
                println!("\n\nEller's\n\n{}\n\n", grid.to_asci());
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
                Ellers.generate(&mut grid).expect("Eller's maze generation failed");
                println!("\n\nEller's\n\n{}\n\n", grid.to_asci());
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn reject_5_x_5_delta_ellers_maze() {
        match Grid::new(MazeType::Delta, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                match Ellers.generate(&mut grid) {
                    Ok(()) => panic!("Successfully generated an Eller's maze for a Delta grid, which should have been rejected!"),
                    Err(e) => println!("As expected, Delta grid is rejected for Eller's maze generation: {:?}", e),
                }
            }
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }

    #[test]
    fn reject_5_x_5_sigma_ellers_maze() {
        match Grid::new(MazeType::Sigma, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                match Ellers.generate(&mut grid) {
                    Ok(()) => panic!("Successfully generated an Eller's maze for a Sigma grid, which should have been rejected!"),
                    Err(e) => println!("As expected, Sigma grid is rejected for Eller's maze generation: {:?}", e),
                }
            }
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }

    #[test]
    fn reject_12_x_12_polar_ellers_maze() {
        match Grid::new(MazeType::Polar, 12, 12, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 11 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                match Ellers.generate(&mut grid) {
                    Ok(()) => panic!("Successfully generated an Eller's maze for a Polar grid, which should have been rejected!"),
                    Err(e) => println!("As expected, Polar grid is rejected for Eller's maze generation: {:?}", e),
                }
            }
            Err(e) => panic!("Unexpected error generating grid: {:?}", e),
        }
    }
}