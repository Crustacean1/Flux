use atlas::{
    components::{
        skybox_renderer::SkyboxRenderer, text_renderer::TextRenderer, transform::Transform,
    },
    entity_manager::{EntityManager, EntityManagerTrait},
    game_root::GameError,
    graphics::{
        lights::{Light, LightColor},
        material::{phong_material::PhongMaterial, skybox_material::SkyboxMaterial},
        mesh::Mesh,
        shaders::mesh_shader::MeshShader,
    },
    resource_manager::{font::Font, scene_resource_manager::SceneResourceManager, ResourceManager},
};
use glam::Vec3;

pub fn asteroids(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
) -> Result<(), GameError> {
    let font: Font = resource_manager.get("main").res;

    let skybox_material: SkyboxMaterial = resource_manager.get("space1").res;
    let skybox = SkyboxRenderer::new(1.0, skybox_material);

    let meshes = ["spaceship1", "spaceship2", "spaceship3"];

    let meshes: Vec<Mesh<MeshShader, PhongMaterial>> = meshes
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
            Vec3::new(10.0, -100.0, 0.0),
            LightColor {
                ambient: Vec3::new(0.01, 0.01, 0.01),
                diffuse: Vec3::new(0.01, 0.01, 0.01),
                specular: Vec3::new(0.3, 0.3, 0.3),
            },
        ),
    ));

    entity_manager.add_entity((
        Transform {
            position: Vec3::new(500., 500., 0.),
            scale: Vec3::new(1., 1., 1.),
            rotation: Vec3::new(0.0, 0.0, 0.0),
        },
        TextRenderer::new("ABCDEFGHIJKLMNOPRSTUWQ123456", font),
    ));

    entity_manager.add_entity(skybox);

    Ok(())
}
