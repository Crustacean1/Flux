use std::rc::Rc;

use crate::{graphics::graphics_context::GraphicsContext, logger::Logger};

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

/*pub struct Stage {
    entities: Vec<Entity>,
    allocators: Vec<*mut std::ffi::c_void>,
    entity_count: usize,
}

impl Stage {
    pub fn new() -> Self {
        let allocator: *mut std::ffi::c_void = unsafe {
            let allocator = Box::new(ComponentAllocator::<ShapeRenderer>::new());
            mem::transmute(Box::into_raw(allocator))
        };
        Stage {
            entities: vec![],
            allocators: vec![allocator],
            entity_count: 0,
        }
    }

    pub fn add_entity(&mut self) -> usize {
        self.entity_count += 1;
        self.entities.push(Entity::new(self.entity_count));
        self.entity_count
    }

    pub fn remove_entity(&mut self, id: usize) {
        if let Some(entity_to_remove) = self.entities.iter().position(|entity| entity.id() == id) {
            self.entities.remove(entity_to_remove);
        }
    }

    pub fn add_component<T: ComponentType + ComponentEntity + Createable<T>>(
        &mut self,
        entity_id: usize,
    ) -> (usize, &mut T) {
        let allocator: Box<ComponentAllocator<T>> =
            unsafe { mem::transmute(self.allocators[T::component_id()]) };
        let component_id = allocator.add_component(entity_id);
        let component = allocator.get_component_mut(component_id).unwrap();
        Box::into_raw(allocator);
        (component_id, component)
    }

    pub fn remove_component<T: ComponentType + ComponentEntity + Createable<T>>(
        &mut self,
        component_id: usize,
    ) {
        todo!()
    }

    pub fn get_component_mut<T: ComponentType + ComponentEntity + Createable<T>>(
        &mut self,
        component_id: usize,
    ) {
        todo!()
    }
}*/

/*impl Scene {
    pub fn load(path: &str, logger: Rc<dyn Logger>) -> Result<Scene, ResourceError> {
        Ok(Scene {
            logger,
            action: SceneAction::None,
        })
    }

    pub fn run(&mut self, graphics_context: &mut GraphicsContext) -> SceneAction {
        loop {
            self.handle_user_events(graphics_context);
        }
    }

    fn handle_user_events(&mut self, graphics_context: &mut GraphicsContext) {
        graphics_context.get_events().for_each(|event| match event {
            UserEvent::Other => self.logger.log(LogMsg::Info("Unrecognized event")),
            UserEvent::Close => self.action = SceneAction::Exit,
            _ => {}
        });
    }
}*/
