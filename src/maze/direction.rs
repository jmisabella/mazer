#[derive(Debug, Clone, PartialEq)]
pub enum HexDirection {
    Northwest,
    North,
    Northeast,
    Southwest,
    South,
    Southeast,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PolarDirection {
    Clockwise,
    CounterClockwise,
    Inward,
    Outward,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SquareDirection {
    North,
    East,
    South,
    West
}

#[derive(Debug, Clone, PartialEq)]
pub enum TriangleDirection {
    UpperLeft,
    UpperRight,
    Down,
    Up,
    LowerLeft,
    LowerRight,
}
