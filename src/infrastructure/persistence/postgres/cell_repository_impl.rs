use crate::domain::entities::cell::Cell;
use crate::domain::repositories::cell_repository::CellRepository;
use postgres::{Client, NoTls};

pub struct CellRepositoryImpl {
    client: Client,
}

impl CellRepositoryImpl {
    pub fn new() -> Self {
        let client = Client::connect("host=localhost user=postgres password=secret dbname=bioengineer_db", NoTls).unwrap();
        Self { client }
    }
}

impl CellRepository for CellRepositoryImpl {
    fn save_cell(&self, cell: &Cell) -> Result<(), String> {
        self.client.execute(
            "INSERT INTO cells (id, genes) VALUES ($1, $2)",
            &[&cell.id, &cell.genes],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn find_cell_by_id(&self, id: i32) -> Option<Cell> {
        let row = self.client.query_opt(
            "SELECT id, genes FROM cells WHERE id = $1",
            &[&id],
        ).ok()??;
        Some(Cell::new(row.get(0), row.get(1)))
    }

    fn find_all_cells(&self) -> Vec<Cell> {
        let rows = self.client.query("SELECT id, genes FROM cells", &[]).unwrap();
        rows.into_iter().map(|row| Cell::new(row.get(0), row.get(1))).collect()
    }

    fn delete_cell(&self, id: i32) -> Result<(), String> {
        self.client.execute("DELETE FROM cells WHERE id = $1", &[&id]).map_err(|e| e.to_string())?;
        Ok(())
    }
}
