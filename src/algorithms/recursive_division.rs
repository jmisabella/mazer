use crate::behaviors::maze::MazeGeneration;
use crate::algorithms::MazeAlgorithm;
use crate::grid::Grid;
use crate::cell::{Coordinates, MazeType};
use crate::error::Error;
use std::collections::{HashMap, HashSet};

pub struct RecursiveDivision;

impl MazeGeneration for RecursiveDivision {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        // Check supported maze types
        match grid.maze_type {
            MazeType::Orthogonal | MazeType::Rhombic => {},
            maze_type => {
                return Err(Error::AlgorithmUnavailableForMazeType {
                    algorithm: MazeAlgorithm::RecursiveDivision,
                    maze_type,
                });
            }
        }

        // Link all possible neighbors to create a fully connected grid
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.has_cell(x, y) {
                    let coords = Coordinates { x, y };
                    let cell = grid.get(coords).unwrap();

                    let neighbors: Vec<Coordinates> = grid.get(coords).unwrap().neighbors_by_direction.values().copied().collect();
                    for neighbor_coords in neighbors {
                        grid.link(coords, neighbor_coords)?;
                    }
                }
            }
        }

        // Capture initial fully linked state if required
        if grid.capture_steps {
            let changed_cells = HashSet::new();
            self.capture_step(grid, &changed_cells);
        }

        // Collect all cells and start division
        let all_cells: HashSet<Coordinates> = (0..grid.height)
            .flat_map(|y| (0..grid.width).map(move |x| Coordinates { x, y }))
            .filter(|&coords| grid.has_cell(coords.x, coords.y))
            .collect();
        self.divide(grid, &all_cells)?;

        Ok(())
    }
}

impl RecursiveDivision {
    fn divide(&self, grid: &mut Grid, region: &HashSet<Coordinates>) -> Result<(), Error> {
        if region.len() <= 1 {
            return Ok(());
        }

        // Compute transformed coordinates based on maze type
        let u_v: HashMap<Coordinates, (isize, isize)> = region.iter().map(|&coords| {
            let x = coords.x as isize;
            let y = coords.y as isize;
            let (u, v) = match grid.maze_type {
                MazeType::Orthogonal => (x, y),
                MazeType::Rhombic => ((x + y) / 2, (x - y) / 2),
                _ => unreachable!(), // Checked in generate
            };
            (coords, (u, v))
        }).collect();

        // Determine min and max u and v
        let min_u = u_v.values().map(|&(u, _)| u).min().unwrap();
        let max_u = u_v.values().map(|&(u, _)| u).max().unwrap();
        let min_v = u_v.values().map(|&(_, v)| v).min().unwrap();
        let max_v = u_v.values().map(|&(_, v)| v).max().unwrap();

        // Choose division direction based on range
        let divide_along_u = if (max_u - min_u) > (max_v - min_v) {
            true
        } else if (max_v - min_v) > (max_u - min_u) {
            false
        } else {
            grid.random_bool()
        };

        if divide_along_u && max_u > min_u {
            let u_wall = min_u + (grid.bounded_random_usize((max_u - min_u) as usize) as isize);
            // Find connections between u = u_wall and u = u_wall + 1
            let mut wall_pairs = Vec::new();
            for &coords in region {
                let (u, _) = u_v[&coords];
                if u == u_wall {
                    let cell = grid.get(coords).unwrap();
                    for &neighbor_coords in cell.linked.iter() {
                        if region.contains(&neighbor_coords) {
                            let (neighbor_u, _) = u_v[&neighbor_coords];
                            if neighbor_u == u_wall + 1 {
                                wall_pairs.push((coords, neighbor_coords));
                            }
                        }
                    }
                }
            }

            // Create wall with a passage
            if !wall_pairs.is_empty() {
                let passage_index = grid.bounded_random_usize(wall_pairs.len());
                let passage = wall_pairs.remove(passage_index);
                let mut changed_cells = HashSet::new();
                for (c1, c2) in wall_pairs {
                    grid.unlink(c1, c2)?;
                    if grid.capture_steps {
                        changed_cells.insert(c1);
                        changed_cells.insert(c2);
                    }
                }
                if grid.capture_steps && !changed_cells.is_empty() {
                    self.capture_step(grid, &changed_cells);
                }
            }

            // Split and recurse
            let left_region: HashSet<Coordinates> = region.iter()
                .filter(|&&coords| u_v[&coords].0 <= u_wall)
                .cloned()
                .collect();
            let right_region: HashSet<Coordinates> = region.iter()
                .filter(|&&coords| u_v[&coords].0 >= u_wall + 1)
                .cloned()
                .collect();
            self.divide(grid, &left_region)?;
            self.divide(grid, &right_region)?;
        } else if max_v > min_v {
            let v_wall = min_v + (grid.bounded_random_usize((max_v - min_v) as usize) as isize);
            // Find connections between v = v_wall and v = v_wall + 1
            let mut wall_pairs = Vec::new();
            for &coords in region {
                let (_, v) = u_v[&coords];
                if v == v_wall {
                    let cell = grid.get(coords).unwrap();
                    for &neighbor_coords in cell.linked.iter() {
                        if region.contains(&neighbor_coords) {
                            let (_, neighbor_v) = u_v[&neighbor_coords];
                            if neighbor_v == v_wall + 1 {
                                wall_pairs.push((coords, neighbor_coords));
                            }
                        }
                    }
                }
            }

            // Create wall with a passage
            if !wall_pairs.is_empty() {
                let passage_index = grid.bounded_random_usize(wall_pairs.len());
                let passage = wall_pairs.remove(passage_index);
                let mut changed_cells = HashSet::new();
                for (c1, c2) in wall_pairs {
                    grid.unlink(c1, c2)?;
                    if grid.capture_steps {
                        changed_cells.insert(c1);
                        changed_cells.insert(c2);
                    }
                }
                if grid.capture_steps && !changed_cells.is_empty() {
                    self.capture_step(grid, &changed_cells);
                }
            }

            // Split and recurse
            let bottom_region: HashSet<Coordinates> = region.iter()
                .filter(|&&coords| u_v[&coords].1 <= v_wall)
                .cloned()
                .collect();
            let top_region: HashSet<Coordinates> = region.iter()
                .filter(|&&coords| u_v[&coords].1 >= v_wall + 1)
                .cloned()
                .collect();
            self.divide(grid, &bottom_region)?;
            self.divide(grid, &top_region)?;
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
        let mut grid = Grid::new(
            MazeType::Orthogonal,
            5,
            5,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 4, y: 4 },
            false
        ).unwrap();
        RecursiveDivision.generate(&mut grid).unwrap();
        assert!(grid.is_perfect_maze().unwrap());
        println!("\n\nOrthogonal 5x5\n\n{}\n\n", grid.to_asci());
    }

    #[test]
    fn generate_and_print_5_x_5_rhombic_maze() {
        let mut grid = Grid::new(
            MazeType::Rhombic,
            5,
            5,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 4, y: 4 },
            false
        ).unwrap();
        RecursiveDivision.generate(&mut grid).unwrap();
        assert!(grid.is_perfect_maze().unwrap());
    }

    #[test]
    fn test_recursive_division_with_capture_steps_orthogonal() {
        let mut grid = Grid::new(
            MazeType::Orthogonal,
            5,
            5,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 4, y: 4 },
            true
        ).unwrap();
        RecursiveDivision.generate(&mut grid).unwrap();
        assert!(grid.is_perfect_maze().unwrap());
        let steps = grid.generation_steps.unwrap();
        assert!(!steps.is_empty());
        assert!(steps.iter().any(|step| step.count_edges() > 0));
    }

    #[test]
    fn test_recursive_division_with_capture_steps_rhombic() {
        let mut grid = Grid::new(
            MazeType::Rhombic,
            5,
            5,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 4, y: 4 },
            true
        ).unwrap();
        RecursiveDivision.generate(&mut grid).unwrap();
        assert!(grid.is_perfect_maze().unwrap());
        let steps = grid.generation_steps.unwrap();
        assert!(!steps.is_empty());
        assert!(steps.iter().any(|step| step.count_edges() > 0));
    }

    #[test]
    fn reject_unsupported_maze_type() {
        let mut grid = Grid::new(
            MazeType::Delta,
            5,
            5,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 4, y: 4 },
            false
        ).unwrap();
        assert!(RecursiveDivision.generate(&mut grid).is_err());
    }

    #[test]
    fn generate_12_x_24_rhombic_recursive_division() {
        match Grid::new(MazeType::Rhombic, 12, 24, Coordinates { x: 0, y: 0 }, Coordinates { x: 5, y: 23 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                RecursiveDivision.generate(&mut grid).expect("RecursiveDivision maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }
}

// use crate::behaviors::maze::MazeGeneration;
// use crate::algorithms::MazeAlgorithm;
// use crate::grid::Grid;
// use crate::cell::{Coordinates, MazeType};
// use crate::error::Error;
// use std::collections::HashSet;

// pub struct RecursiveDivision;

// impl MazeGeneration for RecursiveDivision {
//     fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
//         match grid.maze_type {
//             MazeType::Orthogonal => {}
//             maze_type => {
//                 return Err(Error::AlgorithmUnavailableForMazeType {
//                     algorithm: MazeAlgorithm::RecursiveDivision,
//                     maze_type,
//                 });
//             }
//         }

//         // Link all adjacent cells to start with no walls
//         for y in 0..grid.height {
//             for x in 0..grid.width {
//                 let coords = Coordinates { x, y };
//                 if x + 1 < grid.width {
//                     grid.link(coords, Coordinates { x: x + 1, y })?;
//                 }
//                 if y + 1 < grid.height {
//                     grid.link(coords, Coordinates { x, y: y + 1 })?;
//                 }
//             }
//         }

//         if grid.capture_steps {
//             let changed_cells = HashSet::new();
//             self.capture_step(grid, &changed_cells); // Captures fully open grid
//         }

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
//         if width <= 1 || height <= 1 {
//             return Ok(());
//         }

//         let divide_horizontally = if height > width {
//             true // Prefer horizontal if taller
//         } else if width > height {
//             false // Prefer vertical if wider
//         } else {
//             grid.random_bool() // Random if square
//         };

//         if divide_horizontally {
//             let wall_y = y + grid.bounded_random_usize(height - 1);
//             if wall_y >= grid.height - 1 {
//                 return Ok(());
//             }
//             let passage_x = x + grid.bounded_random_usize(width);

//             let mut changed_cells = HashSet::new();
//             for col in x..x + width {
//                 if col != passage_x {
//                     let coords = Coordinates { x: col, y: wall_y };
//                     let below = Coordinates { x: col, y: wall_y + 1 };
//                     grid.unlink(coords, below)?; // Adds wall by removing link
//                     if grid.capture_steps {
//                         changed_cells.insert(coords);
//                         changed_cells.insert(below);
//                     }
//                 }
//             }

//             if grid.capture_steps && !changed_cells.is_empty() {
//                 self.capture_step(grid, &changed_cells);
//             }

//             let top_height = wall_y - y + 1;
//             let bottom_height = height - top_height;
//             self.divide(grid, x, y, width, top_height)?;
//             self.divide(grid, x, wall_y + 1, width, bottom_height)?;
//         } else {
//             let wall_x = x + grid.bounded_random_usize(width - 1);
//             if wall_x >= grid.width - 1 {
//                 return Ok(());
//             }
//             let passage_y = y + grid.bounded_random_usize(height);

//             let mut changed_cells = HashSet::new();
//             for row in y..y + height {
//                 if row != passage_y {
//                     let coords = Coordinates { x: wall_x, y: row };
//                     let right = Coordinates { x: wall_x + 1, y: row };
//                     grid.unlink(coords, right)?; // Adds wall by removing link
//                     if grid.capture_steps {
//                         changed_cells.insert(coords);
//                         changed_cells.insert(right);
//                     }
//                 }
//             }

//             if grid.capture_steps && !changed_cells.is_empty() {
//                 self.capture_step(grid, &changed_cells);
//             }

//             let left_width = wall_x - x + 1;
//             let right_width = width - left_width;
//             self.divide(grid, x, y, left_width, height)?;
//             self.divide(grid, wall_x + 1, y, right_width, height)?;
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
//                 let has_linked_cells = steps.iter().any(|step| {
//                     step.cells.iter().filter_map(|opt| opt.as_ref()).any(|cell| !cell.linked.is_empty())
//                 });
//                 assert!(has_linked_cells, "No cells were linked during maze generation");
//                 let has_open_walls = steps.iter().any(|step| {
//                     step.cells.iter().filter_map(|opt| opt.as_ref()).any(|cell| !cell.open_walls.is_empty())
//                 });
//                 assert!(has_open_walls, "No cells have open walls in generation steps");
//             }
//             Err(e) => panic!("Unexpected error generating grid: {:?}", e),
//         }
//     }
// }
