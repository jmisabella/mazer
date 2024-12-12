use crate::maze::cell::{ CellOrientation, MazeType, Cell, Coordinates };
use crate::maze::direction::{ Direction, SquareDirection, TriangleDirection, HexDirection, PolarDirection };
use rand::{ thread_rng, Rng };
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    maze_type: MazeType,
    cells: Vec<Vec<Cell>>,
    seed: u64,
    start_coords: Coordinates,
    goal_coords: Coordinates,
}
impl Grid {
    pub fn set(&mut self, cell: Cell) {
        if cell.x() >= self.width || cell.y() >= self.height {
            panic!("Cell's coordinates {:?} exceed grid dimensions {:?} by {:?}", cell.coords.to_string(), self.width, self.height);
        }
        self.cells[cell.y()][cell.x()] = cell.clone();
    }
    pub fn set_cells(&mut self, cells: Vec<Vec<Cell>>) {
        self.cells = cells;
    }
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }
    pub fn bounded_random_u64(&mut self, upper_bound: u64) -> u64 {
        let mut rng = thread_rng();
        let seed: u64 = rng.gen_range(0..upper_bound + 1);
        self.seed = seed;
        return seed;
    }
    pub fn generate_triangle_cells(&mut self) {
        if self.maze_type != MazeType::Delta {
            panic!("Cannot generate triangle cells for non-Delta maze_type {:?}", self.maze_type);
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
                self.cells[row].push(cell);
            }
        }
    }
    
    pub fn generate_non_triangle_cells(&mut self) {
        if self.maze_type == MazeType::Delta {
            panic!("Cannot generate non-triangle cells for maze_type {:?}", self.maze_type);
        }
        for row in 0..self.height {
            for col in 0..self.width {
                let coords = Coordinates { x: col, y: row };
                let is_start = coords == self.start_coords;
                let is_goal = coords == self.goal_coords;
                let cell: Cell = Cell::init(col, row, self.maze_type, is_start, is_goal);
                self.cells[row].push(cell);
            }
        }
    }

    pub fn new(maze_type: MazeType, width: usize, height: usize, start: Coordinates, goal: Coordinates) -> Self {
        let mut init_rng = thread_rng();
        let seed: u64 = init_rng.gen_range(0..(width * height + 1)) as u64;
        // let mut empty: Grid = Grid { width, height, maze_type, cells: Vec::new(), seed, start_coords: start, goal_coords: goal };
        let mut empty: Grid = Grid { width, height, maze_type, cells: vec![vec![Cell::new(0,0,maze_type); width]; height], seed, start_coords: start, goal_coords: goal };
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
                            neighbors.insert(SquareDirection::North.to_string(), grid.cells[(cell.y() - 1)][cell.x()].coords);
                        }
                        if cell.x() < grid.width - 1 {
                            neighbors.insert(SquareDirection::East.to_string(), grid.cells[cell.y()][(cell.x() + 1)].coords);
                        }
                        if cell.y() < grid.height - 1 {
                            neighbors.insert(SquareDirection::South.to_string(), grid.cells[(cell.y() + 1)][cell.x()].coords);
                        }
                        if cell.x() != 0 {
                            neighbors.insert(SquareDirection::West.to_string(), grid.cells[cell.y()][(cell.x() - 1)].coords);
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
                        if col > 0 && north_diagonal >= 0 && north_diagonal < height {
                            neighbors.insert(HexDirection::Northwest.to_string(), grid.cells[(col-1)][north_diagonal].coords);
                        }
                        if col >= 0 && col < width && row > 0 {
                            neighbors.insert(HexDirection::North.to_string(), grid.cells[col][(row-1)].coords);
                        }
                        if col < width - 1 && north_diagonal >= 0 && north_diagonal < height {
                            neighbors.insert(HexDirection::Northeast.to_string(), grid.cells[(col+1)][north_diagonal].coords);
                        }
                        if col > 0 && south_diagonal >= 0 && south_diagonal < height {
                            neighbors.insert(HexDirection::Southwest.to_string(), grid.cells[(col-1)][south_diagonal].coords);
                        }
                        if row < height - 1 && col >= 0 && col < width {
                            neighbors.insert(HexDirection::South.to_string(), grid.cells[col][(row+1)].coords);
                        }
                        if col < width - 1 && south_diagonal >= 0 && south_diagonal < height {
                            neighbors.insert(HexDirection::Southeast.to_string(), grid.cells[(col+1)][south_diagonal].coords);
                        }
                        cell.set_neighbors(neighbors);
                        grid.set(cell); 
                    }
                }
            }
        }
        return grid;
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


}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn init_orthogonal_grid() {
        let grid = Grid::new(MazeType::Orthogonal, 10, 10, Coordinates{x:0, y:0}, Coordinates{x:9, y:9});
        assert!(grid.cells.len() != 0);
    }
}