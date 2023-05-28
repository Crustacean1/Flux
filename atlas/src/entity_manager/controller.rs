use crate::components::{camera::Camera, controller::Controller};

use super::{ComponentIteratorGenerator, EntityManager, EntityManagerTrait};

impl EntityManagerTrait<(Box<dyn Controller>, Camera)> for EntityManager {
    fn add_entity(&mut self, (controller, camera): (Box<dyn Controller>, Camera)) -> usize {
        let entity_id = self.next_entity_id;
        let entity_ref = self.cameras.0.len();
        self.next_entity_id += 1;

        self.cameras.0.push(entity_id);
        self.cameras.1.push(controller);
        self.cameras.2.push(camera);

        entity_id
    }

    fn remove_entity(&mut self, entity: usize) {
        todo!();
    }
}

