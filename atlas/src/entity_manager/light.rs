use crate::{components::transform::Transform, graphics::lights::Light};

use super::{EntityManager, EntityManagerTrait};

impl EntityManagerTrait<(Transform, Light)> for EntityManager {
    fn add_entity(&mut self, (transform, light): (Transform, Light)) -> usize {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;

        self.lights.0.push(entity_id);
        self.lights.1.push(transform);
        self.lights.2.push(light);

        entity_id
    }

    fn remove_entity(&mut self, entity: usize) {
        todo!()
    }
}
