use crate::behaviors::maze::MazeGeneration;
use crate::cell::Coordinates;
use crate::error::Error;
use crate::grid::Grid;
use rand::seq::SliceRandom;
use std::collections::HashSet;

pub struct ReverseDelete;

impl MazeGeneration for ReverseDelete {
    fn generate(&self, grid: &mut Grid) -> Result<(), Error> {
        // Step 1: Link all cells to their neighbors to create a fully connected grid
        // Collect coordinates and neighbors to avoid borrowing issues
        let links: Vec<(Coordinates, Coordinates)> = grid
            .cells
            .iter()
            .flat_map(|cell| {
                cell.neighbors_by_direction
                    .values()
                    .map(move |&neighbor| (cell.coords, neighbor))
            })
            .collect();

        for (coords, neighbor) in links {
            grid.link(coords, neighbor)?;
        }

        // Step 2: Collect all edges in the grid
        let mut edges = collect_all_edges(grid);

        // Step 3: Shuffle edges randomly
        let mut rng = rand::thread_rng();
        edges.shuffle(&mut rng);

        // Step 4: Process each edge, removing those that don't disconnect the graph
        for (u, v) in edges {
            // Temporarily remove the link
            {
                let cell_u = grid.get_mut(u)?;
                cell_u.linked.remove(&v);
                let cell_v = grid.get_mut(v)?;
                cell_v.linked.remove(&u);
            }

            // Check if u and v are still connected without this edge
            let still_connected = grid.all_connected_cells(u).contains(&v);

            if still_connected {
                // Edge can be removed permanently (already removed, so just capture if needed)
                if grid.capture_steps {
                    let mut changed_cells = HashSet::new();
                    changed_cells.insert(u);
                    changed_cells.insert(v);
                    self.capture_step(grid, &changed_cells);
                }
            } else {
                // Edge is necessary to maintain connectivity, add it back
                {
                    let cell_u = grid.get_mut(u)?;
                    cell_u.linked.insert(v);
                    let cell_v = grid.get_mut(v)?;
                    cell_v.linked.insert(u);
                }
            }
        }

        Ok(())
    }
}

/// Collects all unique edges in the grid, ensuring each edge is represented as (min, max) coordinates
fn collect_all_edges(grid: &Grid) -> Vec<(Coordinates, Coordinates)> {
    let mut edges = HashSet::new();
    for cell in &grid.cells {
        for &neighbor in cell.neighbors_by_direction.values() {
            let mut pair = [cell.coords, neighbor];
            pair.sort(); // Ensure consistent ordering: smaller coord first
            edges.insert((pair[0], pair[1]));
        }
    }
    edges.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::MazeType;

    #[test]
    fn test_reverse_delete_orthogonal_5x5() {
        let mut grid = Grid::new(
            MazeType::Orthogonal,
            5,
            5,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 4, y: 4 },
            false,
        )
        .unwrap();
        ReverseDelete.generate(&mut grid).unwrap();
        assert!(grid.is_perfect_maze().unwrap());
    }

    #[test]
    fn test_reverse_delete_with_capture_steps() {
        let mut grid = Grid::new(
            MazeType::Orthogonal,
            3,
            3,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 2, y: 2 },
            true,
        )
        .unwrap();
        ReverseDelete.generate(&mut grid).unwrap();
        assert!(grid.is_perfect_maze().unwrap());
        let steps = grid.generation_steps.unwrap();
        assert!(!steps.is_empty());
    }
}