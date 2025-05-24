use crate::cell::{Cell, Coordinates};
use crate::direction::Direction;
use crate::Grid;

/// Returns the 6 unit points of a flat-top hexagon as (x, y) coordinates.
pub fn flat_top_unit_points() -> [(f32, f32); 6] {
    let s = 1.0_f32;
    let h = (3.0_f32).sqrt() * s;
    [
        (0.5, 0.0),      // Vertex 0
        (1.5, 0.0),      // Vertex 1
        (2.0, h / 2.0),  // Vertex 2
        (1.5, h),        // Vertex 3
        (0.5, h),        // Vertex 4
        (0.0, h / 2.0),  // Vertex 5
    ]
}

/// Given a cell and the full map, return the unit-point edge indices to stroke.
/// For a hex cell: returns pairs of vertex-indices (into the 6 unit points)
/// that should be stroked (i.e. where there is _no_ passage).
pub fn sigma_wall_segments(
    cell: &Cell,
    grid: &Grid,
) -> Vec<(usize, usize)> {
    let mut walls = Vec::new();
    let q = cell.coords.x;
    let r = cell.coords.y;
    let is_odd = (q & 1) == 1;

    for &dir in Direction::sigma_neighbors().iter() {
        let (dq, dr) = dir.offset_delta(is_odd);
        // Compute neighbor coordinates, checking for underflow/overflow
        let x = (q as isize + dq);
        let y = (r as isize + dr);
        // Skip if coordinates are negative or out of bounds
        if x < 0 || y < 0 || x >= grid.width as isize || y >= grid.height as isize {
            continue;
        }
        let neighbor_coord = Coordinates {
            x: x as usize,
            y: y as usize,
        };
        // Skip if neighbor is the same cell
        if neighbor_coord == cell.coords {
            continue;
        }

        if let Ok(neighbor) = grid.get(neighbor_coord) {
            if cell.on_solution_path
                && neighbor.on_solution_path
                && (cell.distance - neighbor.distance).abs() == 1
            {
                continue;
            }

            let linked = cell.linked.contains(&neighbor_coord);
            let back_linked = neighbor.linked.contains(&cell.coords);
            println!(
                "Dir: {:?}, Neighbor: {:?}, Linked: {}, BackLinked: {}",
                dir, neighbor_coord, linked, back_linked
            );

            if !(linked || back_linked) {
                let (i, j) = dir.vertex_indices();
                walls.push((i, j));
            }
        }
    }

    walls
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};
    use crate::cell::{Cell, Coordinates, CellOrientation, MazeType};

    #[test]
    fn hex_unit_points_correctness() {
        let pts = flat_top_unit_points();
        let s = 1.0_f32;
        let h = (3.0_f32).sqrt() * s;

        let expected = [
            (0.5, 0.0),
            (1.5, 0.0),
            (2.0, h / 2.0),
            (1.5, h),
            (0.5, h),
            (0.0, h / 2.0),
        ];

        let eps = 1e-6_f32;

        for (i, &(x, y)) in pts.iter().enumerate() {
            let (ex, ey) = expected[i];
            assert!(
                (x - ex).abs() < eps,
                "x[{}]: got {}, expected {}",
                i,
                x,
                ex
            );
            assert!(
                (y - ey).abs() < eps,
                "y[{}]: got {}, expected {}",
                i,
                y,
                ey
            );
        }
    }

    /// Helper to make a default Sigma-cell at (x, y).
    fn mk_cell(x: usize, y: usize) -> Cell {
        let mut c = Cell::default();
        c.coords = Coordinates { x, y };
        c.maze_type = MazeType::Sigma;
        c.orientation = CellOrientation::Normal;
        c
    }

    #[test]
    fn no_neighbors_yields_no_walls() {
        let cell = mk_cell(0, 0);
        let grid = Grid {
            cells: vec![cell.clone()],
            width: 1,
            height: 1,
            maze_type: MazeType::Sigma,
            seed: 0,
            start_coords: Coordinates { x: 0, y: 0 },
            goal_coords: Coordinates { x: 0, y: 0 },
        };
        let walls = sigma_wall_segments(&cell, &grid);
        assert!(walls.is_empty(), "expected no walls when there are no neighbors");
    }

    #[test]
    fn down_neighbor_unlinked_yields_down_wall() {
        let cell = mk_cell(0, 0);
        let neighbor = mk_cell(0, 1);
        let grid = Grid {
            cells: vec![cell.clone(), neighbor],
            width: 1,
            height: 2,
            maze_type: MazeType::Sigma,
            seed: 0,
            start_coords: Coordinates { x: 0, y: 0 },
            goal_coords: Coordinates { x: 0, y: 1 },
        };
        let walls = sigma_wall_segments(&cell, &grid);
        assert_eq!(walls, vec![(3, 4)]);
    }


    #[test]
    fn down_neighbor_linked_yields_no_walls() {
        let mut cell = mk_cell(0, 0);
        let mut neighbor = mk_cell(0, 1);
        cell.linked.insert(neighbor.coords);
        neighbor.linked.insert(cell.coords);

        // Ensure cells match row-major order: (0,0) at index 0, (0,1) at index 1
        let cells = vec![cell.clone(), neighbor.clone()];
        let grid = Grid {
            cells,
            width: 1,
            height: 2,
            maze_type: MazeType::Sigma,
            seed: 0,
            start_coords: Coordinates { x: 0, y: 0 },
            goal_coords: Coordinates { x: 0, y: 1 },
        };

        let walls = sigma_wall_segments(&cell, &grid);

        // Debug output
        println!("Walls: {:?}", walls);

        assert!(walls.is_empty(), "expected no walls when cells are linked");
    }

    #[test]
    fn solution_path_cells_are_skipped() {
        let mut cell = mk_cell(0, 0);
        let mut neighbor = mk_cell(0, 1);
        cell.on_solution_path = true;
        cell.distance = 0;
        neighbor.on_solution_path = true;
        neighbor.distance = 1;
        let grid = Grid {
            cells: vec![cell.clone(), neighbor.clone()],
            width: 1,
            height: 2,
            maze_type: MazeType::Sigma,
            seed: 0,
            start_coords: Coordinates { x: 0, y: 0 },
            goal_coords: Coordinates { x: 0, y: 1 },
        };
        let walls = sigma_wall_segments(&cell, &grid);
        assert!(walls.is_empty(), "expected solution-path walls to be skipped");
    }
}