use atlas::{
    components::transform::Transform,
    entity_manager::{EntityManager, EntityManagerTrait},
    game_root::GameError,
    graphics::{
        lights::{Light, LightColor},
        material::TextureMaterial,
        mesh::Mesh,
        shaders::MeshShader,
    },
    resource_manager::{scene_resource_manager::SceneResourceManager, ResourceManager},
};
use glam::Vec3;

pub fn asteroids(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
) -> Result<(), GameError> {
    let meshes = ["spaceship3", "spaceship2", "spaceship1", "impostor"];

    let meshes: Vec<Mesh<MeshShader, TextureMaterial>> = meshes
        .iter()
        .map(|mesh| {
            return resource_manager.get(mesh).res;
        })
        .collect();

    meshes.iter().enumerate().for_each(|(i, mesh)| {
        let position = Vec3::new(0.0, 10.0 * i as f32, 0.0);
        let scale = Vec3::new(1., 1., 1.);
        let rotation = Vec3::new(0.0, 0.0, 0.0);

        entity_manager.add_entity((
            Transform {
                position,
                scale,
                rotation,
            },
            mesh.clone(),
        ));
    });

    entity_manager.add_entity((
        Transform::pos(Vec3::new(0.0, 0.0, 0.0)),
        Light::DirectionalLight(
            Vec3::new(0.0, 1.0, 0.0),
            LightColor {
                ambient: Vec3::new(0.01, 0.01, 0.01),
                diffuse: Vec3::new(0.02, 0.02, 0.02),
                specular: Vec3::new(0.6, 0.6, 0.6),
            },
        ),
    ));

    Ok(())
}
