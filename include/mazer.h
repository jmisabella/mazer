#ifndef MAZER_H
#define MAZER_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>   // for size_t
#include <stdint.h>   // for int32_t
#include <stdbool.h>  // for bool

/* Opaque type declarations.
 * The actual definitions of these types are hidden from the Swift side.
 */
typedef struct Grid Grid;

typedef struct FFICell {
    size_t x;
    size_t y;
    const char* maze_type;
    const char** linked;
    size_t linked_len;
    int32_t distance;
    bool is_start;
    bool is_goal;
    bool is_visited;
    bool on_solution_path;
    const char* orientation;
} FFICell;

/**
 * Generates a maze from a JSON request.
 *
 * This function takes a null-terminated JSON string representing the maze generation
 * request and attempts to generate a maze. On success, it returns a pointer to a newly
 * allocated Grid instance. In case of an error (such as an invalid JSON or a failure in
 * maze generation), it returns NULL.
 *
 * @param request_json A null-terminated C string containing the JSON request.
 * @return A pointer to the generated Grid if successful, or NULL on failure.
 */
Grid* mazer_generate_maze(const char *request_json);

/**
 * Destroys a maze instance.
 *
 * This function deallocates the memory and any associated resources for the given maze (Grid).
 * If the provided maze pointer is NULL, the function does nothing.
 *
 * @param maze A pointer to the Grid instance to be destroyed.
 */
void mazer_destroy(Grid *maze);

/**
 * Retrieves the cells of the maze.
 *
 * This function returns an array of FFICell structures that represent the individual cells
 * of the maze. It also writes the number of cells into the provided 'length' pointer.
 *
 * @param maze A pointer to the Grid whose cells are to be retrieved.
 * @param length A pointer to a size_t variable where the function will store the number of cells.
 * @return A pointer to an array of FFICell structures, or NULL if the input pointers are invalid.
 */
FFICell* mazer_get_cells(Grid *maze, size_t *length);

/**
 * Frees an array of FFICell.
 *
 * This function deallocates the memory allocated for an array of FFICell structures that was
 * previously returned by mazer_get_cells. The 'length' parameter must match the number of
 * elements in the array.
 *
 * @param ptr A pointer to the array of FFICell to be freed.
 * @param length The number of FFICell elements in the array.
 */
void mazer_free_cells(FFICell *ptr, size_t length);

/**
 * To verify FFI connectivity, call verify this returns 42.
 */
int mazer_ffi_integration_test();

#ifdef __cplusplus
}
#endif

#endif /* MAZER_H */
