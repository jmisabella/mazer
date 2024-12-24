use crate::grid::Grid;
use crate::cell::Coordinates;

use std::collections::HashSet;

pub struct Wilsons;

impl Wilsons {
    pub fn generate(grid: &mut Grid) {
        let mut visited: HashSet<Coordinates> = HashSet::new();
        let mut rng = rand::thread_rng();

        // Mark the start cell as visited
        visited.insert(grid.start_coords);

        let grid_size = grid.cells.len() * grid.cells[0].len();

        while visited.len() < grid_size {
            // Choose a random unvisited cell to start the random walk
            let mut walk_start = loop {
                let x = grid.bounded_random_usize(grid.cells[0].len() - 1);
                let y = grid.bounded_random_usize(grid.cells.len() - 1);
                let coords = Coordinates { x, y };
                if !visited.contains(&coords) {
                    break coords;
                }
            };

            // Perform a random walk
            let mut walk: Vec<Coordinates> = vec![walk_start];
            let mut walk_set: HashSet<Coordinates> = HashSet::new();
            walk_set.insert(walk_start);

            while !visited.contains(walk.last().unwrap()) {
                let current = *walk.last().unwrap();
                let cell = grid.get(current.x, current.y); // Store the result of grid.get in a variable
                let neighbors_list: HashSet<Coordinates> = cell.neighbors();
                let neighbors: Vec<&Coordinates> = neighbors_list.iter().collect();
                let index = grid.bounded_random_usize(neighbors.len() - 1); 
                // Choose a random neighbor
                if let next = neighbors[index] {
                    if let Some(pos) = walk.iter().position(|&c| c == *next) {
                        // Loop detected: truncate the path
                        walk.truncate(pos + 1);
                    } else {
                        walk.push(*next);
                        walk_set.insert(*next);
                    }
                }
            }

            // Carve the path into the maze
            for pair in walk.windows(2) {
                if let [current, next] = pair {
                    grid.link(*current, *next);
                    visited.insert(*current);
                    visited.insert(*next);
                }
            }
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::{ MazeType, Coordinates };
    
    #[test]
    fn print_5_x_5_maze() {
        let mut grid = Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 });
        assert!(!grid.is_perfect_maze());
        Wilsons::generate(&mut grid);
        println!("\n\nWilson's\n\n{}\n\n", grid.to_asci());
        assert!(grid.is_perfect_maze());
    }

    #[test]
    fn print_12_x_6_maze() {
        let mut grid = Grid::new(MazeType::Orthogonal, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 });
        assert!(!grid.is_perfect_maze());
        Wilsons::generate(&mut grid);
        println!("\n\nWilson's\n\n{}\n\n", grid.to_asci());
        assert!(grid.is_perfect_maze());
    }

}
