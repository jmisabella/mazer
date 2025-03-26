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
    // Deserialize the request JSON (safe since it's small)
    let request_str = unsafe { CStr::from_ptr(request_json).to_str().unwrap() };
    
    // Generate the maze in Rust
    let maze = generate(request_str).unwrap();
    
    // Convert the internal cells into the exposed `ExposedCell` format
    let exposed_cells: Vec<ExposedCell> = maze.cells.iter().map(|cell| ExposedCell::from(cell)).collect();
    
    // Store the length to help Swift know how many cells to read
    unsafe { *length = exposed_cells.len(); }
    
    // Convert the Vec into a raw pointer to return
    let boxed_slice = exposed_cells.into_boxed_slice();
    Box::into_raw(boxed_slice) as *mut ExposedCell
}

#[no_mangle]
pub extern "C" fn mazer_free_cells(ptr: *mut ExposedCell, length: usize) {
    if ptr.is_null() { return; }
    unsafe {
        drop(Vec::from_raw_parts(ptr, length, length));
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
