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
    Sigma,
    Delta,
    Polar
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellOrientation {
    Normal,
    Inverted
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
        }
    }
}

impl Cell {
    pub fn neighbors(&self) -> HashSet<Coordinates> {
        return self.neighbors_by_direction.values().cloned().collect();        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_cell_to_json() {
        let mut neighbors = HashMap::new();
        neighbors.insert("East".to_string(), Coordinates { x: 1, y: 0 });
        neighbors.insert("South".to_string(), Coordinates { x: 0, y: 1 });

        let mut linked = HashSet::new();
        linked.insert(Coordinates { x: 1, y: 0 });

        let cell = Cell {
            coords: Coordinates { x: 1, y: 1 },
            maze_type: MazeType::Orthogonal,
            neighbors_by_direction: neighbors,
            linked,
            distance: 10,
            is_start: true,
            is_goal: false,
            on_solution_path: true,
            orientation: CellOrientation::Normal,
        };

        let json = serde_json::to_string(&cell).expect("Serialization failed");
        println!("Serialized JSON: {}", json);

        assert!(json.contains("\"x\":1"));
        assert!(json.contains("\"y\":1"));
        assert!(json.contains("\"Orthogonal\""));
        assert!(json.contains("\"East\""));
        assert!(json.contains("\"on_solution_path\":true"));
    }

    #[test]
    fn deserialize_json_to_cell() {
        let json = r#"
        {
            "coords": { "x": 2, "y": 3 },
            "maze_type": "Delta",
            "neighbors_by_direction": {
                "LowerLeft": { "x": 3, "y": 3 },
                "Up": { "x": 1, "y": 3 }
            },
            "linked": [
                { "x": 2, "y": 2 },
                { "x": 2, "y": 4 }
            ],
            "distance": 5,
            "is_start": true,
            "is_goal": true,
            "on_solution_path": false,
            "orientation": "Inverted"
        }
        "#;

        let cell: Cell = serde_json::from_str(json).expect("Deserialization failed");

        assert_eq!(cell.coords, Coordinates { x: 2, y: 3 });
        assert_eq!(cell.maze_type, MazeType::Delta);
        assert_eq!(
            cell.neighbors_by_direction.get("LowerLeft"),
            Some(&Coordinates { x: 3, y: 3 })
        );
        assert!(cell.linked.contains(&Coordinates { x: 2, y: 2 }));
        assert_eq!(cell.distance, 5);
        assert!(cell.is_start);
        assert!(cell.is_goal);
        assert_eq!(cell.orientation, CellOrientation::Inverted);
    }

    #[test]
    fn serialize_and_deserialize_cell_round_trip() {
        let cell = Cell::default();

        let json = serde_json::to_string(&cell).expect("Serialization failed");
        let deserialized_cell: Cell = serde_json::from_str(&json).expect("Deserialization failed");

        assert_eq!(cell, deserialized_cell);
    }

}