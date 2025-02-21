use crate::cell::{ CellOrientation, MazeType, Cell, Coordinates };
use crate::direction::{ Direction, SquareDirection, TriangleDirection, HexDirection };
use crate::error::Error;
use crate::request::MazeRequest;

use std::fmt;
use serde::ser::{ Serialize, Serializer, SerializeStruct };
use rand::{ thread_rng, Rng };
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub maze_type: MazeType,
    pub cells: Vec<Vec<Cell>>,
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

impl Grid {

    // retrieve a cell by its coordinates
    pub fn get(&self, x: usize, y: usize) -> &Cell {
        return &self.cells[y][x];
    }
    
    // retrieve a cell as an option by its coordinates
    pub fn get_cell(&self, coords: Coordinates) -> Option<&Cell> {
        self.cells
            .get(coords.y as usize)
            .and_then(|row| row.get(coords.x as usize))
    }

    pub fn set(&mut self, cell: Cell) -> Result<(), Error> {
        if cell.x() >= self.width || cell.y() >= self.height {
            return Err(Error::OutOfBoundsCoordinates { coordinates: cell.coords, maze_width: self.width, maze_height: self.height } );
        }
        self.cells[cell.y()][cell.x()] = cell.clone();
        Ok(())
    }

    pub fn set_cells(&mut self, cells: Vec<Vec<Cell>>) {
        self.cells = cells;
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    // pub fn width(&self) -> usize {
    //     return self.width;
    // }
    // pub fn height(&self) -> usize {
    //     return self.height;
    // }
    // pub fn maze_type(&self) -> MazeType {
    //     return self.maze_type;
    // }
    // pub fn cells(&self) -> &Vec<Vec<Cell>> {
    //     return &self.cells;
    // }
    // pub fn start_coords(&self) -> Coordinates {
    //     return self.start_coords;
    // }
    // pub fn goal_coords(&self) -> Coordinates {
    //     return self.goal_coords;
    // }
    
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

    pub fn flatten(&self) -> Vec<Cell>
    where
        Cell: Clone,
    {
        self.cells.iter().flat_map(|row| row.clone()).collect()
    }

    pub fn unflatten(&mut self, flattened: Vec<Cell>) -> Result<(), Error> {
        if flattened.len() != (self.width * self.height) {
            return Err(Error::FlattenedVectorDimensionsMismatch {
                vector_size: flattened.len(),
                maze_width: self.width,
                maze_height: self.height,
            });
        }

        // Use `chunks` to divide the flattened vector into rows
        self.cells = flattened
            .chunks(self.width)
            .map(|chunk| chunk.to_vec())
            .collect();
    
        Ok(())
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
                let mut cell: Cell = Cell::init(col, row, self.maze_type, is_start, is_goal);
                cell.set_orientation(triangle_orientation(upright));
                self.cells[row][col] = cell;
            }
        }
        Ok(())
    }
    
    pub fn generate_non_triangle_cells(&mut self) -> Result<(), Error> {
        if self.maze_type == MazeType::Delta {
            return Err(Error::InvalidCellForDeltaMaze { cell_maze_type: self.maze_type } );
        }
        for row in 0..self.height {
            for col in 0..self.width {
                let coords = Coordinates { x: col, y: row };
                let is_start = coords == self.start_coords;
                let is_goal = coords == self.goal_coords;
                let cell: Cell = Cell::init(col, row, self.maze_type, is_start, is_goal);
                self.cells[row][col] = cell;
            }
        }
        Ok(())
    }

    pub fn new(maze_type: MazeType, width: usize, height: usize, start: Coordinates, goal: Coordinates) -> Self {
        let mut init_rng = thread_rng();
        let seed: u64 = init_rng.gen_range(0..(width * height + 1)) as u64;

        let mut empty: Grid = Grid { 
            width, 
            height, 
            maze_type, 
            cells: vec![vec![Cell::new(0,0,maze_type); width]; height], 
            seed, 
            start_coords: 
            start, 
            goal_coords: 
            goal 
        };

        let mut grid: Grid = match maze_type {
            MazeType::Delta => {
                empty.generate_triangle_cells();
                empty.clone()
            }
            _ => {
                empty.generate_non_triangle_cells();
                empty.clone()
            }
        };

        match maze_type {
            MazeType::Polar => {
                unimplemented!("MazeType Polar is not yet supported.");
            }
            MazeType::Orthogonal => {
                for row in 0..height as usize {
                    for col in 0..width as usize {
                        let mut neighbors: HashMap<String, Coordinates> = HashMap::new();
                        let mut cell = grid.cells[row][col].clone();
                        if cell.y() != 0 {
                            neighbors.insert(SquareDirection::North.to_string(), grid.cells[cell.y() - 1][cell.x()].coords);
                        }
                        if cell.x() < grid.width - 1 {
                            neighbors.insert(SquareDirection::East.to_string(), grid.cells[cell.y()][cell.x() + 1].coords);
                        }
                        if cell.y() < grid.height - 1 {
                            neighbors.insert(SquareDirection::South.to_string(), grid.cells[cell.y() + 1][cell.x()].coords);
                        }
                        if cell.x() != 0 {
                            neighbors.insert(SquareDirection::West.to_string(), grid.cells[cell.y()][cell.x() - 1].coords);
                        }
                        cell.set_neighbors(neighbors);
                        grid.set(cell); 
                    }
                }
            }
            MazeType::Delta => {
                for row in 0..height as usize {
                    for col in 0..width as usize {
                        let mut neighbors: HashMap<String, Coordinates> = HashMap::new();
                        let mut cell = grid.cells[row][col].clone();
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
                        grid.set(cell); 
                    }
                }
            }
            MazeType::Sigma => {
                for row in 0..height as usize {
                    for col in 0..width as usize {
                        let mut neighbors: HashMap<String, Coordinates> = HashMap::new();
                        let mut cell = grid.cells[row][col].clone();
                        fn is_even(value: usize) -> bool {
                            return value % 2 == 0; 
                        }
                        let (north_diagonal, south_diagonal) = match is_even(col) {
                            true => (row - 1, row),
                            false => (row, row + 1)
                        };
                        if col > 0 && north_diagonal < height {
                            neighbors.insert(HexDirection::Northwest.to_string(), grid.cells[col-1][north_diagonal].coords);
                        }
                        if col < width && row > 0 {
                            neighbors.insert(HexDirection::North.to_string(), grid.cells[col][row-1].coords);
                        }
                        if col < width - 1 && north_diagonal < height {
                            neighbors.insert(HexDirection::Northeast.to_string(), grid.cells[col+1][north_diagonal].coords);
                        }
                        if col > 0 && south_diagonal < height {
                            neighbors.insert(HexDirection::Southwest.to_string(), grid.cells[col-1][south_diagonal].coords);
                        }
                        if row < height - 1 && col < width {
                            neighbors.insert(HexDirection::South.to_string(), grid.cells[col][row+1].coords);
                        }
                        if col < width - 1 && south_diagonal < height {
                            neighbors.insert(HexDirection::Southeast.to_string(), grid.cells[col+1][south_diagonal].coords);
                        }
                        cell.set_neighbors(neighbors);
                        grid.set(cell); 
                    }
                }
            }
        }
        return grid;
    }

    pub fn from_request(request: MazeRequest) -> Grid {
        let mut grid = Grid::new(request.maze_type, request.width, request.height,request.start, request.goal);
        request.algorithm.generate(&mut grid);
        return grid;
    }

    // TODO:test
    pub fn from_json(json: &str) -> Grid {
    // pub fn from_json(json: &str) -> Result<Grid, Error> {
        return Grid::from_request(serde_json::from_str(json).expect(&format!("Failed to deserialize MazeRequest from json {}", json)));
        // let deserialized: MazeRequest = serde_json::from_str(json)?;
        // Ok(Grid::from_request(deserialized)) 
    }

    pub fn row(&self, y: usize) -> Vec<Cell> {
        // Ensure the index is within bounds
        if let Some(row) = self.cells.get(y) {
            row.clone() // Clone the row to return a new Vec<Cell>
        } else {
            // Return an empty vector if the index is out of bounds
            Vec::new()
        }
    }

    pub fn column(&self, x: usize) -> Vec<Cell> {
        self.cells
            .iter()
            .filter_map(|row| row.get(x).cloned()) // Extract and clone the cell if it exists
            .collect()
    }

    pub fn link(&mut self, coord1: Coordinates, coord2: Coordinates) {
        let (row1, col1) = (coord1.y, coord1.x);
        let (row2, col2) = (coord2.y, coord2.x);
    
        // Collect indices, defer mutable access
        let idx1 = (row1, col1);
        let idx2 = (row2, col2);
    
        // Sequential mutable access
        {
            let cell1 = &mut self.cells[idx1.0][idx1.1];
            cell1.linked.insert(coord2);
        }
        {
            let cell2 = &mut self.cells[idx2.0][idx2.1];
            cell2.linked.insert(coord1);
        }
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
            if let Some(cell) = self.get_cell(current) {
                // Iterate over all linked neighbors
                for neighbor in &cell.linked {
                    if !distances.contains_key(neighbor) {
                        // Update distance and enqueue the neighbor
                        distances.insert(*neighbor, current_distance + 1);
                        queue.push_back(*neighbor);
                    }
                }
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
    ) -> HashMap<Coordinates, u32> {
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
            return breadcrumbs;
        }

        // Trace the path back to the start
        while current != (Coordinates { x: start_x, y: start_y }) {
            if let Some(cell) = self.get_cell(current) {
                for &neighbor in &cell.linked {
                    if dist.get(&neighbor).unwrap_or(&u32::MAX) < dist.get(&current).unwrap_or(&u32::MAX) {
                        breadcrumbs.insert(neighbor, dist[&neighbor]);
                        current = neighbor;
                        break;
                    }
                }
            }
        }

        breadcrumbs
    }

    /// Returns all cells reachable from the given start coordinates
    pub fn all_connected_cells(&self, start: &Coordinates) -> HashSet<Coordinates> {
        let mut connected = HashSet::new();
        let mut frontier = VecDeque::new();
        frontier.push_back(*start);
        connected.insert(*start);

        while let Some(current) = frontier.pop_front() {
            let cell = &self.cells[current.y][current.x];
            for neighbor_coords in &cell.linked {
                if !connected.contains(neighbor_coords) {
                    connected.insert(*neighbor_coords);
                    frontier.push_back(*neighbor_coords);
                }
            }
        }
        connected
    }

    /// Counts the number of edges in the maze
    pub fn count_edges(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .map(|cell| cell.linked.len())
            .sum::<usize>()
            / 2 // Each edge is stored twice
    }

    /// Checks if the maze is perfect
    pub fn is_perfect_maze(&self) -> bool {
        // Total number of cells
        let total_cells = self.cells.len() * self.cells[0].len();

        // Fully connected check
        let start_coords = self.start_coords;
        let connected_cells = self.all_connected_cells(&start_coords);
        if connected_cells.len() != total_cells {
            return false;
        }

        // Tree check (no cycles)
        let total_edges = self.count_edges();
        total_edges == total_cells - 1
    }

    /// ASCI display, only applicable to Orthogonal (square cell) mazes
    pub fn to_asci(&self) -> String {
        assert!(self.maze_type == MazeType::Orthogonal, "Rejecting displaying ASCI for MazeType {}! ASCI display behavior is only applicable to the Orthogonal MazeType", self.maze_type.to_string());
        let mut output = format!("+{}\n", "---+".repeat(self.width)); 
        for row in &self.cells {
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

    #[test]
    fn init_orthogonal_grid() {
        let grid = Grid::new(MazeType::Orthogonal, 4, 4, Coordinates{x:0, y:0}, Coordinates{x:3, y:3});
        assert!(grid.cells.len() != 0);
        assert!(grid.cells.len() == 4);
        assert!(grid.cells[0].len() == 4);
        println!("\n\n{}", grid.to_string());
        println!("\n\n{}\n\n", grid.to_asci());
    }

    #[test]
    fn link_cells_in_orthogonal_grid() {
        let mut grid = Grid::new(
            MazeType::Orthogonal,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
        );
        let cell1 = grid.get(0, 0).coords;
        let cell2 = grid.get(0, 1).coords;
        let cell3 = grid.get(1, 1).coords;
        let cell4 = grid.get(1, 2).coords;
        let cell5 = grid.get(2, 2).coords;
        let cell6 = grid.get(2, 3).coords;
        let cell7 = grid.get(3, 3).coords;


        grid.link(cell1, cell2);
        grid.link(cell2, cell3);
        grid.link(cell3, cell4);
        grid.link(cell4, cell5);
        grid.link(cell5, cell6);
        grid.link(cell6, cell7);
        // many cells are walled-off and unreachable, not a perfect maze 
        assert!(!grid.is_perfect_maze());

        println!("\n\n{}\n\n", grid.to_asci());

    }

    #[test]
    fn determine_distances_to_goal() {
        let mut grid = Grid::new(
            MazeType::Orthogonal,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
        );
        let cell1 = grid.get(0, 0).coords;
        let cell2 = grid.get(0, 1).coords;
        let cell3 = grid.get(1, 1).coords;
        let cell4 = grid.get(1, 2).coords;
        let cell5 = grid.get(2, 2).coords;
        let cell6 = grid.get(2, 3).coords;
        let cell7 = grid.get(3, 3).coords;
        
        grid.link(cell1, cell2);
        grid.link(cell2, cell3);
        grid.link(cell3, cell4);
        grid.link(cell4, cell5);
        grid.link(cell5, cell6);
        grid.link(cell6, cell7);

        let distances = grid.distances(Coordinates{ x: 0, y: 0} );

        for (coords, distance) in &distances {
            println!("Distance to {:?}: {}", coords, distance);
        }


    }

    #[test]
    fn test_flatten_and_unflatten() {
        // pub fn new(maze_type: MazeType, width: usize, height: usize, start: Coordinates, goal: Coordinates) -> Self {
        
        let mut grid = Grid::new(
            MazeType::Orthogonal,
            4,
            4,
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 3, y: 3 },
        );

        let initial_cells = grid.cells.clone();

        // Flatten the grid
        let flattened = grid.flatten();

        // Unflatten the grid
        grid.unflatten(flattened);

        // Check that the cells after unflattening match the original
        assert_eq!(grid.cells, initial_cells);
    }

    #[test]
    fn test_perfect_maze_detection() {
        let mut grid = Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 });
        assert!(!grid.is_perfect_maze());
        grid.link(grid.get(0, 0).coords, grid.get(1, 0).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(1, 0).coords, grid.get(2, 0).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(2, 0).coords, grid.get(3, 0).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(3, 0).coords, grid.get(3, 1).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(3, 1).coords, grid.get(2, 1).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(2,1).coords, grid.get(1, 1).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(1,1).coords, grid.get(0, 1).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(0,1).coords, grid.get(0, 2).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(0,2).coords, grid.get(1, 2).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(1,2).coords, grid.get(2, 2).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(2,2).coords, grid.get(3, 2).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(3,2).coords, grid.get(3, 3).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(3,3).coords, grid.get(2, 3).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(2,3).coords, grid.get(1, 3).coords);
        assert!(!grid.is_perfect_maze()); // not perfect
        grid.link(grid.get(1,3).coords, grid.get(0, 3).coords);
        // now it's a perfect maze: only a single path exists for any 2 cells in the maze and there are no unreachable groups of cells
        assert!(grid.is_perfect_maze());
        grid.link(grid.get(0,3).coords, grid.get(0, 2).coords);
        // now it's no longer a perfect maze because some cells can reach each other on multiple paths 
        assert!(!grid.is_perfect_maze());
    }

    #[test]
    fn test_get_path_to() {
        let mut grid = Grid::new(MazeType::Orthogonal, 4, 4, Coordinates { x: 0, y: 0 }, Coordinates { x: 3, y: 3 });
        grid.link(Coordinates { x: 0, y: 0 }, Coordinates { x: 0, y: 1 });
        grid.link(Coordinates { x: 0, y: 1 }, Coordinates { x: 1, y: 1 });
        grid.link(Coordinates { x: 1, y: 1 }, Coordinates { x: 1, y: 2 });
        grid.link(Coordinates { x: 1, y: 2 }, Coordinates { x: 2, y: 2 });

        let path = grid.get_path_to(0, 0, 2, 2);

        assert_eq!(path.len(), 5);
        assert!(path.contains_key(&Coordinates { x: 0, y: 0 }));
        assert!(path.contains_key(&Coordinates { x: 2, y: 2 }));
        assert_eq!(path[&Coordinates { x: 0, y: 0 }], 0);
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
        let maze = Grid::from_json(json);
        assert!(maze.is_perfect_maze());
        println!("\n\nRecursive Backtracker\n\n{}\n\n", maze.to_asci());

    }

}