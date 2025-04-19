use std::ptr;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use crate::grid::Grid;
use crate::cell::Cell;
use crate::direction::Direction;


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

    // Reclaim the grid: convert the opaque pointer back to a mutable reference to Grid.
    #[allow(unused_unsafe)]
    let grid: &mut Grid = unsafe { &mut *(grid_ptr as *mut Grid) };

    // Convert the C string to a Rust &str.
    #[allow(unused_unsafe)]
    let dir_str = unsafe { CStr::from_ptr(direction) }
        .to_str()
        .unwrap_or_default();
        
    if let Ok(dir) = Direction::try_from(dir_str) {
        // attempt to move, but ignore the Err case
        let _ = grid.make_move(dir);
    }

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
                let updated_ptr = mazer_make_move(grid_ptr, direction.as_ptr());

                // Check that the returned pointer is not null.
                assert!(!updated_ptr.is_null());

                // convert the pointer back to a Rust mutable reference.
                let maze: &mut Grid = unsafe { &mut *(updated_ptr as *mut Grid) };
                
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

    #[test]
    fn test_ffi_integration_returns_42() {
        let result = mazer_ffi_integration_test();
        // this FFI integration test function simply returns 42, useful to show integration of the .a C library at Swift, etc... environment 
        assert_eq!(result, 42, "The FFI integration test function should return 42");
    }
}