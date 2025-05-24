
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
    use std::collections::HashSet;

    // Tests for triangle_unit_points
    #[test]
    fn test_triangle_unit_points_inverted() {
        let points = triangle_unit_points(true);
        let h = (3.0f32).sqrt() / 2.0;
        assert_eq!(points, [(0.0, 0.0), (1.0, 0.0), (0.5, h)]);
    }

    #[test]
    fn test_triangle_unit_points_normal() {
        let points = triangle_unit_points(false);
        let h = (3.0f32).sqrt() / 2.0;
        assert_eq!(points, [(0.5, 0.0), (0.0, h), (1.0, h)]);
    }

    // Tests for delta_wall_segments with Normal orientation
    #[test]
    fn test_delta_wall_segments_normal_empty() {
        let linked: HashSet<Direction> = HashSet::new();
        let walls = delta_wall_segments(&linked, CellOrientation::Normal);
        assert_eq!(walls, vec![(0, 1), (0, 2), (1, 2)]);
    }

    #[test]
    fn test_delta_wall_segments_normal_upperleft() {
        let mut linked = HashSet::new();
        linked.insert(Direction::UpperLeft);
        let walls = delta_wall_segments(&linked, CellOrientation::Normal);
        assert_eq!(walls, vec![(0, 2), (1, 2)]);
    }

    #[test]
    fn test_delta_wall_segments_normal_upperleft_upperright() {
        let mut linked = HashSet::new();
        linked.insert(Direction::UpperLeft);
        linked.insert(Direction::UpperRight);
        let walls = delta_wall_segments(&linked, CellOrientation::Normal);
        assert_eq!(walls, vec![(1, 2)]);
    }

    #[test]
    fn test_delta_wall_segments_normal_all_linked() {
        let mut linked = HashSet::new();
        linked.insert(Direction::UpperLeft);
        linked.insert(Direction::UpperRight);
        linked.insert(Direction::Down);
        let walls = delta_wall_segments(&linked, CellOrientation::Normal);
        assert_eq!(walls, vec![]);
    }

    #[test]
    fn test_delta_wall_segments_normal_irrelevant_direction() {
        let mut linked = HashSet::new();
        linked.insert(Direction::Up);
        let walls = delta_wall_segments(&linked, CellOrientation::Normal);
        assert_eq!(walls, vec![(0, 1), (0, 2), (1, 2)]);
    }

    // Tests for delta_wall_segments with Inverted orientation
    #[test]
    fn test_delta_wall_segments_inverted_empty() {
        let linked: HashSet<Direction> = HashSet::new();
        let walls = delta_wall_segments(&linked, CellOrientation::Inverted);
        assert_eq!(walls, vec![(0, 1), (0, 2), (1, 2)]);
    }

    #[test]
    fn test_delta_wall_segments_inverted_up() {
        let mut linked = HashSet::new();
        linked.insert(Direction::Up);
        let walls = delta_wall_segments(&linked, CellOrientation::Inverted);
        assert_eq!(walls, vec![(0, 2), (1, 2)]);
    }

    #[test]
    fn test_delta_wall_segments_inverted_up_lowerleft() {
        let mut linked = HashSet::new();
        linked.insert(Direction::Up);
        linked.insert(Direction::LowerLeft);
        let walls = delta_wall_segments(&linked, CellOrientation::Inverted);
        assert_eq!(walls, vec![(1, 2)]);
    }

    #[test]
    fn test_delta_wall_segments_inverted_all_linked() {
        let mut linked = HashSet::new();
        linked.insert(Direction::Up);
        linked.insert(Direction::LowerLeft);
        linked.insert(Direction::LowerRight);
        let walls = delta_wall_segments(&linked, CellOrientation::Inverted);
        assert_eq!(walls, vec![]);
    }

    #[test]
    fn test_delta_wall_segments_inverted_irrelevant_direction() {
        let mut linked = HashSet::new();
        linked.insert(Direction::UpperLeft);
        let walls = delta_wall_segments(&linked, CellOrientation::Inverted);
        assert_eq!(walls, vec![(0, 1), (0, 2), (1, 2)]);
    }

    #[test]
    fn test_delta_wall_segments_normal_mixed_directions() {
        let mut linked = HashSet::new();
        linked.insert(Direction::UpperLeft);
        linked.insert(Direction::Up);
        let walls = delta_wall_segments(&linked, CellOrientation::Normal);
        assert_eq!(walls, vec![(0, 2), (1, 2)]);
    }

    #[test]
    fn test_delta_wall_segments_inverted_mixed_directions() {
        let mut linked = HashSet::new();
        linked.insert(Direction::Down);
        linked.insert(Direction::LowerRight);
        let walls = delta_wall_segments(&linked, CellOrientation::Inverted);
        assert_eq!(walls, vec![(0, 1), (0, 2)]);
    }
}

