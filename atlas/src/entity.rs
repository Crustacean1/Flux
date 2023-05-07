use crate::{
    allocator::{Createable, GenericComponentAllocator},
    components::{ComponentEntity, ComponentType},
};

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

    pub fn add_component<S, T: ComponentType + ComponentEntity + Createable<T>>(
        &mut self,
        allocator: &mut impl GenericComponentAllocator<T>,
        value: T,
    ) {
        panic!()
        /*(let index = allocator.add_component(self.id, value);
        self.components.push(ComponentRecord {
            component_type: T::component_type_id(),
            component_id: index,
        });*/
    }

    pub fn remove_component<T: ComponentType + ComponentEntity>(
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

    pub fn get_component<'a, 'b, T: ComponentType + ComponentEntity>(
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
