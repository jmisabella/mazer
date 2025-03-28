#ifndef MAZER_H
#define MAZER_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>  // For size_t
#include <stdbool.h> // For bool

// Struct representing a single exposed cell in the maze
typedef struct {
    size_t x;
    size_t y;
    const char* maze_type;
    const char** linked; // Array of linked cell IDs (null-terminated strings)
    int distance;
    bool is_start;
    bool is_goal;
    bool on_solution_path;
    const char* orientation;
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
 * Generates a maze and returns the result as a JSON string.
 * 
 * @param request_json A JSON string specifying the maze parameters.
 * @return A dynamically allocated JSON string representing the maze.
 */
char* mazer_generate_maze_json(const char* request_json);

/**
 * Frees the allocated memory for a JSON string returned by `mazer_generate_maze_json`.
 * 
 * @param ptr Pointer to the dynamically allocated JSON string.
 */
void mazer_free_string(char* ptr);

/**
 * To verify FFI connectivity, call verify this returns 42
 */
int mazer_ffi_integration_test();

#ifdef __cplusplus
}
#endif

#endif // MAZER_H
