use crate::domain::entities::cell::Cell;
use crate::domain::repositories::cell_repository::CellRepository;

pub struct SimulateCells<R: CellRepository> {
    repository: R,
}

impl<R: CellRepository> SimulateCells<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn simulate(&self) -> Vec<Cell> {
        self.repository.find_all_cells()
    }
}
