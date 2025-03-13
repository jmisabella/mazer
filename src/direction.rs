use serde::{ Serialize, Deserialize };
use std::fmt;

pub trait Direction: Serialize { 
    fn as_str(&self) -> String
    where
        Self: Into<String> + Clone,
    {
        self.clone().into()
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

impl From<HexDirection> for String {
    fn from(direction: HexDirection) -> Self {
        match direction {
            HexDirection::Northwest => "Northwest".to_string(),
            HexDirection::North => "North".to_string(),
            HexDirection::Northeast => "Northeast".to_string(),
            HexDirection::Southwest => "Southwest".to_string(),
            HexDirection::South => "South".to_string(),
            HexDirection::Southeast => "Southeast".to_string(),
        }
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

impl From<PolarDirection> for String {
    fn from(direction: PolarDirection) -> Self {
        match direction {
            PolarDirection::Clockwise => "Clockwise".to_string(),
            PolarDirection::CounterClockwise => "CounterClockwise".to_string(),
            PolarDirection::Inward => "Inward".to_string(),
            PolarDirection::Outward => "Outward".to_string(),
        }
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

impl From<SquareDirection> for String {
    fn from(direction: SquareDirection) -> Self {
        match direction {
            SquareDirection::North => "North".to_string(),
            SquareDirection::East => "East".to_string(),
            SquareDirection::South => "South".to_string(),
            SquareDirection::West => "West".to_string(),
        }
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

impl From<TriangleDirection> for String {
    fn from(direction: TriangleDirection) -> Self {
        match direction {
            TriangleDirection::UpperLeft => "UpperLeft".to_string(),
            TriangleDirection::UpperRight => "UpperRight".to_string(),
            TriangleDirection::Down => "Down".to_string(),
            TriangleDirection::Up => "Up".to_string(),
            TriangleDirection::LowerLeft => "LowerLeft".to_string(),
            TriangleDirection::LowerRight => "LowerRight".to_string(),
        }
    }
}
