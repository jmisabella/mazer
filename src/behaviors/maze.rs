use crate::{Grid, Error};

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
    
        for cell in grid.cells.iter_mut() {
            cell.set_open_walls();
        }
    
        let active_count = grid.cells.iter().filter(|cell| cell.is_visited).count();
        if active_count > 1 {
            Err(Error::MultipleActiveCells { count: active_count })
        } else if active_count == 0 {
            Err(Error::NoActiveCells)
        } else {
            Ok(())
        }
    }
    
    fn build<'a>(&self, grid: &'a mut Grid) -> Result<&'a Grid, Error> {
        self.generate(grid)?;
        self.finalize(grid)?;
        Ok(grid)
    }

}
