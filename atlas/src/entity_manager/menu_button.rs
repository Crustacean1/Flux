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
        self.next_entity_id += 1;

        self.menu_buttons.0.push(entity_id);
        self.menu_buttons.1.add_component(entity_id, transform);
        self.menu_buttons.2.add_component(entity_id, renderer);
        self.menu_buttons.3.add_component(entity_id, trigger);
        self.menu_buttons.4.add_component(entity_id, handler);

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
            self.menu_buttons.1.remove_component(index);
            self.menu_buttons.2.remove_component(index);
            self.menu_buttons.3.remove_component(index);
            self.menu_buttons.4.remove_component(index);
        }
    }
}
