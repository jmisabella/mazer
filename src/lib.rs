use std::ffi::CStr;
use std::os::raw::c_char;
use crate::error::Error;
use crate::grid::Grid;
use crate::cell::FFICell;

pub mod cell;
pub mod grid;
pub mod direction;
pub mod request;
pub mod algorithms;
pub mod error;

// algorithms: BinaryTree, Sidewinder, AldousBroder, HuntAndKill, RecursiveBacktracker
// maze_types: Orthogonal, Delta, Hex, Polar
// example json maze request, all fields are required:
// {
//     "maze_type": "Orthogonal",
//     "width": 12,
//     "height": 12,
//     "algorithm": "RecursiveBacktracker",
//     "start": { "x": 0, "y": 0 },
//     "goal": { "x": 11, "y": 11 }
// }

pub fn generate(request_json: &str) -> Result<Grid, Error> {
    return Grid::try_from(request_json);
}

//// This is from Scala `maze` code, putting here because this shows where it's using distances and pathTo...
//def generate(request: MazeRequest): Grid = {
//    distance.pathTo(
//      distance.distances(
//        generate(request.mazeType, request.width, request.height, request.start, request.goal)
//        , request.start.x
//        , request.start.y)
//      , request.start.x
//      , request.start.y 
//      , request.goal.x
//      , request.goal.y)
//  }

//#[no_mangle]
//pub extern "C" fn mazer_generate_maze(request_json: *const c_char, length: *mut usize) -> *mut FFICell {
//    // Check for null pointers early to avoid unnecessary processing.
//    if request_json.is_null() {
//        eprintln!("mazer_generate_maze: request_json is null");
//        return std::ptr::null_mut();
//    }
//    if length.is_null() {
//        eprintln!("mazer_generate_maze: length pointer is null");
//        return std::ptr::null_mut();
//    }
//
//    // Convert C string to Rust string safely.
//    let request_str = match unsafe { CStr::from_ptr(request_json) }.to_str() {
//        Ok(s) => s,
//        Err(err) => {
//            eprintln!("mazer_generate_maze: Failed to convert request JSON to string: {:?}", err);
//            return std::ptr::null_mut();
//        }
//    };
//
//    // Attempt to generate the maze.
//    let maze = match generate(request_str) {
//        Ok(m) => m,
//        Err(err) => {
//            eprintln!("mazer_generate_maze: Maze generation failed: {:?}", err);
//            return std::ptr::null_mut();
//        }
//    };
//
//    // Convert cells to the exposed FFI format.
//    let exposed_cells: Vec<FFICell> = maze.cells.iter().map(FFICell::from).collect();
//
//    // Store the length of the cell array.
//    unsafe {
//        *length = exposed_cells.len();
//    }
//
//    // Convert the vector into a boxed slice and leak it to obtain a raw pointer.
//    // Ownership of this memory is transferred to the caller.
//    let boxed_slice = exposed_cells.into_boxed_slice();
//    Box::into_raw(boxed_slice) as *mut FFICell
//}

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

    // Generate the maze (Grid) using your existing generate() function.
    let maze = match generate(request_str) {
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

#[no_mangle]
pub extern "C" fn mazer_ffi_integration_test() -> i32 {
    42
}

