use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;
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

#[no_mangle]
pub extern "C" fn mazer_generate_maze(request_json: *const c_char, length: *mut usize) -> *mut FFICell {
    // Check for null pointers early to avoid unnecessary processing.
    if request_json.is_null() {
        eprintln!("mazer_generate_maze: request_json is null");
        return std::ptr::null_mut();
    }
    if length.is_null() {
        eprintln!("mazer_generate_maze: length pointer is null");
        return std::ptr::null_mut();
    }

    // Convert C string to Rust string safely.
    let request_str = match unsafe { CStr::from_ptr(request_json) }.to_str() {
        Ok(s) => s,
        Err(err) => {
            eprintln!("mazer_generate_maze: Failed to convert request JSON to string: {:?}", err);
            return std::ptr::null_mut();
        }
    };

    // Attempt to generate the maze.
    let maze = match generate(request_str) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("mazer_generate_maze: Maze generation failed: {:?}", err);
            return std::ptr::null_mut();
        }
    };

    // Convert cells to the exposed FFI format.
    let exposed_cells: Vec<FFICell> = maze.cells.iter().map(FFICell::from).collect();

    // Store the length of the cell array.
    unsafe {
        *length = exposed_cells.len();
    }

    // Convert the vector into a boxed slice and leak it to obtain a raw pointer.
    // Ownership of this memory is transferred to the caller.
    let boxed_slice = exposed_cells.into_boxed_slice();
    Box::into_raw(boxed_slice) as *mut FFICell
}


#[no_mangle]
pub extern "C" fn mazer_free_cells(ptr: *mut FFICell, length: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // Reconstruct a boxed slice from the raw pointer.
        let _ = Box::from_raw(std::slice::from_raw_parts_mut(ptr, length));
        // Dropping the Box will call Drop for every FFICell in the slice.
    }
}

#[no_mangle]
pub extern "C" fn mazer_generate_maze_json(request_json: *const c_char) -> *mut c_char {
    // Check for null input pointer.
    if request_json.is_null() {
        return ptr::null_mut();
    }

    // Safely convert the input C string to a Rust &str.
    let c_str = unsafe { CStr::from_ptr(request_json) };
    let input_json = match c_str.to_str() {
        Ok(s) => s,
        Err(err) => {
            eprintln!("mazer_generate_maze_json: Invalid UTF-8 sequence: {:?}", err);
            return ptr::null_mut();
        }
    };

    // Generate the maze and convert the result to a JSON string.
    // On error, an empty JSON string is returned.
    let result = match generate(input_json) {
        Ok(grid) => match serde_json::to_string(&grid) {
            Ok(json) => json,
            Err(err) => {
                eprintln!("mazer_generate_maze_json: Serialization error: {:?}", err);
                String::new()
            }
        },
        Err(err) => {
            eprintln!("mazer_generate_maze_json: Maze generation failed: {:?}", err);
            String::new()
        }
    };

    // Convert the Rust String to a C string, transferring ownership.
    // If the conversion fails (e.g., if the string contains a null byte), return null.
    CString::new(result)
        .map(|c_string| c_string.into_raw())
        .unwrap_or_else(|err| {
            eprintln!("mazer_generate_maze_json: CString conversion error: {:?}", err);
            ptr::null_mut()
        })
}

#[no_mangle]
pub extern "C" fn mazer_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // Reclaim the memory allocated for the C string.
        drop(CString::from_raw(ptr));
    }
}

#[no_mangle]
pub extern "C" fn mazer_ffi_integration_test() -> i32 {
    42
}

