use crate::direction::Direction;

use std::collections::{HashMap, HashSet};
use std::fmt;
use serde::{ Serialize, Deserialize };
use serde::ser::{SerializeStruct, Serializer};

#[derive(Copy, Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize
}
impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match serde_json::to_string(&self) {
            Ok(json) => write!(f, "{}", json),
            Err(_) => Err(fmt::Error),
        }
    }
}
impl Default for Coordinates {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0            
        }
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MazeType {
    Orthogonal,
    Sigma,
    Delta,
    Polar
}
impl fmt::Display for MazeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match serde_json::to_string(&self) {
            Ok(json) => write!(f, "{}", json),
            Err(_) => Err(fmt::Error),
        }
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellOrientation {
    Normal,
    Inverted
}

#[derive(Debug, Clone, PartialEq)]
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

impl Serialize for Cell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Cell", 7)?;
        state.serialize_field("coords", &self.coords)?;
        // Transform `linked` from coordinates to directions
        let linked_dirs: HashSet<String> = self.linked_directions();
        state.serialize_field("linked", &linked_dirs)?;
        state.serialize_field("distance", &self.distance)?;
        state.serialize_field("is_start", &self.is_start)?;
        state.serialize_field("is_goal", &self.is_goal)?;
        state.serialize_field("on_solution_path", &self.on_solution_path)?;
        state.end()
    }
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match serde_json::to_string(&self) {
            Ok(json) => write!(f, "{}", json),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl Cell {
    pub fn x(&self) -> usize {
        return self.coords.x;
    }

    pub fn y(&self) -> usize {
        return self.coords.y;
    }

    pub fn new(x: usize, y: usize, maze_type: MazeType) -> Self {
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
    
    pub fn init(x: usize, y: usize, maze_type: MazeType, is_start: bool, is_goal: bool) -> Self {
        Self {
            coords: Coordinates{x: x, y: y},
            maze_type,
            neighbors_by_direction: HashMap::new(),
            linked: HashSet::new(),
            distance: 0,
            is_start: is_start,
            is_goal: is_goal,
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

    pub fn linked_directions(&self) -> HashSet<String> {
        // Assuming neighbors_by_direction provides the mapping
        self.neighbors_by_direction
            .iter()
            .filter_map(|(direction, coords)| {
                if self.linked.contains(coords) {
                    // Return direction if the corresponding cell is linked
                    Some(direction.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn is_linked_direction<D: Direction>(&self, direction: D) -> bool {
        // Convert direction to a string key
        let direction_key = direction.as_str().replace("\"", "");

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

    pub fn set_linked(&mut self, linked: HashSet<Coordinates>) {
        self.linked = linked;
    }

    pub fn set_orientation(&mut self, orientation: CellOrientation) {
        self.orientation = orientation;
    }
    
    pub fn set_neighbors(&mut self, neighbors_by_direction: HashMap<String, Coordinates>) {
        self.neighbors_by_direction = neighbors_by_direction;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::SquareDirection;

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
            neighbors_by_direction: neighbors.clone(),
            linked: linked.clone(),
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
        assert!(cell2.linked_directions().contains("North"));
        assert!(cell2.linked_directions().contains("South"));
        assert!(cell2.linked_directions().len() == 2);
        let mut cell3 = Cell {
            neighbors_by_direction: neighbors,
            ..cell1
        }; // nothing linked yet
        assert!(cell3.linked.is_empty());
        cell3.set_linked(linked.clone());
        assert!(cell3.linked.contains(&north));
        assert!(cell3.linked.contains(&south));
        assert!(cell3.linked.len() == 2);
        assert!(cell3.unlinked_neighbors().contains(&east));
        assert!(cell3.unlinked_neighbors().contains(&west));
        assert!(cell3.unlinked_neighbors().len() == 2);
        assert!(cell3.is_linked_direction(SquareDirection::North));
        assert!(cell3.is_linked_direction(SquareDirection::South));
        assert!(!cell3.is_linked_direction(SquareDirection::East));
        assert!(!cell3.is_linked_direction(SquareDirection::West));
        assert!(cell3.is_linked(north));
        assert!(cell3.is_linked(south));
        assert!(!cell3.is_linked(east));
        assert!(!cell3.is_linked(west));
        assert!(cell3.linked_directions().contains("North"));
        assert!(cell3.linked_directions().contains("South"));
        assert!(cell3.linked_directions().len() == 2);

    }

    #[test]
    fn serialize_cell_to_json() {
        let mut neighbors = HashMap::new();
        neighbors.insert("East".to_string(), Coordinates { x: 1, y: 0 });
        neighbors.insert("South".to_string(), Coordinates { x: 0, y: 1 });

        let mut linked = HashSet::new();
        linked.insert(Coordinates { x: 1, y: 0 });
        linked.insert(Coordinates { x: 0, y: 1 });

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

        let json = cell.to_string();
        println!("Serialized JSON: {}", json);

        assert!(json.contains("\"x\":1"));
        assert!(json.contains("\"y\":1"));
        assert!(json.contains("\"East\""));
        assert!(json.contains("\"South\"")); //// why does our linked_directions method only have "East" but not "South" ???
        assert!(json.contains("\"on_solution_path\":true"));
    }


}