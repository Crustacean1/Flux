use atlas::{
    components::{mesh_renderer::MeshRenderer, transform::Transform},
    entity_manager::{ComponentIterator, EntityManager, EntityManagerTrait},
    game_root::GameError,
    graphics::{
        material::TextureMaterial,
        mesh::Primitive,
        vertices::base_vertices::{TriangleIndex, Vertex3PT},
    },
    resource_manager::{
        resource::Resource, scene_resource_manager::SceneResourceManager, ResourceManager,
    },
};
use glam::Vec3;
use rand::Rng;

pub fn asteroids(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
) -> Result<(), GameError> {
    let asteroid_tex: Resource<TextureMaterial> = resource_manager.get("asteroid1")?;

    let meshes: [&str; 0] = [];

    let meshes: Vec<_> = meshes
        .iter()
        .map(|mesh| -> Primitive<Vertex3PT, TriangleIndex> {
            return resource_manager.get(mesh).unwrap().res;
        })
        .collect();

    meshes.iter().for_each(|mesh| {
        let position = Vec3::new(0.0, 0.0, 0.0);
        let scale = Vec3::new(1.0, 1.0, 1.0);
        let rotation = Vec3::new(0.0, 0.0, 0.0);

        entity_manager.add_entity((
            Transform {
                position,
                scale,
                rotation,
            },
            MeshRenderer {
                mesh: mesh.clone(),
                material: asteroid_tex.res.clone(),
            },
        ));
    });

    Ok(())
}
