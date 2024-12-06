use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HexDirection {
    Northwest,
    North,
    Northeast,
    Southwest,
    South,
    Southeast,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PolarDirection {
    Clockwise,
    CounterClockwise,
    Inward,
    Outward,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SquareDirection {
    North,
    East,
    South,
    West
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriangleDirection {
    UpperLeft,
    UpperRight,
    Down,
    Up,
    LowerLeft,
    LowerRight,
}
