use std::rc::Rc;

use crate::{entity::Entity, graphics::graphics_context::GraphicsContext, logger::Logger};

#[derive(Clone, Copy)]
pub enum SceneAction {
    NewScene(&'static str),
    RestartScene,
    Exit,
}

pub trait Scene {
    fn run(
        &mut self,
        logger: Rc<dyn Logger>,
        graphics_context: &mut GraphicsContext,
    ) -> SceneAction;
}

pub struct Stage {
    entities: Vec<Entity>,
    entity_count: usize,
}

impl Stage {
    pub fn new() -> Self {
        Stage {
            entities: vec![],
            entity_count: 0,
        }
    }

    pub fn add_entity(&mut self) -> &mut Entity {
        self.entity_count += 1;
        self.entities.push(Entity::new(self.entity_count));
        self.entities.last_mut().unwrap()
    }

    pub fn remove_entity(&mut self, id: usize) {
        if let Some(entity_to_remove) = self.entities.iter().position(|entity| entity.id() == id) {
            self.entities.remove(entity_to_remove);
        }
    }

    pub fn get_entity_mut(&mut self, id: usize) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id() == id)
    }
}
