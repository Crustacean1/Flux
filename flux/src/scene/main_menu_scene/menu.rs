pub struct Menu {
    entity_id: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Menu {
    pub fn new(id: usize) -> Self {
        Menu {
            entity_id: id,
            parent: None,
            children: vec![],
        }
    }
}
