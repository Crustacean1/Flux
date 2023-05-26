use crate::components::{mesh_renderer::MeshRenderer, transform::Transform};

use super::{ComponentIterator, EntityManager, EntityManagerTrait};

impl EntityManagerTrait<(Transform, MeshRenderer)> for EntityManager {
    fn add_entity(&mut self, (transform, mesh_renderer): (Transform, MeshRenderer)) -> usize {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;

        self.meshes.0.push(entity_id);
        self.meshes.1.push(transform);
        self.meshes.2.push(mesh_renderer);

        ComponentIterator::<((usize, *const Transform), *const MeshRenderer)>::reload(self);

        entity_id
    }

    fn remove_entity(&mut self, entity: usize) {
        todo!()
    }
}
