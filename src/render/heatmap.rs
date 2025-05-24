
/// Given a cell’s distance and the max distance in the maze,
/// returns an index 0–9 for picking a shade.
pub fn shade_index(distance: usize, max_distance: usize) -> usize {
    if max_distance == 0 { return 0 }
    let idx = (distance * 10) / max_distance;
    idx.min(9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_max_distance() {
        // when max_distance is zero, always returns 0
        assert_eq!(shade_index(0, 0), 0);
        assert_eq!(shade_index(5, 0), 0);
        assert_eq!(shade_index(100, 0), 0);
    }

    #[test]
    fn basic_proportions() {
        // exact fractions of the range
        assert_eq!(shade_index(0, 10), 0);   // 0/10 → 0
        assert_eq!(shade_index(5, 10), 5);   // 5/10 → 5
        assert_eq!(shade_index(10, 10), 9);  // 10/10 → 10 → clamped to 9
    }

    #[test]
    fn clamping_above_one() {
        // anything above produces values ≥10, but clamps to 9
        assert_eq!(shade_index(11, 10), 9);
        assert_eq!(shade_index(100, 10), 9);
    }

    #[test]
    fn varied_ranges() {
        // with max_distance = 20, we expect:
        //  1*10/20 = 0, 2*10/20 = 1, 19*10/20 = 9
        assert_eq!(shade_index(1, 20), 0);
        assert_eq!(shade_index(2, 20), 1);
        assert_eq!(shade_index(19, 20), 9);
    } 
}