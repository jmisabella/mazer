use crate::maze::grid::Grid;
use crate::maze::cell::{ MazeType, Cell, Coordinates };
use std::collections::HashSet;

pub struct RecursiveBacktracker;

impl RecursiveBacktracker {
    pub fn generate(grid: &mut Grid) {
        // Create a stack to track the current path
        let mut stack: Vec<Coordinates> = Vec::new();
        let mut visited: HashSet<Coordinates> = HashSet::new();

        // Start at the start_coords
        stack.push(grid.start_coords);
        visited.insert(grid.start_coords);

        while let Some(current_coords) = stack.last().cloned() {
            // Get all unvisited neighbors
            let neighbors: Vec<Coordinates> = grid
                .get(current_coords.x, current_coords.y)
                .neighbors()
                .into_iter()
                .filter(|neighbor| !visited.contains(neighbor))
                .collect();

            if neighbors.is_empty() {
                // Backtrack if no unvisited neighbors
                stack.pop();
            } else {
                // Choose a random unvisited neighbor
                let random_index = {
                    let upper_bound = neighbors.len() - 1;
                    grid.bounded_random_usize(upper_bound)
                };
                let next_coords = neighbors[random_index];

                // Link current cell to the chosen neighbor
                grid.link(current_coords, next_coords);

                // Mark the neighbor as visited and push it onto the stack
                visited.insert(next_coords);
                stack.push(next_coords);
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn print_5_x_5_maze() {
        let mut grid = Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 });
        assert!(!grid.is_perfect_maze());
        RecursiveBacktracker::generate(&mut grid);
        println!("\n\nRecursive Backtracker\n\n{}\n\n", grid.clone().to_asci());
        assert!(grid.is_perfect_maze());
    }

    #[test]
    fn print_12_x_6_maze() {
        let mut grid = Grid::new(MazeType::Orthogonal, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 });
        assert!(!grid.is_perfect_maze());
        RecursiveBacktracker::generate(&mut grid);
        println!("\n\nRecursive Backtracker\n\n{}\n\n", grid.clone().to_asci());
        assert!(grid.is_perfect_maze());
    }

}

