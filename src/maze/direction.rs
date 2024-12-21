use serde::{ Serialize, Deserialize };

pub trait Direction { 
    fn to_string(&self) -> String;
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub enum HexDirection {
    Northwest,
    North,
    Northeast,
    Southwest,
    South,
    Southeast,
}
impl Direction for HexDirection {
    fn to_string(&self) -> String {
        return serde_json::to_string(&self).unwrap().replace("\"", "");
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub enum PolarDirection {
    Clockwise,
    CounterClockwise,
    Inward,
    Outward,
}
impl Direction for PolarDirection {
    fn to_string(&self) -> String {
        return serde_json::to_string(&self).unwrap().replace("\"", "");
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub enum SquareDirection {
    North,
    East,
    South,
    West
}
impl Direction for SquareDirection {
    fn to_string(&self) -> String {
        return serde_json::to_string(&self).unwrap().replace("\"", "");
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriangleDirection {
    UpperLeft,
    UpperRight,
    Down,
    Up,
    LowerLeft,
    LowerRight,
}
impl Direction for TriangleDirection {
    fn to_string(&self) -> String {
        return serde_json::to_string(&self).unwrap().replace("\"", "");
    }
}
