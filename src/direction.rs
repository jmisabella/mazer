use serde::{Serialize, Deserialize};
use std::fmt;
use std::convert::TryFrom;
use crate::cell::MazeType;

#[repr(u32)]
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

impl TryFrom<u32> for Direction {
    type Error = crate::Error;
    fn try_from(code: u32) -> Result<Self, Self::Error> {
        use Direction::*;
        Ok(match code {
            0  => Up,
            1  => Right,
            2  => Down,
            3  => Left,
            4  => UpperRight,
            5  => LowerRight,
            6  => LowerLeft,
            7  => UpperLeft,
            8  => Inward,
            9  => Outward,
            10 => Clockwise,
            11 => CounterClockwise,
            _  => return Err(crate::Error::InvalidDirection { direction: code.to_string() }),
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

    pub fn all() -> &'static [Direction] {
        use Direction::*;
        &[
            Up, Right, Down, Left,
            UpperRight, LowerRight, LowerLeft, UpperLeft,
            Inward, Outward, Clockwise, CounterClockwise,
        ]
    }

    /// Only the six flat-top neighbors for a Sigma (hex) maze.
    pub fn sigma_neighbors() -> &'static [Direction] {
        use Direction::*;
        &[ Up, UpperRight, LowerRight, Down, LowerLeft, UpperLeft ]
    }

    /// For flat-top hexes in odd-q layout, returns (dq, dr).
    /// Only valid for the six hex directions; others map to (0,0).
    pub fn offset_delta(&self, is_odd_column: bool) -> (isize, isize) {
        match self {
            Direction::Up           => ( 0, -1),
            Direction::Down         => ( 0,  1),
            Direction::UpperRight   => {
                if is_odd_column { (1, 0) } else { (1, -1) }
            }
            Direction::LowerRight   => {
                if is_odd_column { (1, 1) } else { (1,  0) }
            }
            Direction::UpperLeft    => {
                if is_odd_column { (-1, 0) } else { (-1, -1) }
            }
            Direction::LowerLeft    => {
                if is_odd_column { (-1, 1) } else { (-1,  0) }
            }
            _ => (0, 0),
        }
    }

    /// The opposite direction.
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up               => Direction::Down,
            Direction::Down             => Direction::Up,
            Direction::UpperRight       => Direction::LowerLeft,
            Direction::LowerLeft        => Direction::UpperRight,
            Direction::LowerRight       => Direction::UpperLeft,
            Direction::UpperLeft        => Direction::LowerRight,
            Direction::Right            => Direction::Left,
            Direction::Left             => Direction::Right,
            Direction::Inward           => Direction::Outward,
            Direction::Outward          => Direction::Inward,
            Direction::Clockwise        => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::Clockwise,
        }
    }

    /// Which two flat-top unit-point indices form the wall edge
    /// for this direction (0..5 indexing your unitPoints array).
    pub fn vertex_indices(&self) -> (usize, usize) {
        match self {
            Direction::Up           => (0, 1),
            Direction::UpperRight   => (1, 2),
            Direction::LowerRight   => (2, 3),
            Direction::Down         => (3, 4),
            Direction::LowerLeft    => (4, 5),
            Direction::UpperLeft    => (5, 0),
            _ => (0, 0),
        }
    }

}
