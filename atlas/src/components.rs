pub trait ComponentEntity {
    fn entity_id(&self) -> usize;
}

pub trait ComponentType {
    fn component_type_id() -> usize;
}
