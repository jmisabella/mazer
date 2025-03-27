use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;
use crate::error::Error;
use crate::grid::Grid;
use crate::cell::ExposedCell;

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

#[no_mangle]
pub extern "C" fn mazer_generate_maze(request_json: *const c_char, length: *mut usize) -> *mut ExposedCell {
    if request_json.is_null() {
        eprintln!("mazer_generate_maze: request_json is null");
        return std::ptr::null_mut();
    }

    // Convert C string to Rust string safely
    let request_str = match unsafe { CStr::from_ptr(request_json) }.to_str() {
        Ok(s) => s,
        Err(err) => {
            eprintln!("mazer_generate_maze: Failed to convert request JSON to string: {:?}", err);
            return std::ptr::null_mut();
        }
    };

    // Attempt to generate the maze
    let maze = match generate(request_str) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("mazer_generate_maze: Maze generation failed: {:?}", err);
            return std::ptr::null_mut();
        }
    };

    // Convert cells to the exposed format
    let exposed_cells: Vec<ExposedCell> = maze.cells.iter().map(ExposedCell::from).collect();

    // Store the length in the provided pointer safely
    if length.is_null() {
        eprintln!("mazer_generate_maze: length pointer is null");
        return std::ptr::null_mut();
    }
    unsafe { *length = exposed_cells.len(); }

    // Convert to a raw pointer
    let boxed_slice = exposed_cells.into_boxed_slice();
    Box::into_raw(boxed_slice) as *mut ExposedCell
}

#[no_mangle]
pub extern "C" fn mazer_free_cells(ptr: *mut ExposedCell, _length: usize) {
    if ptr.is_null() { return; }
    unsafe {
        drop(Box::from_raw(ptr));  // ✅ Correct: Matches Box::into_raw
    }
}


#[no_mangle]
pub extern "C" fn mazer_generate_maze_json(request_json: *const c_char) -> *mut c_char {
    if request_json.is_null() {
        // null input, return mutable null pointer (*mut c_char)
        return ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(request_json) };
    let input_json = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let result = match generate(input_json) {
        Ok(grid) => serde_json::to_string(&grid).unwrap_or_else(|_| String::new()),
        Err(_) => String::new(), // empty JSON string on error
    };

    // convert Rust string to C string and return a pointer
    CString::new(result)
        .map(|c_string| c_string.into_raw())
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn mazer_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(ptr)); // reclaim memory
    }
}
