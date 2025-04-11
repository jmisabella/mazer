use std::fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use rand::{ thread_rng, Rng };
use serde::ser::{ Serialize, Serializer, SerializeStruct };
use crate::cell::{CellOrientation, MazeType, Cell, CellBuilder, Coordinates};
use crate::direction::{SquareDirection, TriangleDirection, HexDirection, PolarDirection};
use crate::error::Error;
use crate::request::MazeRequest;

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub maze_type: MazeType,
    pub cells: Vec<Cell>,
    pub seed: u64,
    pub start_coords: Coordinates,
    pub goal_coords: Coordinates,
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
        match serde_json::to_string(&self) {
            Ok(json) => write!(f, "{}", json),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl TryFrom<MazeRequest> for Grid {
    type Error = crate::Error; // explicitly reference our custom Error type

    fn try_from(request: MazeRequest) -> Result<Self, Self::Error> {
        let mut grid = Grid::new(
            request.maze_type,
            request.width,
            request.height,
            request.start,
            request.goal,
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


impl Grid {

    // get x,y coordinate's index in the flattened 1D vector
    pub fn get_flattened_index(&self, x: usize, y: usize) -> usize {
        // when unflattened to become a 2D vector, cells are stored in row-major order 
        y * self.width + x
    }
    
    // retrieve a cell by its coordinates
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

    pub fn make_move(&mut self, direction: &str) -> Result<(), Error> {
        // Borrow active_cell mutably.
        let active_cell = self.get_active_cell()?;
        let original_coords = active_cell.coords;

        // Extract necessary data from active_cell while it's still borrowed.
        let open_walls = active_cell.open_walls.clone(); // if cloning is acceptable
        let attempted_move = direction.to_string();
        if !open_walls.contains(&attempted_move) {
            return Err(Error::MoveUnavailable { 
                attempted_move: attempted_move, 
                available_moves: open_walls
            });
        }
        
        // Extract the neighbor coordinate. Note: this borrows active_cell immutably.
        let neighbor_coords = *active_cell.neighbors_by_direction.get(direction)
            .ok_or(Error::InvalidDirection { direction: direction.to_string() })?;
        
        // drop active_cell explicitly to end its borrow
        let _ = active_cell;
        
        // mutably borrow the next_cell
        let next_cell = self.get_mut(neighbor_coords)?;
        next_cell.set_active(true);
        // set to visited (unless it's already been visited on current path and is being de-visited)
        // TODO: THIS LINE CAUSES RESULT TO NOT HAVE AN ACTIVE CELL 
        next_cell.set_visited(!next_cell.is_visited); // TODO: test/proofread this!
        let _ = next_cell;

        // re-obtain the previous cell now that we're no longer mutably borrowing next_cell 
        let previous_cell = self.get_mut(original_coords)?;
        previous_cell.set_active(false);
        
        Ok(())
    }
        

    // retrieve a cell by its coordinates
    pub fn get_by_coords(&self, x: usize, y: usize) -> Result<&Cell, Error> {
        self.get(Coordinates { x: x, y: y })
    }
    
    // retrieve a cell by its coordinates
    pub fn get_mut_by_coords(&mut self, x: usize, y: usize) -> Result<&mut Cell, Error> {
        self.get_mut(Coordinates { x: x, y: y })
    }

    pub fn set(&mut self, cell: Cell) -> Result<(), Error> {
        if cell.x() >= self.width || cell.y() >= self.height {
            return Err(Error::OutOfBoundsCoordinates { coordinates: cell.coords, maze_width: self.width, maze_height: self.height } );
        }
        let index = self.get_flattened_index(cell.x(), cell.y());
        self.cells[index] = cell;
        Ok(())
    }

    pub fn bounded_random_usize(&mut self, upper_bound: usize) -> usize {
        let mut rng = thread_rng();
        let seed= rng.gen_range(0..upper_bound + 1);
        self.seed = seed as u64;
        return seed;
    }

    pub fn random_bool(&mut self) -> bool {
        let rando: bool = self.bounded_random_usize(1000000) % 2 == 0;
        return rando;
    }
 
    // transform 1D (flattened) cells into a unflattened 2D vector
    pub fn unflatten(&self) -> Vec<Vec<Cell>> {
        self.cells
            .chunks(self.width) // split into row-sized slices
            .map(|chunk| chunk.to_vec()) // convert row slices to Vec<Cell>
            .collect()
    }

    pub fn generate_triangle_cells(&mut self) -> Result<(), Error> {
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
    
    pub fn generate_non_triangle_cells(&mut self) -> Result<(), Error> {
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
    

    pub fn new(maze_type: MazeType, width: usize, height: usize, start: Coordinates, goal: Coordinates) -> Result<Self, Error> {
        let mut init_rng = thread_rng();
        let seed: u64 = init_rng.gen_range(0..(width * height + 1)) as u64;

        let mut empty: Grid = Grid { 
            width, 
            height, 
            maze_type, 
            cells: vec![CellBuilder::new(0, 0, maze_type).build(); width * height],
            seed, 
            start_coords: 
            start, 
            goal_coords: 
            goal 
        };

        let mut grid: Grid = match maze_type {
            MazeType::Delta => {
                empty.generate_triangle_cells()?;
                empty.clone()
            }
            _ => {
                empty.generate_non_triangle_cells()?;
                empty.clone()
            }
        };

        match maze_type {
            //MazeType::Polar => {
            //    unimplemented!("MazeType Polar is not yet supported.");
            //}
            MazeType::Orthogonal => {
                for row in 0..height as usize {
                    for col in 0..width as usize {
                        let mut neighbors: HashMap<String, Coordinates> = HashMap::new();
                        let mut cell = grid.get_mut_by_coords(col, row)?.clone();
                        if cell.y() != 0 {
                            neighbors.insert(SquareDirection::North.to_string(), grid.get_by_coords(cell.x(), cell.y() - 1)?.coords);
                        }
                        if cell.x() < grid.width - 1 {
                            neighbors.insert(SquareDirection::East.to_string(), grid.get_by_coords(cell.x() + 1, cell.y())?.coords);
                        }
                        if cell.y() < grid.height - 1 {
                            neighbors.insert(SquareDirection::South.to_string(), grid.get_by_coords(cell.x(), cell.y() + 1)?.coords);
                        }
                        if cell.x() != 0 {
                            neighbors.insert(SquareDirection::West.to_string(), grid.get_by_coords(cell.x() - 1, cell.y())?.coords);
                        }
                        cell.set_neighbors(neighbors);
                        grid.set(cell)?; 
                    }
                }
            }
            MazeType::Delta => {
                for row in 0..height as usize {
                    for col in 0..width as usize {
                        let mut neighbors: HashMap<String, Coordinates> = HashMap::new();
                        let mut cell = grid.get_mut_by_coords(col, row)?.clone();
                        let mut left: Option<Coordinates> = if col > 0 { 
                            Some(Coordinates{x: col - 1, y: row})
                        } else { 
                            None
                        };
                        let mut right: Option<Coordinates> = if col < width - 1 { 
                            Some(Coordinates{x: col+1, y: row})
                        } else { 
                            None 
                        };
                        if left.is_some() {
                            let key = if cell.orientation == CellOrientation::Normal { TriangleDirection::UpperLeft.to_string() } else { TriangleDirection::LowerLeft.to_string() };
                            neighbors.insert(key, left.get_or_insert(Coordinates{x: 0, y: 0}).clone());
                        }
                        if right.is_some() {
                            let key = if cell.orientation == CellOrientation::Normal { TriangleDirection::UpperRight.to_string() } else { TriangleDirection::LowerRight.to_string() };
                            neighbors.insert(key, right.get_or_insert(Coordinates{x: 0, y: 0}).clone());
                        }
                        let mut up: Option<Coordinates> = if cell.orientation == CellOrientation::Inverted && row > 0 { 
                            Some(Coordinates{x: col, y: row-1}) 
                        } else { 
                            None 
                        };
                        let mut down: Option<Coordinates> = if cell.orientation == CellOrientation::Normal && row < height - 1 {
                            Some(Coordinates{x: col, y: row+1}) 
                        } else {
                            None
                        };
                        if up.is_some() {
                            neighbors.insert(TriangleDirection::Up.to_string(), up.get_or_insert(Coordinates{x: 0, y: 0}).clone());
                        }
                        if down.is_some() {
                            neighbors.insert(TriangleDirection::Down.to_string(), down.get_or_insert(Coordinates{x: 0, y: 0}).clone());
                        }
                        cell.set_neighbors(neighbors);
                        grid.set(cell)?;
                    }
                }
            }
            MazeType::Sigma => {
                for row in 0..height as usize {
                    for col in 0..width as usize {
                        let mut neighbors: HashMap<String, Coordinates> = HashMap::new();
                        let mut cell = grid.get_mut_by_coords(col, row)?.clone();
                        fn is_even(value: usize) -> bool {
                            return value % 2 == 0; 
                        }
                        let (north_diagonal, south_diagonal) = match is_even(col) {
                            true if row > 0 => (row - 1, row),
                            true => (0, row), // Prevent underflow by clamping to 0
                            false if row < height - 1 => (row, row + 1),
                            false => (row, height - 1), // Prevent out-of-bounds
                        };
                        if col > 0 && north_diagonal < height {
                            neighbors.insert(HexDirection::Northwest.to_string(), grid.get_by_coords(col-1, north_diagonal)?.coords);
                        }
                        if col < width && row > 0 {
                            neighbors.insert(HexDirection::North.to_string(), grid.get_by_coords(col, row-1)?.coords);
                        }
                        if col < width - 1 && north_diagonal < height {
                            neighbors.insert(HexDirection::Northeast.to_string(), grid.get_by_coords(col+1, north_diagonal)?.coords);
                        }
                        if col > 0 && south_diagonal < height {
                            neighbors.insert(HexDirection::Southwest.to_string(), grid.get_by_coords(col-1, south_diagonal)?.coords);
                        }
                        if row < height - 1 && col < width {
                            neighbors.insert(HexDirection::South.to_string(), grid.get_by_coords(col, row+1)?.coords);
                        }
                        if col < width - 1 && south_diagonal < height {
                            neighbors.insert(HexDirection::Southeast.to_string(), grid.get_by_coords(col+1, south_diagonal)?.coords);
                        }
                        cell.set_neighbors(neighbors);
                        grid.set(cell)?;
                    }
                }
            }
            MazeType::Polar => {
                for row in 0..height as usize {
                    for col in 0..width as usize {
                        let mut neighbors: HashMap<String, Coordinates> = HashMap::new();
                        let mut cell = grid.get_mut_by_coords(col, row)?.clone();
                        
                        // Calculate inward/outward neighbors
                        if row > 0 { // check inward (previous row)
                            let inward_neighbor = grid.get_by_coords(col, row - 1)?.coords;
                            neighbors.insert(PolarDirection::Inward.to_string(), inward_neighbor);
                        }
                        if row < height - 1 { // check outward (next row)
                            let outward_neighbor = grid.get_by_coords(col, row + 1)?.coords;
                            neighbors.insert(PolarDirection::Outward.to_string(), outward_neighbor);
                        }
                        
                        // Calculate clockwise/counter-clockwise neighbors
                        if col > 0 { // counter-clockwise (previous column)
                            let ccw_neighbor = grid.get_by_coords((col - 1) % width, row)?.coords;
                            neighbors.insert(PolarDirection::CounterClockwise.to_string(), ccw_neighbor);
                        }
                        if col < width - 1 { // clockwise (next column)
                            let cw_neighbor = grid.get_by_coords((col + 1) % width, row)?.coords;
                            neighbors.insert(PolarDirection::Clockwise.to_string(), cw_neighbor);
                        }
                        
                        // Set the neighbors for the cell
                        cell.set_neighbors(neighbors);
                        grid.set(cell)?;
                    }
                }
            }
        }
        Ok(grid)
    }
    
    pub fn link(&mut self, coord1: Coordinates, coord2: Coordinates) -> Result<(), Error> {
        let (row1, col1) = (coord1.y, coord1.x);
        let (row2, col2) = (coord2.y, coord2.x);
    
        // Collect indices, defer mutable access
        let idx1 = (row1, col1);
        let idx2 = (row2, col2);
    
        // Sequential mutable access
        {
            let cell1 = self.get_mut_by_coords(idx1.1, idx1.0)?;
            cell1.linked.insert(coord2);
        }
        {
            let cell2 = self.get_mut_by_coords(idx2.1, idx2.0)?;
            cell2.linked.insert(coord1);
        }
        Ok(())
    }


    pub fn distances(&self, start_coords: Coordinates) -> HashMap<Coordinates, u32> {
        let mut distances = HashMap::new(); // Map to store distances from `start_coords`
        let mut queue = VecDeque::new(); // Queue for BFS

        // Initialize the BFS with the starting point
        distances.insert(start_coords, 0);
        queue.push_back(start_coords);

        while let Some(current) = queue.pop_front() {
            let current_distance = distances[&current];

            // Get the cell at the current coordinate
            if let Ok(cell) = self.get(current) {
                // Iterate over all linked neighbors
                // Collect neighbors first to avoid borrowing conflicts with `distances`
                cell.linked.iter()
                    .filter(|&&neighbor| !distances.contains_key(&neighbor))
                    .copied() // Convert &&Coordinates to Coordinates
                    .collect::<Vec<_>>() // Collect to break borrowing dependency
                    .into_iter() // Iterate over the owned values
                    .for_each(|neighbor| {
                        distances.insert(neighbor, current_distance + 1);
                        queue.push_back(neighbor);
                    });
            }
        }
        distances
    }

    pub fn get_path_to(
        &self,
        start_x: usize,
        start_y: usize,
        goal_x: usize,
        goal_y: usize,
    ) -> Result<HashMap<Coordinates, u32>, Error> {
        // Calculate distances from the start
        let dist = self.distances(Coordinates { x: start_x, y: start_y });

        // Initialize the breadcrumbs map
        let mut breadcrumbs = HashMap::new();
        let mut current = Coordinates { x: goal_x, y: goal_y };

        // Add the goal cell to breadcrumbs
        if let Some(&distance) = dist.get(&current) {
            breadcrumbs.insert(current, distance);
        } else {
            // Return empty if the goal is unreachable
            return Ok(breadcrumbs);
        }

        // Trace the path back to the start
        while current != (Coordinates { x: start_x, y: start_y }) {
            let cell = self
                .get(current)?;
           
            current = cell.linked.iter()
                .filter_map(|&neighbor| {
                    let neighbor_dist = dist.get(&neighbor)?;
                    let current_dist = dist.get(&current)?;
                    if neighbor_dist < current_dist {
                        breadcrumbs.insert(neighbor, *neighbor_dist);
                        Some(neighbor)
                    } else {
                        None // skip this neighbor because distance to it exceeds distance to current
                    }
                })
                .next()
                .ok_or(Error::NoValidNeighbor { coordinates: cell.coords })?;

        }
        Ok(breadcrumbs)
    }

    /// Returns all cells reachable from the given start coordinates
    pub fn all_connected_cells(&self, start: &Coordinates) -> Result<HashSet<Coordinates>, Error> {
        let mut connected = HashSet::new();
        let mut frontier = VecDeque::new();
        frontier.push_back(*start);
        connected.insert(*start);

        while let Some(current) = frontier.pop_front() {
            let cell = &self.get_by_coords(current.x, current.y)?;
            // collect new coordinates that have not been visited yet
            let new_linked_coords: Vec<Coordinates> = cell.linked
                .iter()
                .filter(|xy| !connected.contains(xy))
                .copied()
                .collect();

            new_linked_coords
                // take ownership because `insert` and `push_back` both require owned values, not references 
                .into_iter()
                .for_each(|xy| {
                    connected.insert(xy);
                    frontier.push_back(xy);
                });
        }
        Ok(connected)
    }

    /// Counts the number of edges in the maze
    pub fn count_edges(&self) -> usize {
        self.cells
            .iter()
            .map(|cell| cell.linked.len()) // For each cell, get the number of linked cells
            .sum::<usize>() // Sum the number of edges
        / 2 // Each edge is stored twice (once for each linked cell)
    }

    /// Checks if the maze is perfect
    pub fn is_perfect_maze(&self) -> Result<bool, Error> {
        // Total number of cells
        let total_cells = self.width * self.height;

        // Fully connected check
        let start_coords = self.start_coords;
        let connected_cells = self.all_connected_cells(&start_coords)?;
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
                let east_boundary = match cell.neighbors_by_direction.get(&SquareDirection::East.to_string()).is_some() {
                    true if cell.is_linked_direction(SquareDirection::East) => " ",
                    _ => "|",
                };
                top.push_str(body);
                top.push_str(east_boundary);
                let south_boundary = match cell.neighbors_by_direction.get(&SquareDirection::South.to_string()).is_some() {
                    true if cell.is_linked_direction(SquareDirection::South) => "   ",
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

    use crate::behaviors::collections::SetDifference;

    #[test]
    fn init_orthogonal_grid() {
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates{x:0, y:0}, Coordinates{x:3, y:3}) {
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
    fn determine_distances_to_goal() {
        match Grid::new(
            MazeType::Orthogonal,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
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
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }) {
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
        match Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 }) {
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
    fn test_make_move() {
        let json = r#"
        {
            "maze_type": "Orthogonal",
            "width": 12,
            "height": 12,
            "algorithm": "AldousBroder",
            "start": { "x": 0, "y": 0 },
            "goal": { "x": 11, "y": 11 }
        }
        "#;
        match Grid::try_from(json) {
            Ok(mut maze) => {
                assert!(maze.is_perfect_maze().unwrap());
                println!("\n\nMaze:\n\n{}\n\n", maze.to_asci());
                
                assert_eq!(
                    maze.cells.iter().filter(|cell| cell.is_visited).count(),
                    1,
                    "There should be 1 visited cell on dynamic path at the beginning"
                );
                
                assert_eq!(
                    maze.cells.iter().filter(|cell| cell.has_been_visited).count(),
                    1,
                    "There should be 1 visited cell on permenant path at the beginning"
                );
                
                // Limit the borrow's scope and return only owned data.
                let (original_coords, available_moves, unavailable_moves) = {
                    if let Ok(active_cell) = maze.get_active_cell() {
                        // Clone the data so we own it.
                        let original_coords = active_cell.coords.clone();
                        // Clone available moves to a Vec<String>.
                        let available_moves: Vec<String> = active_cell.open_walls.clone();
                        // Create a vector of &str from the owned Strings,
                        // which we then use to compute diff.
                        let available_refs: Vec<&str> = available_moves.iter().map(|s| s.as_str()).collect();
                        // Use your diff helper to get the unavailable moves.
                        // Then convert those to owned Strings so that they don't borrow available_moves.
                        //let unavailable_moves: Vec<String> = diff(&["North", "East", "South", "West"], &available_refs)
                        let unavailable_moves: Vec<String> = ["North", "East", "South", "West"].diff(&available_refs)
                            .into_iter()
                            .map(|s| s.to_string())
                            .collect();

                        (original_coords, available_moves, unavailable_moves)
                    } else {
                        panic!("Expected an active cell at the start");
                    }
                }; // All borrows are dropped here.

                // Now it's safe to perform mutable operations.

                // Try a move that is unavailable using a copied maze.
                let mut copied_maze = maze.clone();
                assert!(
                    copied_maze
                        .make_move(unavailable_moves.iter().next().unwrap())
                        .is_err(),
                    "Should not allow an unavailable move"
                );
                
                assert_eq!(
                    copied_maze.cells.iter().filter(|cell| cell.is_visited).count(),
                    1,
                    "There should be 1 visited cell on dynamic path before a successful move is made"
                );
                
                assert_eq!(
                    copied_maze.cells.iter().filter(|cell| cell.has_been_visited).count(),
                    1,
                    "There should be 1 visited cell on permenant path before a successful move is made"
                );
                
                // Try a valid move on the original maze.
                assert!(
                    maze
                        .make_move(available_moves.iter().next().unwrap().as_str())
                        .is_ok(),
                    "Should allow a valid move"
                );
                

                // Verify that exactly one cell is active.
                assert_eq!(
                    maze.cells.iter().filter(|cell| cell.is_active).count(),
                    1,
                    "There should be exactly one active cell"
                );

                assert_eq!(
                    maze.cells.iter().filter(|cell| cell.is_visited).count(),
                    2,
                    "There should be 2 visited cells on dynamic path after first successful move (start cell and current)"
                );
                
                assert_eq!(
                    maze.cells.iter().filter(|cell| cell.has_been_visited).count(),
                    2,
                    "There should be 2 visited cells on permenant path after first successful move (start cell and current)"
                );


                // Verify that the active cell has changed.
                let new_active_coords = maze
                    .get_active_cell()
                    .expect("Expected an active cell after the move")
                    .coords
                    .clone();
                assert_ne!(
                    new_active_coords, original_coords,
                    "The active cell should have moved to a new coordinate"
                );
            }
            Err(e) => panic!("Unexpected error constructing maze: {:?}", e),
        }
    }

}