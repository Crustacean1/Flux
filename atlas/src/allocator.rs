use crate::components::{ComponentEntity, ComponentType};

enum AllocationUnit<T> {
    Full(T),
    Empty(usize),
}

pub trait Createable<T> {
    fn new(entity_id: usize) -> T;
}

pub struct ComponentAllocator<T: ComponentEntity + ComponentType> {
    components: Vec<T>,
}

impl<T: ComponentEntity + ComponentType> ComponentAllocator<T> {
    pub fn new() -> Self {
        ComponentAllocator::<T> { components: vec![] }
    }

    pub fn add_component(&mut self, entity_id: usize, value: T) -> usize {
        self.components.push(value);
        self.components.len() - 1
    }

    pub fn get_component(&self, i: usize) -> Option<&T> {
        self.components.get(i)
    }

    pub fn get_component_mut(&mut self, i: usize) -> Option<&mut T> {
        self.components.get_mut(i)
    }

    pub fn remove_component(&mut self, i: usize) -> Option<()> {
        todo!()
    }

    fn reallocate(&mut self) {}
}

pub trait SelfSufficient<T> {
    fn new() -> T;
}

pub trait GenericComponentAllocator<T: ComponentType> {
    fn add_component(&mut self, entity_id: usize, value: T) -> usize;
    fn remove_component(&mut self, entity_id: usize);
    fn get_component(&mut self, entity_id: usize) -> &mut T;
}
