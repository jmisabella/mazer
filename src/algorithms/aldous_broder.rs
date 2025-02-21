use crate::grid::Grid;
use crate::cell::Coordinates;
use crate::error::Error;

pub struct AldousBroder;

impl AldousBroder {
    pub fn generate(grid: &mut Grid) -> Result<(), Error> {
        // Step 1: Initialize visited tracking using Vec<Vec<bool>>
        let rows = grid.cells.len();
        let cols = grid.cells[0].len();
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
            if let Some(current_cell) = grid.get_cell(current_coords) {
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
                        grid.link(current_coords, random_neighbor);
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
