use serde::{ Serialize, Deserialize };
use std::fmt;

pub trait Direction: Serialize { 
//    fn to_string(&self) -> String;
    fn as_str(&self) -> String {
        serde_json::to_string(&self).unwrap().replace("\"", "")
    }
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
impl Direction for HexDirection {}
impl fmt::Display for HexDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub enum PolarDirection {
    Clockwise,
    CounterClockwise,
    Inward,
    Outward,
}
impl Direction for PolarDirection {}
impl fmt::Display for PolarDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub enum SquareDirection {
    North,
    East,
    South,
    West
}
impl Direction for SquareDirection {}
impl fmt::Display for SquareDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
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
impl Direction for TriangleDirection {}
impl fmt::Display for TriangleDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
