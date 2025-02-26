pub struct Gene {
    pub id: i32,
    pub sequence: String,
}

impl Gene {
    pub fn new(id: i32, sequence: String) -> Self {
        Self { id, sequence }
    }
}
