use std::fmt;
use std::collections::{HashMap, HashSet};
use rand::{ thread_rng, Rng };
use serde::ser::{ Serialize, Serializer, SerializeStruct };
use crate::behaviors::display::JsonDisplay;
use crate::behaviors::graph;
use crate::cell::{CellOrientation, MazeType, Cell, CellBuilder, Coordinates};
use crate::direction::Direction;
use crate::error::Error;
use crate::request::MazeRequest;

#[derive(Debug, Clone)]
/// Represents a grid of maze cells, encapsulating both the cells and their spatial relationships.
///
/// This grid defines the layout of the maze by positioning each cell relative to its neighbors,
/// enabling operations like navigation and pathfinding. It is defined by its dimensions, maze type,
/// and the collection of cells that form the maze. Additionally, the maze generation can be seeded
/// to ensure reproducibility.
pub struct Grid {
    /// The width of the grid.
    pub width: usize,
    /// The height of the grid.
    pub height: usize,
    /// The maze type, which determines the style of the maze (e.g., Orthogonal, Delta, Sigma).
    pub maze_type: MazeType,
    /// A flattened array of cells that make up the maze.
    pub cells: Vec<Cell>,
    /// The random seed used to generate the maze.
    pub seed: u64,
    /// The coordinates of the start cell within the grid.
    pub start_coords: Coordinates,
    /// The coordinates of the goal cell within the grid.
    pub goal_coords: Coordinates,
    /// Enables intermediate grid states to be recorded during maze generation, for education purposes to the user.
    pub capture_steps: bool,
    /// When capture_steps is true, contains a vector of `Grid` states representing each significant step of the maze generation process
    pub generation_steps: Option<Vec<Grid>>,
}

impl Serialize for Grid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut grid_map = serializer.serialize_struct("Grid", 1)?;
        grid_map.serialize_field("rows", &self.cells)?;
        return grid_map.end(); 
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_json() {
            Ok(json) => write!(f, "{}", json),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl TryFrom<MazeRequest> for Grid {
    type Error = crate::Error;

    fn try_from(request: MazeRequest) -> Result<Self, Self::Error> {
        // decide start/goal, falling back to sensible defaults
        let (start_coords, goal_coords) = match (request.start, request.goal) {
            (Some(s), Some(g)) => (s, g),
            _ => Grid::default_endpoints(request.width, request.height),
        };

        let mut grid = Grid::new(
            request.maze_type,
            request.width,
            request.height,
            start_coords,
            goal_coords,
            request.capture_steps.unwrap_or_default(),
        )?;

        request.algorithm.generate(&mut grid)?;
        Ok(grid)
    }
}

impl TryFrom<&str> for Grid {
    type Error = crate::Error; // explicitly reference our custom Error type

    fn try_from(json: &str) -> Result<Self, Self::Error> {
        let deserialized: MazeRequest = serde_json::from_str(json)?;
        Grid::try_from(deserialized)
    }
}

impl TryFrom<String> for Grid {
    type Error = crate::Error;

    fn try_from(json: String) -> Result<Self, Self::Error> {
        // just call the &str implementation
        Grid::try_from(json.as_str())
    }
}

impl Grid {

    ////// TODO: incorporate this behavior, to use these start/goal defaults when not specified in request
    /// if height >= width, middle bottom → middle top,
    /// otherwise middle left → middle right
    pub fn default_endpoints(
        width: usize,
        height: usize,
    ) -> (Coordinates, Coordinates) {
        // stronger preference towards start/goal coords being bottom/top rows
        if height as f64 * 1.35 >= width as f64 {
            let x = width / 2;
            ( Coordinates { x, y: height - 1 },
            Coordinates { x, y: 0 } )
        } else {
            let y = height / 2;
            ( Coordinates { x: 0, y },
            Coordinates { x: width - 1, y } )
        }
    }

    /// Get x,y coordinate's index in the flattened 1D vector
    pub fn get_flattened_index(&self, x: usize, y: usize) -> usize {
        // when unflattened to become a 2D vector, cells are stored in row-major order 
        y * self.width + x
    }
    
    /// Retrieve a cell by its coordinates
    pub fn get(&self, coords: Coordinates) -> Result<&Cell, Error> {
        let index = self.get_flattened_index(coords.x, coords.y);
        self.cells
            .get(index)
            .ok_or_else(|| Error::OutOfBoundsCoordinates {
                coordinates: coords,
                maze_width: self.width,
                maze_height: self.height
            })
    }

    // retrieve a mutable cell by its coordinates
    pub fn get_mut(&mut self, coords: Coordinates) -> Result<&mut Cell, Error> {
        let index = self.get_flattened_index(coords.x, coords.y);
        self.cells
            .get_mut(index)
            .ok_or_else(|| Error::OutOfBoundsCoordinates {
                coordinates: coords,
                maze_width: self.width,
                maze_height: self.height
            })
    }

    /// Get the currently active Cell
    pub fn get_active_cell(&mut self) -> Result<&mut Cell, Error> {
        let active_count = self.cells.iter().filter(|cell| cell.is_active).count();
        if active_count > 1 {
            return Err(Error::MultipleActiveCells { count: active_count });
        }
        if let Some(active_cell) = self.cells.iter_mut().find(|cell| cell.is_active) {
            return Ok(active_cell);
        } else {
            return Err(Error::NoActiveCells);
        }
    }

    /// All the “raw” directions this maze shape can ever use.
    pub fn all_moves(&self) -> &'static [Direction] {
        use Direction::*;
        match self.maze_type {
            MazeType::Orthogonal => &[Up, Right, Down, Left],
            MazeType::Sigma      => &[Up, UpperRight, Right, LowerRight,
                                       Down, LowerLeft, Left, UpperLeft],
            MazeType::Delta      => &[Up, UpperLeft, UpperRight,
                                       Down, LowerLeft, LowerRight],
            MazeType::Rhombille => &[UpperRight, UpperLeft, LowerLeft, LowerRight],
        }
    }

    /// Which directions would make_move reject *right now*?
    pub fn unavailable_moves(&self) -> Vec<Direction> {
        self.all_moves()
            .iter()
            .cloned()
            .filter(|d| {
                let mut copy = self.clone();
                copy.make_move(*d).is_err()
            })
            .collect()
    }

    /// Which directions would make_move accept *right now*? 
    pub fn effective_moves(&self) -> Vec<Direction> {
        self.all_moves()
            .iter()
            .cloned()
            .filter(|d| {
                let mut copy = self.clone();
                copy.make_move(*d).is_ok()
            })
            .collect()
    }

    /// Manually make a user move to a specified direction.
    pub fn make_move(&mut self, direction: Direction) -> Result<Direction, Error> {
        // Get the current active cell and record its coordinates.
        let active_cell = self.get_active_cell()?;
        let original_coords = active_cell.coords;

        // Determine the effective direction to use, accounting for Delta maze fallback logic.
        // let picked: Option<Direction> = if maze_type == MazeType::Delta {
        // Determine the effective direction to use, accounting for fallback logic.
        let picked: Option<Direction> = {
            // Define a helper closure: it checks whether a candidate move is both open (in open_walls)
            // and valid (exists in neighbors_by_direction).
            let try_direction = |cell: &Cell, cand: &Direction| -> Option<Direction> {
                if cell.open_walls.contains(&cand)
                    && cell.neighbors_by_direction.contains_key(cand)
                {
                    Some(cand.clone())
                } else {
                    None
                }
            };

            match direction {
                Direction::Left => {
                    // For "Left", try Left then UpperLeft then LowerLeft.
                    try_direction(active_cell, &Direction::Left)
                        .or_else(|| try_direction(active_cell, &Direction::UpperLeft))
                        .or_else(|| try_direction(active_cell, &Direction::LowerLeft))
                },
                Direction::Right => {
                    // For "Right", try Right then UpperRight then LowerRight.
                    try_direction(active_cell, &Direction::Right)
                        .or_else(|| try_direction(active_cell, &Direction::UpperRight))
                        .or_else(|| try_direction(active_cell, &Direction::LowerRight))
                },
                Direction::UpperLeft =>{
                    // For "UpperLeft", try UpperLeft then Up then Left.
                    try_direction(active_cell, &Direction::UpperLeft)
                        .or_else(|| try_direction(active_cell, &Direction::Up))
                        .or_else(|| try_direction(active_cell, &Direction::Left))
                },
                Direction::LowerLeft => {
                    // For "LowerLeft", try LowerLeft then Down then Left.
                    try_direction(active_cell, &Direction::LowerLeft)
                        .or_else(|| try_direction(active_cell, &Direction::Down))
                        .or_else(|| try_direction(active_cell, &Direction::Left))
                },
                Direction::UpperRight => {
                    // For "UpperRight", try UpperRight then Up then Right.
                    try_direction(active_cell, &Direction::UpperRight)
                        .or_else(|| try_direction(active_cell, &Direction::Up))
                        .or_else(|| try_direction(active_cell, &Direction::Right))
                },
                Direction::LowerRight => {
                    // For "LowerRight", try LowerRight then Down then Right.
                    try_direction(active_cell, &Direction::LowerRight)
                        .or_else(|| try_direction(active_cell, &Direction::Down))
                        .or_else(|| try_direction(active_cell, &Direction::Right))
                },
                Direction::Up => {
                    // For Up, try Up first then fall back to UpperLeft, then fail back to UpperRight.
                    try_direction(active_cell, &Direction::Up)
                        .or_else(|| try_direction(active_cell, &Direction::UpperLeft))
                        .or_else(|| try_direction(active_cell, &Direction::UpperRight))
                },
                Direction::Down => {
                    // For Down, try Down first then fall back to LowerLeft, then fail back to LowerRight.
                    try_direction(active_cell, &Direction::Down)
                        .or_else(|| try_direction(active_cell, &Direction::LowerLeft))
                        .or_else(|| try_direction(active_cell, &Direction::LowerRight))
                },
                // // If the provided direction isn't one of the Delta-specific ones, use it as given.
                // _ => Some(direction.clone()),
            }
        }; 

        let effective_direction = picked
            .ok_or_else(|| Error::MoveUnavailable {
                attempted_move: direction,
                available_moves: active_cell.open_walls.clone(),
            })?;

        // this is the actual move made successfully, should be returned (used to programmatically backtrack Delta mazes, for example)
        let actual_move = effective_direction.clone();

        // Optional: Verify that the effective direction is valid.
        if !active_cell.open_walls.contains(&effective_direction) {
            return Err(Error::MoveUnavailable {
                attempted_move: effective_direction.clone(),
                available_moves: active_cell.open_walls.clone(),
            });
        }

        // Get the neighbor coordinate based on the effective direction.
        let neighbor_coords = *active_cell.neighbors_by_direction.get(&effective_direction)
            .ok_or(Error::InvalidDirection { direction: effective_direction.to_string() })?;

        // Determine whether this move is a backtracking move by checking if the neighbor is already visited.
        let going_back: bool;
        {
            // Mutably borrow the next cell.
            let next_cell = self.get_mut(neighbor_coords)?;
            going_back = next_cell.is_visited;  // If already visited, then we're going backward.
            if !going_back {
                // For a forward move: mark the new cell as visited.
                next_cell.set_visited(true);
            }
            // Mark the new cell as active.
            next_cell.set_active(true);
        }
        {
            // Now handle the previously active cell.
            let previous_cell = self.get_mut(original_coords)?;
            if going_back {
                // For a backtracking move: unvisit the cell that we are leaving.
                previous_cell.set_visited(false);
            }
            // Mark the previous cell as no longer active.
            previous_cell.set_active(false);
        }
        Ok(actual_move)
    }

    /// Retrieve a cell by its coordinates
    pub fn get_by_coords(&self, x: usize, y: usize) -> Result<&Cell, Error> {
        self.get(Coordinates { x: x, y: y })
    }
    
    /// Retrieve a mutable cell by its coordinates
    pub fn get_mut_by_coords(&mut self, x: usize, y: usize) -> Result<&mut Cell, Error> {
        self.get_mut(Coordinates { x: x, y: y })
    }

    /// Set a particular cell in the grid
    pub fn set(&mut self, cell: Cell) -> Result<(), Error> {
        if cell.x() >= self.width || cell.y() >= self.height {
            return Err(Error::OutOfBoundsCoordinates { coordinates: cell.coords, maze_width: self.width, maze_height: self.height } );
        }
        let index = self.get_flattened_index(cell.x(), cell.y());
        self.cells[index] = cell;
        Ok(())
    }

    /// Random unsigned integer within bounds of an upper boundary
    pub fn bounded_random_usize(&mut self, upper_bound: usize) -> usize {
        let mut rng = thread_rng();
        let seed= rng.gen_range(0..upper_bound);
        self.seed = seed as u64;
        return seed;
    }

    /// Random boolean
    pub fn random_bool(&mut self) -> bool {
        let rando: bool = self.bounded_random_usize(1000000) % 2 == 0;
        return rando;
    }
 
    /// Transform 1D (flattened) cells into a unflattened 2D vector
    pub fn unflatten(&self) -> Vec<Vec<Cell>> {
        self.cells
            .chunks(self.width) // split into row-sized slices
            .map(|chunk| chunk.to_vec()) // convert row slices to Vec<Cell>
            .collect()
    }

    /// Prepare grid for Delta maze type by initialzing cells as triangular cells (e.g. having some cells as Inverted)
    pub fn initialize_triangle_cells(&mut self) -> Result<(), Error> {
        if self.maze_type != MazeType::Delta {
            return Err(Error::InvalidCellForNonDeltaMaze { cell_maze_type: self.maze_type } );
        }
        let mut row_starts_with_upright = true;

        let triangle_orientation = |upward: bool| {
            if upward {
                CellOrientation::Normal
            } else {
                CellOrientation::Inverted
            }
        };

        for row in 0..self.height {
            let mut upright = !row_starts_with_upright;
            row_starts_with_upright = !row_starts_with_upright;

            for col in 0..self.width {
                upright = !upright;
                let coords = Coordinates { x: col, y: row };
                let is_start = coords == self.start_coords;
                let is_goal = coords == self.goal_coords;
                let cell: Cell = CellBuilder::new(
                    col, 
                    row, 
                    self.maze_type
                )
                .is_start(is_start)
                .is_goal(is_goal)
                .is_active(is_start) // start cell is cell user starts on (so, is active)
                .is_visited(is_start) // start cell is cell user starts on (so, is also visited)
                .has_been_visited(is_start) // start cell is cell user starts on (so, is also visited)
                .orientation(triangle_orientation(upright))
                .build();

                self.set(cell)?;
            }
        }
        Ok(())
    }
    
    /// Prepare grid for non-Delta maze type by initialzing cells as non-triangular (e.g. do not have any Inverted)
    pub fn initialize_non_triangle_cells(&mut self) -> Result<(), Error> {
        if self.maze_type == MazeType::Delta {
            return Err(Error::InvalidCellForDeltaMaze { cell_maze_type: self.maze_type });
        }
        let grid_width = self.width;
        let grid_height = self.height; 
        (0..grid_height)
            .flat_map(|row| (0..grid_width).map(move |col| (row, col))) // Combine row and column
            .for_each(|(row, col)| { 
                let coords = Coordinates { x: col, y: row };
                let is_start = coords == self.start_coords;
                let is_goal = coords == self.goal_coords;
                let cell: Cell = CellBuilder::new(
                    col, 
                    row, 
                    self.maze_type
                )
                .is_start(is_start)
                .is_goal(is_goal)
                .is_active(is_start) // start cell is cell user starts on (so, is active)
                .is_visited(is_start) // start cell is cell user starts on (so, is also visited)
                .has_been_visited(is_start) // start cell is cell user starts on (so, is also visited)
                .build();
    
                // Calculate the index in the 1D vector
                let index = self.get_flattened_index(col, row);
                
                // Set the cell in the flattened vector
                self.cells[index] = cell;
            });
    
        Ok(())
    }
    

    /// Create a new grid based on the maze type, dimensions, start, and goal.
    pub fn new(
        maze_type: MazeType, 
        width: usize, 
        height: usize, 
        start: Coordinates, 
        goal: Coordinates,
        capture_steps: bool,
    ) -> Result<Self, Error> {
        if capture_steps && (width > 100 || height > 100) {
            return Err(Error::GridDimensionsExceedLimitForCaptureSteps{width: width, height: height});
        }

        let seed = Self::generate_seed(width, height);

        // Initialize the grid with a flattened vector of cells using CellBuilder.
        let mut grid = Grid { 
            width, 
            height, 
            maze_type,
            cells: vec![CellBuilder::new(0, 0, maze_type).build(); width * height],
            seed, 
            start_coords: start, 
            goal_coords: goal,
            capture_steps: capture_steps,
            generation_steps: if capture_steps { Some(Vec::new()) } else { None }, 
        };

        // Generate different types of cells based on maze_type.
        match maze_type {
            MazeType::Delta => grid.initialize_triangle_cells()?,
            _             => grid.initialize_non_triangle_cells()?,
        };

        // Assign neighbor information based on maze type.
        grid.assign_neighbors()?;

        Ok(grid)
    }

    /// Generate a seed based on the grid dimensions.
    fn generate_seed(width: usize, height: usize) -> u64 {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        rng.gen_range(0..(width * height + 1)) as u64
    }

    /// Assign neighbor relationships for each cell based on the maze type.
    fn assign_neighbors(&mut self) -> Result<(), Error> {
        match self.maze_type {
            MazeType::Orthogonal => self.assign_neighbors_orthogonal(),
            MazeType::Delta      => self.assign_neighbors_delta(),
            MazeType::Sigma      => self.assign_neighbors_sigma(),
            MazeType::Rhombille => self.assign_neighbors_rhombille(),
        }
    }

    /// Assign neighbors for Orthogonal mazes.
    fn assign_neighbors_orthogonal(&mut self) -> Result<(), Error> {
        for row in 0..self.height {
            for col in 0..self.width {
                let mut cell = self.get_mut_by_coords(col, row)?.clone();
                let mut neighbors: HashMap<Direction, Coordinates> = HashMap::new();

                if cell.y() != 0 {
                    neighbors.insert(
                        Direction::Up, 
                        self.get_by_coords(cell.x(), cell.y() - 1)?.coords
                    );
                }
                if cell.x() < self.width - 1 {
                    neighbors.insert(
                        Direction::Right, 
                        self.get_by_coords(cell.x() + 1, cell.y())?.coords
                    );
                }
                if cell.y() < self.height - 1 {
                    neighbors.insert(
                        Direction::Down, 
                        self.get_by_coords(cell.x(), cell.y() + 1)?.coords
                    );
                }
                if cell.x() != 0 {
                    neighbors.insert(
                        Direction::Left, 
                        self.get_by_coords(cell.x() - 1, cell.y())?.coords
                    );
                }
                cell.set_neighbors(neighbors);
                self.set(cell)?;
            }
        }
        Ok(())
    }

    /// Assigns neighbors for Delta mazes.
    fn assign_neighbors_delta(&mut self) -> Result<(), Error> {
        for row in 0..self.height {
            for col in 0..self.width {
                let mut cell = self.get_mut_by_coords(col, row)?.clone();
                let mut neighbors: HashMap<Direction, Coordinates> = HashMap::new();
                
                // Left and right neighbors
                let left  = if col > 0 { Some(Coordinates { x: col - 1, y: row }) } else { None };
                let right = if col < self.width - 1 { Some(Coordinates { x: col + 1, y: row }) } else { None };
                
                if let Some(left_coords) = left {
                    let key = if cell.orientation == CellOrientation::Normal {
                        Direction::UpperLeft
                    } else {
                        Direction::LowerLeft
                    };
                    neighbors.insert(key, left_coords);
                }
                if let Some(right_coords) = right {
                    let key = if cell.orientation == CellOrientation::Normal {
                        Direction::UpperRight
                    } else {
                        Direction::LowerRight
                    };
                    neighbors.insert(key, right_coords);
                }
                
                // Up and down neighbors based on orientation.
                let up = if cell.orientation == CellOrientation::Inverted && row > 0 { 
                    Some(Coordinates { x: col, y: row - 1 })
                } else { 
                    None 
                };
                let down = if cell.orientation == CellOrientation::Normal && row < self.height - 1 {
                    Some(Coordinates { x: col, y: row + 1 })
                } else {
                    None
                };
                if let Some(up_coords) = up {
                    neighbors.insert(Direction::Up, up_coords);
                }
                if let Some(down_coords) = down {
                    neighbors.insert(Direction::Down, down_coords);
                }
                cell.set_neighbors(neighbors);
                self.set(cell)?;
            }
        }
        Ok(())
    }

    /// Assign neighbors for Sigma (hexagonal) mazes.
    fn assign_neighbors_sigma(&mut self) -> Result<(), Error> {
        // The helper function below determines whether a value is even.
        fn is_even(value: usize) -> bool { value % 2 == 0 }
        
        for row in 0..self.height {
            for col in 0..self.width {
                let mut cell = self.get_mut_by_coords(col, row)?.clone();
                let mut neighbors: HashMap<Direction, Coordinates> = HashMap::new();

                let (north_diagonal, south_diagonal) = match is_even(col) {
                    true if row > 0 => (row - 1, row),
                    true => (0, row), // Clamps to avoid underflow
                    false if row < self.height - 1 => (row, row + 1),
                    false => (row, self.height - 1), // Clamps to avoid out-of-bound
                };
                if col > 0 && north_diagonal < self.height {
                    neighbors.insert(
                        Direction::UpperLeft,
                        self.get_by_coords(col - 1, north_diagonal)?.coords,
                    );
                }
                if col < self.width && row > 0 {
                    neighbors.insert(
                        Direction::Up,
                        self.get_by_coords(col, row - 1)?.coords,
                    );
                }
                if col < self.width - 1 && north_diagonal < self.height {
                    neighbors.insert(
                        Direction::UpperRight,
                        self.get_by_coords(col + 1, north_diagonal)?.coords,
                    );
                }
                if col > 0 && south_diagonal < self.height {
                    neighbors.insert(
                        Direction::LowerLeft,
                        self.get_by_coords(col - 1, south_diagonal)?.coords,
                    );
                }
                if row < self.height - 1 && col < self.width {
                    neighbors.insert(
                        Direction::Down,
                        self.get_by_coords(col, row + 1)?.coords,
                    );
                }
                if col < self.width - 1 && south_diagonal < self.height {
                    neighbors.insert(
                        Direction::LowerRight,
                        self.get_by_coords(col + 1, south_diagonal)?.coords,
                    );
                }
                cell.set_neighbors(neighbors);
                self.set(cell)?;
            }
        }
        Ok(())
    }

    /// Assign neighbors for Rhombille mazes.
    fn assign_neighbors_rhombille(&mut self) -> Result<(), Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut cell = self.get_mut(Coordinates { x, y })?.clone();
                let mut neighbors = HashMap::new();
                if y > 0 {
                    neighbors.insert(Direction::UpperRight, Coordinates { x, y: y - 1 });
                }
                if x < self.width - 1 {
                    neighbors.insert(Direction::LowerRight, Coordinates { x: x + 1, y });
                }
                if y < self.height - 1 {
                    neighbors.insert(Direction::LowerLeft, Coordinates { x, y: y + 1 });
                }
                if x > 0 {
                    neighbors.insert(Direction::UpperLeft, Coordinates { x: x - 1, y });
                }
                cell.set_neighbors(neighbors);
                self.set(cell)?;
            }
        }
        Ok(())
    }

    /// Link two cells together by their coordinates.
    pub fn link(&mut self, coord1: Coordinates, coord2: Coordinates) -> Result<(), Error> {
        let (row1, col1) = (coord1.y, coord1.x);
        let (row2, col2) = (coord2.y, coord2.x);

        // Link cell at coord1 to cell at coord2 and update open_walls.
        {
            let cell1 = self.get_mut_by_coords(col1, row1)?;
            cell1.linked.insert(coord2);
            cell1.set_open_walls();
        }
        // Link cell at coord2 to cell at coord1 and update open_walls.
        {
            let cell2 = self.get_mut_by_coords(col2, row2)?;
            cell2.linked.insert(coord1);
            cell2.set_open_walls();
        }
        Ok(())
    }

    /// Unlink two cells by their coordinates, removing the connection between them.
    pub fn unlink(&mut self, coord1: Coordinates, coord2: Coordinates) -> Result<(), Error> {
        let (row1, col1) = (coord1.y, coord1.x);
        let (row2, col2) = (coord2.y, coord2.x);

        // Unlink cell at coord1 from cell at coord2 and update open_walls.
        {
            let cell1 = self.get_mut_by_coords(col1, row1)?;
            cell1.linked.remove(&coord2);
            cell1.set_open_walls();
        }
        // Unlink cell at coord2 from cell at coord1 and update open_walls.
        {
            let cell2 = self.get_mut_by_coords(col2, row2)?;
            cell2.linked.remove(&coord1);
            cell2.set_open_walls();
        }
        Ok(())
    }

    // /// Link two cells together by their coordinates.
    // pub fn link(&mut self, coord1: Coordinates, coord2: Coordinates) -> Result<(), Error> {
    //     let (row1, col1) = (coord1.y, coord1.x);
    //     let (row2, col2) = (coord2.y, coord2.x);

    //     // Link cell at coord1 to cell at coord2.
    //     {
    //         let cell1 = self.get_mut_by_coords(col1, row1)?;
    //         cell1.linked.insert(coord2);
    //     }
    //     // Link cell at coord2 to cell at coord1.
    //     {
    //         let cell2 = self.get_mut_by_coords(col2, row2)?;
    //         cell2.linked.insert(coord1);
    //     }
    //     Ok(())
    // }

    // /// Unlink two cells by their coordinates, removing the connection between them.
    // pub fn unlink(&mut self, coord1: Coordinates, coord2: Coordinates) -> Result<(), Error> {
    //     let (row1, col1) = (coord1.y, coord1.x);
    //     let (row2, col2) = (coord2.y, coord2.x);

    //     // Unlink cell at coord1 from cell at coord2.
    //     {
    //         let cell1 = self.get_mut_by_coords(col1, row1)?;
    //         cell1.linked.remove(&coord2);
    //     }
    //     // Unlink cell at coord2 from cell at coord1.
    //     {
    //         let cell2 = self.get_mut_by_coords(col2, row2)?;
    //         cell2.linked.remove(&coord1);
    //     }
    //     Ok(())
    // }

    /// Get a map of distances from the start coordinate to all other connected coordinates.
    pub fn distances(&self, start: Coordinates) -> HashMap<Coordinates, u32> {
        // Define a closure that returns the linked (neighbor) coordinates for a given coordinate.
        let neighbor_fn = |coords: Coordinates| -> Vec<Coordinates> {
            // Retrieve the cell at `coords`
            if let Ok(cell) = self.get(coords) {
                // Return its linked neighbors (assuming cell.linked is a HashSet<Coordinates>).
                cell.linked.iter().copied().collect()
            } else {
                Vec::new()
            }
        };

        graph::bfs_distances(start, neighbor_fn)
    }    

    /// Compute a path from the given start coordinates to the goal coordinates within the maze grid.
    /// 
    /// The method first calculates the distance from the start cell to all accessible cells, defines
    /// linked neighbors for each cell, and then uses a generic graph pathfinder to determine a valid path.
    /// It returns a `HashMap` mapping each coordinate along the found path to its distance from the start.
    /// If no path exists, an empty map is returned.
    pub fn get_path_to(
        &self,
        start_x: usize,
        start_y: usize,
        goal_x: usize,
        goal_y: usize,
    ) -> Result<HashMap<Coordinates, u32>, Error> {
        let start = Coordinates { x: start_x, y: start_y };
        let goal = Coordinates { x: goal_x, y: goal_y };

        // Compute distances from start using your existing method.
        let distances = self.distances(start);

        // Define the neighbor function inline.
        // Given a coordinate, return its linked neighbors (or an empty vec on error).
        let neighbor_fn = |coords: Coordinates| -> Vec<Coordinates> {
            self.get(coords)
                .map(|cell| cell.linked.iter().copied().collect())
                .unwrap_or_else(|_| Vec::new())
        };

        // Use the generic get_path function to obtain the path from start to goal.
        if let Some(path) = graph::get_path(start, goal, &distances, neighbor_fn) {
            // Convert the path (Vec<Coordinates>) into a breadcrumbs map.
            // Each coordinate is mapped to its distance (as computed in the distances map).
            let breadcrumbs: HashMap<Coordinates, u32> = path
                .into_iter()
                .filter_map(|coord| distances.get(&coord).map(|&d| (coord, d)))
                .collect();
            Ok(breadcrumbs)
        } else {
            // If no path was found, return an empty map.
            Ok(HashMap::new())
        }
    }

    /// Return all cells reachable from the given start coordinates
    /// Get all connected cells from a starting coordinate.
    pub fn all_connected_cells(&self, start: Coordinates) -> HashSet<Coordinates> {
        let neighbor_fn = |coords: Coordinates| -> Vec<Coordinates> {
            if let Ok(cell) = self.get(coords) {
                cell.linked.iter().copied().collect()
            } else {
                Vec::new()
            }
        };

        graph::all_connected(start, neighbor_fn)
    }
    

    /// Count the number of edges in the maze
    pub fn count_edges(&self) -> usize {
        self.cells
            .iter()
            .map(|cell| cell.linked.len()) // For each cell, get the number of linked cells
            .sum::<usize>() // Sum the number of edges
        / 2 // Each edge is stored twice (once for each linked cell)
    }

    /// Whether the maze is perfect
    pub fn is_perfect_maze(&self) -> Result<bool, Error> {
        // Total number of cells
        let total_cells = self.width * self.height;

        // Fully connected check
        let start_coords = self.start_coords;
        //let connected_cells = self.all_connected_cells(&start_coords)?;
        let connected_cells = self.all_connected_cells(start_coords);
        if connected_cells.len() != total_cells {
            return Ok(false);
        }

        // Tree check (no cycles)
        let total_edges = self.count_edges();
        Ok(total_edges == total_cells - 1)
    }

    /// ASCI display, only applicable to Orthogonal (square cell) mazes
    pub fn to_asci(&self) -> String {
        assert!(self.maze_type == MazeType::Orthogonal, "Rejecting displaying ASCI for MazeType {}! ASCI display behavior is only applicable to the Orthogonal MazeType", self.maze_type.to_string());
        let mut output = format!("+{}\n", "---+".repeat(self.width)); 
        let unflattened: Vec<Vec<Cell>> = self.unflatten(); 
        for row in unflattened {
            let mut top =String::from( "|");
            let mut bottom = String::from("+");
            for cell in row {
                let body = "   ";
                let east_boundary = match cell.neighbors_by_direction.get(&Direction::Right).is_some() {
                    true if cell.is_linked_direction(Direction::Right) => " ",
                    _ => "|",
                };
                top.push_str(body);
                top.push_str(east_boundary);
                let south_boundary = match cell.neighbors_by_direction.get(&Direction::Down).is_some() {
                    true if cell.is_linked_direction(Direction::Down) => "   ",
                    _ => "---"
                };
                let corner ="+";
                bottom.push_str(south_boundary);
                bottom.push_str(corner);
            }
            output.push_str(top.as_str());
            output.push_str("\n");
            output.push_str(bottom.as_str());
            output.push_str("\n");
        }
        return output;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::hunt_and_kill::HuntAndKill;
    use crate::behaviors::maze::MazeGeneration;

    #[test]
    fn init_orthogonal_grid() {
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates{x:0, y:0}, Coordinates{x:3, y:3}, false) {
            Ok(grid) => {
                assert!(grid.cells.len() != 0);
                assert!(grid.cells.len() == 4 * 4);
                println!("\n\n{}", grid.to_string());
                println!("\n\n{}\n\n", grid.to_asci());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn get_grid_cells_by_coordinates() {
        match Grid::new(
            MazeType::Orthogonal,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
            false,
        ) {
            Ok(grid) => {
                let cell1 = grid.get(Coordinates { x: 0, y: 0 }).unwrap();
                let cell2 = grid.get(Coordinates { x: 0, y: 1 }).unwrap();
                let cell3 = grid.get(Coordinates { x: 1, y: 1 }).unwrap();
                let cell4 = grid.get(Coordinates { x: 1, y: 2 }).unwrap();
                let cell5 = grid.get(Coordinates { x: 2, y: 2 }).unwrap();
                let cell6 = grid.get(Coordinates { x: 2, y: 3 }).unwrap();
                let cell7 = grid.get(Coordinates { x: 3, y: 3 }).unwrap();
                assert!(cell1.coords.x == 0);
                assert!(cell1.coords.y == 0);
                assert!(cell2.coords.x == 0);
                assert!(cell2.coords.y == 1);
                assert!(cell3.coords.x == 1);
                assert!(cell3.coords.y == 1);
                assert!(cell4.coords.x == 1);
                assert!(cell4.coords.y == 2);
                assert!(cell5.coords.x == 2);
                assert!(cell5.coords.y == 2);
                assert!(cell6.coords.x == 2);
                assert!(cell6.coords.y == 3);
                assert!(cell7.coords.x == 3);
                assert!(cell7.coords.y == 3);
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn link_cells_in_orthogonal_grid() {
        match Grid::new(
            MazeType::Orthogonal,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
            false,
        ) {
            Ok(mut grid) => {
                let cell1 = grid.get_by_coords(0, 0).unwrap().coords;
                let cell2 = grid.get_by_coords(0, 1).unwrap().coords;
                let cell3 = grid.get_by_coords(1, 1).unwrap().coords;
                let cell4 = grid.get_by_coords(1, 2).unwrap().coords;
                let cell5 = grid.get_by_coords(2, 2).unwrap().coords;
                let cell6 = grid.get_by_coords(2, 3).unwrap().coords;
                let cell7 = grid.get_by_coords(3, 3).unwrap().coords;

                grid.link(cell1, cell2).unwrap();
                grid.link(cell2, cell3).unwrap();
                grid.link(cell3, cell4).unwrap();
                grid.link(cell4, cell5).unwrap();
                grid.link(cell5, cell6).unwrap();
                grid.link(cell6, cell7).unwrap();
                // many cells are walled-off and unreachable, not a perfect maze 
                assert!(!grid.is_perfect_maze().unwrap());
                println!("\n\n{}\n\n", grid.to_asci());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn unlink_cells_in_orthogonal_grid() {
        match Grid::new(
            MazeType::Orthogonal,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
            false,
        ) {
            Ok(mut grid) => {
                let cell1 = grid.get_by_coords(0, 0).unwrap().coords;
                let cell2 = grid.get_by_coords(0, 1).unwrap().coords;

                // Link cells
                grid.link(cell1, cell2).unwrap();
                assert!(grid.get(cell1).unwrap().linked.contains(&cell2));
                assert!(grid.get(cell2).unwrap().linked.contains(&cell1));

                // Unlink cells
                grid.unlink(cell1, cell2).unwrap();
                assert!(!grid.get(cell1).unwrap().linked.contains(&cell2));
                assert!(!grid.get(cell2).unwrap().linked.contains(&cell1));
                assert!(!grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn unlink_multiple_cells_in_orthogonal_grid() {
        match Grid::new(
            MazeType::Orthogonal,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
            false,
        ) {
            Ok(mut grid) => {
                let cell1 = grid.get_by_coords(0, 0).unwrap().coords;
                let cell2 = grid.get_by_coords(0, 1).unwrap().coords;
                let cell3 = grid.get_by_coords(1, 1).unwrap().coords;
                let cell4 = grid.get_by_coords(1, 2).unwrap().coords;

                // Link cells in a chain
                grid.link(cell1, cell2).unwrap();
                grid.link(cell2, cell3).unwrap();
                grid.link(cell3, cell4).unwrap();
                assert_eq!(grid.count_edges(), 3);

                // Unlink one pair
                grid.unlink(cell2, cell3).unwrap();
                assert!(grid.get(cell1).unwrap().linked.contains(&cell2));
                assert!(!grid.get(cell2).unwrap().linked.contains(&cell3));
                assert!(!grid.get(cell3).unwrap().linked.contains(&cell2));
                assert!(grid.get(cell3).unwrap().linked.contains(&cell4));
                assert_eq!(grid.count_edges(), 2);
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn determine_distances_to_goal() {
        match Grid::new(
            MazeType::Orthogonal,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
            false,
        ) {
            Ok(mut grid) => {
                let cell1 = grid.get_by_coords(0, 0).unwrap().coords;
                let cell2 = grid.get_by_coords(0, 1).unwrap().coords;
                let cell3 = grid.get_by_coords(1, 1).unwrap().coords;
                let cell4 = grid.get_by_coords(1, 2).unwrap().coords;
                let cell5 = grid.get_by_coords(2, 2).unwrap().coords;
                let cell6 = grid.get_by_coords(2, 3).unwrap().coords;
                let cell7 = grid.get_by_coords(3, 3).unwrap().coords;
                
                grid.link(cell1, cell2).unwrap();
                grid.link(cell2, cell3).unwrap();
                grid.link(cell3, cell4).unwrap();
                grid.link(cell4, cell5).unwrap();
                grid.link(cell5, cell6).unwrap();
                grid.link(cell6, cell7).unwrap();

                let distances = grid.distances(Coordinates{ x: 0, y: 0} );

                for (coords, distance) in &distances {
                    println!("Distance to {:?}: {}", coords, distance);
                }
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn test_flatten_and_unflatten() {
        match Grid::new(
            MazeType::Orthogonal,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
            false,
        ) {
            Ok(grid) => {
                let initial_cells = grid.cells.clone();

                // Unflatten the grid
                grid.unflatten();

                // Check that the cells after unflattening match the original
                assert_eq!(grid.cells, initial_cells);
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }

    #[test]
    fn test_perfect_maze_detection() {
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                assert!(!grid.is_perfect_maze().unwrap());
                let _ = grid.link(grid.get_by_coords(0, 0).unwrap().coords, grid.get_by_coords(1, 0).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(1, 0).unwrap().coords, grid.get_by_coords(2, 0).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(2, 0).unwrap().coords, grid.get_by_coords(3, 0).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(3, 0).unwrap().coords, grid.get_by_coords(3, 1).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(3, 1).unwrap().coords, grid.get_by_coords(2, 1).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(2,1).unwrap().coords, grid.get_by_coords(1, 1).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(1,1).unwrap().coords, grid.get_by_coords(0, 1).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(0,1).unwrap().coords, grid.get_by_coords(0, 2).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(0,2).unwrap().coords, grid.get_by_coords(1, 2).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(1,2).unwrap().coords, grid.get_by_coords(2, 2).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(2,2).unwrap().coords, grid.get_by_coords(3, 2).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(3,2).unwrap().coords, grid.get_by_coords(3, 3).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(3,3).unwrap().coords, grid.get_by_coords(2, 3).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(2,3).unwrap().coords, grid.get_by_coords(1, 3).unwrap().coords);
                assert!(!grid.is_perfect_maze().unwrap()); // not perfect
                let _ = grid.link(grid.get_by_coords(1,3).unwrap().coords, grid.get_by_coords(0, 3).unwrap().coords);
                // now it's a perfect maze: only a single path exists for any 2 cells in the maze and there are no unreachable groups of cells
                assert!(grid.is_perfect_maze().unwrap());
                let _ = grid.link(grid.get_by_coords(0,3).unwrap().coords, grid.get_by_coords(0, 2).unwrap().coords);
                // now it's no longer a perfect maze because some cells can reach each other on multiple paths 
                assert!(!grid.is_perfect_maze().unwrap());
            }
            Err(e) => panic!("Unexpected error occurred running Grid test test_perfect_maze_detection: {:?}", e),
        }
    }

    #[test]
    fn test_get_path_to() {
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }, false) {
            Ok(mut grid) => {
                let _ = grid.link(Coordinates { x: 0, y: 0 }, Coordinates { x: 0, y: 1 });
                let _ = grid.link(Coordinates { x: 0, y: 1 }, Coordinates { x: 1, y: 1 });
                let _ = grid.link(Coordinates { x: 1, y: 1 }, Coordinates { x: 1, y: 2 });
                let _ = grid.link(Coordinates { x: 1, y: 2 }, Coordinates { x: 2, y: 2 });
                match grid.get_path_to(0, 0, 2, 2) {
                    Ok(path) => {
                        assert_eq!(path.len(), 5);
                        assert!(path.contains_key(&Coordinates { x: 0, y: 0 }));
                        assert!(path.contains_key(&Coordinates { x: 2, y: 2 }));
                        assert_eq!(path[&Coordinates { x: 0, y: 0 }], 0);
                    }
                    Err(e) => panic!("Unexpected error occurred running Grid test test_get_path_to: {:?}", e),
                }  
            }
            Err(e) => panic!("Unexpected error occurred running Grid test test_get_path_to: {:?}", e),
        }
    }

    #[test]
    fn test_recursive_backtracker_orthogonal_12_x_12_maze_generation_from_json() {
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
            Ok(maze) => {
                assert!(maze.is_perfect_maze().unwrap());
                println!("\n\nRecursive Backtracker\n\n{}\n\n", maze.to_asci());

                let nonzero_count = maze
                    .cells
                    .iter()
                    .filter(|cell| cell.distance > 0)
                    .count();
                assert!(
                    nonzero_count > 0,
                    "Expected some cells to have non-zero distance, but all were 0"
                );

                let solution_path_count = maze
                    .cells
                    .iter()
                    .filter(|cell| cell.on_solution_path)
                    .count();
                assert!(
                    solution_path_count > 0,
                    "Expected some cells to be on the solution path, but none were"
                );
                assert!(
                    solution_path_count < maze.cells.len(),
                    "All cells are marked on the solution path — expected only a subset"
                );
            }
            Err(e) => panic!("Unexpected error running test: {:?}", e),
        }
    }


    #[test]
    fn render_maze_default_start_and_goal() {
        let json = format!(r#"
        {{
            "maze_type": "Orthogonal",
            "width": 12,
            "height": 12,
            "algorithm": "AldousBroder"
        }}
        "#);

        let maze = Grid::try_from(json)
            .expect("Unexpected error constructing maze");
        
        assert!(maze.is_perfect_maze().unwrap());
        assert!(maze.goal_coords == Coordinates{ x: maze.width / 2, y: 0 });
        assert!(maze.start_coords == Coordinates{ x: maze.width / 2, y: maze.height - 1 });
    }

    fn run_make_move_orthogonal_test(algorithm: &str) {
        let json = format!(r#"
        {{
            "maze_type": "Orthogonal",
            "width": 12,
            "height": 12,
            "algorithm": "{algorithm}",
            "start": {{ "x": 0, "y": 0 }},
            "goal":  {{ "x": 11, "y": 11 }}
        }}
        "#);

        let mut maze = Grid::try_from(json)
            .expect("Unexpected error constructing maze");
        assert!(maze.is_perfect_maze().unwrap());

        // --- initial counts ---
        assert_eq!(
            maze.cells.iter().filter(|c| c.is_visited).count(),
            1,
            "1 visited cell at start"
        );
        assert_eq!(
            maze.cells.iter().filter(|c| c.has_been_visited).count(),
            1,
            "1 permanently visited cell at start"
        );

        // pull out start‐cell info
        let (original_coords, available_moves, unavailable_moves) = {
            // 1) Temporarily borrow mutably just to pull out coords, open_walls, and maze_type
            let (orig, open_walls) = {
                let c = maze
                    .get_active_cell()
                    .expect("Expected start cell");
                (c.coords.clone(), c.open_walls.clone())
            }; // ← `c` (and its &mut borrow) drops here
        
            // 2) available_moves is just the cloned open_walls
            let available_moves: Vec<Direction> = open_walls.into_iter().collect();
        
            // 3) Compute unavailable = all possible minus available
            let unavailable_moves: Vec<Direction> = maze
                .all_moves()           // &'static [Direction]
                .iter()
                .filter(|d| !available_moves.contains(d))
                .cloned()
                .collect();
        
            (orig, available_moves, unavailable_moves)
        };


        // unavailable move must error, and counts stay the same
        {
            let mut copy = maze.clone();
            let bad = &unavailable_moves[0];
            assert!(copy.make_move(*bad).is_err(), "Unavailable move `{}` should fail", bad);
            assert_eq!(
                copy.cells.iter().filter(|c| c.is_visited).count(),
                1,
                "Visited count unchanged after bad move"
            );
            assert_eq!(
                copy.cells.iter().filter(|c| c.has_been_visited).count(),
                1,
                "Permanent‐visited count unchanged after bad move"
            );
        }

        // ================================
        // STEP 1: first valid move
        // ================================
        let mv1 = available_moves.iter().next().unwrap();
        assert!(maze.make_move(*mv1).is_ok(), "Valid move `{}` should succeed", mv1);

        // after first move
        assert_eq!(
            maze.cells.iter().filter(|c| c.is_active).count(),
            1,
            "Exactly one active cell after first move"
        );
        assert_eq!(
            maze.cells.iter().filter(|c| c.is_visited).count(),
            2,
            "Two visited cells after first move"
        );
        assert_eq!(
            maze.cells.iter().filter(|c| c.has_been_visited).count(),
            2,
            "Two permanent‐visited cells after first move"
        );

        let cell1_coords = maze.get_active_cell()
            .expect("Expected active cell after first move")
            .coords
            .clone();

        // helper to reverse orthogonal directions
        let reverse_direction = |dir: Direction| -> Direction {
            match dir {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Right  => Direction::Left,
                Direction::Left  => Direction::Right,
                other   => panic!("Unknown direction: {}", other),
            }
        };

        // ================================
        // STEP 2: backtrack to start
        // ================================
        let back1 = reverse_direction(*mv1);
        assert!(maze.make_move(back1).is_ok(), "Backtracking `{}` should succeed", back1);

        // after backtrack
        // exactly one active (the start)
        assert_eq!(
            maze.cells.iter().filter(|c| c.is_active).count(),
            1,
            "Exactly one active cell after backtrack"
        );
        let active = maze.get_active_cell().unwrap();
        assert_eq!(
            active.coords, original_coords,
            "Active cell after backtrack should be the start"
        );

        // dynamic‐visited count goes back to 1
        let visited_count = maze.cells.iter().filter(|c| c.is_visited).count();
        assert_eq!(visited_count, 1, "Visited count should drop back to 1 after backtrack");

        // permanent‐visited remains at 2
        let perm_count = maze.cells.iter().filter(|c| c.has_been_visited).count();
        assert_eq!(perm_count, 2, "Permanent‐visited count stays at 2 after backtrack");

        // ensure the cell we backtracked from is no longer visited but still permanent
        let back_cell = maze
            .cells
            .iter()
            .find(|c| c.coords == cell1_coords)
            .expect("Backtracked cell not found");
        assert!(!back_cell.is_visited, "Backtracked cell should no longer be marked visited");
        assert!(back_cell.has_been_visited, "Backtracked cell should keep has_been_visited flag");

        // ensure start cell remains visited and permanent
        let start_cell = maze
            .cells
            .iter()
            .find(|c| c.coords == original_coords)
            .unwrap();
        assert!(start_cell.is_visited, "Start cell should remain is_visited");
        assert!(start_cell.has_been_visited, "Start cell should remain has_been_visited");
    }

    fn run_make_move_delta_test(algorithm: &str) {
        let json = format!(r#"
        {{
            "maze_type": "Delta",
            "width": 12,
            "height": 12,
            "algorithm": "{algorithm}",
            "start": {{ "x": 0, "y": 0 }},
            "goal":  {{ "x": 11, "y": 11 }}
        }}
        "#);
    
        // grab the starting cell’s coords, open_walls, and the set of unavailable moves
        // 0) construct & verify perfectness
        let mut maze = Grid::try_from(json).unwrap();
        assert!(maze.is_perfect_maze().unwrap());

        // 1) Pull coords & open_walls in their own scope
        let (original_coords, _open_walls) = {
            let start = maze.get_active_cell()
                .expect("Expected active start cell");
            (start.coords.clone(), start.open_walls.clone())
        }; // <— `start` (and its &mut borrow) dies here

        // 2) Now it’s safe to borrow `maze` again, immutably, to get all moves
        let _unavailable_moves: Vec<Direction> = maze
            .all_moves()                    // &'static [Direction]
            .iter()
            .cloned()
            .filter(|d| {
                let mut copy = maze.clone();
                copy.make_move(*d).is_err()
            })
            .collect();

        // sanity: only one visited
        assert_eq!(maze.cells.iter().filter(|c| c.is_visited).count(), 1);
        assert_eq!(maze.cells.iter().filter(|c| c.has_been_visited).count(), 1);
    
        // unavailable move must error
        {
            let mut copy = maze.clone();
            let bad = maze.unavailable_moves()[0];
            assert!(copy.make_move(bad).is_err(), "Should not allow {:?}", bad);
        }
   
        // ===== 1) First forward move =====
        let initial_moves = maze.effective_moves(); 
        assert!(!initial_moves.is_empty(), "Expected at least 1 valid move");
        let requested1 = initial_moves[0];
        let actual1 = maze
            .make_move(requested1)
            .expect("First valid move should succeed");
        assert!(
            initial_moves.contains(&actual1),
            "Returned {:?} must be one of {:?}",
            actual1,
            initial_moves
        );
        let cell_after_first = maze.get_active_cell().unwrap().coords.clone();
        assert_ne!(cell_after_first, original_coords);
    
        // ===== 2–4) Try a second forward, then backtrack twice =====
        {
            // a) compute which wall would go *back* to the start
            let first_cell = maze.get_active_cell().unwrap();
            let back_to_start: Direction = first_cell
                .neighbors_by_direction
                .iter()
                .find_map(|(dir, &coords)| {
                    if coords == original_coords { Some(dir.clone()) } else { None }
                })
                .expect("Expected a reverse link back to the start cell");
    
            // b) pick *any* other open wall (if there is one)
            if let Some(requested2) = first_cell.open_walls.iter()
                .find(|dir| *dir != &back_to_start)
                .cloned()
            {
                // 2c) make the second forward move
                let _ = maze
                    .make_move(requested2)
                    .expect("Second valid move should succeed");
                let cell_after_second = maze.get_active_cell().unwrap().coords.clone();
                assert_ne!(cell_after_second, cell_after_first);
    
                // 3) backtrack from the second cell → first
                let second_cell = maze.get_active_cell().unwrap();
                let back2: Direction = second_cell
                    .neighbors_by_direction
                    .iter()
                    .find_map(|(dir, &coords)| {
                        if coords == cell_after_first { Some(dir.clone()) } else { None }
                    })
                    .expect("Expected a neighbor mapping back to the first cell");
                let actual_back2 = maze
                    .make_move(back2)
                    .expect("Backtracking from the second to the first should succeed");
                assert_eq!(actual_back2, back2);
                let cell_after_back = maze.get_active_cell().unwrap().coords.clone();
                assert_eq!(cell_after_back, cell_after_first);
    
                // 4) backtrack from the first cell → start
                let first_again = maze.get_active_cell().unwrap();
                let back1_again: Direction = first_again
                    .neighbors_by_direction
                    .iter()
                    .find_map(|(dir, &coords)| {
                        if coords == original_coords { Some(dir.clone()) } else { None }
                    })
                    .expect("Expected a neighbor mapping back to the start cell");
                let actual_back1 = maze
                    .make_move(back1_again)
                    .expect("Backtracking to the start cell should succeed");
                assert_eq!(actual_back1, back1_again);
                let cell_after_back_to_start = maze.get_active_cell().unwrap().coords.clone();
                assert_eq!(cell_after_back_to_start, original_coords);
    
                assert!(
                    maze.get(maze.start_coords).expect("error getting start coords").is_visited == true,
                    "Start cell should remain visited"
                );
            } else {
                println!("Not enough available moves for a second forward move; skipping Delta backtracking tests.");
            }
        }
    }

    fn run_make_move_rhombille_test(algorithm: &str) {
        let json = format!(r#"
        {{
            "maze_type": "Rhombille",
            "width": 3,
            "height": 3,
            "algorithm": "{algorithm}",
            "start": {{ "x": 1, "y": 1 }},
            "goal":  {{ "x": 2, "y": 2 }}
        }}
        "#);

        let mut maze = Grid::try_from(json).expect("Failed to create Rhombille maze");
        assert!(maze.is_perfect_maze().unwrap(), "Maze should be perfect");

        // Initial state
        let original_coords = maze.start_coords;
        let active_cell = maze.get_active_cell().unwrap();
        assert_eq!(active_cell.coords, original_coords, "Active cell should be at start");
        assert!(active_cell.is_visited, "Start cell should be visited");
        assert!(active_cell.has_been_visited, "Start cell should have been visited");
        let available_moves = active_cell.open_walls.clone();

        // Test 1: Valid Move
        if let Some(&direction) = available_moves.iter().find(|&&d| d == Direction::UpperRight) {
            let result = maze.make_move(direction);
            assert!(result.is_ok(), "Valid move UpperRight should succeed");
            let new_active = maze.get_active_cell().unwrap();
            assert_eq!(new_active.coords, Coordinates { x: 1, y: 0 }, "Should move up to (1,0)");
            assert!(new_active.is_visited, "New cell should be visited");
            assert!(new_active.has_been_visited, "New cell should have been visited");
            assert_eq!(maze.cells.iter().filter(|c| c.is_visited).count(), 2, "Two cells visited");
        } else {
            panic!("UpperRight should be available from (1,1)");
        }

        // Test 2: Backtracking
        let current_cell = maze.get_active_cell().unwrap();
        let back_direction = current_cell.neighbors_by_direction.iter()
            .find_map(|(dir, &coords)| if coords == original_coords { Some(*dir) } else { None })
            .expect("Should have a direction back to start");
        let result = maze.make_move(back_direction);
        assert!(result.is_ok(), "Backtracking should succeed");
        let active_after_back = maze.get_active_cell().unwrap();
        assert_eq!(active_after_back.coords, original_coords, "Should return to start");
        assert!(active_after_back.is_visited, "Start should remain visited");
        assert!(active_after_back.has_been_visited, "Start should remain has_been_visited");
        let backtracked_cell = maze.get(Coordinates { x: 1, y: 0 }).unwrap();
        assert!(!backtracked_cell.is_visited, "Backtracked cell should not be visited");
        assert!(backtracked_cell.has_been_visited, "Backtracked cell should have been visited");
        assert_eq!(maze.cells.iter().filter(|c| c.is_visited).count(), 1, "Only start visited");

        // Test 3: Invalid Move
        let all_possible = maze.all_moves();
        let invalid_direction = all_possible.iter()
            .find(|d| !available_moves.contains(d))
            .expect("Should find an unavailable direction");
        let result = maze.make_move(*invalid_direction);
        assert!(result.is_err(), "Invalid move should fail");
        let active_after_invalid = maze.get_active_cell().unwrap();
        assert_eq!(active_after_invalid.coords, original_coords, "Active cell should not change");

        // Test 4: Boundary Move (move to edge)
        let mut maze_edge = maze.clone();
        if available_moves.contains(&Direction::LowerRight) {
            maze_edge.make_move(Direction::LowerRight).unwrap();
            let edge_cell = maze_edge.get_active_cell().unwrap();
            assert_eq!(edge_cell.coords, Coordinates { x: 2, y: 1 }, "Should move right to (2,1)");
            let edge_moves = edge_cell.open_walls.clone();
            if edge_moves.contains(&Direction::LowerLeft) {
                maze_edge.make_move(Direction::LowerLeft).unwrap();
                assert_eq!(maze_edge.get_active_cell().unwrap().coords, Coordinates { x: 2, y: 2 }, "Should move to bottom-right corner");
            }
        }
    }

    #[test]
    fn test_make_move_rhombille_recursive_backtracker() {
        run_make_move_rhombille_test("RecursiveBacktracker");
    }
    
    #[test]
    fn test_make_move_rhombille_aldous_broder() {
        run_make_move_rhombille_test("AldousBroder");
    }
    
    #[test]
    fn test_make_move_orthogonal_binary_tree() {
        run_make_move_orthogonal_test("BinaryTree");
    }
    
    #[test]
    fn test_make_move_orthogonal_sidewinder() {
        run_make_move_orthogonal_test("Sidewinder");
    }

    #[test]
    fn test_make_move_orthogonal_aldous_broder() {
        run_make_move_orthogonal_test("AldousBroder");
    }

    #[test]
    fn test_make_move_orthogonal_hunt_and_kill() {
        run_make_move_orthogonal_test("HuntAndKill");
    }

    #[test]
    fn test_make_move_orthogonal_recursive_backtracker() {
        run_make_move_orthogonal_test("RecursiveBacktracker");
    }
    
    #[test]
    fn test_make_move_orthogonal_wilsons() {
        run_make_move_orthogonal_test("Wilsons");
    }

    #[test]
    fn test_make_move_delta_aldous_broder() {
        run_make_move_delta_test("AldousBroder");
    }

    #[test]
    fn test_make_move_delta_hunt_and_kill() {
        run_make_move_delta_test("HuntAndKill");
    }
    
    #[test]
    fn test_make_move_delta_recursive_backtracker() {
        run_make_move_delta_test("RecursiveBacktracker");
    }

    #[test]
    fn test_make_move_delta_wilsons() {
        run_make_move_delta_test("Wilsons");
    }

    /// Manually linking two cells should produce a bidirectional link.
    #[test]
    fn test_manual_link_is_bidirectional() {
        let mut grid = Grid::new(
            MazeType::Orthogonal,
            3,
            3,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 2, y: 2 },
            false
        ).unwrap();

        let a = Coordinates { x: 0, y: 1 };
        let b = Coordinates { x: 1, y: 1 };

        grid.link(a, b).unwrap();

        let cell_a = grid.get(a).unwrap();
        let cell_b = grid.get(b).unwrap();

        assert!(
            cell_a.linked.contains(&b),
            "cell at {:?} should be linked to {:?}",
            a,
            b
        );
        assert!(
            cell_b.linked.contains(&a),
            "cell at {:?} should be linked to {:?}",
            b,
            a
        );
    }

    /// Any link created by the maze‐generation algorithm must also be bidirectional.
    #[test]
    fn test_generated_maze_links_are_bidirectional() {
        // Use a small perfect maze so we know it's fully linked
        let json = r#"
        {
            "maze_type": "Sigma",
            "width": 99,
            "height": 99,
            "algorithm": "RecursiveBacktracker",
            "start": { "x": 0, "y": 0 },
            "goal":  { "x": 98, "y": 98 }
        }
        "#;
        let maze = Grid::try_from(json).unwrap();

        for cell in &maze.cells {
            for &neighbor_coords in &cell.linked {
                let neighbor = maze.get(neighbor_coords).unwrap();
                assert!(
                    neighbor.linked.contains(&cell.coords),
                    "Link not mutual: {:?} → {:?} exists but not {:?} → {:?}",
                    cell.coords,
                    neighbor.coords,
                    neighbor.coords,
                    cell.coords
                );
            }
        }
    }


    // Helper function to check bidirectional links in a grid
    fn check_bidirectional_links(grid: &Grid, step_index: usize) {
        for cell in &grid.cells {
            for &neighbor_coords in &cell.linked {
                let neighbor = grid.get(neighbor_coords).unwrap();
                assert!(
                    neighbor.linked.contains(&cell.coords),
                    "Link from {:?} to {:?} is not bidirectional in step {}",
                    cell.coords,
                    neighbor_coords,
                    step_index
                );
            }
        }
    }

    #[test]
    fn test_hunt_and_kill_orthogonal_bidirectional_links_in_steps() {
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 19, y: 19 };
        let mut grid = Grid::new(MazeType::Orthogonal, 20, 20, start, goal, true).unwrap();
        
        HuntAndKill.generate(&mut grid).unwrap();
        
        assert!(grid.is_perfect_maze().unwrap(), "Generated maze should be perfect");
        let steps = grid.generation_steps.unwrap();
        assert!(!steps.is_empty(), "Expected some generation steps");
        
        for (i, step) in steps.iter().enumerate() {
            check_bidirectional_links(step, i);
        }
    }

    #[test]
    fn test_hunt_and_kill_delta_bidirectional_links_in_steps() {
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 19, y: 19 };
        let mut grid = Grid::new(MazeType::Delta, 20, 20, start, goal, true).unwrap();
        
        HuntAndKill.generate(&mut grid).unwrap();
        
        assert!(grid.is_perfect_maze().unwrap(), "Generated maze should be perfect");
        let steps = grid.generation_steps.unwrap();
        assert!(!steps.is_empty(), "Expected some generation steps");
        
        for (i, step) in steps.iter().enumerate() {
            check_bidirectional_links(step, i);
        }
    }

    #[test]
    fn test_hunt_and_kill_rhombille_bidirectional_links_in_steps() {
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 11, y: 11 };
        let mut grid = Grid::new(MazeType::Rhombille, 12, 12, start, goal, true).unwrap();
        
        HuntAndKill.generate(&mut grid).unwrap();
        
        assert!(grid.is_perfect_maze().unwrap(), "Generated maze should be perfect");
        let steps = grid.generation_steps.unwrap();
        assert!(!steps.is_empty(), "Expected some generation steps");
        
        for (i, step) in steps.iter().enumerate() {
            check_bidirectional_links(step, i);
        }
    }
}