mod component_manager;
mod menu_button;

use crate::components::{
    button_handler::ButtonHandler, button_trigger::ButtonTrigger, shape_renderer::ShapeRenderer,
    transform::Transform,
};

use self::component_manager::ComponentManager;

pub enum ComponentEvent<T> {
    AddComponent(T),
    RemoveComponent(usize),
}

pub trait ComponentIterator<'a, T> {
    fn iter(&'a self) -> Box<(dyn Iterator<Item = T> + 'a)>;
}

pub trait EntityManagerTrait<T> {
    fn add_entity(&mut self, entity: T) -> usize;
    fn remove_entity(&mut self, entity: usize);
}

pub struct EntityManager {
    menu_buttons: (
        Vec<usize>,
        ComponentManager<Transform>,
        ComponentManager<ShapeRenderer>,
        ComponentManager<ButtonTrigger>,
        ComponentManager<Box<dyn ButtonHandler>>,
    ),

    next_entity_id: usize,
}

impl EntityManager {
    pub fn new() -> Self {
        let menu_buttons = (
            vec![],
            ComponentManager::<Transform>::new(),
            ComponentManager::<ShapeRenderer>::new(),
            ComponentManager::<ButtonTrigger>::new(),
            ComponentManager::<Box<dyn ButtonHandler>>::new(),
        );
        EntityManager {
            menu_buttons,
            next_entity_id: 0,
        }
    }
}

impl<'a> ComponentIterator<'a, (&'a Transform, &'a ShapeRenderer)> for EntityManager {
    fn iter(&'a self) -> Box<(dyn Iterator<Item = (&'a Transform, &'a ShapeRenderer)> + 'a)> {
        let transforms = self.menu_buttons.1.iter();
        let shapes = self.menu_buttons.2.iter();
        Box::new(transforms.zip(shapes))
    }
}

pub type ButtonIterator<'a> = (&'a ButtonTrigger, &'a Box<dyn ButtonHandler>);

impl<'a> ComponentIterator<'a, ButtonIterator<'a>> for EntityManager {
    fn iter(&'a self) -> Box<(dyn Iterator<Item = ButtonIterator> + 'a)> {
        let button_triggers = self.menu_buttons.3.iter();
        let button_handlers = self.menu_buttons.4.iter();
        Box::new(button_triggers.zip(button_handlers))
    }
}
