use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{ Serialize, Deserialize };
use serde::ser::{SerializeStruct, Serializer};

use crate::behaviors::collections::FilterKeys;
use crate::behaviors::display::JsonDisplay;
use crate::direction::Direction;

#[derive(Copy, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize
}
impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_json() {
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
    Upsilon,
    Rhombille,
}
impl fmt::Display for MazeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_json() {
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
/// Representation of a single Cell of the maze Grid
pub struct Cell {
    /// The x,y coordinates of the cell.
    pub coords: Coordinates,
    /// The maze type (e.g., Orthogonal, Delta, Sigma).
    pub maze_type: MazeType,
    /// Maps directions to the coordinates of neighboring cells.
    pub neighbors_by_direction: HashMap<Direction, Coordinates>,
    /// Coordinates of neighboring cells that are linked to this cell (i.e., no walls in between).
    pub linked: HashSet<Coordinates>,
    /// Distance to the goal cell.
    pub distance: i32,
    /// Whether this cell is the starting cell.
    pub is_start: bool,
    /// Whether this cell is the goal cell.
    pub is_goal: bool,
    /// Whether this is the cell the user is currently visiting.
    pub is_active: bool,
    /// Whether the user has visited (or unvisited via backtracking) this cell in the current path.
    ///
    /// When this flag is modified (set to `true` for visiting or `false` for backtracking),
    /// the cell's permanent marker is also set: `has_been_visited` remains `true` once it has been touched.
    pub is_visited: bool,
    /// Once a cell is ever visited (i.e., when `is_visited` is set), this flag is permanently set to `true`.
    ///
    /// This flag serves as the permanent trail marker for visual representations,
    /// ensuring that the cell is recognized as visited even if the dynamic trail (`is_visited`) is later undone.
    pub has_been_visited: bool,
    /// Indicates whether this cell is on the solution path from the start to the goal.
    pub on_solution_path: bool,
    /// The orientation of the cell (Normal or Inverted); applicable only for delta cells.
    pub orientation: CellOrientation,
    /// The directions in which there are no walls restricting movement.
    pub open_walls: Vec<Direction>,
    /// Used primarily for Upsilon maze_type, to indicate whether cell's square or octagon
    pub is_square: bool,
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
            is_active: false, 
            is_visited: false,
            has_been_visited: false,
            on_solution_path: false,
            orientation: CellOrientation::Normal, // Assuming CellOrientation has a Normal variant
            open_walls: Vec::new(),
            is_square: false,
        }
    }
}

impl Serialize for Cell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Cell", 10)?;
        state.serialize_field("coords", &self.coords)?;
        let linked_dirs: Vec<String> = self.get_user_facing_linked_directions()
            .iter()
            .map(|d| d.to_string())
            .collect();
        state.serialize_field("linked", &linked_dirs)?;
        state.serialize_field("distance", &self.distance)?;
        state.serialize_field("is_start", &self.is_start)?;
        state.serialize_field("is_goal", &self.is_goal)?;
        state.serialize_field("is_active", &self.is_active)?;
        state.serialize_field("is_visited", &self.is_visited)?;
        state.serialize_field("has_been_visited", &self.has_been_visited)?;
        state.serialize_field("on_solution_path", &self.on_solution_path)?;
        state.serialize_field("is_square", &self.is_square)?;
        state.end()
    } 
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_json() {
            Ok(json) => write!(f, "{}", json),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl Cell {
    /// X coordinate (on horizontal axis)
    pub fn x(&self) -> usize {
        return self.coords.x;
    }

    /// Y coordinate (on vertical axis)
    pub fn y(&self) -> usize {
        return self.coords.y;
    }

    /// Coordinates of neighboring Cells
    pub fn neighbors(&self) -> HashSet<Coordinates> {
        return self.neighbors_by_direction.values().cloned().collect();        
    }

    /// Coordinates of linked neighboring Cells (linked indicating no walls separating these linked neighbors from this Cell)
    pub fn unlinked_neighbors(&self) -> HashSet<Coordinates> {
        let all_neighbors = self.neighbors();
        return all_neighbors.difference(&self.linked).cloned().collect();
    }

    /// Directions from this Cell to linked neighboring Cells (linked indicating no walls separating these linked neighbors from this Cell)
    pub fn linked_directions(&self) -> HashSet<Direction> {
        // Assuming neighbors_by_direction provides the mapping
        self.neighbors_by_direction
            .filter_keys(|coords| self.linked.contains(coords))
            .into_iter()
            .collect() 
    }

    /// Whether neighbor in specified Direction is linked to this Cell
    pub fn is_linked_direction(&self, direction: Direction) -> bool {
        // Find the neighbor for the given direction
        if let Some(neighbor_coords) = self.neighbors_by_direction.get(&direction) {
            self.linked.contains(neighbor_coords)
        } else {
            false
        }
    }

    /// Whether specified Coordinates belong to a neighboring Cell which is linked to this Cell (meaning no separating wall)
    pub fn is_linked(&self, coordinates: Coordinates) -> bool {
        return self.linked.contains(&coordinates); 
    }

    /// Whether specified optional Coordinates belong to a neighboring Cell which is linked to this Cell (meaning no separating wall)
    pub fn is_linked_opt(&self, coordinates: Option<Coordinates>) -> bool {
        match coordinates {
            Some(coords) => self.is_linked(coords),
            None => false,
        }
    }

    /// Set is_active
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    /// Set is_visited, the dynamic marker, to specified boolean value. This also sets has_been_visited, the permanent marker, to true 
    pub fn set_visited(&mut self, visited: bool) {
        if self.is_start {
            // start cell is always visited and cannot become unvisited 
            self.is_visited = true;
        } else {
            self.is_visited = visited;
        }
        // has_been_visited is the permanent path marker
        self.has_been_visited = true;
    }

    /// Set linked to a specified set of Coordinates    
    pub fn set_linked(&mut self, linked: HashSet<Coordinates>) {
        self.linked = linked;
    }

    /// Set orientation
    pub fn set_orientation(&mut self, orientation: CellOrientation) {
        self.orientation = orientation;
    }
    
    /// Set neighbors_by_direction 
    pub fn set_neighbors(&mut self, neighbors_by_direction: HashMap<Direction, Coordinates>) {
        self.neighbors_by_direction = neighbors_by_direction;
    }

    /// Set open_walls based on neighbors_by_direction to indicate which walls are omitted (e.g. open walls)
    pub fn set_open_walls(&mut self) {
        self.open_walls = self.linked.iter()
            .filter_map(|coords| {
                self.neighbors_by_direction
                    .iter()
                    .find(|(_, &v)| v == *coords)
                    .map(|(direction, _)| direction.clone())
            })
            .collect()
    }

    /// Returns neighbors mapped to user-facing directions (diagonal for Rhombille).
    pub fn get_user_facing_neighbors(&self) -> HashMap<Direction, Coordinates> {
        if self.maze_type == MazeType::Rhombille {
            let mut mapped = HashMap::new();
            if let Some(&coords) = self.neighbors_by_direction.get(&Direction::Up) {
                mapped.insert(Direction::UpperRight, coords);
            }
            if let Some(&coords) = self.neighbors_by_direction.get(&Direction::Right) {
                mapped.insert(Direction::LowerRight, coords);
            }
            if let Some(&coords) = self.neighbors_by_direction.get(&Direction::Down) {
                mapped.insert(Direction::LowerLeft, coords);
            }
            if let Some(&coords) = self.neighbors_by_direction.get(&Direction::Left) {
                mapped.insert(Direction::UpperLeft, coords);
            }
            mapped
        } else {
            self.neighbors_by_direction.clone()
        }
    }

    /// Returns linked directions mapped to user-facing directions (diagonal for Rhombille).
    pub fn get_user_facing_linked_directions(&self) -> Vec<Direction> {
        if self.maze_type == MazeType::Rhombille {
            self.linked_directions()
                .iter()
                .map(|&d| match d {
                    Direction::Up => Direction::UpperRight,
                    Direction::Right => Direction::LowerRight,
                    Direction::Down => Direction::LowerLeft,
                    Direction::Left => Direction::UpperLeft,
                    _ => d,
                })
                .collect()
        } else {
            self.linked_directions().into_iter().collect()
        }
    }

    pub fn get_user_facing_open_walls(&self) -> Vec<Direction> {
        if self.maze_type == MazeType::Rhombille {
            self.open_walls.iter().map(|&d| match d {
                Direction::Up => Direction::UpperRight,
                Direction::Right => Direction::LowerRight,
                Direction::Down => Direction::LowerLeft,
                Direction::Left => Direction::UpperLeft,
                d => d,
            }).collect()
        } else {
            self.open_walls.clone()
        }
    }
}

/// A builder for creating and customizing a `Cell` using the builder pattern.
///
/// The `CellBuilder` wraps a `Cell` and provides a fluent API to set properties
/// such as coordinates, maze type, start/goal status, visit state, linked neighbors,
/// orientation, and neighbor relationships. Once configured, the final `Cell`
/// can be obtained via the `build()` method, which returns a cloned instance.
pub struct CellBuilder(Cell);

impl CellBuilder {

    pub fn build(&self) -> Cell {
        self.0.clone()
    }

    pub fn new(x: usize, y: usize, maze_type: MazeType) -> Self {
        Self(Cell {
            coords: Coordinates{x: x, y: y},
            maze_type,
            neighbors_by_direction: HashMap::new(),
            linked: HashSet::new(),
            distance: 0,
            is_start: false,
            is_goal: false,
            is_visited: false, 
            has_been_visited: false, 
            is_active: false, 
            on_solution_path: false,
            orientation: CellOrientation::Normal,
            open_walls: Vec::new(),
            is_square: false,
        })
    }

    pub fn is_start(mut self, is_start: bool) -> Self {
        self.0.is_start = is_start;
        self
    }

    pub fn is_goal(mut self, is_goal: bool) -> Self {
        self.0.is_goal = is_goal;
        self
    }
    
    pub fn is_visited(mut self, is_visited: bool) -> Self {
        self.0.is_visited = is_visited;
        self
    }

    pub fn has_been_visited(mut self, has_been_visited: bool) -> Self {
        self.0.has_been_visited = has_been_visited; // permenant path parker
        self
    }
    
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.0.is_active= is_active;
        self
    }

    pub fn linked(mut self, linked: HashSet<Coordinates>) -> Self {
        self.0.linked = linked;
        self
    }

    pub fn orientation(mut self, orientation: CellOrientation) -> Self {
        self.0.orientation = orientation;
        self
    }
    
    pub fn neighbors(&mut self, neighbors_by_direction: HashMap<Direction, Coordinates>) -> &Self {
        self.0.neighbors_by_direction = neighbors_by_direction;
        self
    }

    pub fn is_square(mut self, is_square: bool) -> Self { 
        self.0.is_square = is_square; 
        self 
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::Direction;

    #[test]
    fn access_neighbors() {
        let cell1 = CellBuilder::new(1, 1, MazeType::Orthogonal).build();
        let mut neighbors = HashMap::new();
        neighbors.insert(Direction::Up, Coordinates{ x: 1, y: 0});
        neighbors.insert(Direction::Right, Coordinates{ x: 2, y: 1});
        neighbors.insert(Direction::Down, Coordinates{ x: 1, y: 2});
        neighbors.insert(Direction::Left, Coordinates{ x: 0, y: 1});
        let cell2 = Cell {
            neighbors_by_direction: neighbors,
            ..cell1
        };
        assert!(cell2.neighbors().contains(&Coordinates{x: 1, y: 0}));
        assert!(cell2.neighbors().contains(&Coordinates{x: 2, y: 1}));
        assert!(cell2.neighbors().contains(&Coordinates{x: 1, y: 2}));
        assert!(cell2.neighbors().contains(&Coordinates{x: 0, y: 1}));
        assert!(cell2.neighbors().len() == 4);
        assert!(*cell2.neighbors_by_direction.get(&Direction::Up).expect("Missing North neighbor") == Coordinates{x: 1, y: 0});
        assert!(*cell2.neighbors_by_direction.get(&Direction::Right).expect("Missing East neighbor") == Coordinates{x: 2, y: 1});
        assert!(*cell2.neighbors_by_direction.get(&Direction::Down).expect("Missing South neighbor") == Coordinates{x: 1, y: 2});
        assert!(*cell2.neighbors_by_direction.get(&Direction::Left).expect("Missing West neighbor") == Coordinates{x: 0, y: 1});

        // cell with no neighbors assigned
        let cell3 = CellBuilder::new(1, 1, MazeType::Orthogonal).build();
        assert!(cell3.neighbors().is_empty());
        assert!(cell3.neighbors_by_direction.get(&Direction::Up).is_none());
        
    }

    #[test]
    fn access_linked_neighbors() {
        let cell1 = CellBuilder::new(1, 1, MazeType::Orthogonal).build();
        let mut neighbors = HashMap::new();
        let north = Coordinates{ x: 1, y: 0 };
        let east = Coordinates{ x: 2, y: 1 };
        let south = Coordinates{ x: 1, y: 2 };
        let west = Coordinates{ x: 0, y: 1 };
        neighbors.insert(Direction::Up, north.clone());
        neighbors.insert(Direction::Right, east.clone());
        neighbors.insert(Direction::Down, south.clone());
        neighbors.insert(Direction::Left, west.clone());
        let mut linked: HashSet<Coordinates> = HashSet::new();
        linked.insert(north.clone());
        linked.insert(south.clone());
        // Clone cell1 for use in cell2
        let cell2 = Cell {
            neighbors_by_direction: neighbors.clone(),
            linked: linked.clone(),
            ..cell1.clone()  // clone cell1 entirely
        };
        assert!(cell2.linked.contains(&north));
        assert!(cell2.linked.contains(&south));
        assert!(cell2.linked.len() == 2);
        assert!(cell2.unlinked_neighbors().contains(&east));
        assert!(cell2.unlinked_neighbors().contains(&west));
        assert!(cell2.unlinked_neighbors().len() == 2);
        assert!(cell2.is_linked_direction(Direction::Up));
        assert!(cell2.is_linked_direction(Direction::Down));
        assert!(!cell2.is_linked_direction(Direction::Right));
        assert!(!cell2.is_linked_direction(Direction::Left));
        assert!(cell2.is_linked(north));
        assert!(cell2.is_linked(south));
        assert!(!cell2.is_linked(east));
        assert!(!cell2.is_linked(west));
        assert!(cell2.linked_directions().contains(&Direction::Up));
        assert!(cell2.linked_directions().contains(&Direction::Down));
        assert!(cell2.linked_directions().len() == 2);
        let mut cell3 = Cell {
            neighbors_by_direction: neighbors,
            ..cell1.clone()
        }; // nothing linked yet
        assert!(cell3.linked.is_empty());
        cell3.set_linked(linked.clone());
        assert!(cell3.linked.contains(&north));
        assert!(cell3.linked.contains(&south));
        assert!(cell3.linked.len() == 2);
        assert!(cell3.unlinked_neighbors().contains(&east));
        assert!(cell3.unlinked_neighbors().contains(&west));
        assert!(cell3.unlinked_neighbors().len() == 2);
        assert!(cell3.is_linked_direction(Direction::Up));
        assert!(cell3.is_linked_direction(Direction::Down));
        assert!(!cell3.is_linked_direction(Direction::Right));
        assert!(!cell3.is_linked_direction(Direction::Left));
        assert!(cell3.is_linked(north));
        assert!(cell3.is_linked(south));
        assert!(!cell3.is_linked(east));
        assert!(!cell3.is_linked(west));
        assert!(cell3.linked_directions().contains(&Direction::Up));
        assert!(cell3.linked_directions().contains(&Direction::Down));
        assert!(cell3.linked_directions().len() == 2);

    }

    #[test]
    fn serialize_cell_to_json() {
        let mut neighbors = HashMap::new();
        neighbors.insert(Direction::Right, Coordinates { x: 1, y: 0 });
        neighbors.insert(Direction::Down, Coordinates { x: 0, y: 1 });

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
            is_active: false,
            is_visited: false,
            has_been_visited: false,
            on_solution_path: true,
            orientation: CellOrientation::Normal,
            open_walls: Vec::new(),
            is_square: true,
        };

        let json = cell.to_string();
        println!("Serialized JSON: {}", json);

        assert!(json.contains("\"x\":1"));
        assert!(json.contains("\"y\":1"));
        assert!(json.contains("\"Right\""));
        assert!(json.contains("\"Down\""));
        assert!(json.contains("\"on_solution_path\":true"));
        assert!(json.contains("\"is_square\":true"));
    }


}