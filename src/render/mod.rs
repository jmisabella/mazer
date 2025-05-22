use crate::cell::{Cell, Coordinates};

pub mod delta;
pub mod sigma;
pub mod heatmap;

/// Returns a Vec of all on‐path, unvisited cell coordinates,
/// **sorted by each cell’s `distance`** ascending.
pub fn solution_path_order(cells: &[Cell]) -> Vec<Coordinates> {
    // 1) grab all the cells we actually want
    let mut on_path: Vec<&Cell> = cells
        .iter()
        .filter(|c| c.on_solution_path && !c.is_visited)
        .collect();

    // 2) sort those by their i32 distance
    on_path.sort_unstable_by_key(|c| c.distance);

    // 3) pull out just the coords
    on_path.into_iter().map(|c| c.coords).collect()
}