use std::fmt;
use serde_json;
use crate::cell::{ Coordinates, MazeType };
use crate::direction::Direction;
use crate::algorithms::MazeAlgorithm;

#[derive(Debug)]
pub enum Error {
    InvalidCellForDeltaMaze { cell_maze_type: MazeType },
    InvalidCellForNonDeltaMaze { cell_maze_type: MazeType }, 
    AlgorithmUnavailableForMazeType { algorithm: MazeAlgorithm, maze_type: MazeType },
    FlattenedVectorDimensionsMismatch { vector_size: usize, maze_width: usize, maze_height: usize },
    OutOfBoundsCoordinates { coordinates: Coordinates, maze_width: usize, maze_height: usize },
    MissingCoordinates { coordinates: Coordinates },
    NoValidNeighbor { coordinates: Coordinates },
    MultipleActiveCells { count: usize },
    NoActiveCells,
    InvalidDirection { direction: String },
    MoveUnavailable { attempted_move: Direction, available_moves: Vec<Direction>},
    GridDimensionsExceedLimitForCaptureSteps { width: usize, height: usize },
    NoCellAtCoordinates { coordinates: Coordinates },
    InvalidCellCoordinates { coordinates: Coordinates },
    SerializationError(serde_json::Error),
    EmptyList,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidCellForDeltaMaze { cell_maze_type } => {
                write!(f, "Cannot generate non-triangle cells for Delta maze_type {:?}", cell_maze_type)
            }
            Error::InvalidCellForNonDeltaMaze { cell_maze_type } => {
                write!(f, "Cannot generate triangle cells for non-Delta maze_type {:?}", cell_maze_type)
            }
            Error::AlgorithmUnavailableForMazeType{ algorithm, maze_type } => {
                write!(f, "MazeAlgorithm {:?} is unavailable for MazeType {:?}", algorithm, maze_type)
            }
            Error::FlattenedVectorDimensionsMismatch { vector_size, maze_width, maze_height } => {
                write!(f, "Flattened vector size mismatch: expected size {} ({} x {}), but got {}", maze_width * maze_height, maze_width, maze_height, vector_size)
            }
            Error::OutOfBoundsCoordinates { coordinates, maze_width, maze_height } => {
                write!(f, "Out of bounds coordinates: coordinates {:?} exceed maze dimensions {} x {}", coordinates, maze_width, maze_height)
            }
            Error::MissingCoordinates { coordinates } => {
                write!(f, "Missing coordinates: {:?}", coordinates )
            }
            Error::NoValidNeighbor { coordinates } => {
                write!(f, "No valid neighbors: {:?}", coordinates )
            }
            Error::MultipleActiveCells { count } => {
                write!(f, "{:?} active cells, there should only ever be exactly 1 active cell", count )
            }
            Error::NoActiveCells => {
                write!(f, "No active cells, there should always be exactly 1 active cell" )
            }
            Error::MoveUnavailable { attempted_move, available_moves } => {
                write!(f, "Cannot make move {:?} because it is unavailable. Available moves are: {:?}", attempted_move, available_moves.into_iter().map(|dir| dir.to_string()).collect::<Vec<_>>().join(", ") )
            }
            Error::InvalidDirection { direction } => {
                write!(f, "Invalid Direction: {:?}", direction )
            }
            Error::GridDimensionsExceedLimitForCaptureSteps  { width, height } => {
                write!(f, "Grid dimensions {:?},{:?} exceed limit for enabling capture_steps. Neither width nor height can exceed 100 for this feature to be enabled", width, height )
            }
            Error::NoCellAtCoordinates{ coordinates } => {
                write!(f, "No cell exists at coordinates {:?}", coordinates )
            }
            Error::InvalidCellCoordinates{ coordinates } => {
                write!(f, "Invalid cell coordinates {:?}", coordinates )
            }
            Error::SerializationError(e) => {
                write!(f, "Serialization error: {}", e)
            }
            Error::EmptyList => {
                write!(f, "Attempted operation on an empty list")
            } 
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::SerializationError(e) => Some(e), // Return a reference to the error
            _ => None,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerializationError(e)
    }
}