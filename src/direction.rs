use serde::{Serialize, Deserialize};
use std::fmt;
use std::convert::TryFrom;
use crate::cell::MazeType;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    // Orthogonal & intercardinal
    Up, Right, Down, Left,
    UpperRight, LowerRight, LowerLeft, UpperLeft,
    // Polar‑only
    Inward, Outward, Clockwise, CounterClockwise,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Direction::Up               => "Up",
            Direction::Right            => "Right",
            Direction::Down             => "Down",
            Direction::Left             => "Left",
            Direction::UpperRight       => "UpperRight",
            Direction::LowerRight       => "LowerRight",
            Direction::LowerLeft        => "LowerLeft",
            Direction::UpperLeft        => "UpperLeft",
            Direction::Inward           => "Inward",
            Direction::Outward          => "Outward",
            Direction::Clockwise        => "Clockwise",
            Direction::CounterClockwise => "CounterClockwise",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for Direction {
    type Error = crate::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "Up"               => Direction::Up,
            "Right"            => Direction::Right,
            "Down"             => Direction::Down,
            "Left"             => Direction::Left,
            "UpperRight"       => Direction::UpperRight,
            "LowerRight"       => Direction::LowerRight,
            "LowerLeft"        => Direction::LowerLeft,
            "UpperLeft"        => Direction::UpperLeft,
            "Inward"           => Direction::Inward,
            "Outward"          => Direction::Outward,
            "Clockwise"        => Direction::Clockwise,
            "CounterClockwise" => Direction::CounterClockwise,
            other =>
                return Err(crate::Error::InvalidDirection { direction: other.to_string() }),
        })
    }
}

impl Direction {
    /// “Which of these variants are legal for a given MazeType?”
    pub fn valid_for(&self, maze_type: MazeType) -> bool {
        use Direction::*;
        match maze_type {
            MazeType::Orthogonal => matches!(self, Up | Right | Down | Left),
            MazeType::Sigma      => matches!(self, Up | Right | Down | Left | UpperRight | LowerRight | LowerLeft | UpperLeft),
            MazeType::Delta      => matches!(self, Up | Down | UpperLeft | UpperRight | LowerLeft | LowerRight),
            MazeType::Polar      => matches!(self, Inward | Outward | Clockwise | CounterClockwise),
        }
    }
}
