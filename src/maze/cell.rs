use std::collections::{HashMap, HashSet};
use serde::{ Serialize, Deserialize };
use crate::maze::direction::{ SquareDirection, TriangleDirection, HexDirection, PolarDirection };

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
    pub fn new(x: u32, y: u32, maze_type: MazeType) -> Self {
        Self {
            coords: Coordinates{x: x, y: y},
            maze_type,
            neighbors_by_direction: HashMap::new(),
            linked: HashSet::new(),
            distance: 0,
            is_start: false,
            is_goal: false,
            on_solution_path: false,
            orientation: CellOrientation::Normal,
        }
    }

    pub fn neighbors(&self) -> HashSet<Coordinates> {
        return self.neighbors_by_direction.values().cloned().collect();        
    }

    pub fn unlinked_neighbors(&self) -> HashSet<Coordinates> {
        let all_neighbors = self.neighbors();
        return all_neighbors.difference(&self.linked).cloned().collect();
    }

    pub fn is_linked_direction<D>(&self, direction: D) -> bool
    where
        D: Into<String>,
    {
        // Convert direction to a string key
        let direction_key = direction.into();

        // Find the neighbor for the given direction
        if let Some(neighbor_coords) = self.neighbors_by_direction.get(&direction_key) {
            self.linked.contains(neighbor_coords)
        } else {
            false
        }
    }

    pub fn is_linked(&self, coordinates: Coordinates) -> bool {
        return self.linked.contains(&coordinates); 
    }

    pub fn is_linked_opt(&self, coordinates: Option<Coordinates>) -> bool {
        match coordinates {
            Some(coords) => self.is_linked(coords),
            None => false,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn access_neighbors() {
        let cell1 = Cell::new(1, 1, MazeType::Orthogonal);
        let mut neighbors = HashMap::new();
        neighbors.insert("North".to_string(), Coordinates{ x: 1, y: 0});
        neighbors.insert("East".to_string(), Coordinates{ x: 2, y: 1});
        neighbors.insert("South".to_string(), Coordinates{ x: 1, y: 2});
        neighbors.insert("West".to_string(), Coordinates{ x: 0, y: 1});
        let cell2 = Cell {
            neighbors_by_direction: neighbors,
            ..cell1
        };
        assert!(cell2.neighbors().contains(&Coordinates{x: 1, y: 0}));
        assert!(cell2.neighbors().contains(&Coordinates{x: 2, y: 1}));
        assert!(cell2.neighbors().contains(&Coordinates{x: 1, y: 2}));
        assert!(cell2.neighbors().contains(&Coordinates{x: 0, y: 1}));
        assert!(cell2.neighbors().len() == 4);
        assert!(*cell2.neighbors_by_direction.get("North").expect("Missing North neighbor") == Coordinates{x: 1, y: 0});
        assert!(*cell2.neighbors_by_direction.get("East").expect("Missing East neighbor") == Coordinates{x: 2, y: 1});
        assert!(*cell2.neighbors_by_direction.get("South").expect("Missing South neighbor") == Coordinates{x: 1, y: 2});
        assert!(*cell2.neighbors_by_direction.get("West").expect("Missing West neighbor") == Coordinates{x: 0, y: 1});

        // cell with no neighbors assigned
        let cell3 = Cell::new(1, 1, MazeType::Orthogonal);
        assert!(cell3.neighbors().is_empty());
        assert!(cell3.neighbors_by_direction.get("North").is_none());
        
    }

    #[test]
    fn access_linked_neighbors() {
        let cell1 = Cell::new(1, 1, MazeType::Orthogonal);
        let mut neighbors = HashMap::new();
        let north = Coordinates{ x: 1, y: 0 };
        let east = Coordinates{ x: 2, y: 1 };
        let south = Coordinates{ x: 1, y: 2 };
        let west = Coordinates{ x: 0, y: 1 };
        neighbors.insert("North".to_string(), north.clone());
        neighbors.insert("East".to_string(), east.clone());
        neighbors.insert("South".to_string(), south.clone());
        neighbors.insert("West".to_string(), west.clone());
        let mut linked: HashSet<Coordinates> = HashSet::new();
        linked.insert(north.clone());
        linked.insert(south.clone());
        let cell2 = Cell {
            neighbors_by_direction: neighbors,
            linked: linked,
            ..cell1
        };
        assert!(cell2.linked.contains(&north));
        assert!(cell2.linked.contains(&south));
        assert!(cell2.linked.len() == 2);
        assert!(cell2.unlinked_neighbors().contains(&east));
        assert!(cell2.unlinked_neighbors().contains(&west));
        assert!(cell2.unlinked_neighbors().len() == 2);
        assert!(cell2.is_linked_direction(SquareDirection::North));
        assert!(cell2.is_linked_direction(SquareDirection::South));
        assert!(!cell2.is_linked_direction(SquareDirection::East));
        assert!(!cell2.is_linked_direction(SquareDirection::West));
        assert!(cell2.is_linked(north));
        assert!(cell2.is_linked(south));
        assert!(!cell2.is_linked(east));
        assert!(!cell2.is_linked(west));
    }

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