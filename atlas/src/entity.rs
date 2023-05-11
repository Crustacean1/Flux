use crate::{allocator::GenericComponentAllocator, components::ComponentType};

struct ComponentRecord {
    component_type: usize,
    component_id: usize,
}

pub struct Entity {
    id: usize,
    components: Vec<ComponentRecord>,
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Entity {
            id,
            components: vec![],
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn add_component<S, T: ComponentType>(
        &mut self,
        allocator: &mut impl GenericComponentAllocator<T>,
        value: T,
    ) {
        allocator.add_component(self.id, value);
    }

    pub fn remove_component<T: ComponentType>(
        &mut self,
        allocator: &mut impl GenericComponentAllocator<T>,
    ) -> Option<()> {
        if let Some(component) = self
            .components
            .iter()
            .find(|component| component.component_type == T::component_type_id())
        {
            //allocator.remove_component::<T>(component.component_id);
            Some(())
        } else {
            None
        }
    }

    pub fn get_component<'a, 'b, T: ComponentType>(
        &'a self,
        allocator: &'b mut impl GenericComponentAllocator<T>,
    ) -> Option<&'b mut T> {
        if let Some(component) = self
            .components
            .iter()
            .find(|component| component.component_type == T::component_type_id())
        {
            //Some(allocator.get_component::<T>(component.component_id))
            None
        } else {
            None
        }
    }
}

impl Drop for Entity {
    fn drop(&mut self) {
        todo!()
    }
}
