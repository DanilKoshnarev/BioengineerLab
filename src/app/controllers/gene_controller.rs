use crate::domain::use_cases::manage_genes::ManageGenes;
use crate::infrastructure::persistence::postgres::gene_repository_impl::GeneRepositoryImpl;
use crate::domain::entities::gene::Gene;

pub fn create_gene_handler(id: i32, sequence: String) {
    let repo = GeneRepositoryImpl::new();
    let use_case = ManageGenes::new(repo);
    use_case.create_gene(id, sequence).unwrap();
}

pub fn view_gene_handler(id: i32) {
    let repo = GeneRepositoryImpl::new();
    let use_case = ManageGenes::new(repo);
    let gene = use_case.view_gene(id);
    println!("{:?}", gene);
}
