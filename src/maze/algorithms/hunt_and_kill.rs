use crate::maze::grid::Grid;
use crate::maze::cell::{ MazeType, Coordinates };

pub struct HuntAndKill;

impl HuntAndKill {
    pub fn generate(grid: &mut Grid) {
        let mut visited = std::collections::HashSet::new();
        let mut current_coords = Coordinates {
            x: grid.bounded_random_usize(grid.width - 1),
            y: grid.bounded_random_usize(grid.height - 1),
        };
        visited.insert(current_coords);

        loop {
            // Kill Phase: Randomly carve passages to unvisited neighbors
            while let Some(next_coords) = Self::random_unvisited_neighbor(grid, &current_coords, &visited) {
                // Link the current cell with the chosen neighbor
                grid.link(current_coords, next_coords);
                visited.insert(next_coords);
                current_coords = next_coords;
            }

            // Hunt Phase: Find the first unvisited cell with at least one visited neighbor
            if let Some((new_coords, neighbor)) = Self::find_hunt_target(grid, &visited) {
                // Link the new cell with one of its visited neighbors
                grid.link(new_coords, neighbor);
                visited.insert(new_coords);
                current_coords = new_coords;
            } else {
                // No more unvisited cells, maze generation complete
                break;
            }
        }
    }

    /// Finds a random unvisited neighbor of the current cell.
    fn random_unvisited_neighbor(
        grid: &mut Grid,
        coords: &Coordinates,
        visited: &std::collections::HashSet<Coordinates>,
    ) -> Option<Coordinates> {

        if let Some(current_cell) = grid.get_cell(*coords) {
            let neighbors: Vec<_> = current_cell 
                .neighbors()
                .into_iter()
                .filter(|neighbor| !visited.contains(neighbor))
                .collect();

            if neighbors.is_empty() {
                None
            } else {
                let index = {
                    grid.bounded_random_usize(neighbors.len() - 1)
                };
                Some(neighbors[index])
            }
        } else {
            None
        }
    }

    /// Finds the first unvisited cell with at least one visited neighbor.
    fn find_hunt_target(
        grid: &Grid,
        visited: &std::collections::HashSet<Coordinates>,
    ) -> Option<(Coordinates, Coordinates)> {
        for y in 0..grid.height {
            for x in 0..grid.width {
                let coords = Coordinates { x, y };
                if visited.contains(&coords) {
                    continue;
                }
                if let Some(current_cell) = grid.get_cell(coords) {
                    if let Some(neighbor) = current_cell 
                        .neighbors()
                        .into_iter()
                        .find(|neighbor| visited.contains(neighbor))
                    {
                        return Some((coords, neighbor));
                    }
                } 
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn print_5_x_5_maze() {
        let mut grid = Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 });
        assert!(!grid.is_perfect_maze());
        HuntAndKill::generate(&mut grid);
        println!("\n\nHunt-and-Kill\n\n{}\n\n", grid.to_asci());
        assert!(grid.is_perfect_maze());
    }
    
    #[test]
    fn print_12_x_6_maze() {
        let mut grid = Grid::new(MazeType::Orthogonal, 12, 6, Coordinates { x: 0, y: 0 }, Coordinates { x: 11, y: 5 });
        assert!(!grid.is_perfect_maze());
        HuntAndKill::generate(&mut grid);
        println!("\n\nHunt-and-Kill\n\n{}\n\n", grid.to_asci());
        assert!(grid.is_perfect_maze());
    }
}

