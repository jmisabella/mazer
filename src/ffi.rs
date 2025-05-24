use std::collections::HashSet;
use std::ptr;
use std::slice;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void, c_uint};
use libc::size_t;
use crate::Grid;
use crate::cell::{Cell, CellOrientation, Coordinates};
use crate::direction::Direction;
use crate::render::*;
use crate::render::delta::*;
use crate::render::sigma::*;
use crate::render::heatmap::*;


/// Representation of a cell for the FFI layer.
///
/// The fields represent the properties of a maze cell.
/// 
/// Fields:
/// - `x`: The x-coordinate of the cell.
/// - `y`: The y-coordinate of the cell.
/// - `maze_type`: A pointer to a null-terminated C string identifying the maze type.
/// - `linked`: A pointer to an array of null-terminated C strings represe_ting linked cells.
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
                CString::new(direction.to_string())
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

#[repr(C)]
pub struct FFICoordinates {
    pub x: usize,
    pub y: usize,
}

#[no_mangle]
pub extern "C" fn mazer_free_coordinates(ptr: *mut FFICoordinates, len: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, len, len);
        }
    }
}

/// Frees an EdgePairs buffer previously returned by `mazer_delta_wall_segments`.
#[no_mangle]
pub extern "C" fn mazer_free_edge_pairs(ep: EdgePairs) {
    if ep.ptr.is_null() { return; }
    // reconstruct the Vec to drop it
    unsafe { Vec::from_raw_parts(ep.ptr, ep.len, ep.len); }
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
    #[allow(unused_unsafe)]
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
    #[allow(unused_unsafe)]
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
    #[allow(unused_unsafe)]
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
    #[allow(unused_unsafe)]
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
        // bad inputs -> null
        return ptr::null_mut();
    }

    // reclaim the grid: convert the opaque pointer back to a mutable reference to Grid.
    #[allow(unused_unsafe)]
    let grid: &mut Grid = unsafe { &mut *(grid_ptr as *mut Grid) };

    // convert the C string to a Rust &str.
    let dir_str = match unsafe { CStr::from_ptr(direction) }.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let dir_enum = match Direction::try_from(dir_str) {
        Ok(d) => d,
        Err(_) => return std::ptr::null_mut(),
    };

    // attempt the move
    if grid.make_move(dir_enum).is_ok() {
        // on successful move, return the same pointer to the grid.
        grid_ptr
    } else {
        eprintln!("mazer_make_move failed on {:?} at {:?}", dir_enum, grid_ptr);
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn mazer_solution_path_order(grid: *const Grid, out_length: *mut size_t) -> *mut FFICoordinates {
    if grid.is_null() || out_length.is_null() {
        return std::ptr::null_mut(); // Return null if inputs are invalid
    }
    let grid = unsafe { &*grid };
    let path = solution_path_order(&grid.cells); // Get Vec<Coordinates>
    let c_path: Vec<FFICoordinates> = path
        .iter()
        .map(|c| FFICoordinates { x: c.x, y: c.y })
        .collect();
    let len = c_path.len();
    let boxed_slice = c_path.into_boxed_slice(); // Convert to Box<[FFICoordinates]>
    let ptr = boxed_slice.as_ptr() as *mut FFICoordinates; // Get raw pointer
    std::mem::forget(boxed_slice); // Prevent Rust from freeing it
    unsafe { *out_length = len }; // Set the length for the caller
    ptr
}

/// A 2-tuple of vertex indices for one wall segment.
#[repr(C)]
pub struct EdgePair {
    pub first: usize,
    pub second: usize,
}

/// A pointer+length describing an array of `EdgePair`s.
#[repr(C)]
pub struct EdgePairs {
    pub ptr: *mut EdgePair,
    pub len: usize,
}


/// C-ABI wrapper for `delta_wall_segments`.
///
/// - `linked_dirs` is a pointer to an array of `u32` codes (one per direction).
///   Each code should match the `as u32` of your `Direction` enum variants (see below).
/// - `linked_len` is the length of that array.
/// - `orientation_code` is 0 for Normal, 1 for Inverted (matching `CellOrientation as u32`).
/// 
/// Returns an `EdgePairs` (pointer+length) you must later free by calling
/// `mazer_free_edge_pairs`.
#[no_mangle]
pub extern "C" fn mazer_delta_wall_segments(
    linked_dirs: *const c_uint,
    linked_len: usize,
    orientation_code: c_uint,
) -> EdgePairs {
    // 1) Reconstruct the incoming slice of u32 codes
    let slice = unsafe { slice::from_raw_parts(linked_dirs, linked_len) };
    let mut linked: HashSet<Direction> = HashSet::with_capacity(linked_len);
    
    // 2) Convert each u32 → Direction
    for &code in slice {
        if let Ok(dir) = Direction::try_from(code) {
            linked.insert(dir);
        }
    }

    // 3) Convert the orientation code into your enum
    let orientation = match orientation_code {
        0 => CellOrientation::Normal,
        1 => CellOrientation::Inverted,
        _ => CellOrientation::Normal,  // fallback
    };

    // 4) Call the pure-Rust logic
    let segments = delta_wall_segments(&linked, orientation);

    // 5) Move into a C buffer
    let mut v: Vec<EdgePair> = segments
        .into_iter()
        .map(|(a, b)| EdgePair { first: a, second: b })
        .collect();

    let len = v.len();
    let ptr = v.as_mut_ptr();
    std::mem::forget(v);

    EdgePairs { ptr, len }
}

#[no_mangle]
pub extern "C" fn mazer_sigma_wall_segments(grid: *const Grid, cell_coords: FFICoordinates) -> EdgePairs {
    // Check for null pointer
    if grid.is_null() {
        return EdgePairs { ptr: std::ptr::null_mut(), len: 0 };
    }

    let grid = unsafe { &*grid };

    // Convert C coordinates to Rust coordinates
    let rust_coords = Coordinates { x: cell_coords.x, y: cell_coords.y };

    // Retrieve the cell using grid.get
    let cell = match grid.get(rust_coords) {
        Ok(c) => c,
        Err(_) => return EdgePairs { ptr: std::ptr::null_mut(), len: 0 },
    };

    // Get wall segments
    let segments = sigma_wall_segments(cell, grid);

    // Convert to EdgePairs for C
    let edge_pairs: Vec<EdgePair> = segments
        .into_iter()
        .map(|(first, second)| EdgePair { first, second })
        .collect();

    let len = edge_pairs.len();
    let ptr = edge_pairs.as_ptr() as *mut EdgePair;
    std::mem::forget(edge_pairs); // Prevent Rust from deallocating the memory

    EdgePairs { ptr, len }
}

/// C-ABI wrapper for `shade_index(distance, max_distance) ⇒ usize`
#[no_mangle]
pub extern "C" fn mazer_shade_index(
    distance: usize,
    max_distance: usize,
) -> usize {
    shade_index(distance, max_distance)
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
        let mut neighbors: HashMap<Direction, Coordinates> = HashMap::new();
        neighbors.insert(Direction::Up, Coordinates { x: 1, y: 1 });
        neighbors.insert(Direction::Right, Coordinates { x: 2, y: 2 });
        neighbors.insert(Direction::Down, Coordinates { x: 1, y: 3 });
        neighbors.insert(Direction::Left, Coordinates { x: 0, y: 2 });

        let mut linked: HashSet<Coordinates> = HashSet::new();
        linked.insert(Coordinates { x: 2, y: 2 });
        linked.insert(Coordinates { x: 1, y: 3 });

        let mut open_walls: Vec<Direction> = Vec::new();
        open_walls.push(Direction::Right);
        open_walls.push(Direction::Down);

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
                    Some(k.to_string().clone())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(linked_rust, expected_linked);

        // No manual cleanup is necessary.
        // The Drop implementation for FFICell will automatically free all allocated memory.
    }

    #[test]
    fn test_mazer_generate_maze() {
        let json_request = r#"
        {
            "maze_type": "Orthogonal",
            "width": 22,
            "height": 22,
            "algorithm": "Wilsons",
            "start": { "x": 0, "y": 0 },
            "goal": { "x": 21, "y": 21 }
        }
        "#;
        let json_req_c_string = CString::new(json_request)
            .unwrap()
            .into_raw();
        
        
        // Call the FFI function within an unsafe block.
        let grid_ptr = mazer_generate_maze(json_req_c_string);

        // Check that the returned pointer is not null.
        assert!(!grid_ptr.is_null());

        // convert the pointer back to a Rust mutable reference.
        let maze: &mut Grid = unsafe { &mut *(grid_ptr as *mut Grid) };
        
        assert!(maze.is_perfect_maze().unwrap());
        println!("\n\nMaze:\n\n{}\n\n", maze.to_asci());

        // clean up
        unsafe {
            // clean up memory used by maze 
            mazer_destroy(maze);
            // reclaim the C string from the raw pointer so Rust would clean it up after it leaves scope
            let _ = CString::from_raw(json_req_c_string);
        }
    }


    #[test]
    fn test_mazer_get_cells_length_argument_with_free() {
        // Create a JSON definition for a simple maze.
        let json = r#"
        {
            "maze_type": "Orthogonal",
            "width": 50,
            "height": 60,
            "algorithm": "HuntAndKill",
            "start": { "x": 0, "y": 0 },
            "goal": { "x": 49, "y": 59 }
        }
        "#;

        // Create a Grid from the JSON (assuming Grid::try_from exists and works).
        let grid = Grid::try_from(json).expect("Failed to create Grid from JSON");

        // Box the Grid so it is heap-allocated.
        let boxed_grid = Box::new(grid);

        // Convert the Box into a raw pointer.
        // This pointer will be passed to the FFI function.
        let maze_ptr: *mut Grid = Box::into_raw(boxed_grid);

        // Create a mutable length variable, and get its pointer.
        let mut length: usize = 0;
        let length_ptr: *mut usize = &mut length;

        // Call the FFI function. It writes the number of cells to the location pointed by length_ptr.
        let cells_ptr = mazer_get_cells(maze_ptr, length_ptr);

        // Verify that a non-null pointer is returned.
        assert!(!cells_ptr.is_null(), "Expected non-null pointer from mazer_get_cells");

        // The length should now equal the number of cells in the original Grid.
        // Since the grid has been moved to the heap, we recover a reference to it.
        let grid_ref = unsafe { &*maze_ptr };
        assert_eq!(
            grid_ref.cells.len(),
            length,
            "Length returned by FFI should equal the number of cells in the grid"
        );

        // Convert the returned raw pointer into a slice for further examination.
        let ffi_cells: &[FFICell] = unsafe { std::slice::from_raw_parts(cells_ptr, length) };
        println!("Number of FFICells: {}", ffi_cells.len());

        // Optionally add more validations about the FFICell content here.

        // *** Memory Cleanup ***
        // Use the provided mazer_free_cells to free the allocated FFICell array.
        // clean up FFICells 
        mazer_free_cells(cells_ptr, length);
        // clean up memory used by maze 
        mazer_destroy(maze_ptr);
    }

    #[test]
    fn test_mazer_make_move() {
        let json = r#"
        {
            "maze_type": "Orthogonal",
            "width": 12,
            "height": 12,
            "algorithm": "RecursiveBacktracker",
            "start": { "x": 0, "y": 0 },
            "goal": { "x": 11, "y": 11 }
        }
        "#;
        match Grid::try_from(json) {
            Ok(grid) => {
                // Allocate the grid on the heap.
                let boxed_grid = Box::new(grid);

                // Convert the Box into a raw pointer, then cast to *mut c_void.
                let grid_ptr: *mut c_void = Box::into_raw(boxed_grid) as *mut c_void;

                // Create a CString for the direction.
                let direction = CString::new("Up").expect("CString::new failed");

                // Call the FFI function within an unsafe block.
                let unsuccessful_move_ptr = mazer_make_move(grid_ptr, direction.as_ptr());

                // Check that the result didn't succeed to move to a different cell 
                assert!(unsuccessful_move_ptr.is_null());

                // Convert the pointer back to a Rust mutable reference.
                let maze: &mut Grid = unsafe { &mut *(grid_ptr as *mut Grid) };
                
                assert!(maze.is_perfect_maze().unwrap());
                println!("\n\nMaze:\n\n{}\n\n", maze.to_asci());
                
                assert_eq!(
                    maze.cells.iter().filter(|cell| cell.is_visited).count(),
                    1,
                    "There should be 1 visited cell on dynamic path at the beginning"
                );
                
                assert_eq!(
                    maze.cells.iter().filter(|cell| cell.has_been_visited).count(),
                    1,
                    "There should be 1 visited cell on permenant path at the beginning"
                );
                
                // Limit the borrow's scope and return only owned data.
                let (original_coords, available_moves, unavailable_moves) = {
                    // 1) Grab the active cell or panic
                    let active = maze
                        .get_active_cell()
                        .expect("Expected an active cell at the start");
                
                    // 2) Clone its coords
                    let original_coords = active.coords.clone();
                
                    // 3) Collect its open_walls directly as a Vec<Direction>
                    //    (assuming open_walls: HashSet<Direction> or Vec<Direction>)
                    let available_moves: Vec<Direction> = active.open_walls.iter().cloned().collect();
                
                    // 4) Anything in `all_moves` not in `available_moves` is “unavailable”
                    let unavailable_moves: Vec<Direction> = maze.all_moves()
                        .iter()
                        .filter(|d| !available_moves.contains(d))
                        .cloned()
                        .collect();
                
                    (original_coords, available_moves, unavailable_moves)
                }; // all borrows dropped here

                // Now it's safe to perform mutable operations.

                // Try a move that is unavailable using a copied maze.
                let mut copied_maze = maze.clone();

                let bad_move = unavailable_moves
                    .first()
                    .expect("Expected at least 1 unavailable move");

                assert!(
                    copied_maze.make_move(*bad_move).is_err(),
                    "Should not allow unavailable move {}",
                    bad_move
                );

                assert_eq!(
                    copied_maze.cells.iter().filter(|cell| cell.is_visited).count(),
                    1,
                    "There should be 1 visited cell on dynamic path before a successful move is made"
                );
                
                assert_eq!(
                    copied_maze.cells.iter().filter(|cell| cell.has_been_visited).count(),
                    1,
                    "There should be 1 visited cell on permenant path before a successful move is made"
                );
                
                // Try a valid move on the original maze.
                let next = available_moves.first().expect("There should be available moves"); 
                assert!(maze.make_move(*next).is_ok(), "Should allow a valid move");

                // Verify that exactly one cell is active.
                assert_eq!(
                    maze.cells.iter().filter(|cell| cell.is_active).count(),
                    1,
                    "There should be exactly one active cell"
                );

                assert_eq!(
                    maze.cells.iter().filter(|cell| cell.is_visited).count(),
                    2,
                    "There should be 2 visited cells on dynamic path after first successful move (start cell and current)"
                );
                
                assert_eq!(
                    maze.cells.iter().filter(|cell| cell.has_been_visited).count(),
                    2,
                    "There should be 2 visited cells on permenant path after first successful move (start cell and current)"
                );


                // Verify that the active cell has changed.
                let new_active_coords = maze
                    .get_active_cell()
                    .expect("Expected an active cell after the move")
                    .coords
                    .clone();
                assert_ne!(
                    new_active_coords, original_coords,
                    "The active cell should have moved to a new coordinate"
                );
                // clean up memory used by maze 
                mazer_destroy(maze);
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }       
    }

    // Test for mazer_solution_path_order
    #[test]
    fn test_mazer_solution_path_order() {
        // Create a simple orthogonal maze with a known solution path
        let json = r#"{
            "maze_type": "Orthogonal",
            "width": 3,
            "height": 3,
            "algorithm": "BinaryTree",
            "start": { "x": 0, "y": 0 },
            "goal": { "x": 2, "y": 2 }
        }"#;
        let c_json = CString::new(json).expect("Failed to create CString");
        let grid_ptr = mazer_generate_maze(c_json.as_ptr());
        assert!(!grid_ptr.is_null(), "Grid pointer should not be null");

        // Call the FFI function
        let mut length: usize = 0;
        let coords_ptr = mazer_solution_path_order(grid_ptr, &mut length);
        assert!(!coords_ptr.is_null(), "Coordinates pointer should not be null");
        assert!(length > 0, "Solution path should have at least one coordinate");

        // Convert to a slice for validation
        let coords_slice = unsafe { slice::from_raw_parts(coords_ptr, length) };

        // Get the expected result from the Rust function
        let grid = unsafe { &*grid_ptr };
        let expected = solution_path_order(&grid.cells);

        // Validate length and contents
        assert_eq!(length, expected.len(), "Length mismatch between FFI and Rust function");
        for (ffi_coord, rust_coord) in coords_slice.iter().zip(expected.iter()) {
            assert_eq!(ffi_coord.x, rust_coord.x, "X coordinate mismatch");
            assert_eq!(ffi_coord.y, rust_coord.y, "Y coordinate mismatch");
        }

        // Clean up
        mazer_free_coordinates(coords_ptr, length);
        mazer_destroy(grid_ptr);
    }

    // Test for mazer_sigma_wall_segments
    #[test]
    fn test_mazer_sigma_wall_segments() {
        // Create a sigma (hexagonal) maze
        let json = r#"{
            "maze_type": "Sigma",
            "width": 3,
            "height": 3,
            "algorithm": "RecursiveBacktracker",
            "start": { "x": 0, "y": 0 },
            "goal": { "x": 2, "y": 2 }
        }"#;
        let c_json = CString::new(json).expect("Failed to create CString");
        let grid_ptr = mazer_generate_maze(c_json.as_ptr());
        assert!(!grid_ptr.is_null(), "Grid pointer should not be null");

        // Define Coordinates for Rust usage
        let rust_coords = Coordinates { x: 1, y: 1 };
        // Convert to FFICoordinates for the FFI function
        let ffi_coords = FFICoordinates { x: rust_coords.x, y: rust_coords.y };
        let ep = mazer_sigma_wall_segments(grid_ptr, ffi_coords);
        assert!(!ep.ptr.is_null(), "EdgePairs pointer should not be null");
        assert!(ep.len > 0, "Should return at least one wall segment");

        // Convert to a slice for validation
        let pairs_slice = unsafe { slice::from_raw_parts(ep.ptr, ep.len) };

        // Get the expected result from the Rust function
        let grid = unsafe { &*grid_ptr };
        let cell = grid.get(Coordinates { x: 1, y: 1 }).expect("Cell should exist");
        let expected = sigma_wall_segments(cell, &grid);

        // Validate length and contents
        assert_eq!(ep.len, expected.len(), "Length mismatch between FFI and Rust function");
        for (ffi_pair, rust_pair) in pairs_slice.iter().zip(expected.iter()) {
            assert_eq!(ffi_pair.first, rust_pair.0, "First index mismatch");
            assert_eq!(ffi_pair.second, rust_pair.1, "Second index mismatch");
        }

        // Clean up
        mazer_free_edge_pairs(ep);
        mazer_destroy(grid_ptr);
    }

    #[test]
    fn test_ffi_integration_returns_42() {
        let result = mazer_ffi_integration_test();
        // this FFI integration test function simply returns 42, useful to show integration of the .a C library at Swift, etc... environment 
        assert_eq!(result, 42, "The FFI integration test function should return 42");
    }
}