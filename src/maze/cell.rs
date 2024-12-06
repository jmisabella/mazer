use std::collections::{HashMap, HashSet};
use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32
}
impl Default for Coordinates {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0            
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MazeType {
    Orthogonal,
    // Sigma,
    // Delta,
    // Polar
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellOrientation {
    Normal,
    // Inverted
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    pub coords: Coordinates,
    pub maze_type: MazeType,
    pub neighbors_by_direction: HashMap<String, Coordinates>, // Map[String, Coordinates]
    pub linked: HashSet<Coordinates>,                        // Set[Coordinates]
    pub distance: i32,                                       // Int
    pub is_start: bool,                                      // Boolean
    pub is_goal: bool,                                       // Boolean
    pub on_solution_path: bool,                              // Boolean
    pub orientation: CellOrientation,                        // CellOrientation (enum)
    pub visited: bool,                                       // Boolean
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            coords: Coordinates::default(),
            maze_type: MazeType::Orthogonal,
            neighbors_by_direction: HashMap::new(),
            linked: HashSet::new(),
            distance: 0,
            is_start: false,
            is_goal: false,
            on_solution_path: false,
            orientation: CellOrientation::Normal, // Assuming CellOrientation has a Normal variant
            visited: false,
        }
    }
}
