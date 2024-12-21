use crate::maze::grid::Grid;
use crate::maze::cell::{ MazeType, Cell, Coordinates };
use std::collections::HashSet;

pub struct AldousBroder;

impl AldousBroder {
    pub fn generate(grid: &mut Grid) {
        let mut visited = HashSet::new(); // To track visited cells
        let total_cells = grid.cells.iter().map(|row| row.len()).sum::<usize>();

        // Choose a random starting cell
        let mut current_coords = Coordinates {
            x: grid.bounded_random_usize(grid.cells[0].len()),
            y: grid.bounded_random_usize(grid.cells.len()),
        };
        visited.insert(current_coords);

        while visited.len() < total_cells {
           if let Some(current_cell) = grid.get_cell(current_coords) {
                // Get neighbors of the current cell
                let neighbors: Vec<_> = current_cell.neighbors().iter().cloned().collect();
                if neighbors.is_empty() {
                    continue; // In case of edge issues
                }

                // Select a random neighbor
                let random_index = grid.bounded_random_usize(neighbors.len() - 1);
                let neighbor_coords = neighbors[random_index];

                // If the neighbor hasn't been visited, link the cells
                if !visited.contains(&neighbor_coords) {
                    grid.link(current_coords, neighbor_coords);
                    visited.insert(neighbor_coords);
                }

                // Move to the neighbor
                current_coords = neighbor_coords;
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
        AldousBroder::generate(&mut grid);
        println!("\n\nAldous Broder\n\n{}\n\n", grid.to_asci());
        assert!(grid.is_perfect_maze());
    }
    
    #[test]
    fn print_12_x_6_maze() {
        let mut grid = Grid::new(MazeType::Orthogonal, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 });
        assert!(!grid.is_perfect_maze());
        AldousBroder::generate(&mut grid);
        println!("\n\nAldous Broder\n\n{}\n\n", grid.to_asci());
        assert!(grid.is_perfect_maze());
    }
}
