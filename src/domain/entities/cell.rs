pub struct Cell {
    pub id: i32,
    pub genes: Vec<Gene>,
}

impl Cell {
    pub fn new(id: i32, genes: Vec<Gene>) -> Self {
        Self { id, genes }
    }
}
