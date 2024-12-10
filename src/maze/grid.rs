use crate::maze::cell::CellOrientation;
use crate::maze::cell::MazeType;
use crate::maze::cell::Cell;
use crate::maze::cell::Coordinates;
use rand::rngs::{ ThreadRng, StdRng };
use rand::{ thread_rng, Rng, SeedableRng };

#[derive(Debug, Clone)]
pub struct Grid {
    width: u32,
    height: u32,
    maze_type: MazeType,
    cells: Vec<Vec<Cell>>,
    seed: u64,
    start_coords: Coordinates,
    goal_coords: Coordinates,
}
impl Grid {
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
                self.cells[row as usize].push(cell);
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
                self.cells[row as usize].push(cell);
            }
        }
    }

    pub fn new(maze_type: MazeType, height: u32, width: u32, start: Coordinates, goal: Coordinates) -> Self {
        let mut init_rng = thread_rng();
        let seed: u64 = init_rng.gen_range(0..(width * height + 1)) as u64;
        let mut empty: Grid = Grid { width, height, maze_type, cells: Vec::new(), seed, start_coords: start, goal_coords: goal };
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
        let mut cells: Vec<Vec<Cell>> = Vec::new();  

        //// TODO: set cells based on maze_type

        grid.set_cells(cells);
        return grid;
    }

    pub fn row(&self, y: u32) -> Vec<Cell> {
        // Ensure the index is within bounds
        if let Some(row) = self.cells.get(y as usize) {
            row.clone() // Clone the row to return a new Vec<Cell>
        } else {
            // Return an empty vector if the index is out of bounds
            Vec::new()
        }
    }

    pub fn column(&self, x: u32) -> Vec<Cell> {
        self.cells
            .iter()
            .filter_map(|row| row.get(x as usize).cloned()) // Extract and clone the cell if it exists
            .collect()
    }


}
