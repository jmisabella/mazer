use crate::behaviors::maze::MazeGeneration;
use crate::grid::Grid;
use crate::cell::Coordinates;
use crate::error::Error;

use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
use rand::Rng;

// Disjoint-set data structure for tracking cell sets
struct DisjointSet {
    parent: HashMap<Coordinates, Coordinates>,
    rank: HashMap<Coordinates, u32>,
}

impl DisjointSet {
    fn new() -> Self {
        DisjointSet {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    fn make_set(&mut self, coords: Coordinates) {
        self.parent.insert(coords, coords);
        self.rank.insert(coords, 0);
    }

    fn find(&mut self, coords: Coordinates) -> Option<Coordinates> {
        if let Some(&parent) = self.parent.get(&coords) {
            if parent != coords {
                let root = self.find(parent)?;
                self.parent.insert(coords, root); // Path compression
                Some(root)
            } else {
                Some(coords)
            }
        } else {
            None
        }
    }

    fn union(&mut self, coords1: Coordinates, coords2: Coordinates) -> bool {
        let root1 = self.find(coords1);
        let root2 = self.find(coords2);

        if let (Some(r1), Some(r2)) = (root1, root2) {
            if r1 == r2 {
                return false; // Already in the same set
            }

            // Union by rank
            let rank1 = *self.rank.get(&r1).unwrap_or(&0);
            let rank2 = *self.rank.get(&r2).unwrap_or(&0);
            if rank1 < rank2 {
                self.parent.insert(r1, r2);
            } else if rank1 > rank2 {
                self.parent.insert(r2, r1);
            } else {
                self.parent.insert(r2, r1);
                self.rank.insert(r1, rank1 + 1);
            }
            true
        } else {
            false
        }
    }
}

pub struct Kruskals;

impl MazeGeneration for Kruskals {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        let mut rng = rand::thread_rng();
        let mut disjoint_set = DisjointSet::new();
        let mut edges: Vec<(Coordinates, Coordinates, u32)> = Vec::new();

        // Step 1: Initialize sets for each cell
        for y in 0..grid.height {
            for x in 0..grid.width {
                let coords = Coordinates { x, y };
                disjoint_set.make_set(coords);
            }
        }

        // Step 2: Collect all possible edges
        for y in 0..grid.height {
            for x in 0..grid.width {
                let coords = Coordinates { x, y };
                if let Ok(cell) = grid.get(coords) {
                    for &neighbor_coords in cell.neighbors().iter() {
                        // Only add edges in one direction to avoid duplicates
                        if neighbor_coords.x > coords.x || neighbor_coords.y > coords.y {
                            edges.push((coords, neighbor_coords, rng.gen()));
                        }
                    }
                }
            }
        }

        // Step 3: Shuffle edges for random selection
        edges.shuffle(&mut rng);

        // Capture initial state with no changed cells
        if grid.capture_steps {
            let changed_cells = HashSet::new();
            self.capture_step(grid, &changed_cells);
        }

        // Step 4: Process edges to build the maze
        for (coords1, coords2, _weight) in edges {
            if disjoint_set.union(coords1, coords2) {
                grid.link(coords1, coords2)?;
                // Capture step with changed cells after linking
                if grid.capture_steps {
                    let mut changed_cells = HashSet::new();
                    changed_cells.insert(coords1);
                    changed_cells.insert(coords2);
                    self.capture_step(grid, &changed_cells);
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
                Kruskals.generate(&mut grid).expect("Kruskal's maze generation failed");
                println!("\n\nKruskal's\n\n{}\n\n", grid.to_asci());
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
                Kruskals.generate(&mut grid).expect("Kruskal's maze generation failed");
                println!("\n\nKruskal's\n\n{}\n\n", grid.to_asci());
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
                Kruskals.generate(&mut grid).expect("Kruskal's maze generation failed");
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
                Kruskals.generate(&mut grid).expect("Kruskal's maze generation failed");
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
                Kruskals.generate(&mut grid).expect("Kruskal's maze generation failed");
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
                Kruskals.generate(&mut grid).expect("Kruskal's maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn generate_12_x_6_rhombille_maze_kruskals() {
        match Grid::new(MazeType::Rhombille, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Kruskals.generate(&mut grid).expect("Kruskals maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn test_kruskals_with_capture_steps() {
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 19, y: 19 };
        match Grid::new(MazeType::Orthogonal, 20, 20, start, goal, true) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                Kruskals.generate(&mut grid).expect("Maze generation failed");
                assert!(grid.is_perfect_maze().unwrap());
                assert!(grid.generation_steps.is_some());
                let steps = grid.generation_steps.as_ref().unwrap();
                assert!(!steps.is_empty());
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
