use crate::{entity::Entity, scene::Stage};

pub trait ComponentType {
    fn component_type_id() -> usize;
}

pub struct Component<T: ComponentType> {
    entity_id: usize,
    pub component: T,
}

impl<T: ComponentType> Component<T> {
    pub fn new(entity_id: usize, component: T) -> Self {
        Component::<T> {
            entity_id,
            component,
        }
    }

    pub fn get_parent<'a, 'b>(&'a self, stage: &'b mut Stage) -> &'b mut Entity {
        stage
            .get_entity_mut(self.entity_id)
            .expect("All components should have a parent")
    }
}
