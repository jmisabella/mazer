use std::ptr;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use crate::grid::Grid;
use crate::cell::Cell;

/// Representation of a cell for the FFI layer.
///
/// The fields represent the properties of a maze cell.
/// 
/// Fields:
/// - `x`: The x-coordinate of the cell.
/// - `y`: The y-coordinate of the cell.
/// - `maze_type`: A pointer to a null-terminated C string identifying the maze type.
/// - `linked`: A pointer to an array of null-terminated C strings representing linked cells.
/// - `linked_len`: The number of elements in the `linked` array.
/// - `distance`: An integer metric (e.g., the distance from the start).
/// - `is_start`: Indicates if this cell is the starting cell.
/// - `is_goal`: Indicates if this cell is the goal cell.
/// - `is_active`: Indicates if this cell is currently active.
/// - `is_visited`: Indicates if this cell has been visited.
/// - `has_been_visited`: Indicates if this cell has ever been visited.
/// - `on_solution_path`: Indicates if this cell is part of the solution path.
/// - `orientation`: A pointer to a null-terminated C string indicating the cell's orientation.
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
    pub is_active: bool,
    pub is_visited: bool,
    pub has_been_visited: bool,
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
            is_active: cell.is_active,
            is_visited: cell.is_visited,
            has_been_visited: cell.has_been_visited,
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

/// Generates a maze from a JSON request.
///
/// This function takes a null-terminated JSON string representing the maze generation
/// request and attempts to generate a maze. On success, it returns a pointer to a newly
/// allocated `Grid` instance. In case of an error (such as an invalid JSON or a failure in
/// maze generation), it returns a null pointer.
///
/// # Parameters
///
/// - `request_json`: A null-terminated C string containing the JSON request.
///
/// # Returns
///
/// A pointer to the generated `Grid` if successful, or a null pointer on failure.
#[no_mangle]
pub extern "C" fn mazer_generate_maze(request_json: *const c_char) -> *mut Grid {
    // Check for null pointer.
    if request_json.is_null() {
        eprintln!("mazer_generate_maze: request_json is null");
        return std::ptr::null_mut();
    }

    // Convert the C string to a Rust &str.
    let request_str = match unsafe { CStr::from_ptr(request_json) }.to_str() {
        Ok(s) => s,
        Err(err) => {
            eprintln!("mazer_generate_maze: Failed to convert request JSON to string: {:?}", err);
            return std::ptr::null_mut();
        }
    };

    let maze = match Grid::try_from(request_str) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("mazer_generate_maze: Maze generation failed: {:?}", err);
            return std::ptr::null_mut();
        }
    };

    // Allocate the Grid on the heap and return its raw pointer.
    // This pointer serves as an opaque handle on the Swift side.
    Box::into_raw(Box::new(maze))
}

/// Destroys a maze instance.
///
/// This function deallocates the memory and any associated resources for the given maze (`Grid`).
/// If the provided maze pointer is null, the function does nothing.
///
/// # Parameters
///
/// - `maze`: A pointer to the `Grid` instance to be destroyed.
#[no_mangle]
pub extern "C" fn mazer_destroy(maze: *mut Grid) {
    if maze.is_null() {
        return;
    }
    unsafe {
        // Convert the raw pointer back into a Box.
        // When the Box goes out of scope, the Grid (and its resources) are dropped.
        drop(Box::from_raw(maze));
    }
}

/// Retrieves the cells of the maze.
///
/// This function returns an array of `FFICell` structures that represent the individual cells
/// of the maze. It also writes the number of cells into the provided `length` pointer.
///
/// # Parameters
///
/// - `maze`: A pointer to the `Grid` whose cells are to be retrieved.
/// - `length`: A pointer to a `usize` variable where the number of cells will be stored.
///
/// # Returns
///
/// A pointer to an array of `FFICell` structures, or a null pointer if the input pointers are invalid.
#[no_mangle]
pub extern "C" fn mazer_get_cells(maze: *mut Grid, length: *mut usize) -> *mut FFICell {
    // Validate input pointers.
    if maze.is_null() || length.is_null() {
        return std::ptr::null_mut();
    }

    // Obtain a reference to the Grid.
    let grid = unsafe { &*maze };

    // Convert each Cell into an FFICell.
    let ffi_cells: Vec<FFICell> = grid.cells.iter().map(FFICell::from).collect();

    // Write the number of FFICells into the provided length pointer.
    let len = ffi_cells.len();
    unsafe {
        *length = len;
    }

    // Convert the Vec into a boxed slice and leak it to obtain a stable raw pointer.
    Box::into_raw(ffi_cells.into_boxed_slice()) as *mut FFICell
}

/// Frees an array of `FFICell` structures.
///
/// This function deallocates the memory allocated for an array of `FFICell` structures that was
/// previously returned by `mazer_get_cells`. The `length` parameter must match the number of
/// elements in the array.
///
/// # Parameters
///
/// - `ptr`: A pointer to the array of `FFICell` to be freed.
/// - `length`: The number of `FFICell` elements in the array.
#[no_mangle]
pub extern "C" fn mazer_free_cells(ptr: *mut FFICell, length: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // Reconstruct a boxed slice from the raw pointer.
        // The cast to *mut [FFICell] is required to correctly reconstruct the Box.
        let slice: *mut [FFICell] = std::slice::from_raw_parts_mut(ptr, length) as *mut [FFICell];
        drop(Box::from_raw(slice));
        // Dropping the Box will call Drop for every FFICell in the slice.
    }
}

/// Performs a move on the maze grid based on the provided direction.
///
/// This function takes an opaque pointer to a mutable `Grid` instance and a null-terminated C string
/// representing the direction in which to move. It then calls the internal `make_move` function on the
/// `Grid` and returns a pointer to the updated `Grid` instance.
///
/// # Parameters
///
/// - `grid_ptr`: An opaque pointer (`*mut c_void`) to a mutable `Grid`.
/// - `direction`: A null-terminated C string (`*const c_char`) indicating the move direction.
///
/// # Returns
///
/// A pointer to the updated `Grid` instance if successful, or a null pointer if an error occurs.
#[no_mangle]
pub extern "C" fn mazer_make_move(grid_ptr: *mut c_void, direction: *const c_char) -> *mut c_void {
    // Safety: Ensure that both pointers are non-null.
    if grid_ptr.is_null() || direction.is_null() {
        // Optionally, return a null pointer or handle errors in a way that your client (Swift)
        // can understand.
        return ptr::null_mut();
    }

    // Convert the opaque pointer back to a mutable reference to Grid.
    let grid: &mut Grid = unsafe { &mut *(grid_ptr as *mut Grid) };

    // Convert the C string to a Rust &str.
    let c_str_direction = unsafe { CStr::from_ptr(direction) };
    let direction_str = match c_str_direction.to_str() {
        Ok(s) => s,
        Err(_) => {
            // If the string conversion fails, return null or decide on an error handling strategy.
            return ptr::null_mut();
        }
    };

    // Call the make_move method on the mutable grid.
    let _ = grid.make_move(direction_str);

    // Return the same pointer to the grid.
    grid_ptr
}

/// Verifies FFI connectivity.
///
/// This function is used to verify that the FFI layer is working correctly. It should return 42.
///
/// # Returns
///
/// The integer `42` to indicate that the FFI connectivity is operational.
#[no_mangle]
pub extern "C" fn mazer_ffi_integration_test() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashSet, HashMap};
    use crate::cell::{CellOrientation, MazeType, Cell, Coordinates};

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
            is_active: false,
            is_visited: false,
            has_been_visited: false,
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