use crate::{Grid, Error};
use crate::cell::Coordinates;

use std::collections::HashSet;

pub trait MazeGeneration {
    /// Carve a maze on the provided grid.
    fn generate(&self, grid: &mut Grid) -> Result<(), Error>;

    /// Finalize the maze (set distances, mark solution, etc.).
    fn finalize(&self, grid: &mut Grid) -> Result<(), Error> {
        let start = grid.start_coords;
        let goal = grid.goal_coords;
    
        let all_distances = grid.distances(start);
        for (coords, distance) in all_distances {
            if let Ok(cell) = grid.get_mut(coords) {
                cell.distance = distance as i32;
            }
        }
    
        if let Ok(path) = grid.get_path_to(start.x, start.y, goal.x, goal.y) {
            for coords in path.keys() {
                if let Ok(cell) = grid.get_mut(*coords) {
                    cell.on_solution_path = true;
                }
            }
        }
   
        for cell_option in grid.cells.iter_mut() {
            if let Some(cell) = cell_option {
                cell.set_open_walls();
            }
        }
    
        let active_count = grid.cells.iter().filter(|cell| cell.as_ref().map_or(false, |c| c.is_visited)).count();
        if active_count > 1 {
            Err(Error::MultipleActiveCells { count: active_count })
        } else if active_count == 0 {
            Err(Error::NoActiveCells)
        } else {
            Ok(())
        }
    }

    // Capture a step with minimal overhead
    fn capture_step(&self, grid: &mut Grid, changed_cells: &HashSet<Coordinates>) {
        if grid.capture_steps {
            // Update open_walls only for changed cells
            for coord in changed_cells {
                if let Ok(cell) = grid.get_mut(*coord) {
                    cell.set_open_walls();
                }
            }
            // Clone the grid minimally for storage
            let mut grid_clone = grid.clone();
            grid_clone.capture_steps = false;
            grid_clone.generation_steps = None; // Prevent recursive cloning
            grid.generation_steps.as_mut().unwrap().push(grid_clone);
        }
    }

    fn build<'a>(&self, grid: &'a mut Grid) -> Result<&'a Grid, Error> {
        self.generate(grid)?;
        self.finalize(grid)?;
        Ok(grid)
    }

}
