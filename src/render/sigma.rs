use std::collections::HashMap;
use crate::cell::{Cell, Coordinates};
use crate::direction::Direction;


/// Flat-top unit points for a hex of “side length = 1”
fn flat_top_unit_points() -> [(f32, f32); 6] {
    let s = 1.0_f32;
    let h = (3.0f32).sqrt() * s;
    [
        (0.5, 0.0),
        (1.5, 0.0),
        (2.0, h / 2.0),
        (1.5, h),
        (0.5, h),
        (0.0, h / 2.0),
    ]
}

/// Given a cell and the full map, return the unit-point edge indices to stroke.
/// For a hex cell: returns pairs of vertex-indices (into the 6 unit points)
/// that should be stroked (i.e. where there is _no_ passage).
pub fn sigma_wall_segments(
    cell: &Cell,
    cell_map: &HashMap<Coordinates, Cell>,
) -> Vec<(usize, usize)> {
    // If you need the points themselves, you can do:
    // let _unit_pts = flat_top_unit_points();

    let mut walls = Vec::new();
    let q = cell.coords.x;
    let r = cell.coords.y;
    let is_odd = (q & 1) == 1;

    for &dir in Direction::sigma_neighbors().iter() {
        // 1) where is the neighbor?
        let (dq, dr) = dir.offset_delta(is_odd);
        let neighbor_coord = Coordinates {
            x: (q as isize + dq) as usize,
            y: (r as isize + dr) as usize,
        };

        // 2) do we have that neighbor?
        if let Some(neighbor) = cell_map.get(&neighbor_coord) {
            // 3) skip walls along the solution path
            if cell.on_solution_path
               && neighbor.on_solution_path
               && (cell.distance - neighbor.distance).abs() == 1
            {
                continue;
            }

            // 4) check linkage by coords, not by Direction
            let linked       = cell.linked.contains(&neighbor_coord);
            let back_linked  = neighbor.linked.contains(&cell.coords);

            // 5) only draw if neither side thinks it’s open
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

        // allow for tiny floating-point rounding errors
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

    /// Helper to make a default Sigma‐cell at (x,y).
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
        let map = HashMap::new();
        let walls = sigma_wall_segments(&cell, &map);
        assert!(walls.is_empty(), "expected no walls when there are no neighbors");
    }

    #[test]
    fn down_neighbor_unlinked_yields_down_wall() {
        // cell at (0,0), neighbor at (0,1)
        let cell = mk_cell(0, 0);
        let mut map = HashMap::new();
        let neighbor = mk_cell(0, 1);
        map.insert(neighbor.coords, neighbor);

        let walls = sigma_wall_segments(&cell, &map);
        // flat-top Down maps to vertex‐indices (3,4)
        assert_eq!(walls, vec![(3, 4)]);
    }

    #[test]
    fn down_neighbor_linked_yields_no_walls() {
        // same as above but mark them linked
        let mut cell = mk_cell(0, 0);
        let mut map = HashMap::new();
        let mut neighbor = mk_cell(0, 1);

        // link both ways
        cell.linked.insert(neighbor.coords);
        neighbor.linked.insert(cell.coords);

        map.insert(neighbor.coords, neighbor);

        let walls = sigma_wall_segments(&cell, &map);
        assert!(
            walls.is_empty(),
            "expected no walls when cells are linked"
        );
    }

    #[test]
    fn solution_path_cells_are_skipped() {
        // same layout, but both on solution path and distances differ by 1
        let mut cell = mk_cell(0, 0);
        let mut map = HashMap::new();
        let mut neighbor = mk_cell(0, 1);

        cell.on_solution_path = true;
        cell.distance = 0;

        neighbor.on_solution_path = true;
        neighbor.distance = 1;

        map.insert(neighbor.coords, neighbor);

        let walls = sigma_wall_segments(&cell, &map);
        assert!(
            walls.is_empty(),
            "expected solution‐path walls to be skipped"
        );
    }
}