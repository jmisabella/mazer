
use std::collections::HashSet;
use crate::cell::CellOrientation;
use crate::direction::Direction;

fn triangle_height(cell_size: f32) -> f32 {
    cell_size * (3.0f32).sqrt() / 2.0
}

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

    #[test]
    fn test_triangle_height_unit() {
        // for cell_size = 2.0, height == sqrt(3)
        let h = triangle_height(2.0);
        let expected = (3.0f32).sqrt();
        assert!((h - expected).abs() < 1e-6);
    }

    #[test]
    fn test_triangle_height_various() {
        // arbitrary values
        for &size in &[0.0_f32, 1.0, 4.5, 10.0] {
            let h = triangle_height(size);
            let expected = size * (3.0_f32).sqrt() / 2.0;
            // 1e-6 is scientific notation, for 1 x 10 to the negative 6th, or 0.000001
            assert!((h - expected).abs() < 1e-6,
                "size {} → got {}, expected {}", size, h, expected);
        }
    }
}
