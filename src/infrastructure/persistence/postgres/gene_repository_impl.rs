use crate::domain::entities::gene::Gene;
use crate::domain::repositories::gene_repository::GeneRepository;
use postgres::{Client, NoTls};

pub struct GeneRepositoryImpl {
    client: Client,
}

impl GeneRepositoryImpl {
    pub fn new() -> Self {
        let client = Client::connect("host=localhost user=postgres password=secret dbname=bioengineer_db", NoTls).unwrap();
        Self { client }
    }
}

impl GeneRepository for GeneRepositoryImpl {
    fn save_gene(&self, gene: &Gene) -> Result<(), String> {
        self.client.execute(
            "INSERT INTO genes (id, sequence) VALUES ($1, $2)",
            &[&gene.id, &gene.sequence],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn find_gene_by_id(&self, id: i32) -> Option<Gene> {
        let row = self.client.query_opt(
            "SELECT id, sequence FROM genes WHERE id = $1",
            &[&id],
        ).ok()??;
        Some(Gene::new(row.get(0), row.get(1)))
    }

    fn find_all_genes(&self) -> Vec<Gene> {
        let rows = self.client.query("SELECT id, sequence FROM genes", &[]).unwrap();
        rows.into_iter().map(|row| Gene::new(row.get(0), row.get(1))).collect()
    }

    fn delete_gene(&self, id: i32) -> Result<(), String> {
        self.client.execute("DELETE FROM genes WHERE id = $1", &[&id]).map_err(|e| e.to_string())?;
        Ok(())
    }
}
