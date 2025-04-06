use std::ffi::CString;
use std::os::raw::c_char;
use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{ Serialize, Deserialize };
use serde::ser::{SerializeStruct, Serializer};

use crate::direction::Direction;

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
    pub neighbors_by_direction: HashMap<String, Coordinates>,
    pub linked: HashSet<Coordinates>,
    pub distance: i32,
    pub is_start: bool,
    pub is_goal: bool,
    pub is_visited: bool,
    pub on_solution_path: bool,
    pub orientation: CellOrientation,
    pub open_walls: Vec<String>,
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
            is_visited: false,
            on_solution_path: false,
            orientation: CellOrientation::Normal, // Assuming CellOrientation has a Normal variant
            open_walls: Vec::new(),
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
        
        let linked_dirs: HashSet<String> = self.linked_directions();
        state.serialize_field("linked", &linked_dirs)?;
        state.serialize_field("distance", &self.distance)?;
        state.serialize_field("is_start", &self.is_start)?;
        state.serialize_field("is_goal", &self.is_goal)?;
        state.serialize_field("is_visited", &self.is_visited)?;
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

    pub fn is_linked_direction<D: Direction + Into<String> + Clone>(&self, direction: D) -> bool {
        // Find the neighbor for the given direction
        if let Some(neighbor_coords) = self.neighbors_by_direction.get(&direction.as_str()) {
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

}

#[repr(C)]
pub struct FFICell {
    pub x: usize,
    pub y: usize,
    // *const c_char is a pointer to a single null-terminated C string
    // (e.g., "Orthogonal"). Required for FFI compatibility with Swift.
    pub maze_type: *const c_char,

    // *const *const c_char is a pointer to an array of pointers to
    // null-terminated C strings (i.e., a list of strings like ["North", "East"]).
    pub linked: *const *const c_char,

    // Number of items in the `linked` array
    pub linked_len: usize,

    pub distance: i32,
    pub is_start: bool,
    pub is_goal: bool,
    pub is_visited: bool,
    pub on_solution_path: bool,

    // *const c_char is a pointer to a single null-terminated C string
    // (e.g., "North"). Required for FFI compatibility with Swift.
    pub orientation: *const c_char,
}


impl From<&Cell> for FFICell {
    fn from(cell: &Cell) -> Self {
        // Convert maze_type and orientation into raw C strings.
        let maze_type_c = CString::new(format!("{:?}", cell.maze_type))
            .unwrap()
            .into_raw();
        let orientation_c = CString::new(format!("{:?}", cell.orientation))
            .unwrap()
            .into_raw();

        // Create a vector of raw pointers for the open_walls strings.
        let open_walls_raw: Vec<*const c_char> = cell.open_walls.iter()
            .map(|direction| {
                // Convert each Rust string into a raw C string.
                CString::new(direction.clone())
                    .unwrap()
                    .into_raw() as *const c_char
            })
            .collect();

        // Leak the vector of pointers by converting it into a boxed slice.
        let open_walls_len = open_walls_raw.len();
        let open_walls_ptr = Box::leak(open_walls_raw.into_boxed_slice()).as_ptr();

        FFICell {
            x: cell.coords.x,
            y: cell.coords.y,
            maze_type: maze_type_c,
            linked: open_walls_ptr, // now holds the open_walls raw pointers
            linked_len: open_walls_len,
            distance: cell.distance,
            is_start: cell.is_start,
            is_goal: cell.is_goal,
            is_visited: cell.is_visited,
            on_solution_path: cell.on_solution_path,
            orientation: orientation_c,
        }
    }
}

impl Drop for FFICell {
    fn drop(&mut self) {
        unsafe {
            // Reclaim the maze_type C string.
            if !self.maze_type.is_null() {
                let _ = CString::from_raw(self.maze_type as *mut c_char);
            }
            
            // Reclaim the orientation C string.
            if !self.orientation.is_null() {
                let _ = CString::from_raw(self.orientation as *mut c_char);
            }
            
            // Reclaim each of the linked C strings.
            let linked_slice = std::slice::from_raw_parts(self.linked, self.linked_len);
            for &ptr in linked_slice {
                if !ptr.is_null() {
                    let _ = CString::from_raw(ptr as *mut c_char);
                }
            }
            
            // Reclaim and free the leaked pointer array.
            let _ = Vec::from_raw_parts(self.linked as *mut *const c_char, self.linked_len, self.linked_len);
        }
    }
}


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
            on_solution_path: false,
            orientation: CellOrientation::Normal,
            open_walls: Vec::new(),
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
        self.0.is_goal = is_visited;
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
    
    pub fn neighbors(&mut self, neighbors_by_direction: HashMap<String, Coordinates>) -> &Self {
        self.0.neighbors_by_direction = neighbors_by_direction;
        self
    }

}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use super::*;
    use crate::direction::SquareDirection;

    #[test]
    fn access_neighbors() {
        let cell1 = CellBuilder::new(1, 1, MazeType::Orthogonal).build();
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
        let cell3 = CellBuilder::new(1, 1, MazeType::Orthogonal).build();
        assert!(cell3.neighbors().is_empty());
        assert!(cell3.neighbors_by_direction.get("North").is_none());
        
    }

    #[test]
    fn access_linked_neighbors() {
        let cell1 = CellBuilder::new(1, 1, MazeType::Orthogonal).build();
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
            is_visited: false,
            on_solution_path: true,
            orientation: CellOrientation::Normal,
            open_walls: Vec::new(),
        };

        let json = cell.to_string();
        println!("Serialized JSON: {}", json);

        assert!(json.contains("\"x\":1"));
        assert!(json.contains("\"y\":1"));
        assert!(json.contains("\"East\""));
        assert!(json.contains("\"South\""));
        assert!(json.contains("\"on_solution_path\":true"));
    }

    #[test]
    fn test_memory_allocation_for_ffi_cell() {
        let mut neighbors: HashMap<String, Coordinates> = HashMap::new();
        neighbors.insert("North".to_string(), Coordinates { x: 1, y: 1 });
        neighbors.insert("East".to_string(), Coordinates { x: 2, y: 2 });
        neighbors.insert("South".to_string(), Coordinates { x: 1, y: 3 });
        neighbors.insert("West".to_string(), Coordinates { x: 0, y: 2 });

        let mut linked: HashSet<Coordinates> = HashSet::new();
        linked.insert(Coordinates { x: 2, y: 2 });
        linked.insert(Coordinates { x: 1, y: 3 });

        let mut open_walls: Vec<String> = Vec::new();
        open_walls.push(String::from("East"));
        open_walls.push(String::from("South"));

        let cell = Cell {
            coords: Coordinates { x: 1, y: 2 },
            maze_type: MazeType::Orthogonal,
            neighbors_by_direction: neighbors,
            linked,
            distance: 10,
            is_start: true,
            is_goal: false,
            is_visited: false,
            on_solution_path: true,
            orientation: CellOrientation::Normal,
            open_walls: open_walls,
        };

        let ffi_cell: FFICell = (&cell).into();

        // Convert C strings back to Rust strings for assertions.
        let maze_type_str = unsafe { CStr::from_ptr(ffi_cell.maze_type).to_str().unwrap() };
        let orientation_str = unsafe { CStr::from_ptr(ffi_cell.orientation).to_str().unwrap() };

        // Convert linked pointers back to Rust Strings and collect.
        let linked_rust: HashSet<String> = unsafe {
            std::slice::from_raw_parts(ffi_cell.linked, ffi_cell.linked_len)
                .iter()
                .map(|&ptr| CStr::from_ptr(ptr).to_string_lossy().into_owned())
                .collect()
        };

        // Assert that the strings are as expected.
        assert_eq!(maze_type_str, format!("{:?}", cell.maze_type));
        assert_eq!(orientation_str, format!("{:?}", cell.orientation));

        // Create expected linked set from the matching neighbor keys.
        let expected_linked: HashSet<String> = cell
            .neighbors_by_direction
            .iter()
            .filter_map(|(k, &v)| {
                if cell.linked.contains(&v) {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(linked_rust, expected_linked);

        // No manual cleanup is necessary.
        // The Drop implementation for FFICell will automatically free all allocated memory.
    }

}