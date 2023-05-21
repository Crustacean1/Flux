use crate::components::{
    button_handler::ButtonHandler, button_trigger::ButtonTrigger, shape_renderer::ShapeRenderer,
    transform::Transform,
};

use super::{EntityManager, EntityManagerTrait};

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
        let entity_ref = self.menu_buttons.0.len();
        self.next_entity_id += 1;

        self.menu_buttons.0.push(entity_id);
        self.menu_buttons.1.push(transform);
        self.menu_buttons.2.push(renderer);
        self.menu_buttons.3.push(trigger);
        self.menu_buttons.4.push(handler);

        self.transform_shape_iter.push((
            entity_id,
            &self.menu_buttons.1[entity_ref],
            &self.menu_buttons.2[entity_ref],
        ));

        self.trigger_handler_iter.push((
            entity_id,
            &self.menu_buttons.3[entity_ref],
            self.menu_buttons.4[entity_ref].as_ref(),
        ));

        entity_id
    }

    fn remove_entity(&mut self, entity: usize) {
        if let Some(index) = self
            .transform_shape_iter
            .iter()
            .position(|(entity_id, ..)| *entity_id == entity)
        {
            self.transform_shape_iter.remove(entity);
        }

        if let Some(index) = self
            .trigger_handler_iter
            .iter()
            .position(|(entity_id, ..)| *entity_id == entity)
        {
            self.trigger_handler_iter.remove(entity);
        }

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
        }
    }
}
