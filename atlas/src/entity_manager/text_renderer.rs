use crate::components::{text_renderer::TextRenderer, transform::Transform};

use super::{EntityManager, EntityManagerTrait};

impl EntityManagerTrait<(Transform, TextRenderer)> for EntityManager {
    fn add_entity(&mut self, (transform, light): (Transform, TextRenderer)) -> usize {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;

        self.text_renderers.0.push(entity_id);
        self.text_renderers.1.push(transform);
        self.text_renderers.2.push(light);

        entity_id
    }

    fn remove_entity(&mut self, entity: usize) {
        todo!()
    }
}
