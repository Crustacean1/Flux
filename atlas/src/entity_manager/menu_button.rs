use crate::components::{
    button_handler::ButtonHandler, button_trigger::ButtonTrigger, shape_renderer::ShapeRenderer,
    transform::Transform,
};

use super::{ComponentIterator, EntityManager, EntityManagerTrait};

impl
    EntityManagerTrait<(
        Transform,
        ShapeRenderer,
        ButtonTrigger,
        Box<dyn ButtonHandler>,
    )> for EntityManager
{
    fn add_entity(
        &mut self,
        (transform, renderer, trigger, handler): (
            Transform,
            ShapeRenderer,
            ButtonTrigger,
            Box<dyn ButtonHandler>,
        ),
    ) -> usize {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;

        self.menu_buttons.0.push(entity_id);
        self.menu_buttons.1.push(transform);
        self.menu_buttons.2.push(renderer);
        self.menu_buttons.3.push(trigger);
        self.menu_buttons.4.push(handler);

        ComponentIterator::<((usize, *const Transform), *const ShapeRenderer)>::reload(self);
        ComponentIterator::<((usize, *const ButtonTrigger), *const dyn ButtonHandler)>::reload(
            self,
        );

        entity_id
    }

    fn remove_entity(&mut self, entity: usize) {
        if let Some(index) = self
            .menu_buttons
            .0
            .iter()
            .position(|&entity_id| entity == entity_id)
        {
            self.menu_buttons.0.remove(index);
            self.menu_buttons.1.remove(index);
            self.menu_buttons.2.remove(index);
            self.menu_buttons.3.remove(index);
            self.menu_buttons.4.remove(index);

            ComponentIterator::<((usize, *const Transform), *const ShapeRenderer)>::reload(self);
            ComponentIterator::<((usize, *const ButtonTrigger), *const dyn ButtonHandler)>::reload(
                self,
            );
        }
    }
}
