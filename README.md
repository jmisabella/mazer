# MazeR

A high-performance maze generation and solving library written in Rust, featuring **13 algorithms**, **5 grid types**, built-in **pathfinding with heat-map distances**, and a **C-compatible FFI** for seamless integration into iOS and Android apps.

---

## Highlights

- **13 maze generation algorithms** ranging from simple biased carvers to sophisticated graph-theoretic approaches
- **5 grid geometries** -- square, hexagonal, triangular, octagonal, and rhombic
- **BFS-based pathfinding** with per-cell distance values, enabling heat-map visualizations and hint systems
- **Solution path tracking** with `on_solution_path` flags for every cell
- **Full C FFI** with an included header file (`mazer.h`), designed for Swift/Kotlin interop on mobile
- **Generation step capture** for animating the maze-building process in a UI
- **Interactive move support** with visited/backtrack trail tracking for game-like experiences
- **Thread-safe** -- each maze is independently owned; concurrent generation is tested with 20 simultaneous threads
- **Perfect maze validation** (`is_perfect_maze`) ensures every generated maze has exactly one solution

---

## Maze Generation Algorithms

| Algorithm | Technique | Grid Support |
|---|---|---|
| Binary Tree | Biased carving | Orthogonal |
| Sidewinder | Row-based carving | Orthogonal |
| Aldous-Broder | Unbiased random walk | All |
| Wilson's | Loop-erased random walk | All |
| Hunt and Kill | Carving with hunt phase | All |
| Recursive Backtracker | Depth-first carving | All |
| Prim's | Minimum spanning tree (randomized weights) | All |
| Kruskal's | Edge-based union-find | All |
| Growing Tree (Random) | Random frontier selection | All |
| Growing Tree (Newest) | Stack-based frontier (newest cell) | All |
| Eller's | Row-by-row set merging | All |
| Recursive Division | Wall addition via subdivision | Orthogonal, Rhombic |
| Reverse Delete | Inverse Kruskal's -- removes edges from complete graph | All |

---

## Grid Types

| Type | Shape | Directions | Description |
|---|---|---|---|
| **Orthogonal** | Square | 4 (N/S/E/W) | Classic square grid |
| **Sigma** | Hexagonal | 6 | Hex cells using odd-q offset coordinates |
| **Delta** | Triangular | 6 | Alternating normal/inverted triangles |
| **Upsilon** | Octagonal + Square | 8 | Mixed octagons and squares |
| **Rhombic** | Diamond | 4 (diagonals) | Diamond-shaped cells with diagonal connectivity |

---

## Heat Map and Pathfinding

Every cell carries a `distance` field representing its BFS distance from the start, enabling:

- **Heat-map coloring** -- map distance values to a color gradient so players can visually gauge proximity to the goal
- **Hint system** -- highlight cells closer to the goal with warmer/cooler colors, giving users an optional nudge without revealing the full solution
- **Solution overlay** -- each cell's `on_solution_path` flag marks the optimal route from start to goal

---

## FFI: Use MazeR from Swift, Kotlin, or C

MazeR compiles as a static library (`.a` / `.so`) with a C-compatible API, making it straightforward to integrate into native mobile apps.

### API Overview

```c
// Generate a maze from a JSON request
Grid* mazer_generate_maze(const char *request_json);

// Retrieve all cells as a flat array
FFICell* mazer_get_cells(Grid *maze, size_t *length);

// Move the active cell in a direction (returns updated grid)
void* mazer_make_move(void *grid_ptr, const char *direction);

// Retrieve generation step snapshots for animation
size_t mazer_get_generation_steps_count(Grid *grid);
FFICell* mazer_get_generation_step_cells(Grid *grid, size_t step_index, size_t *length);

// Cleanup
void mazer_destroy(Grid *maze);
void mazer_free_cells(FFICell *ptr, size_t length);
```

### FFICell Structure

Each cell exposed through the FFI carries all the state a UI needs:

```c
typedef struct FFICell {
    size_t x, y;              // Grid coordinates
    const char* maze_type;    // "Orthogonal", "Sigma", etc.
    const char** linked;      // Open passages (neighbor coordinates)
    size_t linked_len;
    int32_t distance;         // BFS distance from start (heat map)
    bool is_start;
    bool is_goal;
    bool is_active;           // Current player position
    bool is_visited;          // Visited trail (resets on backtrack)
    bool has_been_visited;    // Permanent trail (never resets)
    bool on_solution_path;    // Part of the optimal solution
    const char* orientation;  // "Normal" / "Inverted" (Delta grids)
    bool is_square;           // Square vs octagon (Upsilon grids)
} FFICell;
```

---

## Usage (Rust)

```rust
use mazer::generate;

fn main() {
    let request = r#"{
        "maze_type": "Orthogonal",
        "width": 20,
        "height": 20,
        "algorithm": "RecursiveBacktracker",
        "start": { "x": 0, "y": 0 },
        "goal": { "x": 19, "y": 19 }
    }"#;

    let maze = generate(request).expect("Failed to generate maze");
    assert!(maze.is_perfect_maze().unwrap());
}
```

### JSON Request Fields

| Field | Type | Description |
|---|---|---|
| `maze_type` | string | `Orthogonal`, `Sigma`, `Delta`, `Upsilon`, or `Rhombic` |
| `width` | integer | Grid width |
| `height` | integer | Grid height |
| `algorithm` | string | Any algorithm name from the table above |
| `start` | `{x, y}` | Starting cell coordinates |
| `goal` | `{x, y}` | Goal cell coordinates |
| `capture_steps` | bool | *(Optional)* Record intermediate states for animation (disabled for grids > 100x100) |

---

## Interactive Move System

MazeR supports game-style navigation through `make_move`, which:

- Validates the move against open walls
- Updates `is_active`, `is_visited`, and `has_been_visited` flags
- Applies **direction fallback** for non-orthogonal grids (e.g., "Left" tries Left, then UpperLeft, then LowerLeft) so a 4-direction input device works naturally across all grid types

---

## Generation Step Capture

Enable `capture_steps` in the request to record each intermediate state as the algorithm carves the maze. This powers step-by-step animation in a UI, letting users watch the maze being built wall by wall.

---

## Building

```bash
# Build the library
cargo build --release

# Run tests
cargo test

# The static library for FFI will be at:
# target/release/libmazer.a (macOS/iOS)
# target/release/libmazer.so (Linux/Android)
```

For cross-compilation to iOS/Android targets, use the appropriate Rust target triples (e.g., `aarch64-apple-ios`, `aarch64-linux-android`).

---

## Project Structure

```
mazer/
  src/
    lib.rs              # Public API entry point
    cell.rs             # Cell, Coordinates, CellBuilder
    grid.rs             # Grid: generation, pathfinding, move handling
    direction.rs        # 8 directions with utility methods
    ffi.rs              # C-compatible FFI bindings
    request.rs          # JSON request deserialization
    error.rs            # 27 error types
    algorithms/         # 13 algorithm implementations
    behaviors/          # Traits: MazeGeneration, JsonDisplay, Graph (BFS)
  include/
    mazer.h             # C header for FFI consumers
  tests/
    generate_maze.rs    # Integration tests across all grid types
```

---

## License

Apache-2.0
