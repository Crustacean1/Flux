pub struct ComponentManager<T> {
    components: Vec<T>,
}

impl<T> ComponentManager<T> {
    pub fn new() -> Self {
        ComponentManager { components: vec![] }
    }

    pub fn add_component(&mut self, entity_id: usize, component: T) -> usize {
        let component_id = self.components.len();
        self.components.push(component);
        component_id
    }

    pub fn components(&self) -> &[T] {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut [T] {
        &mut self.components
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.components.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.components.iter_mut()
    }

    pub fn remove_component(&mut self, entity_id: usize) {
        self.components.remove(entity_id);
    }
}
