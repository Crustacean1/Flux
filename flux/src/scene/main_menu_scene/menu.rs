use atlas::components::ComponentEntity;

pub struct Menu {
    entity_id: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl ComponentEntity for Menu {
    fn entity_id(&self) -> usize {
        self.entity_id
    }
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
