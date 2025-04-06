#include <stddef.h>
#include <stdbool.h>
#include <stdint.h>

#ifndef mazer_bridge_h
#define mazer_bridge_h

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    size_t x;
    size_t y;
    // const char * (or *const c_char in Rust) is a pointer to a single C string (e.g. null-terminated)
    const char *maze_type;
    // const char ** (or *const *const c_char in Rust) is a pointer to an array of pointers to C strings (i.e., a list of strings)
    const char **linked;
    size_t linked_len;
    int32_t distance;
    bool is_start;
    bool is_goal;
    bool is_visited; 
    bool on_solution_path;
    // const char * (or *const c_char in Rust) is a pointer to a single C string (e.g. null-terminated)
    const char *orientation;
} FFICell;


/**
 * Generates a maze based on the given JSON request string.
 *
 * @param request_json A JSON string specifying the maze type, size, algorithm, etc.
 * @param length A pointer to store the number of cells in the generated maze.
 * @return A pointer to an array of `FFICell` structs.
 */
FFICell* mazer_generate_maze(const char* request_json, size_t* length);

/**
 * Frees the allocated memory for the array of `FFICell` structs.
 *
 * @param ptr Pointer to the first element of the `FFICell` array.
 * @param length The number of elements in the array.
 */
void mazer_free_cells(FFICell* ptr, size_t length);

/**
 * To verify FFI connectivity, call verify this returns 42
 */
int mazer_ffi_integration_test();

#ifdef __cplusplus
}
#endif

#endif
