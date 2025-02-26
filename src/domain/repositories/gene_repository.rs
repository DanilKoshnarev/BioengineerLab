use crate::domain::entities::gene::Gene;

pub trait GeneRepository {
    fn save_gene(&self, gene: &Gene) -> Result<(), String>;
    fn find_gene_by_id(&self, id: i32) -> Option<Gene>;
    fn find_all_genes(&self) -> Vec<Gene>;
    fn delete_gene(&self, id: i32) -> Result<(), String>;
}
