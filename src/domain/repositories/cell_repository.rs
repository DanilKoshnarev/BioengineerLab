use crate::domain::entities::cell::Cell;

pub trait CellRepository {
    fn save_cell(&self, cell: &Cell) -> Result<(), String>;
    fn find_cell_by_id(&self, id: i32) -> Option<Cell>;
    fn find_all_cells(&self) -> Vec<Cell>;
    fn delete_cell(&self, id: i32) -> Result<(), String>;
}
