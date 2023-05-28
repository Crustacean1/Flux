use crate::components::skybox_renderer::SkyboxRenderer;

use super::{EntityManager, EntityManagerTrait};

impl EntityManagerTrait<SkyboxRenderer> for EntityManager {
    fn add_entity(&mut self, renderer: SkyboxRenderer) -> usize {
        let entity_id = self.next_entity_id;
        let entity_ref = self.skyboxes.0.len();

        self.next_entity_id += 1;
        self.skyboxes.0.push(entity_id);
        self.skyboxes.1.push(renderer);

        entity_id
    }

    fn remove_entity(&mut self, entity: usize) {
        if let Some(index) = self
            .skyboxes
            .0
            .iter()
            .position(|&entity_id| entity_id == entity)
        {
            self.skyboxes.0.remove(index);
            self.skyboxes.1.remove(index);
        }
    }
}

