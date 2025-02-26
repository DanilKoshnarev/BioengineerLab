use crate::domain::entities::gene::Gene;
use crate::domain::repositories::gene_repository::GeneRepository;

pub struct ManageGenes<R: GeneRepository> {
    repository: R,
}

impl<R: GeneRepository> ManageGenes<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create_gene(&self, id: i32, sequence: String) -> Result<(), String> {
        let gene = Gene::new(id, sequence);
        self.repository.save_gene(&gene)
    }

    pub fn view_gene(&self, id: i32) -> Option<Gene> {
        self.repository.find_gene_by_id(id)
    }

    pub fn view_all_genes(&self) -> Vec<Gene> {
        self.repository.find_all_genes()
    }

    pub fn remove_gene(&self, id: i32) -> Result<(), String> {
        self.repository.delete_gene(id)
    }
}
