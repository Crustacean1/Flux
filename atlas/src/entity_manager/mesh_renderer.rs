use crate::{
    components::transform::Transform,
    graphics::{
        material::phong_material::PhongMaterial, mesh::Mesh, shaders::mesh_shader::MeshShader,
    },
};

use super::{EntityManager, EntityManagerTrait};

impl EntityManagerTrait<(Transform, Mesh<MeshShader, PhongMaterial>)> for EntityManager {
    fn add_entity(
        &mut self,
        (transform, mesh_renderer): (Transform, Mesh<MeshShader, PhongMaterial>),
    ) -> usize {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;

        self.meshes.0.push(entity_id);
        self.meshes.1.push(transform);
        self.meshes.2.push(mesh_renderer);

        entity_id
    }

    fn remove_entity(&mut self, _entity: usize) {
        todo!()
    }
}
