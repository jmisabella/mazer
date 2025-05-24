
use std::collections::HashSet;
use crate::cell::CellOrientation;
use crate::direction::Direction;

fn triangle_unit_points(inverted: bool) -> [(f32, f32); 3] {
    let h = (3.0f32).sqrt() / 2.0;
    if inverted { 
        [(0.0, 0.0), (1.0, 0.0), (0.5, h)]
    } else {
        [(0.5, 0.0), (0.0, h), (1.0, h)]
    }
}

/// For a triangle cell: returns pairs of vertex‐indices that should be stroked.
pub fn delta_wall_segments(
    linked: &HashSet<Direction>,        // e.g. UpperLeft, Down, etc.
    orientation: CellOrientation         // Normal vs Inverted
) -> Vec<(usize, usize)> {
    // compute unit‐points with the correct "inverted" boolean
    let inverted = orientation == CellOrientation::Inverted;
    let _pts = triangle_unit_points(inverted);

    let mut walls = Vec::new();

    match orientation {
        CellOrientation::Normal => {
            if !linked.contains(&Direction::UpperLeft)  { walls.push((0, 1)); }
            if !linked.contains(&Direction::UpperRight) { walls.push((0, 2)); }
            if !linked.contains(&Direction::Down)       { walls.push((1, 2)); }
        }

        CellOrientation::Inverted => {
            if !linked.contains(&Direction::Up)         { walls.push((0, 1)); }
            if !linked.contains(&Direction::LowerLeft)  { walls.push((0, 2)); }
            if !linked.contains(&Direction::LowerRight) { walls.push((1, 2)); }
        }
    }

    walls
}

#[cfg(test)]
mod tests {
    use super::*;

}
