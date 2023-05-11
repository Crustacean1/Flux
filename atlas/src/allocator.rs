use super::components::Component;
use crate::components::ComponentType;

enum AllocationUnit<T> {
    Full(T),
    Empty(usize),
}

pub trait Createable<T> {
    fn new(entity_id: usize) -> T;
}

pub struct ComponentAllocator<T: ComponentType> {
    components: Vec<Component<T>>,
}

impl<T: ComponentType> ComponentAllocator<T> {
    pub fn new() -> Self {
        ComponentAllocator::<T> { components: vec![] }
    }

    pub fn add_component(&mut self, entity_id: usize, value: T) -> usize {
        self.components.push(Component::<T>::new(entity_id, value));
        self.components.len() - 1
    }

    pub fn get_component(&self, i: usize) -> Option<&Component<T>> {
        self.components.get(i)
    }

    pub fn get_component_mut(&mut self, i: usize) -> Option<&mut Component<T>> {
        self.components.get_mut(i)
    }

    pub fn remove_component(&mut self, i: usize) -> Option<()> {
        todo!()
    }

    pub fn components(&mut self) -> impl Iterator {

    }

    fn reallocate(&mut self) {}
}

pub trait GenericComponentAllocator<T: ComponentType> {
    fn add_component(&mut self, entity_id: usize, value: T) -> usize;
    fn remove_component(&mut self, entity_id: usize);
    fn get_component_mut(&mut self, entity_id: usize) -> Option<&mut Component<T>>;
}
