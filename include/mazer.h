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
    bool is_active;
    bool is_visited;
    bool has_been_visited;
    bool on_solution_path;
    const char* orientation;
} FFICell;

typedef struct {
    size_t x;
    size_t y;
} FFICoordinates;

typedef struct {
    size_t first;
    size_t second;
} EdgePair;

typedef struct {
    EdgePair* ptr;
    size_t len;
} EdgePairs;

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
 * Frees an array of Coordinates previously allocated by mazer_solution_path_order.
 *
 * This function deallocates the memory for an array of FFICoordinates that was previously
 * allocated by mazer_solution_path_order. It checks if the pointer is NULL and does nothing
 * in that case, ensuring safe operation. The caller should use this function to free memory
 * when done with the array.
 *
 * @param ptr A pointer to the array of FFICoordinates to be freed.
 * @param len The number of elements in the array.
 * @return This function does not return a value.
 */
void mazer_free_coordinates(FFICoordinates* ptr, size_t len);

/**
 * Frees the memory allocated for an EdgePairs structure.
 *
 * This function releases the memory allocated for the array of EdgePair within the provided
 * EdgePairs structure. It should be called by the user to free the memory returned by functions
 * like mazer_sigma_wall_segments. If the pointer in the structure is NULL, the function does nothing.
 *
 * @param ep The EdgePairs structure containing the pointer to the array and its length.
 * @return This function does not return a value.
 */
void mazer_free_edge_pairs(EdgePairs ep);

/**
 * Computes the solution path order for the maze.
 *
 * This function analyzes the maze grid and computes an ordered sequence of coordinates
 * representing the solution path. It returns a dynamically allocated array of FFICoordinates
 * and writes the number of coordinates into the provided out_length pointer. The caller
 * must free the returned array using mazer_free_coordinates. If the grid is invalid or
 * no solution exists, it returns NULL and sets *out_length to 0.
 *
 * @param grid A pointer to the Grid whose solution path is to be computed.
 * @param out_length A pointer to a size_t variable where the function will store the number of coordinates.
 * @return A pointer to an array of FFICoordinates, or NULL if the grid is invalid or no solution exists.
 */
FFICoordinates* mazer_solution_path_order(Grid* grid, size_t* out_length);

/**
 * Computes the height of an equilateral triangle given its side length.
 *
 * This function calculates the height of an equilateral triangle based on the provided
 * side length, using the formula height = (sqrt(3) / 2) * cell_size. It is a pure
 * mathematical utility for geometric calculations, such as in triangular maze rendering,
 * and does not depend on the maze grid.
 *
 * @param cell_size The side length of the equilateral triangle as a float.
 * @return The height of the triangle as a float.
 */
float mazer_triangle_height(float cell_size);

/**
 * Computes wall segments for a sigma (hexagonal) cell in the maze grid.
 *
 * This function determines the wall segments of a hexagonal cell within the maze grid,
 * returning an EdgePairs structure containing vertex index pairs to be drawn as walls.
 * The caller must free the returned structure using mazer_free_edge_pairs. If the grid
 * or coordinates are invalid, it returns an EdgePairs with ptr set to NULL and len set to 0.
 *
 * @param grid A pointer to the Grid containing the hexagonal cell.
 * @param cell_coords The coordinates of the cell to analyze.
 * @return An EdgePairs structure with vertex index pairs, or one with ptr = NULL if inputs are invalid.
 */
EdgePairs mazer_sigma_wall_segments(Grid* grid, FFICoordinates cell_coords);

/**
 * Updates the maze by performing a move in the specified direction.
 *
 * This function takes an opaque pointer to the mutable Grid and a null-terminated
 * C string that indicates the direction for the move. It calls the internal `make_move`
 * function on the Grid instance and returns an updated opaque pointer.
 *
 * @param grid_ptr A pointer to the mutable Grid.
 * @param direction A null-terminated C string indicating the move direction.
 * @return A pointer to the updated `Grid` instance if successful, or a null pointer if an error occurs.
 */
void* mazer_make_move(void* grid_ptr, const char* direction);

/**
 * To verify FFI connectivity, call verify this returns 42.
 */
int mazer_ffi_integration_test();

#ifdef __cplusplus
}
#endif

#endif /* MAZER_H */
